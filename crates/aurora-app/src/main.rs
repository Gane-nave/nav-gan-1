//! AURORA NAV / GMIN — Main Entry Point
//!
//! Usage:
//!   aurora-nav                          # run with defaults
//!   aurora-nav -c aurora.toml           # run with config file
//!   aurora-nav --port 9090              # override API port
//!   aurora-nav --dump-config            # print default config and exit
//!   aurora-nav --version                # print version and exit
//!   aurora-nav --status                 # print subsystem status and exit

use aurora_app::cli::CliArgs;
use aurora_app::health::build_health_report;
use aurora_app::pipeline::{print_banner, NavigationPipeline};
use aurora_config::{load_config, AuroraConfig, ConfigBuilder};
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    // --version
    if args.version {
        println!(
            "AURORA NAV / GMIN v{} — Global Mobility Intelligence Network",
            env!("CARGO_PKG_VERSION")
        );
        return Ok(());
    }

    // --dump-config
    if args.dump_config {
        println!("{}", aurora_config::defaults::default_toml());
        return Ok(());
    }

    // Load configuration
    let config = load_configuration(&args)?;

    // Initialise tracing
    init_tracing(&config.system.log_level);

    // Create the navigation pipeline
    let pipeline = NavigationPipeline::new(config.clone());

    // Print startup banner
    let status = pipeline.subsystem_status();
    print_banner(&status);

    // --status (print and exit)
    if args.status_only {
        let report = build_health_report(&pipeline);
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    // Build and start the API server
    let addr: SocketAddr = format!("{}:{}", config.api.host, config.api.port).parse()?;
    info!("Starting AURORA NAV API server on {}", addr);

    let state = std::sync::Arc::new(aurora_api::state::AppState {
        gnss: pipeline.gnss.clone(),
        fusion: pipeline.fusion.clone(),
        integrity: pipeline.integrity.clone(),
        continuity: pipeline.continuity.clone(),
        telemetry: pipeline.telemetry.clone(),
        event_bus: pipeline.event_bus.clone(),
        last_position: pipeline.last_position.clone(),
    });

    let app = aurora_api::server::build_router(state);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("AURORA NAV is ready — listening on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

fn load_configuration(args: &CliArgs) -> Result<AuroraConfig, Box<dyn std::error::Error>> {
    let mut config = if let Some(ref path) = args.config_path {
        let (cfg, result) = load_config(path)?;
        for w in &result.warnings {
            eprintln!("config warning: {}", w);
        }
        cfg
    } else {
        let mut builder = ConfigBuilder::new();
        // Apply CLI overrides
        if let Some(port) = args.port {
            builder = builder.api_port(port);
        }
        if let Some(ref level) = args.log_level {
            builder = builder.log_level(level);
        }
        builder.build_unchecked()
    };

    // CLI overrides take precedence over file values
    if let Some(port) = args.port {
        config.api.port = port;
    }
    if let Some(ref level) = args.log_level {
        config.system.log_level = level.clone();
    }

    Ok(config)
}

fn init_tracing(log_level: &str) {
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}
