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

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Hello;

use commands::*;
use tauri_specta::{collect_commands, ts};

#[macro_export]
macro_rules! specta_builder {
    () => {
        ts::builder().commands(collect_commands![ping])
    };
}

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the hello APIs.
pub trait HelloExt<R: Runtime> {
    fn hello(&self) -> &Hello<R>;
}

impl<R: Runtime, T: Manager<R>> crate::HelloExt<R> for T {
    fn hello(&self) -> &Hello<R> {
        self.state::<Hello<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    let invoke_handler = specta_builder!().build_plugin_utils(PLUGIN_NAME).unwrap();

    Builder::new("hello")
        .invoke_handler(invoke_handler)
        .setup(|app, api| {
            #[cfg(mobile)]
            let hello = mobile::init(app, api)?;
            #[cfg(desktop)]
            let hello = desktop::init(app, api)?;
            app.manage(hello);
            Ok(())
        })
        .build()
}
