# Harmony Configuration Schema Guide

## Overview

This guide explains how to use the TOML Schema DSL to validate Harmony Proxy configuration files. The schema definitions provide a cross-language contract between:

- **Rust** (harmony-proxy) - Local config validation before startup
- **PHP** (Runbeam Cloud API) - Remote config validation and generation

## Schema Files

### `harmony-config-schema.toml`
Defines the structure for main configuration files (`config.toml`):
- Proxy settings (ID, log level, paths)
- Management API configuration
- Network definitions
- Logging and storage backends
- Service and middleware type registrations

### `harmony-pipeline-schema.toml`
Defines the structure for pipeline configuration files (`pipelines/*.toml`):
- Pipeline definitions (routing rules)
- Endpoint configurations
- Backend configurations
- Middleware instances

## Architecture

### Configuration Hierarchy

```
config.toml                    ← Main config (validated by harmony-config-schema.toml)
├── [proxy]                    ← Core settings
├── [management]               ← Management API
├── [network.*]                ← Network definitions
├── [logging]                  ← Logging config
├── [storage]                  ← Storage backend
├── [services.*]               ← Service type registry
└── [middleware_types.*]       ← Middleware type registry

pipelines/
├── pipeline1.toml             ← Pipeline configs (validated by harmony-pipeline-schema.toml)
│   ├── [pipelines.*]          ← Pipeline definitions
│   ├── [endpoints.*]          ← Endpoint configs
│   ├── [backends.*]           ← Backend configs
│   └── [middleware.*]         ← Middleware instances
└── pipeline2.toml
```

### Validation Flow

#### Harmony Proxy (Rust)
```rust
// 1. Load schemas at build/startup
let config_schema = load_schema("docs/harmony-config-schema.toml")?;
let pipeline_schema = load_schema("docs/harmony-pipeline-schema.toml")?;

// 2. Validate main config
let config = toml::from_str(&config_file)?;
config_schema.validate(&config)?;

// 3. Validate each pipeline file
for pipeline_file in pipeline_files {
    let pipeline = toml::from_str(&pipeline_file)?;
    pipeline_schema.validate(&pipeline)?;
}

// 4. Proceed with normal operation
```

#### Runbeam Cloud API (PHP)
```php
// 1. Load schemas
$configSchema = TomlSchema::fromFile('harmony-config-schema.toml');
$pipelineSchema = TomlSchema::fromFile('harmony-pipeline-schema.toml');

// 2. Validate gateway configuration from database
$config = $gateway->getConfigToml();
$validator = new TomlValidator($configSchema);
$validator->validate($config);

// 3. Generate validated config for download
$generatedConfig = $configSchema->generate($gateway->settings);
```

## Schema Features

### Pattern Matching
Tables with `pattern = true` match multiple instances:

```toml
# Schema definition
[[table]]
name = "network.*"
pattern = true

# Matches these config tables:
[network.default]
[network.management]
[network.vpn_only]
```

### Conditional Requirements
Fields can be required based on other field values:

```toml
# Schema definition
[[table.field]]
name = "network"
required = true
required_if = "enabled == true"

# Valid config:
[management]
enabled = true
network = "management"  # Required because enabled=true

# Invalid config:
[management]
enabled = true
# network is missing!
```

### Enum Validation
Restrict values to a predefined set:

```toml
# Schema definition
[[table.field]]
name = "log_level"
enum = ["trace", "debug", "info", "warn", "error"]

# Valid: log_level = "debug"
# Invalid: log_level = "verbose"
```

### Array Validation
Validate array contents:

```toml
# Schema definition
[[table.field]]
name = "networks"
type = "array"
array_item_type = "string"
min_items = 1

# Valid:
networks = ["default", "management"]

# Invalid (too few items):
networks = []
```

### Numeric Boundaries
Enforce min/max values:

```toml
# Schema definition
[[table.field]]
name = "bind_port"
type = "integer"
min = 1
max = 65535

# Valid: bind_port = 8080
# Invalid: bind_port = 70000
```

## API Design Implications

The schema DSL guides the Runbeam Cloud API design in several ways:

### 1. Gateway Configuration Endpoints

```
GET /api/v1/gateways/{id}/config
  → Returns validated config.toml (validated against harmony-config-schema.toml)

PUT /api/v1/gateways/{id}/config
  → Validates and saves config (rejects invalid configs)

POST /api/v1/gateways/{id}/config/validate
  → Validates config without saving (dry-run)
```

### 2. Pipeline Management Endpoints

```
GET /api/v1/gateways/{id}/pipelines
  → Lists all pipeline configs

GET /api/v1/gateways/{id}/pipelines/{name}
  → Returns specific pipeline TOML (validated against harmony-pipeline-schema.toml)

PUT /api/v1/gateways/{id}/pipelines/{name}
  → Validates and saves pipeline config

DELETE /api/v1/gateways/{id}/pipelines/{name}
  → Removes pipeline config
```

### 3. Service Registry Endpoints

```
GET /api/v1/services
  → Lists available service types (http, fhir, dicom, echo, jmix, dicomweb)

GET /api/v1/services/{type}/schema
  → Returns JSON schema for service options (derived from TOML schema)
```

### 4. Middleware Registry Endpoints

```
GET /api/v1/middleware-types
  → Lists available middleware types

GET /api/v1/middleware-types/{type}/schema
  → Returns JSON schema for middleware options
```

### 5. Configuration Templates

```
GET /api/v1/templates/config
  → Returns template config.toml with defaults from schema

GET /api/v1/templates/pipeline/{service}
  → Returns template pipeline for service type (e.g., FHIR, DICOM)
```

## Implementation Roadmap

### Phase 1: Schema Loading (Rust & PHP)
- [ ] Rust: Create `schema` module in harmony-proxy
- [ ] Rust: Implement `Schema`, `Table`, `Field` structs
- [ ] Rust: Implement TOML schema loading
- [ ] PHP: Create `TomlSchema`, `TomlTable`, `TomlField` classes
- [ ] PHP: Implement TOML schema loading via yosymfony/toml

### Phase 2: Validation (Rust & PHP)
- [ ] Rust: Implement `Schema::validate()` method
- [ ] Rust: Implement pattern matching for wildcard tables
- [ ] Rust: Implement conditional requirements (`required_if`)
- [ ] Rust: Implement enum validation
- [ ] Rust: Implement array validation (items, min/max)
- [ ] PHP: Implement equivalent validation logic

### Phase 3: Integration (Harmony Proxy)
- [ ] Add schema validation to config loading
- [ ] Add schema validation to pipeline loading
- [ ] Provide detailed error messages for validation failures
- [ ] Add `--validate-only` CLI flag for dry-run validation

### Phase 4: Integration (Runbeam Cloud API)
- [ ] Add schema loading to API bootstrap
- [ ] Implement config validation endpoint
- [ ] Implement pipeline validation endpoint
- [ ] Add validation to gateway config save operations

### Phase 5: Code Generation
- [ ] Rust: Generate Rust structs from schema
- [ ] PHP: Generate PHP DTOs from schema
- [ ] Generate JSON Schema for OpenAPI documentation
- [ ] Generate template configs with defaults

## Testing Strategy

### Rust Tests
```rust
#[test]
fn test_valid_config() {
    let schema = Schema::load("harmony-config-schema.toml").unwrap();
    let config = toml::from_str(include_str!("fixtures/valid-config.toml")).unwrap();
    assert!(schema.validate(&config).is_ok());
}

#[test]
fn test_missing_required_field() {
    let schema = Schema::load("harmony-config-schema.toml").unwrap();
    let config = toml::from_str("[proxy]\nlog_level = \"debug\"").unwrap(); // missing id
    assert!(schema.validate(&config).is_err());
}

#[test]
fn test_conditional_requirement() {
    let schema = Schema::load("harmony-config-schema.toml").unwrap();
    let config = toml::from_str("[management]\nenabled = true").unwrap(); // missing network
    assert!(schema.validate(&config).is_err());
}
```

### PHP Tests
```php
public function testValidConfig(): void
{
    $schema = TomlSchema::fromFile('harmony-config-schema.toml');
    $config = Toml::parseFile('fixtures/valid-config.toml');
    $validator = new TomlValidator($schema);
    
    $this->assertTrue($validator->validate($config));
}

public function testEnumValidation(): void
{
    $schema = TomlSchema::fromFile('harmony-config-schema.toml');
    $config = ['proxy' => ['id' => 'test', 'log_level' => 'invalid']];
    $validator = new TomlValidator($schema);
    
    $this->assertFalse($validator->validate($config));
}
```

### Integration Tests
- Test full config loading and validation in harmony-proxy
- Test API config validation endpoints
- Test config generation from templates
- Test error message clarity for common mistakes

## Best Practices

### Schema Design
1. **Start permissive** - Make fields optional initially, tighten later
2. **Document everything** - Use `description` fields extensively
3. **Use patterns wisely** - Only for truly dynamic table names
4. **Provide defaults** - Set sensible defaults for optional fields
5. **Version schemas** - Increment `schema.version` on breaking changes

### Validation Messages
Provide clear, actionable error messages:
```
❌ Bad:  "Validation failed"
✅ Good: "Field 'proxy.id' is required but missing"

❌ Bad:  "Invalid value"
✅ Good: "Field 'proxy.log_level' must be one of: trace, debug, info, warn, error. Got: 'verbose'"

❌ Bad:  "Type error"
✅ Good: "Field 'network.http.bind_port' must be an integer between 1 and 65535. Got: 70000"
```

### Config Evolution
When schemas change:
1. **Increment version** - Update `schema.version`
2. **Maintain compatibility** - Support multiple schema versions
3. **Provide migration** - Offer tools to upgrade old configs
4. **Document changes** - Maintain a CHANGELOG for schema versions

## Example Usage

### Validating a Config (CLI)
```bash
# Validate without starting proxy
harmony-proxy --validate-only --config config.toml

# Output:
# ✅ config.toml validated successfully
# ✅ pipelines/basic-echo.toml validated successfully
# ✅ pipelines/fhir.toml validated successfully
```

### Validating via API
```bash
# Validate config before saving
curl -X POST https://api.runbeam.io/api/v1/gateways/123/config/validate \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: text/plain" \
  --data-binary @config.toml

# Response:
{
  "valid": true,
  "errors": []
}
```

### Generating a Template
```bash
# Generate template config with defaults
curl https://api.runbeam.io/api/v1/templates/config \
  -H "Authorization: Bearer $TOKEN" \
  > config.toml

# Generate FHIR pipeline template
curl https://api.runbeam.io/api/v1/templates/pipeline/fhir \
  -H "Authorization: Bearer $TOKEN" \
  > pipelines/fhir.toml
```

## Future Enhancements

### Schema Composition
```toml
[schema]
version = "1.0.0"
imports = ["common-middleware.toml", "dicom-services.toml"]
```

### Custom Types
```toml
[[custom_type]]
name = "url"
base_type = "string"
pattern = "^https?://.+"

[[table.field]]
name = "base_url"
type = "url"
```

### Cross-Field Validation
```toml
[[table.field]]
name = "end_port"
type = "integer"
validate = "self > start_port"
```

### JSON Schema Export
```bash
# Convert TOML schema to JSON Schema
harmony-schema convert harmony-config-schema.toml \
  --output harmony-config-schema.json
```

## Conclusion

The TOML Schema DSL provides a unified, declarative way to define configuration structure across Rust and PHP codebases. By validating configs against these schemas, we ensure:

- **Correctness** - Catch errors before deployment
- **Consistency** - Same rules in proxy and cloud API
- **Documentation** - Schema serves as source of truth
- **Automation** - Enable code/template generation
- **Evolution** - Manage schema versions over time

This foundation supports the Runbeam Cloud API design by clearly defining what configuration data the API needs to store, validate, and provide to gateways.
