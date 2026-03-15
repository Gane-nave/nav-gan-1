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
use aurora_config::loader::apply_env_overrides;
use aurora_config::{load_config, AuroraConfig, ConfigBuilder};
use aurora_metrics::registry::MetricRegistry;
use aurora_observability::probes::ProbeManager;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;

// Re-export validate for post-override re-validation.
use aurora_config::validate;

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

    let metrics = Arc::new(MetricRegistry::new());
    let probes = Arc::new(ProbeManager::new());

    let state = Arc::new(aurora_api::state::AppState {
        gnss: pipeline.gnss.clone(),
        fusion: pipeline.fusion.clone(),
        integrity: pipeline.integrity.clone(),
        continuity: pipeline.continuity.clone(),
        telemetry: pipeline.telemetry.clone(),
        event_bus: pipeline.event_bus.clone(),
        last_position: pipeline.last_position.clone(),
        metrics,
        probes: probes.clone(),
    });

    // Mark probes as started and ready
    probes.mark_started();

    let app = aurora_api::server::build_router(state);
    let listener = tokio::net::TcpListener::bind(addr).await?;

    // Mark ready to receive traffic
    probes.mark_ready();
    info!("AURORA NAV is ready — listening on {}", addr);

    // Graceful shutdown: listen for SIGTERM/SIGINT
    let probes_shutdown = probes.clone();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(probes_shutdown))
        .await?;

    info!("AURORA NAV shutdown complete");
    Ok(())
}

/// Wait for a shutdown signal (SIGTERM or SIGINT).
async fn shutdown_signal(probes: Arc<ProbeManager>) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { info!("Received SIGINT, initiating graceful shutdown..."); },
        _ = terminate => { info!("Received SIGTERM, initiating graceful shutdown..."); },
    }

    // Mark not ready so load balancers stop sending traffic
    probes.mark_not_ready();
    info!("Draining in-flight requests...");
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
        let (mut cfg, result) = builder
            .build()
            .map_err(|e| -> Box<dyn std::error::Error> { Box::new(e) })?;
        // Apply AURORA_* env overrides (Dockerfile/k8s env vars)
        apply_env_overrides(&mut cfg).map_err(|e| -> Box<dyn std::error::Error> { Box::new(e) })?;
        for w in &result.warnings {
            eprintln!("config warning: {}", w);
        }
        cfg
    };

    // CLI overrides take precedence over file values
    if let Some(port) = args.port {
        config.api.port = port;
    }
    if let Some(ref level) = args.log_level {
        config.system.log_level = level.clone();
    }

    // Re-validate after CLI overrides to catch invalid override values.
    validate::validate(&config).map_err(|e| -> Box<dyn std::error::Error> { Box::new(e) })?;

    Ok(config)
}

fn init_tracing(log_level: &str) {
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}
