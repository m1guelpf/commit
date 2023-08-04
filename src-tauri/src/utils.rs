use git2::{DiffOptions, ErrorCode, IndexAddOption, Repository};
use priority_queue::PriorityQueue;
use rayon::prelude::*;
use std::{fs, io, path::PathBuf, time::SystemTime};
use walkdir::{DirEntry, WalkDir};

pub fn commit(repo: &Repository, message: &str) -> Result<(), git2::Error> {
	let mut index = repo.index()?;
	let tree_oid = index.write_tree()?;
	let tree = repo.find_tree(tree_oid)?;

	let parent_commit = match repo.revparse_single("HEAD") {
		Ok(obj) => Some(obj.into_commit().unwrap()),
		Err(e) if e.code() == ErrorCode::NotFound => None,
		Err(e) => return Err(e),
	};

	let mut parents = Vec::new();
	if parent_commit.is_some() {
		parents.push(parent_commit.as_ref().unwrap());
	}

	let signature = repo.signature()?;

	repo.commit(
		Some("HEAD"),
		&signature,
		&signature,
		message,
		&tree,
		&parents[..],
	)?;

	Ok(())
}

pub fn get_branch_name(repo: &Repository) -> Option<String> {
	let r#ref = repo.find_reference("HEAD").ok()?;
	let current_branch = r#ref.symbolic_target()?;

	Some(current_branch.replacen("refs/heads/", "", 1))
}

pub fn get_repo_name(repo: &Repository) -> Option<String> {
	let remote = repo.find_remote("origin").ok()?;
	let url = remote.url()?;

	let split_by_colon: Vec<&str> = url.rsplitn(2, ':').collect();
	let split_by_slash: Vec<String> = if split_by_colon.len() > 1 {
		// If it's an SSH url, we split the second part by /
		split_by_colon[0]
			.rsplitn(2, '/')
			.map(ToString::to_string)
			.collect()
	} else {
		// If it's an HTTPS url, we remove .git and split by /
		url.replace(".git", "")
			.rsplitn(3, '/')
			.map(ToString::to_string)
			.collect()
	};

	if split_by_slash.len() < 2 {
		return None;
	}

	Some(format!("{}/{}", split_by_slash[1], split_by_slash[0]))
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DiffStats {
	deletions: usize,
	insertions: usize,
	files_changed: usize,
}

pub fn get_diff(repo: &Repository) -> Option<DiffStats> {
	let mut index = repo.index().ok()?;
	index
		.add_all(["."].iter(), IndexAddOption::DEFAULT, None)
		.unwrap();

	index.update_all(["."].iter(), None).unwrap();
	index.write().unwrap();

	let diff = repo
		.diff_tree_to_index(None, None, Some(DiffOptions::new().ignore_submodules(true)))
		.ok()?;

	let stats = diff.stats().ok()?;

	Some(DiffStats {
		deletions: stats.deletions(),
		insertions: stats.insertions(),
		files_changed: stats.files_changed(),
	})
}

pub fn find_latest_repo(paths: &[PathBuf]) -> io::Result<Option<PathBuf>> {
	let pq = paths
		.par_iter()
		.flat_map(|path| {
			WalkDir::new(path)
				.max_depth(1)
				.into_iter()
				.filter_map(Result::ok)
				.par_bridge()
		})
		.filter_map(process_repo)
		.fold(PriorityQueue::new, |mut pq, (path, modified_time)| {
			pq.push(path, modified_time);
			pq
		})
		.reduce(PriorityQueue::new, |mut pq1, mut pq2| {
			pq1.append(&mut pq2);
			pq1
		});

	Ok(pq.peek().map(|(path, _)| path.clone()))
}

fn process_repo(dir: DirEntry) -> Option<(PathBuf, SystemTime)> {
	let repo = Repository::open(dir.path()).ok()?;

	let diff = repo
		.diff_tree_to_workdir(
			None,
			Some(
				DiffOptions::new()
					.ignore_case(true)
					.recurse_ignored_dirs(false)
					.recurse_untracked_dirs(true)
					.force_binary(true)
					.ignore_filemode(true)
					.skip_binary_check(true)
					.ignore_submodules(true)
					.include_untracked(true)
					.ignore_whitespace(true),
			),
		)
		.ok()?;

	let mut latest_modified_time = None;

	for delta in diff.deltas() {
		let file_path = dir.path().join(delta.new_file().path().unwrap());

		let modified_time = match fs::metadata(&file_path).and_then(|metadata| metadata.modified())
		{
			Err(_) => continue,
			Ok(time) => time,
		};

		if latest_modified_time.map_or(true, |time| modified_time > time) {
			latest_modified_time = Some(modified_time);
		}
	}

	latest_modified_time.map(|modified_time| (dir.path().to_path_buf(), modified_time))
}
