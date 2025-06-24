pub mod cli;
pub mod config;
pub mod log;
pub mod web;

use ::log::info;
use cli::Cli;
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

    // Initialize configuration
    if let Err(e) = config::init_config(cli.config.as_deref()) {
        eprintln!("❌ Failed to initialize configuration: {}", e);
        process::exit(1);
    }
    let config = config::get_config();

    // Print configuration if verify mode
    if cli.verify {
        config.print_config(
            cli.config.as_deref(),
            Some(&cli.loglevel),
            cli.log.as_deref(),
            cli.verify,
        );
        config.print_summary();

        // Verify webroot directory exists
        if config.content.webroot.exists() && config.content.webroot.is_dir() {
            println!(
                "  Webroot directory: ✅ Found at '{}'",
                config.content.webroot.display()
            );
        } else {
            println!(
                "  Webroot directory: ❌ Not found at '{}'",
                config.content.webroot.display()
            );
            println!(
                "    Create the '{}' directory and add your static files there.",
                config.content.webroot.display()
            );
        }

        // Test markdown parsing
        if config.content.markdown_enabled {
            println!("  Markdown support: ✅ Enabled");
        } else {
            println!("  Markdown support: ⚠️  Disabled");
        }

        // Test rocket configuration
        println!("  Rocket framework: ✅ Ready");

        println!("\nConfiguration is valid! ✅");
        println!("Run without --verify to start the server.");
        return;
    }

    // Setup logging based on CLI options and config (only after verify check)
    if let Err(e) = log::setup_logging(&cli) {
        eprintln!("❌ Failed to initialize logging: {}", e);
        process::exit(1);
    }

    // Log startup information
    info!("🍺 Starting Barleywine...");
    if let Some(ref config_file) = cli.config {
        info!("📝 Using config file: {}", config_file.display());
    }
    info!(
        "📁 Serving files from: {}",
        config.content.webroot.display()
    );
    info!(
        "📝 Markdown conversion: {}",
        if config.content.markdown_enabled {
            "Enabled"
        } else {
            "Disabled"
        }
    );
    info!(
        "🚀 Log level: {}",
        config.get_log_level(Some(&cli.loglevel))
    );
    info!(
        "🌐 Server will bind to: {}:{}",
        config.server.host, config.server.port
    );

    // Verify webroot exists (should be validated by config, but double-check)
    if !config.content.webroot.exists() {
        eprintln!(
            "❌ Error: webroot directory not found at '{}'!",
            config.content.webroot.display()
        );
        eprintln!(
            "   Create the '{}' directory and add your files there.",
            config.content.webroot.display()
        );
        process::exit(1);
    }

    // Launch rocket server
    log::log_server_startup(
        config.server.port,
        &config.content.webroot.display().to_string(),
    );
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
