Here’s a full **Markdown design document** version of the cross-language TOML schema DSL spec — suitable for inclusion in both your Rust and PHP repos.
# TOML Schema DSL Design Document

## Overview

This document defines a **Domain-Specific Language (DSL)** for describing the structure and validation rules of TOML configuration files.  
It provides a unified schema format that can be parsed and validated in both **Rust** and **PHP**, allowing consistent behaviour across components.

The DSL itself is expressed in **TOML**, making it human-readable, versionable, and easy to extend.

---

## Goals

- Define TOML configuration structure in a machine-readable DSL.  
- Support type definitions, validation rules, and default values.  
- Enable both Rust and PHP codebases to:
  - Parse and validate TOML configs.
  - Generate TOML templates or code structures.
  - Evolve schema versions consistently.

---

## DSL Structure

The DSL is defined in TOML itself, using `[[table]]` and `[[table.field]]` sections.

### Example

```toml
[schema]
version = "1.0."

[[table]]
name = "app"
required = true

[[table.field]]
name = "name"
type = "string"
required = true

[[table.field]]
name = "port"
type = "integer"
min = 1
max = 65535
default = 8080
````

---

## Field Definitions

| Key                       | Type      | Description                                                                 |
| ------------------------- | --------- | --------------------------------------------------------------------------- |
| `schema.version`          | `string`  | Schema version for backward compatibility.                                  |
| `schema.description`      | `string`  | Human-readable description of this schema.                                  |
| `table.name`              | `string`  | Name of the TOML table being described. Use `*` for wildcard patterns.      |
| `table.pattern`           | `bool`    | If true, `table.name` is a pattern (e.g., `network.*` matches all networks).|
| `table.pattern_constraint`| `string`  | Regex pattern that wildcard portion must match (e.g., `^[a-z0-9_-]+$`).     |
| `table.required`          | `bool`    | Whether the table must exist in the TOML file.                              |
| `table.description`       | `string`  | Human-readable description of this table.                                   |
| `field.name`              | `string`  | Field name within the table.                                                |
| `field.type`              | `string`  | Supported types: `string`, `integer`, `boolean`, `float`, `array`, `table`. |
| `field.required`          | `bool`    | Whether this field is mandatory.                                            |
| `field.description`       | `string`  | Human-readable description of this field.                                   |
| `field.min` / `field.max` | `integer` | Numeric boundaries (for integer/float fields).                              |
| `field.min_items`         | `integer` | Minimum array length (for array fields).                                    |
| `field.max_items`         | `integer` | Maximum array length (for array fields).                                    |
| `field.array_item_type`   | `string`  | Type of array items (for array fields).                                     |
| `field.enum`              | `array`   | Allowed values (for string/integer fields).                                 |
| `field.default`           | `any`     | Default value if not provided.                                              |
| `field.required_if`       | `string`  | Conditional rule (e.g. `mode == "dev"`).                                    |

---

## Example Usage

### Example Config TOML

```toml
[app]
name = "Aurabox"
port = 443
```

### Example Schema TOML

See schema example above.

Validation will confirm:

* `[app]` exists.
* `name` is a non-empty string.
* `port` is an integer between 1 and 65535.

---

## Rust Implementation

### Data Structures

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Schema {
    version: String,
    table: Vec<Table>,
}

#[derive(Debug, Deserialize)]
struct Table {
    name: String,
    required: Option<bool>,
    field: Vec<Field>,
}

#[derive(Debug, Deserialize)]
struct Field {
    name: String,
    #[serde(rename = "type")]
    field_type: String,
    required: Option<bool>,
    min: Option<i64>,
    max: Option<i64>,
    default: Option<toml::Value>,
}
```

### Loading the Schema

```rust
let schema: Schema = toml::from_str(&std::fs::read_to_string("schema.toml")?)?;
println!("{:#?}", schema);
```

This can be extended to:

* Validate TOML configurations.
* Generate TOML templates.
* Emit Rust structs or JSON Schema.

---

## PHP Implementation

### Dependencies

Use [`yosymfony/toml`](https://github.com/yosymfony/toml) for parsing.

### Schema Classes

```php
final class TomlSchema
{
    public string $version;
    /** @var TomlTable[] */
    public array $tables;

    public static function fromArray(array $data): self
    {
        $schema = new self();
        $schema->version = $data['schema']['version'] ?? '1.0';
        $schema->tables = array_map(
            fn(array $t) => TomlTable::fromArray($t),
            $data['table'] ?? []
        );
        return $schema;
    }
}

final class TomlTable
{
    public string $name;
    public bool $required;
    /** @var TomlField[] */
    public array $fields;

    public static function fromArray(array $data): self
    {
        $table = new self();
        $table->name = $data['name'];
        $table->required = $data['required'] ?? false;
        $table->fields = array_map(
            fn(array $f) => TomlField::fromArray($f),
            $data['field'] ?? []
        );
        return $table;
    }
}

final class TomlField
{
    public string $name;
    public string $type;
    public bool $required;
    public ?int $min = null;
    public ?int $max = null;
    public mixed $default = null;

    public static function fromArray(array $data): self
    {
        $f = new self();
        $f->name = $data['name'];
        $f->type = $data['type'];
        $f->required = $data['required'] ?? false;
        $f->min = $data['min'] ?? null;
        $f->max = $data['max'] ?? null;
        $f->default = $data['default'] ?? null;
        return $f;
    }
}
```

### Loading the Schema

```php
use Yosymfony\Toml\Toml;

$schemaArray = Toml::parseFile('schema.toml');
$schema = TomlSchema::fromArray($schemaArray);
```

You can then walk the schema and validate a config file against it.

---

## Future Enhancements

* **Custom Types:** Define reusable type aliases for complex structures.
* **Conditional Rules:** Add support for `required_if`, `mutually_exclusive_with`, etc.
* **Schema Imports:** Allow schema reuse and composition.
* **Code Generation:** Emit Rust structs or PHP DTOs from schema automatically.
* **Interoperability:** Optionally export to JSON Schema or CUE.

---
---

## Summary

This DSL enables both Rust and PHP components to share a single, declarative definition of TOML configuration structure.
It serves as a portable contract between systems, supporting validation, generation, and future code synthesis across both ecosystems.

```