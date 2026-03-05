use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{bail, Context, Result};
use semver::Version;
use serde::Deserialize;
use serde_json::Value;

use crate::config;

const REMOTE_PACKAGE_URL_ENV: &str = "REFLY_CLONE_PACKAGE_URL";
const DEFAULT_REMOTE_PACKAGE_URL: &str =
    "https://raw.githubusercontent.com/refly-ai/agent-digital-cowork/main/clone/package.json";

#[derive(Debug, Deserialize)]
struct PackageJson {
    version: Option<String>,
}

pub fn run_version() -> Result<()> {
    let local_package = clone_path()?.join("package.json");
    if !local_package.is_file() {
        bail!("local package.json not found: {}", local_package.display());
    }

    let local = read_version_file(&local_package)?;
    println!("local version : {}", local);

    let remote_url = std::env::var(REMOTE_PACKAGE_URL_ENV)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_REMOTE_PACKAGE_URL.to_string());

    let remote = match fetch_remote_version(&remote_url) {
        Ok(value) => value,
        Err(error) => {
            println!("remote version: unavailable");
            println!("note: failed to fetch remote package.json: {error}");
            return Ok(());
        }
    };

    println!("remote version: {}", remote);
    match compare_versions(&local, &remote) {
        VersionRelation::RemoteNewer => println!("update available: run `cowork clone update`"),
        VersionRelation::UpToDate => println!("up to date"),
        VersionRelation::LocalAhead => println!("local version is ahead of remote"),
        VersionRelation::Unknown => {
            println!("version format not semver-compatible; compare manually")
        }
    }

    Ok(())
}

pub fn run_init() -> Result<()> {
    let repo_url = config::required_repo_url()?;
    let repo_root = repo_root_path()?;

    if repo_root.exists() {
        bail!("clone target already exists: {}", repo_root.display());
    }

    let parent = repo_root
        .parent()
        .context("failed to resolve clone target parent directory")?;
    fs::create_dir_all(parent).with_context(|| format!("failed to create {}", parent.display()))?;

    run_command(
        Command::new("git")
            .arg("clone")
            .arg(&repo_url)
            .arg(&repo_root),
        "git clone failed",
    )?;

    let clone_path = clone_path()?;
    if !clone_path.is_dir() {
        bail!(
            "clone path does not exist (check {}): {}",
            config::COWORK_CLONE_REPO_SUBDIR_ENV,
            clone_path.display()
        );
    }

    println!("initialized clone at {}", clone_path.display());
    Ok(())
}

pub fn run_update() -> Result<()> {
    let path = clone_path()?;
    ensure_dir_exists(&path)?;

    run_command(
        Command::new("git")
            .arg("-C")
            .arg(&path)
            .arg("pull")
            .arg("--ff-only"),
        "git pull --ff-only failed",
    )
}

pub fn run_metadata() -> Result<()> {
    let path = clone_path()?;
    ensure_dir_exists(&path)?;

    let depth = config::metadata_tree_depth()?;
    println!("clone path: {}", path.display());

    let meta_path = path.join(".meta.json");
    if meta_path.is_file() {
        let content = fs::read_to_string(&meta_path)
            .with_context(|| format!("failed to read {}", meta_path.display()))?;
        let parsed: Value = serde_json::from_str(&content)
            .with_context(|| format!("invalid JSON in {}", meta_path.display()))?;
        println!(".meta.json: {}", meta_path.display());
        if let Some(hash) = parsed.get("hash").and_then(Value::as_str) {
            println!("meta.hash: {}", hash);
        }
        if let Some(files_count) = parsed.get("filesCount") {
            println!("meta.filesCount: {}", files_count);
        }
        if let Some(generated_at) = parsed.get("generatedAt") {
            println!("meta.generatedAt: {}", generated_at);
        }
    } else {
        println!(".meta.json: not found");
    }

    println!("tree depth: {}", depth);
    print_tree(&path, depth)?;
    Ok(())
}

pub fn run_preview() -> Result<()> {
    let path = clone_path()?;
    ensure_dir_exists(&path)?;

    let session = config::clone_session();
    let preview_cmd = config::preview_cmd();

    let has_session = Command::new("tmux")
        .arg("has-session")
        .arg("-t")
        .arg(&session)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("failed to check tmux session")?
        .success();

    if has_session {
        bail!(
            "tmux session '{}' already exists; attach with: tmux attach -t {}",
            session,
            session
        );
    }

    run_command(
        Command::new("tmux")
            .arg("new-session")
            .arg("-d")
            .arg("-s")
            .arg(&session)
            .arg("-c")
            .arg(&path)
            .arg(&preview_cmd),
        "failed to start tmux preview session",
    )?;

    println!("started tmux session '{}' in {}", session, path.display());
    println!("attach with: tmux attach -t {}", session);
    Ok(())
}

pub fn run_contribute() -> Result<()> {
    let path = clone_path()?;
    println!("Worktree contribution flow (print-only)");
    println!("1) Ensure base clone exists at: {}", path.display());
    println!("2) Create a branch worktree for your task");
    println!("3) Work in the worktree, commit, and open a PR");
    println!("4) Remove the worktree after merge");
    println!();
    println!("Example commands:");
    println!("git -C {} fetch origin", path.display());
    println!(
        "git -C {} worktree add ../{}-feature -b feat/example origin/main",
        path.display(),
        config::repo_alias()
    );
    println!("cd ../{}-feature", config::repo_alias());
    println!("git status");
    println!("git add -A && git commit -m \"feat: example\"");
    println!("git push -u origin feat/example");
    println!(
        "git -C {} worktree remove ../{}-feature",
        path.display(),
        config::repo_alias()
    );
    Ok(())
}

pub fn run_resource() -> Result<()> {
    let path = clone_path()?;
    let resources_root = path.join("resources");

    println!("Undefined resource guide (print-only)");
    println!("root: {}", resources_root.display());
    println!();
    println!("Convention:");
    println!("- Store raw materials under clone/resources/<ts>/<contributor>/...");
    println!("- Keep file type open (for example: .md, .json, .txt, images)");
    println!("- Use this area for valuable but not yet system-extracted materials");
    println!();
    println!("Examples:");
    println!(
        "mkdir -p {}/20260304T180000Z/alice",
        resources_root.display()
    );
    println!(
        "cp ./meeting-notes.md {}/20260304T180000Z/alice/meeting-notes.md",
        resources_root.display()
    );
    println!(
        "cp ./design-sketch.json {}/20260304T180000Z/alice/design-sketch.json",
        resources_root.display()
    );

    Ok(())
}

pub fn clone_path() -> Result<PathBuf> {
    let repo_root = repo_root_path()?;
    let subdir = config::repo_subdir();
    if subdir.trim().is_empty() {
        return Ok(repo_root);
    }
    Ok(repo_root.join(subdir))
}

fn repo_root_path() -> Result<PathBuf> {
    Ok(config::cowork_home()?
        .join("clones")
        .join(config::repo_alias()))
}

fn ensure_dir_exists(path: &Path) -> Result<()> {
    if !path.is_dir() {
        bail!("clone path does not exist: {}", path.display());
    }
    Ok(())
}

fn print_tree(path: &Path, depth: usize) -> Result<()> {
    println!(".");
    if depth == 0 {
        return Ok(());
    }
    print_tree_children(path, "", depth)?;
    Ok(())
}

fn print_tree_children(path: &Path, prefix: &str, depth: usize) -> Result<()> {
    if depth == 0 {
        return Ok(());
    }

    let mut entries = fs::read_dir(path)
        .with_context(|| format!("failed to read directory {}", path.display()))?
        .collect::<std::result::Result<Vec<_>, _>>()
        .with_context(|| format!("failed to list directory {}", path.display()))?;

    entries.sort_by_key(|entry| entry.file_name());
    let total = entries.len();

    for (index, entry) in entries.into_iter().enumerate() {
        let is_last = index + 1 == total;
        let branch = if is_last { "`-- " } else { "|-- " };
        let name = entry.file_name();
        let child_name = os_string_to_string(name)?;
        let child_path = entry.path();
        let child_is_dir = child_path.is_dir();

        println!(
            "{}{}{}{}",
            prefix,
            branch,
            child_name,
            if child_is_dir { "/" } else { "" }
        );

        if child_is_dir {
            let next_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}|   ", prefix)
            };
            print_tree_children(&child_path, &next_prefix, depth - 1)?;
        }
    }

    Ok(())
}

fn os_string_to_string(value: OsString) -> Result<String> {
    value
        .into_string()
        .map_err(|_| anyhow::anyhow!("encountered non-UTF-8 file name"))
}

fn run_command(command: &mut Command, context_message: &str) -> Result<()> {
    let output = command
        .output()
        .with_context(|| context_message.to_string())?;

    if !output.stdout.is_empty() {
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }
    if !output.stderr.is_empty() {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }

    if !output.status.success() {
        bail!("{}", context_message);
    }

    Ok(())
}

fn read_version_file(path: &Path) -> Result<String> {
    let content =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let parsed: PackageJson = serde_json::from_str(&content)
        .with_context(|| format!("invalid package.json at {}", path.display()))?;
    Ok(parsed.version.unwrap_or_else(|| "0.0.0".to_string()))
}

fn fetch_remote_version(url: &str) -> Result<String> {
    let response = reqwest::blocking::get(url)
        .with_context(|| format!("failed to request {}", url))?
        .error_for_status()
        .with_context(|| format!("remote request failed for {}", url))?;
    let parsed: PackageJson = response
        .json()
        .with_context(|| format!("failed to parse package.json from {}", url))?;
    Ok(parsed.version.unwrap_or_else(|| "0.0.0".to_string()))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VersionRelation {
    RemoteNewer,
    UpToDate,
    LocalAhead,
    Unknown,
}

fn compare_versions(local: &str, remote: &str) -> VersionRelation {
    match (Version::parse(local), Version::parse(remote)) {
        (Ok(local_v), Ok(remote_v)) => {
            if remote_v > local_v {
                VersionRelation::RemoteNewer
            } else if remote_v == local_v {
                VersionRelation::UpToDate
            } else {
                VersionRelation::LocalAhead
            }
        }
        _ => VersionRelation::Unknown,
    }
}
