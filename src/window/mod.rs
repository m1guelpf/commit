use tauri::{
	AppHandle, GlobalWindowEvent, Manager, RunEvent, Window, WindowEvent,
};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

pub const MAIN: &str = "main";
pub const SETTINGS: &str = "settings";

pub mod main_window;
pub mod settings;
pub mod ns_panel;

#[allow(clippy::collapsible_if)]
pub fn handler(event: GlobalWindowEvent) {
	match event.event() {
		WindowEvent::Focused(true) => {
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
				main_window::hide(event.window());
			}

			if event.window().label() == SETTINGS {
				api.prevent_close();
				event.window().hide().unwrap();
			}
		},
		_ => {},
	}
}

pub fn prevent_exit(app: &AppHandle, event: RunEvent) {
	if let tauri::RunEvent::ExitRequested { api, .. } = event {
		api.prevent_exit();
		main_window::hide(&app.get_window(MAIN).unwrap());
	}
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
