use std::env;
use std::path::PathBuf;

use anyhow::{Context, Result};

pub const COWORK_HOME_ENV: &str = "COWORK_HOME";
pub const COWORK_CLONE_REPO_URL_ENV: &str = "COWORK_CLONE_REPO_URL";
pub const COWORK_CLONE_REPO_ALIAS_ENV: &str = "COWORK_CLONE_REPO_ALIAS";
pub const COWORK_CLONE_SESSION_ENV: &str = "COWORK_CLONE_SESSION";
pub const COWORK_CLONE_PREVIEW_CMD_ENV: &str = "COWORK_CLONE_PREVIEW_CMD";
pub const COWORK_CLONE_METADATA_TREE_DEPTH_ENV: &str = "COWORK_CLONE_METADATA_TREE_DEPTH";
pub const COWORK_CLONE_PACKAGE_URL_ENV: &str = "COWORK_CLONE_PACKAGE_URL";
pub const COWORK_SELF_UPDATE_INSTALL_URL_ENV: &str = "COWORK_SELF_UPDATE_INSTALL_URL";

pub const DEFAULT_ALIAS: &str = "default";
pub const DEFAULT_SESSION: &str = "cowork-preview";
pub const DEFAULT_PREVIEW_CMD: &str = "npm run dev";
pub const DEFAULT_METADATA_TREE_DEPTH: usize = 3;
pub const DEFAULT_INSTALL_URL: &str =
    "https://raw.githubusercontent.com/powerformer/cowork-cli/main/install.sh";

pub fn cowork_home() -> Result<PathBuf> {
    match env::var(COWORK_HOME_ENV) {
        Ok(value) => Ok(PathBuf::from(value)),
        Err(env::VarError::NotPresent) => {
            let home = env::var("HOME").context("HOME is required when COWORK_HOME is not set")?;
            Ok(PathBuf::from(home).join(".cowork"))
        }
        Err(env::VarError::NotUnicode(_)) => {
            anyhow::bail!("{} must be valid UTF-8", COWORK_HOME_ENV)
        }
    }
}

pub fn repo_alias() -> String {
    read_env_or_default(COWORK_CLONE_REPO_ALIAS_ENV, DEFAULT_ALIAS)
}

pub fn clone_session() -> String {
    read_env_or_default(COWORK_CLONE_SESSION_ENV, DEFAULT_SESSION)
}

pub fn preview_cmd() -> String {
    read_env_or_default(COWORK_CLONE_PREVIEW_CMD_ENV, DEFAULT_PREVIEW_CMD)
}

pub fn metadata_tree_depth() -> Result<usize> {
    let raw = read_env_or_default(
        COWORK_CLONE_METADATA_TREE_DEPTH_ENV,
        &DEFAULT_METADATA_TREE_DEPTH.to_string(),
    );

    raw.parse::<usize>().with_context(|| {
        format!(
            "{} must be a non-negative integer",
            COWORK_CLONE_METADATA_TREE_DEPTH_ENV
        )
    })
}

pub fn install_url() -> String {
    read_env_or_default(COWORK_SELF_UPDATE_INSTALL_URL_ENV, DEFAULT_INSTALL_URL)
}

pub fn required_repo_url() -> Result<String> {
    let value = env::var(COWORK_CLONE_REPO_URL_ENV)
        .with_context(|| format!("{} is required", COWORK_CLONE_REPO_URL_ENV))?;
    let trimmed = value.trim();
    if trimmed.is_empty() {
        anyhow::bail!("{} is required", COWORK_CLONE_REPO_URL_ENV);
    }
    Ok(trimmed.to_string())
}

fn read_env_or_default(key: &str, default: &str) -> String {
    match env::var(key) {
        Ok(value) if !value.trim().is_empty() => value,
        _ => default.to_string(),
    }
}
