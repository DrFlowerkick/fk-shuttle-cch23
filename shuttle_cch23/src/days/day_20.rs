//!day_20.rs

use crate::app_error::{AppError, AppResult};
use axum::{body::Bytes, routing::post, Router};
use git2::{Commit, Repository};
use image::EncodableLayout;
use std::env;
use std::path::Path;
use std::process::Command;
use tar::Archive;
use tempfile::tempdir;

pub fn get_routes() -> Router {
    Router::new()
        .route("/20/archive_files", post(num_of_files_in_tar))
        .route("/20/archive_files_size", post(sum_of_size_of_files_in_tar))
        .route("/20/cookie_command", post(search_in_git_command_line))
        .route("/20/cookie", post(search_in_git_with_git2))
}

async fn num_of_files_in_tar(data: Bytes) -> AppResult<String> {
    let mut archive = Archive::new(data.as_bytes());
    Ok(format!("{}", archive.entries()?.count()))
}

async fn sum_of_size_of_files_in_tar(data: Bytes) -> AppResult<String> {
    let mut archive = Archive::new(data.as_bytes());
    let mut sum_file_size = 0;
    for file in archive.entries()? {
        let file = file?;
        sum_file_size += file.header().size()?;
    }
    Ok(format!("{}", sum_file_size))
}

async fn search_in_git_command_line(data: Bytes) -> AppResult<String> {
    let mut archive = Archive::new(data.as_bytes());
    let tmpdir = tempdir()?;
    // use fn call to catch possible errors to enable deleting tmpdir
    let result = prepare_git(tmpdir.path(), &mut archive);
    // move one dir up to enable deleting
    // not using ? operator since I expect this to work. otherwise a panic is the correct reaction
    env::set_current_dir(tmpdir.path().parent().unwrap()).unwrap();
    tmpdir.close()?;
    result
}

fn prepare_git(repo_path: &Path, archive: &mut Archive<&[u8]>) -> AppResult<String> {
    archive.unpack(repo_path)?;
    find_commit(repo_path, "christmas", "santa.txt", "COOKIE")
}

fn find_commit(
    repo_path: &Path,
    branch_name: &str,
    file_name: &str,
    content: &str,
) -> AppResult<String> {
    env::set_current_dir(repo_path)?;
    // execute git log and search for commits in branch_name
    // each returned line (formatted as "Author Hash Tree") corresponds to one commit
    // commits are ordered by commit date with newest first
    let commit_hash_output = Command::new("git")
        .arg("log")
        .arg("--format=%an %H %T")
        .arg(format!("{}", branch_name))
        .output()?;

    if commit_hash_output.status.success() {
        // analyze output of successfull command
        let output_str = String::from_utf8_lossy(&commit_hash_output.stdout);
        for (author, hash, tree) in output_str.lines().map(|l| {
            let mut log_output = l.trim().split_ascii_whitespace().map(|l| l.trim());
            (
                log_output.next().unwrap(),
                log_output.next().unwrap(),
                log_output.next().unwrap(),
            )
        }) {
            // execute git grep command to search for "content" in all files "filename"
            // * in front of filename finds all files in tree sub dirs, which are santa.txt
            let grep_output = Command::new("git")
                .arg("grep")
                .arg(content)
                .arg(tree)
                .arg("--")
                .arg(format!("*{}", file_name))
                .output()?;

            if grep_output.status.success() {
                // if succesfull, "content" was found in file_name of tree
                return Ok(format!("{} {}", author, hash));
            }
        }
    }
    Err(AppError::bad_request("commit not found"))
}

// the following code is inspired and partly copied from
// https://github.com/MatzHilven/shuttle-cch23/blob/master/src/days/twenty/mod.rs by Matz Hilven
// all praise to him 👏 🙏
async fn search_in_git_with_git2(data: Bytes) -> AppResult<String> {
    let mut archive = Archive::new(data.as_bytes());
    let tmpdir = tempdir()?;
    // use fn call to catch possible errors to enable deleting tmpdir
    let result = search_tar_with_git2(tmpdir.path(), &mut archive);
    tmpdir.close()?;
    result
}

fn search_tar_with_git2(repo_path: &Path, archive: &mut Archive<&[u8]>) -> AppResult<String> {
    // unpack archive in repo_path
    if repo_path.read_dir()?.count() != 0 {
        return Err(AppError::internal_error("provided directory is not empty"));
    }
    archive.unpack(repo_path)?;
    // set static search values
    let branch_name = "christmas";
    let file_name = "santa.txt";
    let content = "COOKIE";
    // open repository
    let repository = Repository::open(repo_path)?;
    // get Branch in repository
    let branch = repository.find_branch(branch_name, git2::BranchType::Local)?;
    // get tip commit in branch
    let tip = branch.into_reference().peel_to_commit()?;
    if let Some(commit) =
        find_first_commit_fitting_search_values(&repository, tip, file_name, content)
    {
        let author = commit.author().name().unwrap().to_owned();
        let hash = commit.id().to_string();
        return Ok(format!("{} {}", author, hash));
    }
    Err(AppError::bad_request(
        "could not find commit with static defined search values",
    ))
}

fn find_first_commit_fitting_search_values<'a>(
    repository: &Repository,
    commit: Commit<'a>,
    file_name: &str,
    content: &str,
) -> Option<Commit<'a>> {
    if let Ok(tree) = commit.tree() {
        // Traverse the tree
        if find_file_and_content_in_tree(repository, &tree, file_name, content) {
            // If the file is found in this commit's tree, return the commit
            return Some(commit);
        }
    }

    // If the file is not found in this commit, traverse its parent commits
    for parent in commit.parents() {
        if let Some(found_commit) =
            find_first_commit_fitting_search_values(repository, parent, file_name, content)
        {
            return Some(found_commit);
        }
    }

    None
}

fn find_file_and_content_in_tree<'a>(
    repo: &'a git2::Repository,
    tree: &'a git2::Tree<'_>,
    file_name: &str,
    content: &str,
) -> bool {
    for entry in tree.iter() {
        let entry_name = entry.name().unwrap_or("").to_string();

        if let Ok(object) = entry.to_object(repo) {
            if let Some(subtree) = object.as_tree() {
                if find_file_and_content_in_tree(repo, subtree, file_name, content) {
                    return true;
                }
            } else if let Some(blob) = object.as_blob() {
                if entry_name == file_name {
                    if let Ok(blob_content) = std::str::from_utf8(blob.content()) {
                        if blob_content.contains(content) {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}
