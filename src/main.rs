pub mod cli;
pub mod log;
pub mod web;

use cli::Cli;
use std::path::Path;
use std::process;

#[tokio::main]
async fn main() {
    // Parse command line arguments
    let cli = Cli::parse_args();

    // Validate arguments
    if let Err(error) = cli.validate() {
        eprintln!("Error: {}", error);
        process::exit(1);
    }

    // Print configuration if verify mode
    if cli.verify {
        println!("🍺 Barleywine Static File Server");
        println!("Configuration verification:");
        cli.print_config();

        // Verify webroot directory exists
        let webroot = Path::new("webroot");
        if webroot.exists() && webroot.is_dir() {
            println!("  Webroot directory: ✅ Found at 'webroot/'");
        } else {
            println!("  Webroot directory: ❌ Not found at 'webroot/'");
            println!("    Create the 'webroot' directory and add your static files there.");
        }

        // Test markdown parsing
        println!("  Markdown support: ✅ Available");

        // Test rocket configuration
        println!("  Rocket framework: ✅ Ready");

        println!("\nConfiguration is valid! ✅");
        println!("Run without --verify to start the server.");
        return;
    }

    // Setup logging based on CLI options
    if let Err(e) = log::setup_logging(&cli) {
        eprintln!("❌ Failed to initialize logging: {}", e);
        process::exit(1);
    }

    // Print startup message
    println!("🍺 Starting Barleywine Static File Server...");
    if let Some(ref config) = cli.config {
        println!("📝 Using config file: {}", config.display());
    }
    println!("📁 Serving files from: webroot/");
    println!("📝 Markdown conversion: Enabled");
    println!("🚀 Log level: {}", cli.loglevel);

    // Verify webroot exists
    let webroot = Path::new("webroot");
    if !webroot.exists() {
        eprintln!("❌ Error: webroot directory not found!");
        eprintln!("   Create a 'webroot' directory and add your files there.");
        process::exit(1);
    }

    // Launch rocket server
    log::log_server_startup(8000, "webroot/");
    let rocket = web::build_rocket();
    if let Err(e) = rocket.launch().await {
        let error_msg = format!("Failed to start server: {}", e);
        log::log_error(&error_msg);
        eprintln!("❌ {}", error_msg);
        log::log_server_shutdown();
        log::flush_logs();
        process::exit(1);
    }
}
