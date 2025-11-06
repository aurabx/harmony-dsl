# Harmony Configuration Schema DSL

## Installation

Install via Composer:

```bash
composer require aurabx/harmony-dsl
```

## Usage

The package provides TOML schema definitions as data files. Access them directly from your PHP application:

```php
$configSchemaPath = __DIR__ . '/vendor/aurabx/harmony-dsl/harmony-config-schema.toml';
$pipelineSchemaPath = __DIR__ . '/vendor/aurabx/harmony-dsl/harmony-pipeline-schema.toml';

// Load and parse the schema files with your TOML parser
$configSchema = parse_toml_file($configSchemaPath);
$pipelineSchema = parse_toml_file($pipelineSchemaPath);
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

### API Endpoints

The schemas guide these endpoint designs:

#### Gateway Configuration
```
GET    /api/v1/gateways/{id}/config          # Get config.toml
PUT    /api/v1/gateways/{id}/config          # Update config.toml (validates first)
POST   /api/v1/gateways/{id}/config/validate # Dry-run validation
```

#### Pipeline Management
```
GET    /api/v1/gateways/{id}/pipelines                # List all pipelines
GET    /api/v1/gateways/{id}/pipelines/{name}         # Get specific pipeline
PUT    /api/v1/gateways/{id}/pipelines/{name}         # Create/update pipeline
DELETE /api/v1/gateways/{id}/pipelines/{name}         # Delete pipeline
POST   /api/v1/gateways/{id}/pipelines/{name}/validate # Dry-run validation
```

#### Service & Middleware Registry
```
GET    /api/v1/services                     # List available services
GET    /api/v1/services/{type}/schema       # Get service options schema
GET    /api/v1/middleware-types             # List available middleware
GET    /api/v1/middleware-types/{type}/schema # Get middleware options schema
```

#### Templates
```
GET    /api/v1/templates/config                    # Generate template config.toml
GET    /api/v1/templates/pipeline/{service}        # Generate template pipeline
```

### Validation Flow

```
User submits config.toml
    ↓
Runbeam Cloud API receives it
    ↓
Load harmony-config-schema.toml
    ↓
Validate structure, types, enums, ranges
    ↓
If valid: Store in database
If invalid: Return detailed error messages
    ↓
Gateway downloads validated config
    ↓
Harmony Proxy validates again locally
    ↓
If valid: Start proxy
If invalid: Fail with error
```

## Schema Features

### Type System
- `string` - Text values
- `integer` - Whole numbers
- `boolean` - true/false
- `float` - Decimal numbers
- `array` - Lists (with item type validation)
- `table` - Nested structures

### Validation Rules
- `required` - Field must exist
- `required_if` - Conditional requirement (e.g., "network required if enabled=true")
- `enum` - Restrict to specific values
- `min` / `max` - Numeric boundaries
- `min_items` / `max_items` - Array length constraints
- `default` - Default value if omitted

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

## Implementation Status

### Schemas
- ✅ `harmony-config-schema.toml` - Covers main config structure
- ✅ `harmony-pipeline-schema.toml` - Covers pipeline structure
- 🔲 Service-specific option schemas (future enhancement)
- 🔲 Middleware-specific option schemas (future enhancement)

### Validation Implementation
- 🔲 Rust schema parser and validator (harmony-proxy)
- 🔲 PHP schema parser and validator (runbeam-sdk)
- 🔲 Integration with harmony-proxy config loading
- 🔲 Integration with Runbeam Cloud API endpoints

### Tooling
- 🔲 CLI validation tool (`harmony-proxy --validate-only`)
- 🔲 Template generation from schemas
- 🔲 JSON Schema conversion for OpenAPI docs
- 🔲 Schema versioning and migration tools

## Next Steps

### For Harmony Proxy (Rust)
1. Create `src/schema` module
2. Implement schema loading and validation
3. Integrate with config loading pipeline
4. Add `--validate-only` CLI flag
5. Write comprehensive tests

### For Runbeam SDK (Rust)
1. Create `src/schema` module for shared schema types
2. Implement validation logic that can be reused
3. Export types for use in harmony-proxy

### For Runbeam Cloud API (PHP)
1. Create `TomlSchema` / `TomlValidator` classes
2. Implement validation logic matching Rust behavior
3. Add validation to gateway config endpoints
4. Implement template generation from schemas
5. Add schema documentation to API responses

## Resources

- **DSL Specification**: `dsl.md` - Complete field reference
- **Implementation Guide**: `harmony-schema-guide.md` - Detailed implementation roadmap
- **Example Configs**: `../harmony-proxy/examples/` - Real-world config examples
- **OpenAPI Spec**: `document.json` - Current API endpoint definitions

## Contributing

When modifying schemas:

1. **Update version** - Increment `schema.version` on breaking changes
2. **Add descriptions** - Every table/field should have clear documentation
3. **Test thoroughly** - Validate against real config files
4. **Update guide** - Keep `harmony-schema-guide.md` in sync
5. **Backward compatibility** - Support old schema versions when possible

## Questions?

- Schema design questions → See `harmony-schema-guide.md`
- DSL syntax questions → See `dsl.md`
- API design questions → See "API Design Implications" in guide
- Implementation questions → See "Implementation Roadmap" in guide
