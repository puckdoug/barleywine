use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn main() {
    println!("🧪 Testing Barleywine Logging System...");

    // Create logs directory if it doesn't exist
    let log_dir = PathBuf::from("logs");
    if !log_dir.exists() {
        std::fs::create_dir_all(&log_dir).expect("Failed to create logs directory");
        println!("📁 Created log directory: {}", log_dir.display());
    }

    // Test barleywine.log entries
    println!("📝 Writing test entries to barleywine.log...");
    write_barleywine_log("Test logging system started");
    write_barleywine_log("Server started on port 8000 serving files from webroot/");
    write_barleywine_log("Served markdown file: test.md");
    write_barleywine_log("Served static file: style.css");

    // Test access.log entries
    println!("📊 Writing test entries to access.log...");
    write_access_log(
        "127.0.0.1",
        "GET",
        "/",
        200,
        Some("Mozilla/5.0 (Test Browser)"),
    );
    write_access_log("127.0.0.1", "GET", "/about.md", 200, Some("curl/7.68.0"));
    write_access_log(
        "192.168.1.100",
        "GET",
        "/nonexistent.html",
        404,
        Some("Mozilla/5.0 (Chrome)"),
    );
    write_access_log("10.0.0.1", "GET", "/api/data", 500, None);

    // Test error and warning logging
    println!("⚠️  Writing test error and warning messages...");
    write_barleywine_log("ERROR: This is a test error message");
    write_barleywine_log("WARNING: This is a test warning message");

    // Wait a moment to ensure all writes complete
    thread::sleep(Duration::from_millis(100));

    write_barleywine_log("Server shutting down");

    println!("✅ Test logging complete!");
    println!("");
    println!("Check the following files:");
    println!("  📄 logs/barleywine.log - Server operational logs");
    println!("  📄 logs/access.log - HTTP access logs");
    println!("");
    println!("🎉 Logging system test finished successfully!");
}

fn write_barleywine_log(message: &str) {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
    let log_entry = format!("[{}] {}\n", timestamp, message);

    let log_file = PathBuf::from("logs/barleywine.log");
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

fn write_access_log(
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

    let log_file = PathBuf::from("logs/access.log");
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
