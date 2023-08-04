use std::{path::PathBuf, process::Command};

use git2::Repository;
use tauri::{api::notification::Notification, AppHandle, Invoke, Window, Wry};

use crate::{utils, window};

#[tauri::command]
fn commit(
	app: AppHandle,
	window: Window,
	path: PathBuf,
	title: String,
	description: Option<String>,
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

	window::toggle(&window).unwrap();
	Notification::new(&app.config().tauri.bundle.identifier)
		.title("Commit")
		.body("Commit successful!")
		.show()
		.unwrap();

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

	Ok(())
}

pub fn handler() -> impl Fn(Invoke<Wry>) + Send + Sync + 'static {
	tauri::generate_handler![commit]
}
