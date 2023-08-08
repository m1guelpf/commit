use tauri::{
	plugin::TauriPlugin, AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent,
	SystemTrayMenu, SystemTrayMenuItem,
};
use tauri_plugin_autostart::MacosLauncher;

use crate::window;

pub enum TrayMenu {
	Quit,
	Settings,
	#[cfg(debug_assertions)]
	DevTools,
}

pub fn build() -> SystemTray {
	let tray_menu = SystemTrayMenu::new()
		.add_item(CustomMenuItem::new(TrayMenu::Settings, "Settings...").accelerator("Cmd+,"))
		.add_native_item(SystemTrayMenuItem::Separator);

	#[cfg(debug_assertions)]
	let tray_menu = tray_menu
		.add_item(
			CustomMenuItem::new(TrayMenu::DevTools, "Open DevTools").accelerator("Cmd+Shift+I"),
		)
		.add_native_item(SystemTrayMenuItem::Separator);

	let tray_menu = tray_menu.add_item(CustomMenuItem::new(
		TrayMenu::Quit,
		"Quit Commit Completely",
	));

	SystemTray::new().with_menu(tray_menu)
}

pub fn handle(app: &AppHandle, event: SystemTrayEvent) {
	match event {
		SystemTrayEvent::LeftClick { .. } => {
			let main_window = app.get_window(window::MAIN).unwrap();
			if main_window.is_visible().unwrap() {
				window::main_window::hide(&main_window).unwrap();
			} else {
				window::main_window::show(&main_window).unwrap();
			}
		},
		SystemTrayEvent::MenuItemClick { id, .. } => match id.into() {
			TrayMenu::Quit => std::process::exit(0),
			TrayMenu::Settings => {
				let settings_window = app.get_window(window::SETTINGS).unwrap();
				settings_window.show().unwrap();
				settings_window.set_focus().unwrap();
			},
			#[cfg(debug_assertions)]
			TrayMenu::DevTools => app.get_window(window::MAIN).unwrap().open_devtools(),
		},
		_ => {},
	};
}

pub fn autostart() -> TauriPlugin<tauri::Wry> {
	tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None)
}

impl From<TrayMenu> for String {
	fn from(value: TrayMenu) -> Self {
		match value {
			TrayMenu::Quit => "quit".to_string(),
			TrayMenu::Settings => "settings".to_string(),
			#[cfg(debug_assertions)]
			TrayMenu::DevTools => "devtools".to_string(),
		}
	}
}

impl From<String> for TrayMenu {
	fn from(value: String) -> Self {
		match value.as_str() {
			"quit" => TrayMenu::Quit,
			"settings" => TrayMenu::Settings,
			#[cfg(debug_assertions)]
			"devtools" => TrayMenu::DevTools,
			_ => unreachable!(),
		}
	}
}
