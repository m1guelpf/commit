use git2::Repository;
use std::{path::PathBuf, process::Command};
use tauri::{
	api::{dialog, notification::Notification},
	AppHandle, Invoke, State, Window, Wry,
};
use tauri_plugin_autostart::ManagerExt;

use crate::{
	config::{Config, GetConfig},
	repo, shortcuts, window,
};

#[tauri::command]
fn commit(
	app: AppHandle,
	window: Window,
	path: PathBuf,
	title: String,
	description: Option<String>,
	config: State<GetConfig>,
) -> Result<(), String> {
	let repo = Repository::open(path).map_err(|e| e.to_string())?;

	repo::commit(
		&repo,
		&format!(
			"{title}{}",
			description.map(|m| format!("\n\n{m}")).unwrap_or_default()
		),
	)
	.map_err(|e| e.to_string())?;

	window::main_window::hide(&window);
	Notification::new(&app.config().tauri.bundle.identifier)
		.title("Commit")
		.body("Commit successful!")
		.show()
		.unwrap();

	let config = config.read().unwrap();
	if config.should_push {
		tauri::async_runtime::spawn(async move {
			let status = Command::new("git")
				.arg("push")
				.current_dir(repo.path())
				.status()
				.expect("Failed to execute git push");

			let alert = Notification::new(&app.config().tauri.bundle.identifier);
			if status.success() {
				alert.title("Push").body("Push successful!")
			} else {
				alert
					.title("Failed to push")
					.body("Failed to push to remote repository")
			}
			.show()
			.unwrap()
		});
	}

	Ok(())
}

#[tauri::command]
fn update_config(
	app: AppHandle,
	new_config: Config,
	settings_window: Window,
	config: State<GetConfig>,
) -> Result<(), String> {
	let read_config = config.read().unwrap();
	let old_shortcut = read_config.shortcut.as_str();

	if new_config.shortcut != old_shortcut {
		shortcuts::update_default(&app, old_shortcut, &new_config.shortcut)
			.map_err(|_| "Invalid shortcut provided.".to_string())?;
	}
	drop(read_config);

	let mut config = config.write().unwrap();
	config
		.update(new_config.clone())
		.map_err(|_| "Failed to update config.")?;
	drop(config);

	let autolaunch = app.autolaunch();
	if new_config.autostart {
		autolaunch
			.enable()
			.map_err(|_| "Failed to enable autostart.")?;
	} else {
		autolaunch
			.disable()
			.map_err(|_| "Failed to disable autostart.")?;
	}

	settings_window.emit("config", new_config).unwrap();

	Notification::new(&app.config().tauri.bundle.identifier)
		.title("Settings")
		.body("Settings updated!")
		.show()
		.unwrap();

	Ok(())
}

#[tauri::command]
fn get_config(config: State<GetConfig>, window: Window) {
	let config = config.read().unwrap();

	window.emit("config", config.clone()).unwrap();
}

#[tauri::command]
fn select_folder(window: Window) {
	dialog::FileDialogBuilder::default().pick_folder(move |folder_path| {
		if let Some(folder_path) = folder_path {
			window.emit("folder_selected", folder_path).unwrap();
		}
	});
}

pub fn handler() -> impl Fn(Invoke<Wry>) + Send + Sync + 'static {
	tauri::generate_handler![
		commit,
		update_config,
		select_folder,
		get_config,
		window::ns_panel::show_app,
		window::ns_panel::hide_app
	]
}
