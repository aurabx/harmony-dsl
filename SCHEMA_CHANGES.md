# Harmony DSL Schema Changes (v1.9.0 / v1.8.0)

This document outlines the changes to the Harmony DSL schemas and provides implementation guidance for dependent projects (`harmony-proxy`, `runbeam-sdk`).

## 1. Executive Summary
The configuration architecture has been refactored to support **Global Authentications**. Authentication configurations are no longer defined inline within peers, targets, endpoints, or backends, nor as options in middleware. Instead, they are defined as top-level `authentications.*` objects and referenced by ID.

Additionally, `policies` and `rules` have been moved from the **Pipeline** scope to the **Global Config** scope.

## 2. Schema Version Updates
- **Config Schema**: `harmony-config-schema.toml` updated to **1.9.0**
- **Pipeline Schema**: `harmony-pipeline-schema.toml` updated to **1.8.0**

## 3. Detailed Changes

### 3.1 Global Authentications Table (`config`)
**New Table**: `authentications.*`
**Location**: `harmony-config-schema.toml`

Defines reusable authentication strategies.
```toml
[authentications.auth-id]
id = "auth-id"
method = "jwt" # or basic, bearer, api_key, mutual_tls, etc.

[authentications.auth-id.options]
# Options specific to the method (previously found in middleware options or inline auth)
issuer = "..."
jwks_uri = "..."
credentials_path = "..."
```

### 3.2 Relocation of Policies and Rules (`config` <- `pipeline`)
**Moved**: `policies.*` and `rules.*` tables.
**From**: `harmony-pipeline-schema.toml`
**To**: `harmony-config-schema.toml`

Policies and rules are now considered global resources, available to be referenced by any pipeline or middleware.

### 3.3 Reference-Based Authentication
**Changed Field**: `authentication`
**Type**: Changed from `Table` (inline config) to `String` (Reference ID).
**Format**: `authentications.{id}`

**Affected Components**:
| Component | Schema | Old Behavior | New Behavior |
|-----------|--------|--------------|--------------|
| `peers.*` | Config | Inline Table | String Ref |
| `targets.*` | Config | Inline Table | String Ref |
| `endpoints.*` | Pipeline | Inline Table | String Ref |
| `backends.*` | Pipeline | Inline Table | String Ref |
| `middleware.*` | Pipeline | N/A | Added String Ref |

### 3.4 Middleware Options Migration
Middleware configurations for authentication types (`jwt_auth`, `basic_auth`) no longer accept authentication parameters in `options`.

**Deprecated/Removed Options**:
- `jwt_auth`: `issuer`, `audience`, `trusted_issuers`, `jwks_uri`, `algorithms`, `required_claims`, `leeway_seconds`, `validate_expiry`.
- `basic_auth`: `username`, `password`, `token_path`.

**New Usage**:
```toml
[middleware.my-jwt]
type = "jwt_auth"
authentication = "authentications.central-jwt" # References global config
```

## 4. Implementation Guide

### 4.1 Rust (`harmony-proxy`)
1.  **Config Structs**:
    - Add `authentications: HashMap<String, AuthenticationConfig>` to the root `Config` struct.
    - Move `policies` and `rules` from `PipelineConfig` to `Config`.
2.  **Component Structs** (`Peer`, `Target`, `Endpoint`, `Backend`):
    - Change `authentication` field type from `Option<AuthenticationConfig>` to `Option<String>`.
3.  **Resolution Logic**:
    - During configuration loading/merging, resolve the `authentication` string references to the actual `AuthenticationConfig` objects defined in the global scope.
    - Ensure validation fails if a referenced authentication ID does not exist.
4.  **Middleware Construction**:
    - Update `JwtAuthMiddleware` and `BasicAuthMiddleware` constructors to accept the resolved `AuthenticationConfig` instead of looking for parameters in the `options` map.

### 4.2 PHP (`runbeam-sdk` / `HarmonyTomlGenerator`)
1.  **Generator Update**:
    - Implement generation of `[authentications.*]` blocks in the output TOML.
    - Move generation of `[policies.*]` and `[rules.*]` to the main config section.
    - For Peers/Targets/Endpoints/Backends: Output the `authentication` field as a string reference (e.g., `authentication = "authentications.id"`) instead of expanding the authentication table.
2.  **Validation**:
    - Update validation rules to check for the existence of referenced authentication IDs across the schema boundary (Pipeline -> Config).

### 4.3 Configuration Files Structure
While the schema defines the structure, the physical files may be split.
- **Proposed File Layout**:
    - `config.toml`: Main entry, proxies, storage, etc.
    - `peers.toml`: `[peers.*]` definitions.
    - `targets.toml`: `[targets.*]` definitions.
    - `authentications.toml`: `[authentications.*]` definitions.
    - `policies.toml`: `[policies.*]` and `[rules.*]` definitions.
    - `pipelines/*.toml`: Pipeline definitions.

*Note: The parser should support merging these files into the single schema structure defined in `harmony-config-schema.toml`.*
