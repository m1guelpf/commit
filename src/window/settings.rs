use anyhow::anyhow;
use tauri::{AppHandle, Manager, TitleBarStyle, Window, WindowBuilder, WindowUrl};

use crate::{config::ConfigExt, window};

use super::TransparentWindow;

pub fn create(app: &AppHandle) -> anyhow::Result<Window> {
	let settings_window =
		WindowBuilder::new(app, window::SETTINGS, WindowUrl::App(Default::default()))
			.visible(false)
			.closable(true)
			.transparent(true)
			.always_on_top(true)
			.title("Settings - Commit")
			.title_bar_style(TitleBarStyle::Overlay)
			.initialization_script("window.__COMMIT__ = { page: 'settings' };")
			.build()?;

	settings_window.make_transparent().map_err(|_| {
		anyhow!("Unsupported platform! 'apply_vibrancy' is only supported on macOS")
	})?;

	Ok(settings_window)
}

pub fn on_open(window: &Window) {
	let app = window.app_handle();
	let config = app.user_config();

	window.emit("config", config.inner()).unwrap();
}

pub fn get(app: &AppHandle) -> Option<Window> {
	app.get_window(window::SETTINGS)
}
