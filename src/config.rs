use std::{path::PathBuf, process::Command, sync::RwLock};

use anyhow::Result;

pub type GetConfig = RwLock<Config>;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
	pub autostart: bool,
	pub should_push: bool,
	pub repo_paths: Vec<PathBuf>,
}

impl Config {
	pub fn load() -> Result<Self, confy::ConfyError> {
		let config: Self = confy::load("commit", Some("Settings"))?;

		config.save()?;
		Ok(config)
	}

	pub fn save(&self) -> Result<(), confy::ConfyError> {
		confy::store("commit", Some("Settings"), self)
	}

	pub fn manage(self) -> RwLock<Self> {
		RwLock::new(self)
	}

	pub fn config_path() -> Result<PathBuf, confy::ConfyError> {
		confy::get_configuration_file_path("commit", Some("Settings"))
	}
}

impl Default for Config {
	fn default() -> Self {
		Self {
			autostart: false,
			should_push: true,
			repo_paths: vec![],
		}
	}
}

pub fn edit() -> Result<()> {
	Command::new("open").arg(Config::config_path()?).status()?;

	Ok(())
}
