//! AURORA NAV — Deployment Manifests
//!
//! This crate contains deployment configuration files:
//!
//! - `Dockerfile` — Multi-stage production build
//! - `docker-compose.yml` — Local development stack
//! - `k8s/` — Kubernetes manifests (Deployment, Service, HPA, PDB)
//!
//! No Rust code is compiled from this crate; it exists as a workspace
//! member to keep deployment artefacts version-controlled alongside
//! the source.

/// Returns the embedded Dockerfile content.
pub fn dockerfile() -> &'static str {
    include_str!("../Dockerfile")
}

/// Returns the embedded docker-compose.yml content.
pub fn docker_compose() -> &'static str {
    include_str!("../docker-compose.yml")
}

/// Returns the embedded Kubernetes deployment manifest.
pub fn k8s_deployment() -> &'static str {
    include_str!("../k8s/deployment.yaml")
}

/// Returns the embedded Kubernetes service manifest.
pub fn k8s_service() -> &'static str {
    include_str!("../k8s/service.yaml")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dockerfile_not_empty() {
        let content = dockerfile();
        assert!(!content.is_empty());
        assert!(content.contains("FROM"));
        assert!(content.contains("aurora-nav"));
    }

    #[test]
    fn docker_compose_not_empty() {
        let content = docker_compose();
        assert!(!content.is_empty());
        assert!(content.contains("services"));
        assert!(content.contains("aurora-nav"));
    }

    #[test]
    fn k8s_deployment_not_empty() {
        let content = k8s_deployment();
        assert!(!content.is_empty());
        assert!(content.contains("Deployment"));
        assert!(content.contains("aurora-nav"));
    }

    #[test]
    fn k8s_service_not_empty() {
        let content = k8s_service();
        assert!(!content.is_empty());
        assert!(content.contains("Service"));
        assert!(content.contains("aurora-nav"));
    }

    #[test]
    fn dockerfile_has_health_check() {
        let content = dockerfile();
        assert!(content.contains("HEALTHCHECK"));
    }

    #[test]
    fn docker_compose_has_health_check() {
        let content = docker_compose();
        assert!(content.contains("healthcheck"));
    }

    #[test]
    fn k8s_has_probes() {
        let content = k8s_deployment();
        assert!(content.contains("livenessProbe"));
        assert!(content.contains("readinessProbe"));
        assert!(content.contains("startupProbe"));
    }
}
