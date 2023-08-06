use tauri::{AppHandle, GlobalShortcutManager, Manager, Window};

use crate::window;

pub const DEFAULT_SHORTCUT: &str = "Cmd+Alt+Shift+C";

pub fn update_default(
	app: &AppHandle,
	old_shortcut: &str,
	new_shortcut: &str,
) -> Result<(), tauri::Error> {
	let window = app.get_window(window::MAIN).unwrap();
	let mut shortcuts = app.global_shortcut_manager();

	shortcuts.unregister(old_shortcut)?;
	shortcuts.register(new_shortcut, move || {
		if window.is_visible().unwrap() {
			window::main_window::hide(&window).unwrap();
		} else {
			window::main_window::show(&window).unwrap();
		}
	})?;

	Ok(())
}

pub fn register_settings(app: &AppHandle) -> Result<(), anyhow::Error> {
	let mut shortcuts = app.global_shortcut_manager();

	let settings_window = window::settings::get(app).unwrap();
	shortcuts.register("Cmd+,", move || {
		settings_window.show().unwrap();
		settings_window.set_focus().unwrap();
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
		window::main_window::hide(&window).unwrap();
	})?;

	Ok(())
}

pub fn unregister_escape(app: &AppHandle) -> Result<(), tauri::Error> {
	let mut shortcuts = app.global_shortcut_manager();

	shortcuts.unregister("Escape")?;

	Ok(())
}
