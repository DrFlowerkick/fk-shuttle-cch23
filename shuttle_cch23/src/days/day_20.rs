//!day_20.rs

use crate::app_error::{AppError, AppResult};
use axum::{body::Bytes, routing::post, Router};
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
        .route("/20/cookie", post(search_in_git))
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

async fn search_in_git(data: Bytes) -> AppResult<String> {
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
