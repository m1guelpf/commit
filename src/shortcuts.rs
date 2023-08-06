use tauri::{AppHandle, GlobalShortcutManager, Manager, Window};

use crate::{config, window};

pub const DEFAULT_SHORTCUT: &str = "Cmd+Alt+Shift+C";

pub fn update_default(
	window: Window,
	old_shortcut: &str,
	new_shortcut: &str,
) -> Result<(), tauri::Error> {
	let app = window.app_handle();
	let mut shortcuts = app.global_shortcut_manager();

	shortcuts.unregister(old_shortcut)?;
	shortcuts.register(new_shortcut, move || {
		if window.is_visible().unwrap() {
			window::hide(&window).unwrap();
		} else {
			window::show(&window).unwrap();
		}
	})?;

	Ok(())
}

pub fn register_settings(app: &AppHandle) -> Result<(), anyhow::Error> {
	let mut shortcuts = app.global_shortcut_manager();

	shortcuts.register("Cmd+,", move || {
		config::edit().unwrap();
	})?;

	Ok(())
}

pub fn unregister_settings(app: &AppHandle) -> Result<(), anyhow::Error> {
	let mut shortcuts = app.global_shortcut_manager();

	shortcuts.unregister("Cmd+,")?;

	Ok(())
}

pub fn register_escape(window: Window) -> Result<(), tauri::Error> {
	let app = window.app_handle();
	let mut shortcuts = app.global_shortcut_manager();

	shortcuts.register("Escape", move || {
		window::hide(&window).unwrap();
	})?;

	Ok(())
}

pub fn unregister_escape(app: &AppHandle) -> Result<(), tauri::Error> {
	let mut shortcuts = app.global_shortcut_manager();

	shortcuts.unregister("Escape")?;

	Ok(())
}
