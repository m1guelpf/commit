use tauri::{
	plugin::TauriPlugin, AppHandle, GlobalWindowEvent, Manager, RunEvent, Window, WindowEvent,
};
use tauri_plugin_spotlight::{PluginConfig, WindowConfig};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

use crate::shortcuts;

pub const MAIN: &str = "main";
pub const SETTINGS: &str = "settings";

pub mod main_window;
pub mod settings;

#[allow(clippy::collapsible_if)]
pub fn handler(event: GlobalWindowEvent) {
	match event.event() {
		WindowEvent::Focused(true) => {
			if event.window().label() == SETTINGS {
				settings::on_open(event.window());
			}

			if event.window().label() == MAIN {
				main_window::on_open(event.window().clone());
			}
		},
		WindowEvent::Focused(is_focused) => {
			if !is_focused {
				if event.window().label() == MAIN {
					main_window::on_close(event.window())
				}
			}
		},
		WindowEvent::CloseRequested { api, .. } => {
			if event.window().label() == MAIN {
				api.prevent_close();
				main_window::hide(event.window()).unwrap();
			}
		},
		_ => {},
	}
}

pub fn prevent_exit(app: &AppHandle, event: RunEvent) {
	if let tauri::RunEvent::ExitRequested { api, .. } = event {
		api.prevent_exit();
		main_window::hide(&app.get_window(MAIN).unwrap()).unwrap();
	}
}

pub fn spotlight() -> TauriPlugin<tauri::Wry, Option<PluginConfig>> {
	tauri_plugin_spotlight::init(Some(tauri_plugin_spotlight::PluginConfig {
		windows: Some(vec![WindowConfig {
			label: String::from(MAIN),
			macos_window_level: Some(20),
			shortcut: String::from(shortcuts::DEFAULT_SHORTCUT),
		}]),
		global_close_shortcut: None,
	}))
}

pub trait TransparentWindow {
	fn make_transparent(&self) -> Result<(), window_vibrancy::Error>;
}

impl TransparentWindow for Window {
	fn make_transparent(&self) -> Result<(), window_vibrancy::Error> {
		apply_vibrancy(
			self,
			NSVisualEffectMaterial::HudWindow,
			Some(NSVisualEffectState::Active),
			Some(10.0),
		)
	}
}
