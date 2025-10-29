# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

This repository contains the **TOML Schema DSL definitions** for Harmony Proxy configuration files. The schema DSL enables cross-language validation between Rust (harmony-proxy) and PHP (Runbeam Cloud API), providing a single source of truth for configuration structure.

## Key Concepts

### Two-Layer Configuration System
Harmony uses a hierarchical configuration approach:
- **Main Config** (`config.toml`) - Gateway-level settings validated by `harmony-config-schema.toml`
- **Pipeline Configs** (`pipelines/*.toml`) - Request routing validated by `harmony-pipeline-schema.toml`

### Pattern Matching
Tables with `pattern = true` and wildcards (e.g., `network.*`) match multiple dynamic instances:
- Schema: `[[table]] name = "network.*"`
- Matches: `[network.default]`, `[network.management]`, `[network.vpn]`

This is the core mechanism enabling dynamic, user-defined configurations.

### Conditional Requirements
Fields can be conditionally required using `required_if`:
```toml
[[table.field]]
name = "network"
required_if = "enabled == true"  # Only required when enabled=true
```

## Schema Files

| File | Purpose |
|------|---------|
| `harmony-config-schema.toml` | Main gateway config structure (proxy, networks, storage, services) |
| `harmony-pipeline-schema.toml` | Pipeline config structure (pipelines, endpoints, backends, middleware) |
| `design-doc.md` | DSL specification and field reference |
| `harmony-schema-guide.md` | Implementation guide for Rust and PHP |

## Working With Schemas

### Schema Structure
Each schema defines:
- **Tables** - Configuration sections (e.g., `[proxy]`, `[network.*]`)
- **Fields** - Individual settings with types and validation rules
- **Validation Rules** - Type constraints, enums, ranges, conditional requirements
- **Defaults** - Fallback values for optional fields

### Validation Features
- **Type System**: `string`, `integer`, `boolean`, `float`, `array`, `table`
- **Enums**: Restrict values to predefined sets
- **Numeric Boundaries**: `min`/`max` for integers and floats
- **Array Validation**: `array_item_type`, `min_items`, `max_items`
- **Conditional Logic**: `required_if` for context-dependent requirements

### Adding New Fields
When extending schemas:
1. Increment `schema.version` for breaking changes
2. Add descriptive `description` fields for documentation
3. Set sensible `default` values for optional fields
4. Use `required_if` instead of blanket `required = true` when possible
5. Test against real configuration files

## Architecture Context

### Cross-Language Contract
The schemas define a contract between:
- **Rust (harmony-proxy)**: Local validation before proxy startup
- **PHP (Runbeam Cloud API)**: Remote validation, storage, and config generation

### API Design Implications
The schemas directly inform Runbeam Cloud API endpoints:
- `GET/PUT /api/v1/gateways/{id}/config` - Gateway config management
- `GET/PUT/DELETE /api/v1/gateways/{id}/pipelines/{name}` - Pipeline management
- `POST /api/v1/gateways/{id}/config/validate` - Dry-run validation
- `GET /api/v1/templates/config` - Template generation with defaults

### Validation Flow
```
User submits config.toml
  ↓
Runbeam Cloud API validates (PHP schema parser)
  ↓
If valid: Store in database
  ↓
Gateway downloads validated config
  ↓
harmony-proxy validates locally (Rust schema parser)
  ↓
If valid: Start proxy
```

## Implementation Status

### Current State
- ✅ Schema definitions complete (`harmony-config-schema.toml`, `harmony-pipeline-schema.toml`)
- ✅ Design documentation complete
- 🔲 Rust schema parser/validator (needs implementation in harmony-proxy)
- 🔲 PHP schema parser/validator (needs implementation in runbeam-sdk/API)

### Implementation Roadmap
See `harmony-schema-guide.md` for detailed 5-phase implementation plan covering:
1. Schema loading (Rust & PHP)
2. Validation logic (Rust & PHP)
3. Integration with harmony-proxy
4. Integration with Runbeam Cloud API
5. Code generation and tooling

## Related Repositories

- **harmony-proxy** (Rust) - Main proxy application, needs schema validation integration
- **runbeam-sdk** (Rust) - Shared types, potential home for schema module
- **Runbeam Cloud API** (PHP) - Needs TOML schema validation for config endpoints

Note: The project previously known as "wg-agent" has been renamed to "harmony-agent".

## Documentation

- **DSL Specification**: `design-doc.md` - Complete DSL field reference
- **Implementation Guide**: `harmony-schema-guide.md` - Cross-language implementation roadmap
- **Usage Examples**: See validation flow examples in `harmony-schema-guide.md`
- **API Integration**: See "API Design Implications" section in guide
