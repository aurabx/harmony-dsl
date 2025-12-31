//! # harmony-dsl
//!
//! TOML Schema DSL definitions for Harmony Proxy configuration files.
//!
//! This crate contains schema definitions used for validating Harmony Proxy
//! configuration files:
//!
//! - `harmony-config-schema.toml` - Main gateway configuration schema
//! - `harmony-pipeline-schema.toml` - Pipeline configuration schema
//! - `harmony-mesh-schema.toml` - Data mesh configuration schema
//!
//! ## Usage
//!
//! These schema files are included in the crate and can be accessed by
//! consumers for validation purposes. The schema files are located in the
//! crate root and can be referenced when building validators.
//!
//! ## Schema Files
//!
//! ### harmony-config-schema.toml
//!
//! Defines the structure for main gateway configuration files (`config.toml`).
//! Includes validation rules for:
//! - Proxy settings (ID, logging, paths)
//! - Network definitions (HTTP listeners, WireGuard)
//! - Management API configuration
//! - Storage backends (filesystem, S3)
//! - Service type registry
//!
//! ### harmony-pipeline-schema.toml
//!
//! Defines the structure for pipeline configuration files (`pipelines/*.toml`).
//! Includes validation rules for:
//! - Pipeline routing definitions
//! - Endpoint configurations (how requests arrive)
//! - Backend configurations (where requests go)
//! - Middleware instances (transform, auth, etc.)
//!
//! ### harmony-mesh-schema.toml
//!
//! Defines the structure for data mesh configuration files (`mesh/*.toml`).
//! Includes validation rules for:
//! - Mesh definitions (grouping of ingress/egress points)
//! - Ingress configurations (how mesh requests arrive)
//! - Egress configurations (how mesh requests are sent)
//!
//! ## Cross-Language Support
//!
//! These schemas are designed to work with both:
//! - **Rust** (harmony-proxy) - Local validation before proxy startup
//! - **PHP** (Runbeam Cloud API) - Remote validation and configuration management
//!
//! ## Version
//!
//! Schema version: Config 1.11.0 / Pipeline 1.10.0 / Mesh 1.11.0
//!
//! ## License
//!
//! MIT License - See LICENSE file for details

/// The contents of the harmony-config-schema.toml file
pub const CONFIG_SCHEMA: &str = include_str!("../harmony-config-schema.toml");

/// The contents of the harmony-pipeline-schema.toml file
pub const PIPELINE_SCHEMA: &str = include_str!("../harmony-pipeline-schema.toml");

/// The contents of the harmony-mesh-schema.toml file
pub const MESH_SCHEMA: &str = include_str!("../harmony-mesh-schema.toml");

/// The contents of the harmony-remote-ingress-schema.toml file
pub const REMOTE_INGRESS_SCHEMA: &str =
    include_str!("../harmony-remote-ingress-schema.toml");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_schema_is_not_empty() {
        assert!(!CONFIG_SCHEMA.is_empty());
        assert!(CONFIG_SCHEMA.contains("[schema]"));
    }

    #[test]
    fn config_schema_has_provider_table() {
        assert!(CONFIG_SCHEMA.contains("provider.*"));
        assert!(CONFIG_SCHEMA.contains("Provider configuration for resource resolution"));
    }

    #[test]
    fn pipeline_schema_is_not_empty() {
        assert!(!PIPELINE_SCHEMA.is_empty());
        assert!(PIPELINE_SCHEMA.contains("[schema]"));
    }

    #[test]
    fn mesh_schema_is_not_empty() {
        assert!(!MESH_SCHEMA.is_empty());
        assert!(MESH_SCHEMA.contains("[schema]"));
    }

    #[test]
    fn remote_ingress_schema_is_not_empty() {
        assert!(!REMOTE_INGRESS_SCHEMA.is_empty());
        assert!(REMOTE_INGRESS_SCHEMA.contains("[schema]"));
    }
}
