use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;
mod state;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Klaver;
#[cfg(mobile)]
use mobile::Klaver;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the klaver APIs.
pub trait KlaverExt<R: Runtime> {
    fn klaver(&self) -> &Klaver<R>;
}

impl<R: Runtime, T: Manager<R>> crate::KlaverExt<R> for T {
    fn klaver(&self) -> &Klaver<R> {
        self.state::<Klaver<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("klaver")
        .invoke_handler(tauri::generate_handler![
            commands::vm_open,
            commands::vm_close,
            commands::vm_exec,
            commands::vm_eval_module,
            commands::vm_typings
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let klaver = mobile::init(app, api)?;
            #[cfg(desktop)]
            let klaver = desktop::init(app, api)?;
            app.manage(klaver);
            Ok(())
        })
        .build()
}
