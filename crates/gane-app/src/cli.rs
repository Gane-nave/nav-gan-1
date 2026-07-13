//! Command-line argument parsing.

use std::path::PathBuf;

/// Parsed command-line arguments.
#[derive(Debug, Clone)]
pub struct CliArgs {
    /// Path to the TOML configuration file.
    pub config_path: Option<PathBuf>,
    /// Override API port.
    pub port: Option<u16>,
    /// Override log level.
    pub log_level: Option<String>,
    /// Dump the default configuration and exit.
    pub dump_config: bool,
    /// Print version and exit.
    pub version: bool,
    /// Print subsystem status and exit.
    pub status_only: bool,
}

impl CliArgs {
    /// Parse arguments from `std::env::args`.
    pub fn parse() -> Self {
        let args: Vec<String> = std::env::args().collect();
        Self::from_args(&args)
    }

    /// Parse from an explicit arg list (useful for testing).
    pub fn from_args(args: &[String]) -> Self {
        let mut cli = CliArgs {
            config_path: None,
            port: None,
            log_level: None,
            dump_config: false,
            version: false,
            status_only: false,
        };

        let mut i = 1; // skip binary name
        while i < args.len() {
            match args[i].as_str() {
                "--config" | "-c" => {
                    if i + 1 < args.len() {
                        cli.config_path = Some(PathBuf::from(&args[i + 1]));
                        i += 1;
                    }
                }
                "--port" | "-p" => {
                    if i + 1 < args.len() {
                        cli.port = args[i + 1].parse().ok();
                        i += 1;
                    }
                }
                "--log-level" | "-l" => {
                    if i + 1 < args.len() {
                        cli.log_level = Some(args[i + 1].clone());
                        i += 1;
                    }
                }
                "--dump-config" => cli.dump_config = true,
                "--version" | "-V" => cli.version = true,
                "--status" => cli.status_only = true,
                _ => {} // ignore unknown args
            }
            i += 1;
        }

        cli
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &str) -> Vec<String> {
        s.split_whitespace().map(String::from).collect()
    }

    #[test]
    fn default_args() {
        let cli = CliArgs::from_args(&args("gane-nav"));
        assert!(cli.config_path.is_none());
        assert!(cli.port.is_none());
        assert!(!cli.dump_config);
        assert!(!cli.version);
    }

    #[test]
    fn config_path_parsed() {
        let cli = CliArgs::from_args(&args("gane-nav --config /etc/gane.toml"));
        assert_eq!(cli.config_path.unwrap(), PathBuf::from("/etc/gane.toml"));
    }

    #[test]
    fn short_config_flag() {
        let cli = CliArgs::from_args(&args("gane-nav -c gane.toml"));
        assert_eq!(cli.config_path.unwrap(), PathBuf::from("gane.toml"));
    }

    #[test]
    fn port_override() {
        let cli = CliArgs::from_args(&args("gane-nav --port 9090"));
        assert_eq!(cli.port, Some(9090));
    }

    #[test]
    fn log_level_override() {
        let cli = CliArgs::from_args(&args("gane-nav --log-level debug"));
        assert_eq!(cli.log_level.unwrap(), "debug");
    }

    #[test]
    fn dump_config_flag() {
        let cli = CliArgs::from_args(&args("gane-nav --dump-config"));
        assert!(cli.dump_config);
    }

    #[test]
    fn version_flag() {
        let cli = CliArgs::from_args(&args("gane-nav --version"));
        assert!(cli.version);
    }

    #[test]
    fn combined_flags() {
        let cli = CliArgs::from_args(&args(
            "gane-nav -c config.toml --port 8080 -l warn --status",
        ));
        assert_eq!(cli.config_path.unwrap(), PathBuf::from("config.toml"));
        assert_eq!(cli.port, Some(8080));
        assert_eq!(cli.log_level.unwrap(), "warn");
        assert!(cli.status_only);
    }
}
