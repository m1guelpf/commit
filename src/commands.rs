use git2::Repository;
use std::{path::PathBuf, process::Command};
use tauri::{api::notification::Notification, AppHandle, Invoke, State, Window, Wry};

use crate::{config::GetConfig, utils, window};

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

	utils::commit(
		&repo,
		&format!(
			"{title}{}",
			description.map(|m| format!("\n\n{m}")).unwrap_or_default()
		),
	)
	.map_err(|e| e.to_string())?;

	window::hide(&window).unwrap();
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

pub fn handler() -> impl Fn(Invoke<Wry>) + Send + Sync + 'static {
	tauri::generate_handler![commit]
}
