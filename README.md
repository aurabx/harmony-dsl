# Harmony Configuration Schema DSL

## Installation

### PHP (Composer)

Install via Composer:

```bash
composer require aurabx/harmony-dsl
```

### Rust (Cargo)

Add to your `Cargo.toml`:

```toml
[dependencies]
harmony-dsl = "1.8"
```

## Usage

### PHP

The package provides TOML schema definitions as data files. Access them directly from your PHP application:

```php
$configSchemaPath = __DIR__ . '/vendor/aurabx/harmony-dsl/harmony-config-schema.toml';
$pipelineSchemaPath = __DIR__ . '/vendor/aurabx/harmony-dsl/harmony-pipeline-schema.toml';

// Load and parse the schema files with your TOML parser
$configSchema = parse_toml_file($configSchemaPath);
$pipelineSchema = parse_toml_file($pipelineSchemaPath);
```

### Rust

The schema files are embedded as string constants in the crate:

```rust
use harmony_dsl::{CONFIG_SCHEMA, PIPELINE_SCHEMA};

// Access the schema contents as &str
let config_schema = CONFIG_SCHEMA;
let pipeline_schema = PIPELINE_SCHEMA;

// Parse with your TOML parser
let config: toml::Value = toml::from_str(config_schema)?;
let pipeline: toml::Value = toml::from_str(pipeline_schema)?;
```

## Quick Start

This directory contains the TOML Schema DSL definitions for Harmony Proxy configuration files. These schemas enable cross-language validation between Rust (harmony-proxy) and PHP (Runbeam Cloud API).

## Files

| File | Purpose |
|------|---------|
| `dsl.md` | DSL specification and field reference |
| `harmony-config-schema.toml` | Schema for main config files (`config.toml`) |
| `harmony-pipeline-schema.toml` | Schema for pipeline files (`pipelines/*.toml`) |
| `harmony-schema-guide.md` | Implementation guide and API design implications |

## What is the Schema DSL?

The Schema DSL is a **declarative language written in TOML** that describes:
- Configuration file structure (tables, fields, types)
- Validation rules (required fields, enums, ranges)
- Default values and conditional requirements
- Documentation (descriptions for every element)

### Why Use a DSL?

1. **Single Source of Truth** - One schema definition used by both Rust and PHP
2. **Automatic Validation** - Catch config errors before deployment
3. **API Contract** - Defines what the Runbeam Cloud API needs to store/serve
4. **Code Generation** - Generate structs, DTOs, and templates from schemas
5. **Documentation** - Schema includes human-readable descriptions

## Key Concepts

### Config Hierarchy

Harmony uses a two-level configuration system:

```
config.toml              ← Main gateway config
├── [proxy]             ← Core settings (ID, logging, paths)
├── [network.*]         ← Network definitions (HTTP listeners, WireGuard)
├── [management]        ← Management API config
├── [storage]           ← Storage backend (filesystem, S3)
└── [services.*]        ← Service type registry

pipelines/*.toml         ← Pipeline configs (loaded from config.pipelines_path)
├── [pipelines.*]       ← Pipeline routing definitions
├── [endpoints.*]       ← Endpoint configs (how requests arrive)
├── [backends.*]        ← Backend configs (where requests go)
└── [middleware.*]      ← Middleware instances (transform, auth, etc.)
```

### Pattern Matching

Tables with wildcard names match multiple instances:

```toml
# Schema: network.*
# Matches: [network.default], [network.management], [network.vpn]
```

This is crucial for dynamic configurations where users can define multiple networks, pipelines, endpoints, etc.

## How This Guides API Design

### Data Model

The schemas define what the Runbeam Cloud API needs to store in the database:

```
gateways
├── id
├── code
├── config_toml          ← Validated against harmony-config-schema.toml
└── pipelines
    ├── name
    └── config_toml      ← Validated against harmony-pipeline-schema.toml
```

### Example Schema Definition

```toml
[[table]]
name = "proxy"
required = true
description = "Core proxy configuration"

[[table.field]]
name = "id"
type = "string"
required = true
description = "Unique gateway identifier"

[[table.field]]
name = "log_level"
type = "string"
required = false
default = "error"
enum = ["trace", "debug", "info", "warn", "error"]
description = "Logging verbosity"
```

This defines:
- A required `[proxy]` table
- A required `id` field (string)
- An optional `log_level` field (string, defaults to "error", must be one of 5 values)
