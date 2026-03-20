//! Default configuration TOML template.

/// Returns the default configuration as a TOML string.
/// Useful for generating a starter `aurora.toml` file.
pub fn default_toml() -> String {
    let cfg = crate::sections::AuroraConfig::default();
    toml::to_string_pretty(&cfg).expect("default config must serialise")
}

/// Returns a minimal configuration TOML with only the most commonly
/// changed values.
pub fn minimal_toml() -> &'static str {
    r#"# AURORA NAV / GMIN — Minimal Configuration
# Full reference: run `aurora-nav --dump-config`

[system]
instance_name = "aurora-nav"
log_level = "info"

[gnss]
min_satellites = 4
elevation_mask_deg = 10.0

[api]
host = "0.0.0.0"
port = 3000

[telemetry]
buffer_capacity = 50000
enable_audit = true
"#
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_toml_is_parseable() {
        let toml_str = default_toml();
        let _cfg: crate::sections::AuroraConfig =
            toml::from_str(&toml_str).expect("default TOML must parse");
    }

    #[test]
    fn minimal_toml_is_parseable() {
        let toml_str = minimal_toml();
        let cfg: crate::sections::AuroraConfig =
            toml::from_str(toml_str).expect("minimal TOML must parse");
        assert_eq!(cfg.api.port, 3000);
        assert_eq!(cfg.gnss.min_satellites, 4);
    }
}
