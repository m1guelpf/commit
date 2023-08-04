use tauri::{
	AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
	SystemTrayMenuItem,
};

use crate::window;

pub enum TrayMenu {
	Quit,
	#[cfg(not(feature = "custom_protocol"))]
	DevTools,
}

impl From<TrayMenu> for String {
	fn from(value: TrayMenu) -> Self {
		match value {
			TrayMenu::Quit => "quit".to_string(),
			#[cfg(not(feature = "custom_protocol"))]
			TrayMenu::DevTools => "devtools".to_string(),
		}
	}
}

impl From<String> for TrayMenu {
	fn from(value: String) -> Self {
		match value.as_str() {
			"quit" => TrayMenu::Quit,
			#[cfg(not(feature = "custom_protocol"))]
			"devtools" => TrayMenu::DevTools,
			_ => unreachable!(),
		}
	}
}

pub fn build() -> SystemTray {
	let tray_menu = SystemTrayMenu::new();

	#[cfg(not(feature = "custom_protocol"))]
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
			window::toggle(&app.get_window("main").unwrap()).unwrap()
		},
		SystemTrayEvent::MenuItemClick { id, .. } => match id.into() {
			TrayMenu::Quit => std::process::exit(0),
			#[cfg(not(feature = "custom_protocol"))]
			TrayMenu::DevTools => app.get_window("main").unwrap().open_devtools(),
		},
		_ => {},
	}
}
