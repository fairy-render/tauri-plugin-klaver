use std::collections::HashMap;
use std::path::Path;

use tauri::{command, AppHandle, Runtime};

use crate::state::VmId;
use crate::KlaverExt;
use crate::Result;

#[command]
pub(crate) async fn vm_open<R: Runtime>(app: AppHandle<R>, path: String) -> Result<VmId> {
    app.klaver().state.open(app.clone(), Path::new(&path)).await
}

#[command]
pub(crate) async fn vm_close<R: Runtime>(app: AppHandle<R>, vm: VmId) -> Result<()> {
    app.klaver().state.close(vm).await
}

#[command]
pub(crate) async fn vm_exec<R: Runtime>(
    app: AppHandle<R>,
    vm: VmId,
    code: String,
) -> Result<vaerdi::Value> {
    app.klaver().state.exec(vm, code).await
}

#[command]
pub(crate) async fn vm_eval_module<R: Runtime>(
    app: AppHandle<R>,
    vm: VmId,
    name: String,
    code: String,
) -> Result<()> {
    app.klaver().state.eval_module(vm, name, code).await
}

#[command]
pub(crate) async fn vm_typings<R: Runtime>(
    app: AppHandle<R>,
    vm: VmId,
) -> Result<HashMap<String, String>> {
    app.klaver().state.typings(vm).await
}
