use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU16, Ordering},
        Arc,
    },
};

use klaver::Vm;
use klaver_wintercg::console::{Console, ConsoleWriter};
use rquickjs_util::{
    quick::{CatchResultExt, Class, Module},
    Val,
};
use serde::{Deserialize, Serialize};
use tauri::{async_runtime::Mutex, AppHandle, Emitter, Runtime};

use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct VmId(u16);

impl From<VmId> for vaerdi::Value {
    fn from(value: VmId) -> Self {
        vaerdi::Value::Number(value.0.into())
    }
}

fn next_id() -> VmId {
    static COUNTER: AtomicU16 = AtomicU16::new(1);
    VmId(COUNTER.fetch_add(1, Ordering::Release))
}

#[derive(Clone)]
pub struct ManagedVm {
    vm: Arc<Vm>,
    path: PathBuf,
}

#[derive(Default)]
pub struct KlaverState {
    vms: Mutex<HashMap<VmId, ManagedVm>>,
    #[cfg(feature = "transform")]
    compiler: klaver::modules::transformer::Compiler,
}

impl KlaverState {
    pub async fn open<R: Runtime>(&self, app: AppHandle<R>, path: &Path) -> Result<VmId, Error> {
        let mut vms = self.vms.lock().await;
        if let Some(found) = vms.iter().find(|(_, v)| &v.path == path).map(|m| *m.0) {
            Ok(found)
        } else {
            let vm = Vm::new().search_path(path).build().await?;

            let id = next_id();

            vm.with(|ctx| {
                let ret = ctx.globals().get::<_, Class<Console>>("console")?;
                ret.borrow_mut().set_writer(Printer { app, id })?;
                Ok(())
            })
            .await?;

            vms.insert(
                id,
                ManagedVm {
                    vm: vm.into(),
                    path: path.to_path_buf(),
                },
            );

            Ok(id)
        }
    }

    pub async fn close(&self, vm: VmId) -> Result<(), Error> {
        let _ = self.vms.lock().await.remove(&vm);

        Ok(())
    }

    pub async fn exec(&self, vm: VmId, code: String) -> Result<vaerdi::Value, Error> {
        let Some(vm) = self.vms.lock().await.get(&vm).map(|m| m.clone()) else {
            panic!("no vm")
        };

        #[cfg(feature = "transform")]
        let code = {
            let ret = self
                .compiler
                .compile(&code, "main")
                .map_err(|err| klaver::RuntimeError::Custom(Box::new(err)))?;

            ret.code
        };

        let ret = klaver::async_with!(vm.vm => |ctx| {
            Ok(ctx.eval_promise(code).catch(&ctx)?.into_future::<Val>().await.catch(&ctx)?)
        })
        .await?;

        Ok(ret.0)
    }

    pub async fn eval_module(&self, vm: VmId, name: String, code: String) -> Result<(), Error> {
        let Some(vm) = self.vms.lock().await.get(&vm).map(|m| m.clone()) else {
            panic!("no vm")
        };

        #[cfg(feature = "transform")]
        let code = {
            let ret = self
                .compiler
                .compile(&code, &name)
                .map_err(|err| klaver::RuntimeError::Custom(Box::new(err)))?;

            ret.code
        };

        klaver::async_with!(vm.vm => |ctx| {
            Module::evaluate(ctx.clone(), name, code).catch(&ctx)?.into_future::<Val>().await.catch(&ctx)?;
            Ok(())
        })
        .await?;

        Ok(())
    }

    pub async fn typings(&self, vm: VmId) -> Result<HashMap<String, String>, Error> {
        let Some(vm) = self.vms.lock().await.get(&vm).map(|m| m.clone()) else {
            panic!("no vm")
        };

        let types = vm
            .vm
            .env()
            .typings()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<HashMap<_, _>>();

        Ok(types)
    }
}

pub struct Printer<R: Runtime> {
    app: AppHandle<R>,
    id: VmId,
}

impl<R: Runtime> ConsoleWriter for Printer<R> {
    fn write(&self, level: klaver_wintercg::console::Level, message: String) {
        let v = vaerdi::value!({
            "level": level.to_string(),
            "message": message,
            "vm": self.id,
        });
        self.app.emit("klaver://console", v).ok();
    }
}
