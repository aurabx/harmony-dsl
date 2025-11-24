# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.8.0] - 2025-11-24

### Breaking Changes
- **Global Authentication Architecture**:
  - Introduced `authentications.*` table in config schema for reusable auth definitions.
  - `authentication` field in `peers`, `targets`, `endpoints`, `backends` is now a string reference ID instead of an inline table.
  - Middleware authentication (e.g., for `jwt_auth`) now uses the `authentication` reference instead of inline options.
  - Removed JWT and Basic Auth specific options from middleware configuration.
- **Global Policies and Rules**:
  - Moved `policies.*` and `rules.*` definitions from Pipeline schema to Config schema to make them globally available.

### Added
- New `peers.*` table for defining source systems.
- New `targets.*` table for defining destination systems.
- Enhanced JWT configuration options (now in `authentications`): `trusted_issuers`, `jwks_uri`, `algorithms`, `required_claims`.

### Changed
- Schema version synchronized to 1.8.0 for both `config` and `pipeline`.

## [1.7.0] - 2025-01-16

### Breaking Changes
- **Policies and Rules Structure Refactored**: Both policies and rules are now defined as top-level tables
- Middleware references policies by ID: `options.policies = ['policy_id']`
- Policies reference rules by ID: `rules = ['rule_id']`
- This enables full reusability of both policies and rules across the pipeline

### Added
- New top-level `policies.*` table for policy definitions
- Policy fields: `id` (required), `name`, `enabled`, `rules` (array of rule IDs)
- New top-level `rules.*` table for rule definitions
- Rule fields: `id` (required), `name`, `type` (required), `weight`, `enabled`, `options` (table)
- Middleware `options.policies` now accepts array of policy IDs (strings)
- `example-pipeline-with-rules.toml` demonstrating new structure
- `MIGRATION-v1.7.md` comprehensive migration guide from v1.6 to v1.7

### Removed
- Nested `options.policies` array-of-tables structure for inline policy definitions
- All nested policy field definitions: `options.policies.{id,name,enabled}`
- Nested `options.policies.rules` array
- Pattern-based rule options: `options.policies.*.rule_*.options`

### Changed
- Schema version bumped from 1.6.0 to 1.7.0 in `harmony-pipeline-schema.toml`
- Cargo.toml version updated to 1.7.0
- Policy definitions moved from middleware-scoped to pipeline-scoped (top-level)
- Rule definitions moved from policy-scoped to pipeline-scoped (top-level)
- Three-tier structure: Middleware → Policies → Rules (all reference-based)
- Improved separation: middleware orchestrates, policies group, rules execute

### Migration
See `MIGRATION-v1.7.md` for complete migration instructions. Key changes:
1. Extract nested policies to top-level `[policies.*]` tables
2. Extract nested rules to top-level `[rules.*]` tables
3. Update middleware to reference policies by ID: `options.policies = ['policy_id']`
4. Update policies to reference rules by ID: `rules = ['rule_id']`

## [1.5.0] - 2025-11-12

### Added
- Support for policies middleware type with nested rules
- `options.policies` array field for middleware configurations
- Policy fields: `id`, `name`, `enabled` (boolean)
- Dynamic policy rules: `options.policies.<policy_name>.rules` array
- Rule fields: `id`, `name`, `type` (unrestricted), `weight`, `enabled` (boolean)
- Rule-specific options table: `options.policies.<policy_name>.rule_<rule_id>.options` (open table)

### Changed
- Schema version bumped from 1.4.0 to 1.5.0 in `harmony-pipeline-schema.toml`

## [1.4.2] - 2025-11-10

### Added
- Cargo/crates.io support for Rust projects
- `src/lib.rs` with `CONFIG_SCHEMA` and `PIPELINE_SCHEMA` constants
- Rust usage examples in README.md

### Changed
- Filesystem storage `options.path` is now optional with default value `./tmp`
- Updated `.gitignore` to exclude Rust build artifacts

### Fixed
- Filesystem path configuration now has sensible defaults

## [1.4.1] - 2024-11-06

### Added
- Composer support for PHP projects

## [1.4.0] - 2024-11-06

### Added
- Improved `path_filter` options for pipelines
- Schema cleanup and refinements

## [1.3] - 2024-11-05

### Fixed
- Runbeam Cloud API configuration options

## [1.2] - 2024-11-05

### Changed
- Version bump with configuration improvements

## [1.1] - 2024-11-05

### Added
- ID fields for configuration entities
- Name constraint validation

### Changed
- `log_level` moved to logging section property
- `http` key renamed to `tcp_config`
- Added `poll_interval_secs` and `cloud_api_base_url` configuration options

## [1.0] - 2024-11-05

### Added
- Initial release
- `harmony-config-schema.toml` - Main gateway configuration schema
- `harmony-pipeline-schema.toml` - Pipeline configuration schema
- Schema DSL specification and documentation
- Cross-language validation support (Rust and PHP)

## [1.6.0] - 2025-01-13
- Added Types to Services

[1.7.0]: https://github.com/aurabx/harmony-dsl/compare/1.6.0...1.7.0
[1.6.0]: https://github.com/aurabx/harmony-dsl/compare/1.5.0...1.6.0
[1.5.0]: https://github.com/aurabx/harmony-dsl/compare/1.4.2...1.5.0
[1.4.2]: https://github.com/aurabx/harmony-dsl/compare/1.4.1...1.4.2
[1.4.1]: https://github.com/aurabx/harmony-dsl/compare/1.4.0...1.4.1
[1.4.0]: https://github.com/aurabx/harmony-dsl/compare/1.3...1.4.0
[1.3]: https://github.com/aurabx/harmony-dsl/compare/1.2...1.3
[1.2]: https://github.com/aurabx/harmony-dsl/compare/1.1...1.2
[1.1]: https://github.com/aurabx/harmony-dsl/compare/1.0...1.1
[1.0]: https://github.com/aurabx/harmony-dsl/releases/tag/1.0
