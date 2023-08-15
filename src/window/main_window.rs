use anyhow::anyhow;
use git2::Repository;
use tauri::{api::notification::Notification, AppHandle, Manager, Window};
use tauri_plugin_spotlight::ManagerExt as SpotlightExt;

use crate::{
	config::ConfigExt,
	repo, shortcuts,
	window::{self, TransparentWindow},
};

pub fn create(app: &AppHandle) -> anyhow::Result<Window> {
	let main_window = app
		.get_window(window::MAIN)
		.ok_or_else(|| anyhow!("Window not found"))?;

	main_window.make_transparent().map_err(|_| {
		anyhow!("Unsupported platform! 'apply_vibrancy' is only supported on macOS")
	})?;

	Ok(main_window)
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

pub fn on_open(window: Window) {
	shortcuts::register_escape(window.clone()).unwrap();
	shortcuts::register_settings(&window.app_handle()).unwrap_or_else(|_| {
		eprintln!("Failed to register settings shortcut");
	});

	tauri::async_runtime::spawn(async move {
		let app = window.app_handle();
		let config = app.user_config();
		let config = config.read().unwrap();

		if config.repo_paths.is_empty() {
			Notification::default()
				.title("No folders found")
				.body("Please add at least one folder to the config.")
				.show()
				.unwrap();

			window.hide().unwrap();
			let settings_window = window::settings::get(&window.app_handle()).unwrap();
			settings_window.show().unwrap();
			settings_window.set_focus().unwrap();

			return Ok(());
		}

		let Some(repo_path) = repo::find_latest(&config.repo_paths)? else {
			window.emit("current_dir", Option::<String>::None)?;

			return Err(anyhow!("No repo found"));
		};

		window.emit("current_dir", &repo_path)?;

		let repo = Repository::open(&repo_path)?;

		window.emit("current_branch", repo::branch_name(&repo))?;

		window.emit(
			"current_repo",
			repo::name(&repo).or_else(|| {
				repo_path
					.file_name()
					.and_then(|s| s.to_str())
					.map(ToString::to_string)
			}),
		)?;

		window.emit("current_diff", repo::diff_changes(&repo))?;

		Ok(())
	});
}

pub fn on_close(window: &Window) {
	window.emit("reset", true).unwrap();

	shortcuts::unregister_escape(&window.app_handle()).unwrap();
	shortcuts::unregister_settings(&window.app_handle()).unwrap();
}
