use anyhow::anyhow;
use git2::Repository;
use tauri::{AppHandle, GlobalWindowEvent, Manager, RunEvent, Window, WindowEvent};
use tauri_plugin_spotlight::ManagerExt;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

use crate::{config::GetConfig, shortcuts, utils};

pub const NAME: &str = "main";

pub fn handler(event: GlobalWindowEvent) {
	match event.event() {
		WindowEvent::Focused(true) => {
			on_window(event.window().clone());
			shortcuts::register_escape(event.window().clone()).unwrap();
			shortcuts::register_settings(&event.window().app_handle()).unwrap();
		},
		WindowEvent::Focused(is_focused) => {
			if !is_focused {
				#[cfg(not(debug_assertions))]
				toggle(event.window()).unwrap();

				event.window().emit("reset", true).unwrap();
				shortcuts::unregister_escape(&event.window().app_handle()).unwrap();
				shortcuts::unregister_settings(&event.window().app_handle()).unwrap();
			}
		},
		WindowEvent::CloseRequested { api, .. } => {
			api.prevent_close();
			hide(event.window()).unwrap();
		},
		_ => {},
	}
}

pub fn show(window: &Window) -> Result<(), tauri_plugin_spotlight::Error> {
	let app = window.app_handle();
	let spotlight = app.spotlight();

	spotlight.show(window)
}

pub fn hide(window: &Window) -> Result<(), tauri_plugin_spotlight::Error> {
	let app = window.app_handle();
	let spotlight = app.spotlight();

	spotlight.hide(window)
}

pub fn make_transparent(window: &Window) -> Result<(), window_vibrancy::Error> {
	apply_vibrancy(
		window,
		NSVisualEffectMaterial::HudWindow,
		Some(NSVisualEffectState::Active),
		Some(10.0),
	)
}

pub fn on_window(window: Window) {
	tauri::async_runtime::spawn(async move {
		let app = window.app_handle();
		let config = app.state::<GetConfig>();
		let config = config.read().unwrap();
		let Some(repo_path) = utils::find_latest_repo(&config.repo_paths)? else {
			window.emit("current_dir", Option::<String>::None)?;

			return Err(anyhow!("No repo found"));
		};

		window.emit("current_dir", &repo_path)?;

		let repo = Repository::open(&repo_path)?;

		window.emit("current_branch", utils::get_branch_name(&repo))?;

		window.emit(
			"current_repo",
			utils::get_repo_name(&repo).or_else(|| {
				repo_path
					.file_name()
					.and_then(|s| s.to_str())
					.map(ToString::to_string)
			}),
		)?;

		window.emit("current_diff", utils::get_diff(&repo))?;

		Ok(())
	});
}

pub fn prevent_exit(app: &AppHandle, event: RunEvent) {
	if let tauri::RunEvent::ExitRequested { api, .. } = event {
		api.prevent_exit();
		hide(&app.get_window(NAME).unwrap()).unwrap();
	}
}
