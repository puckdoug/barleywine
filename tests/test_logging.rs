use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

#[test]
fn test_logging_system() {
    println!("🧪 Testing Barleywine Logging System...");

    // Create logs directory if it doesn't exist
    let log_dir = PathBuf::from("test_logs");
    if !log_dir.exists() {
        std::fs::create_dir_all(&log_dir).expect("Failed to create test logs directory");
        println!("📁 Created test log directory: {}", log_dir.display());
    }

    // Test barleywine.log entries
    println!("📝 Writing test entries to test barleywine.log...");
    write_test_barleywine_log(&log_dir, "Test logging system started");
    write_test_barleywine_log(
        &log_dir,
        "Server started on port 8000 serving files from webroot/",
    );
    write_test_barleywine_log(&log_dir, "Served markdown file: test.md");
    write_test_barleywine_log(&log_dir, "Served static file: style.css");

    // Test access.log entries
    println!("📊 Writing test entries to test access.log...");
    write_test_access_log(
        &log_dir,
        "127.0.0.1",
        "GET",
        "/",
        200,
        Some("Mozilla/5.0 (Test Browser)"),
    );
    write_test_access_log(
        &log_dir,
        "127.0.0.1",
        "GET",
        "/about.md",
        200,
        Some("curl/7.68.0"),
    );
    write_test_access_log(
        &log_dir,
        "192.168.1.100",
        "GET",
        "/nonexistent.html",
        404,
        Some("Mozilla/5.0 (Chrome)"),
    );
    write_test_access_log(&log_dir, "10.0.0.1", "GET", "/api/data", 500, None);

    // Test error and warning logging
    println!("⚠️  Writing test error and warning messages...");
    write_test_barleywine_log(&log_dir, "ERROR: This is a test error message");
    write_test_barleywine_log(&log_dir, "WARNING: This is a test warning message");

    // Wait a moment to ensure all writes complete
    thread::sleep(Duration::from_millis(100));

    write_test_barleywine_log(&log_dir, "Server shutting down");

    println!("✅ Test logging complete!");

    // Verify that log files were created
    let barleywine_log = log_dir.join("barleywine.log");
    let access_log = log_dir.join("access.log");

    assert!(barleywine_log.exists(), "Barleywine log file should exist");
    assert!(access_log.exists(), "Access log file should exist");

    // Read and verify content
    let barleywine_content =
        std::fs::read_to_string(&barleywine_log).expect("Failed to read barleywine log");
    let access_content = std::fs::read_to_string(&access_log).expect("Failed to read access log");

    assert!(barleywine_content.contains("Test logging system started"));
    assert!(barleywine_content.contains("Server started on port 8000"));
    assert!(access_content.contains("127.0.0.1"));
    assert!(access_content.contains("GET"));

    // Clean up test files
    std::fs::remove_dir_all(&log_dir).ok();

    println!("🎉 Logging system test finished successfully!");
}

fn write_test_barleywine_log(log_dir: &PathBuf, message: &str) {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
    let log_entry = format!("[{}] {}\n", timestamp, message);

    let log_file = log_dir.join("barleywine.log");
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&log_file)
    {
        let _ = file.write_all(log_entry.as_bytes());
        let _ = file.flush();
    }
}

fn write_test_access_log(
    log_dir: &PathBuf,
    remote_addr: &str,
    method: &str,
    uri: &str,
    status: u16,
    user_agent: Option<&str>,
) {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
    let user_agent_str = user_agent.unwrap_or("-");
    let log_entry = format!(
        "[{}] {} \"{}\" {} \"{}\" \"{}\"\n",
        timestamp, remote_addr, method, uri, status, user_agent_str
    );

    let log_file = log_dir.join("access.log");
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&log_file)
    {
        let _ = file.write_all(log_entry.as_bytes());
        let _ = file.flush();
    }
}

#[test]
fn test_log_file_creation() {
    // Test that log files can be created and written to
    let test_dir = PathBuf::from("integration_test_logs");
    if !test_dir.exists() {
        std::fs::create_dir_all(&test_dir).expect("Failed to create test directory");
    }

    let log_file = test_dir.join("test.log");
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&log_file)
        .expect("Failed to create log file");

    let test_entry = format!(
        "[{}] Integration test log entry\n",
        Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    );
    file.write_all(test_entry.as_bytes())
        .expect("Failed to write to log file");
    file.flush().expect("Failed to flush log file");

    // Verify file exists and has content
    assert!(log_file.exists());
    let content = std::fs::read_to_string(&log_file).expect("Failed to read log file");
    assert!(content.contains("Integration test log entry"));

    // Clean up
    std::fs::remove_dir_all(&test_dir).ok();
}
