use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;
pub use commands::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::WebAuth;
#[cfg(mobile)]
use mobile::WebAuth;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the web-auth APIs.
pub trait WebAuthExt<R: Runtime> {
  fn web_auth(&self) -> &WebAuth<R>;
}

impl<R: Runtime, T: Manager<R>> crate::WebAuthExt<R> for T {
  fn web_auth(&self) -> &WebAuth<R> {
    self.state::<WebAuth<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("web-auth")
    .invoke_handler(tauri::generate_handler![commands::authenticate])
    .setup(|app, api| {
      #[cfg(mobile)]
      let web_auth = mobile::init(app, api)?;
      #[cfg(desktop)]
      let web_auth = desktop::init(app, api)?;
      app.manage(web_auth);
      Ok(())
    })
    .build()
}
