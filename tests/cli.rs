use assert_cmd::Command;
use assert_fs::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;

fn barleywine_cmd() -> Command {
    Command::cargo_bin("barleywine").unwrap()
}

#[test]
fn test_help_long_flag() {
    barleywine_cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("A high-performance web platform"))
        .stdout(predicate::str::contains("USAGE:"));
}

#[test]
fn test_help_short_flag() {
    barleywine_cmd()
        .arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("USAGE:"))
        .stdout(predicate::str::contains("FLAGS:"));
}

#[test]
fn test_version_long_flag() {
    barleywine_cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("barleywine 0.1.0"));
}

#[test]
fn test_version_short_flag() {
    barleywine_cmd()
        .arg("-V")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_verify_flag_with_webroot() {
    let temp_dir = TempDir::new().unwrap();
    let webroot = temp_dir.child("webroot");
    webroot.create_dir_all().unwrap();

    barleywine_cmd()
        .arg("--verify")
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Configuration is valid!"))
        .stdout(predicate::str::contains("Webroot directory: ✅"));
}

#[test]
fn test_verify_flag_without_webroot() {
    let temp_dir = TempDir::new().unwrap();

    barleywine_cmd()
        .arg("--verify")
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Webroot directory: ❌"));
}

#[test]
fn test_valid_log_level_error() {
    let temp_dir = TempDir::new().unwrap();
    let webroot = temp_dir.child("webroot");
    webroot.create_dir_all().unwrap();

    barleywine_cmd()
        .args(&["--verify", "--loglevel", "error"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Log level: error"));
}

#[test]
fn test_valid_log_level_warn() {
    let temp_dir = TempDir::new().unwrap();
    let webroot = temp_dir.child("webroot");
    webroot.create_dir_all().unwrap();

    barleywine_cmd()
        .args(&["--verify", "--loglevel", "warn"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Log level: warn"));
}

#[test]
fn test_valid_log_level_info() {
    let temp_dir = TempDir::new().unwrap();
    let webroot = temp_dir.child("webroot");
    webroot.create_dir_all().unwrap();

    barleywine_cmd()
        .args(&["--verify", "--loglevel", "info"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Log level: info"));
}

#[test]
fn test_valid_log_level_debug() {
    let temp_dir = TempDir::new().unwrap();
    let webroot = temp_dir.child("webroot");
    webroot.create_dir_all().unwrap();

    barleywine_cmd()
        .args(&["--verify", "--loglevel", "debug"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Log level: debug"));
}

#[test]
fn test_valid_log_level_trace() {
    let temp_dir = TempDir::new().unwrap();
    let webroot = temp_dir.child("webroot");
    webroot.create_dir_all().unwrap();

    barleywine_cmd()
        .args(&["--verify", "--loglevel", "trace"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Log level: trace"));
}

#[test]
fn test_invalid_log_level() {
    barleywine_cmd()
        .args(&["--loglevel", "invalid"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid log level"));
}

#[test]
fn test_log_level_case_insensitive_uppercase() {
    let temp_dir = TempDir::new().unwrap();
    let webroot = temp_dir.child("webroot");
    webroot.create_dir_all().unwrap();

    barleywine_cmd()
        .args(&["--verify", "--loglevel", "INFO"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Log level: INFO"));
}

#[test]
fn test_log_level_case_insensitive_mixed() {
    let temp_dir = TempDir::new().unwrap();
    let webroot = temp_dir.child("webroot");
    webroot.create_dir_all().unwrap();

    barleywine_cmd()
        .args(&["--verify", "--loglevel", "Debug"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Log level: Debug"));
}

#[test]
fn test_valid_config_file() {
    let temp_dir = TempDir::new().unwrap();
    let webroot = temp_dir.child("webroot");
    webroot.create_dir_all().unwrap();

    let config_file = temp_dir.child("config.toml");
    config_file.write_str("[server]\nport = 8000\n").unwrap();

    barleywine_cmd()
        .args(&["--config", "config.toml", "--verify"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Config file: config.toml"));
}

#[test]
fn test_config_file_short_form() {
    let temp_dir = TempDir::new().unwrap();
    let webroot = temp_dir.child("webroot");
    webroot.create_dir_all().unwrap();

    let config_file = temp_dir.child("config.toml");
    config_file.write_str("[server]\nport = 8000\n").unwrap();

    barleywine_cmd()
        .args(&["-c", "config.toml", "--verify"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Config file: config.toml"));
}

#[test]
fn test_nonexistent_config_file() {
    barleywine_cmd()
        .args(&["--config", "nonexistent.toml", "--verify"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"));
}

#[test]
fn test_valid_log_file() {
    let temp_dir = TempDir::new().unwrap();
    let webroot = temp_dir.child("webroot");
    webroot.create_dir_all().unwrap();

    let logs_dir = temp_dir.child("logs");
    logs_dir.create_dir_all().unwrap();

    barleywine_cmd()
        .args(&["--log", "logs/test.log", "--verify"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Log file: logs/test.log"));
}

#[test]
fn test_log_file_nonexistent_directory() {
    barleywine_cmd()
        .args(&["--log", "baddir/test.log"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"));
}

#[test]
fn test_multiple_valid_options() {
    let temp_dir = TempDir::new().unwrap();
    let webroot = temp_dir.child("webroot");
    webroot.create_dir_all().unwrap();

    let config_file = temp_dir.child("config.toml");
    config_file.write_str("[server]\nport = 8000\n").unwrap();

    let logs_dir = temp_dir.child("logs");
    logs_dir.create_dir_all().unwrap();

    barleywine_cmd()
        .args(&[
            "--config",
            "config.toml",
            "--loglevel",
            "debug",
            "--log",
            "logs/test.log",
            "--verify",
        ])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Configuration is valid!"))
        .stdout(predicate::str::contains("Config file: config.toml"))
        .stdout(predicate::str::contains("Log level: debug"))
        .stdout(predicate::str::contains("Log file: logs/test.log"));
}

#[test]
fn test_default_values() {
    let temp_dir = TempDir::new().unwrap();
    let webroot = temp_dir.child("webroot");
    webroot.create_dir_all().unwrap();

    barleywine_cmd()
        .arg("--verify")
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Log level: info"))
        .stdout(predicate::str::contains("Config file: default"))
        .stdout(predicate::str::contains("Log file: stdout"));
}

#[test]
fn test_markdown_support_detection() {
    let temp_dir = TempDir::new().unwrap();
    let webroot = temp_dir.child("webroot");
    webroot.create_dir_all().unwrap();

    barleywine_cmd()
        .arg("--verify")
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Markdown support: ✅"));
}

#[test]
fn test_rocket_framework_detection() {
    let temp_dir = TempDir::new().unwrap();
    let webroot = temp_dir.child("webroot");
    webroot.create_dir_all().unwrap();

    barleywine_cmd()
        .arg("--verify")
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Rocket framework: ✅"));
}

#[test]
fn test_empty_log_level_fails() {
    barleywine_cmd()
        .args(&["--loglevel", "", "--verify"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid log level"));
}

#[test]
fn test_verify_only_flag() {
    let temp_dir = TempDir::new().unwrap();
    let webroot = temp_dir.child("webroot");
    webroot.create_dir_all().unwrap();

    barleywine_cmd()
        .arg("--verify")
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Verify only: true"))
        .stdout(predicate::str::contains(
            "Run without --verify to start the server",
        ));
}

#[test]
fn test_config_validation_summary() {
    let temp_dir = TempDir::new().unwrap();
    let webroot = temp_dir.child("webroot");
    webroot.create_dir_all().unwrap();

    barleywine_cmd()
        .arg("--verify")
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("🍺 Barleywine Static File Server"))
        .stdout(predicate::str::contains("Configuration verification:"))
        .stdout(predicate::str::contains("Barleywine Configuration:"));
}

#[test]
fn test_absolute_config_path() {
    let temp_dir = TempDir::new().unwrap();
    let webroot = temp_dir.child("webroot");
    webroot.create_dir_all().unwrap();

    let config_file = temp_dir.child("config.toml");
    config_file.write_str("[server]\nport = 8000\n").unwrap();

    let absolute_path = config_file.path().to_str().unwrap();

    barleywine_cmd()
        .args(&["--config", absolute_path, "--verify"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Configuration is valid!"));
}

#[test]
fn test_absolute_log_path() {
    let temp_dir = TempDir::new().unwrap();
    let webroot = temp_dir.child("webroot");
    webroot.create_dir_all().unwrap();

    let logs_dir = temp_dir.child("logs");
    logs_dir.create_dir_all().unwrap();

    let log_file = logs_dir.child("test.log");
    let absolute_path = log_file.path().to_str().unwrap();

    barleywine_cmd()
        .args(&["--log", absolute_path, "--verify"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Configuration is valid!"));
}

#[test]
fn test_server_startup_messages_not_shown_in_verify() {
    let temp_dir = TempDir::new().unwrap();
    let webroot = temp_dir.child("webroot");
    webroot.create_dir_all().unwrap();

    barleywine_cmd()
        .arg("--verify")
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Configuration verification:"))
        .stdout(predicate::str::contains(
            "Run without --verify to start the server",
        ))
        // Should NOT contain server startup messages
        .stdout(predicate::str::contains("Starting Barleywine").not());
}

#[test]
fn test_log_level_in_verify_output() {
    let temp_dir = TempDir::new().unwrap();
    let webroot = temp_dir.child("webroot");
    webroot.create_dir_all().unwrap();

    for level in &["error", "warn", "info", "debug", "trace"] {
        barleywine_cmd()
            .args(&["--verify", "--loglevel", level])
            .current_dir(temp_dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains(format!("Log level: {}", level)));
    }
}

#[test]
fn test_configuration_with_complex_paths() {
    let temp_dir = TempDir::new().unwrap();
    let webroot = temp_dir.child("webroot");
    webroot.create_dir_all().unwrap();

    // Create nested directories
    let nested_config_dir = temp_dir.child("config").child("environments");
    nested_config_dir.create_dir_all().unwrap();
    let config_file = nested_config_dir.child("test.toml");
    config_file.write_str("[server]\nport = 8000\n").unwrap();

    let nested_log_dir = temp_dir.child("var").child("log").child("barleywine");
    nested_log_dir.create_dir_all().unwrap();

    barleywine_cmd()
        .args(&[
            "--config",
            "config/environments/test.toml",
            "--log",
            "var/log/barleywine/test.log",
            "--verify",
        ])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Configuration is valid!"));
}
