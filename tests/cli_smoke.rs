use assert_cmd::Command;
use std::env;
use std::fs;
use std::path::PathBuf;

fn temp_home(name: &str) -> PathBuf {
    let mut path = env::temp_dir();
    path.push(format!("cowork-tests-{}-{}", name, std::process::id()));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("create temp home");
    path
}

#[test]
fn help_command_works() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_cowork"));
    let output = cmd
        .arg("--help")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8_lossy(&output);
    assert!(text.contains("self-update"));
    assert!(text.contains("clone"));
}

#[test]
fn clone_subcommand_help_works() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_cowork"));
    let output = cmd
        .args(["clone", "--help"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8_lossy(&output);
    assert!(text.contains("version"));
    assert!(text.contains("init"));
    assert!(text.contains("update"));
    assert!(text.contains("metadata"));
    assert!(text.contains("preview"));
    assert!(text.contains("contribute"));
    assert!(text.contains("resource"));
}

#[test]
fn clone_init_requires_repo_url_env() {
    let home = temp_home("init-missing-url");
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_cowork"));
    cmd.env("COWORK_HOME", home)
        .env_remove("COWORK_CLONE_REPO_URL")
        .args(["clone", "init"])
        .assert()
        .failure();
}

#[test]
fn clone_update_fails_when_clone_path_missing() {
    let home = temp_home("update-missing-path");
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_cowork"));
    cmd.env("COWORK_HOME", home)
        .env("COWORK_CLONE_REPO_ALIAS", "missing")
        .args(["clone", "update"])
        .assert()
        .failure();
}

#[test]
fn clone_init_fails_if_target_exists() {
    let home = temp_home("init-existing-target");
    let target = home.join("clones").join("default");
    fs::create_dir_all(&target).expect("create existing target");

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_cowork"));
    cmd.env("COWORK_HOME", home)
        .env("COWORK_CLONE_REPO_URL", "https://example.com/repo.git")
        .args(["clone", "init"])
        .assert()
        .failure();
}
