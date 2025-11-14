# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.6.0] - 2025-14-12
- Added Types to Services

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

[1.5.0]: https://github.com/aurabx/harmony-dsl/compare/1.4.2...1.5.0
[1.4.2]: https://github.com/aurabx/harmony-dsl/compare/1.4.1...1.4.2
[1.4.1]: https://github.com/aurabx/harmony-dsl/compare/1.4.0...1.4.1
[1.4.0]: https://github.com/aurabx/harmony-dsl/compare/1.3...1.4.0
[1.3]: https://github.com/aurabx/harmony-dsl/compare/1.2...1.3
[1.2]: https://github.com/aurabx/harmony-dsl/compare/1.1...1.2
[1.1]: https://github.com/aurabx/harmony-dsl/compare/1.0...1.1
[1.0]: https://github.com/aurabx/harmony-dsl/releases/tag/1.0
