use anyhow::anyhow;
use git2::Repository;
use tauri::{AppHandle, GlobalWindowEvent, RunEvent, Window, WindowEvent};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

use crate::utils;

pub fn handler(event: GlobalWindowEvent) {
	match event.event() {
		WindowEvent::Focused(is_focused) => {
			if !is_focused {
				#[cfg(not(debug_assertions))]
				toggle(event.window()).unwrap();
			}
		},
		WindowEvent::CloseRequested { api, .. } => {
			toggle(event.window()).unwrap();
			api.prevent_close();
		},
		_ => {},
	}
}

pub fn make_transparent(window: &Window) -> Result<(), window_vibrancy::Error> {
	apply_vibrancy(
		window,
		NSVisualEffectMaterial::HudWindow,
		Some(NSVisualEffectState::Active),
		Some(10.0),
	)
}

pub fn toggle(window: &Window) -> anyhow::Result<()> {
	if window.is_visible()? {
		window.emit("reset", true)?;
		window.hide()?;
	} else {
		let window_handle = window.clone();
		tauri::async_runtime::spawn(async move {
			let Some(repo_path) = utils::find_latest_repo(&["/Users/m1guelpf/Code".into()])? else {
				window_handle.emit("current_dir", Option::<String>::None)?;

				return Err(anyhow!("No repo found"));
			};

			window_handle.emit("current_dir", &repo_path)?;

			let repo = Repository::open(&repo_path)?;

			window_handle.emit("current_branch", utils::get_branch_name(&repo))?;

			window_handle.emit(
				"current_repo",
				utils::get_repo_name(&repo).or_else(|| {
					repo_path
						.file_name()
						.and_then(|s| s.to_str())
						.map(ToString::to_string)
				}),
			)?;

			window_handle.emit("current_diff", utils::get_diff(&repo))?;

			Ok(())
		});

		window.show()?;
		window.set_focus()?;
	}

	Ok(())
}

pub fn prevent_exit(_: &AppHandle, event: RunEvent) {
	if let tauri::RunEvent::ExitRequested { api, .. } = event {
		api.prevent_exit();
	}
}
