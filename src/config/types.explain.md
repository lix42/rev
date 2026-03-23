# Breakdown: `src/config/types.rs`

This file defines the data structures for application configuration. It introduces new Rust concepts beyond what `repo.rs` covered: serde deserialization, attribute macros that change runtime behavior, enums with string representations, and the interaction between `#[serde(default)]` and the `Default` trait.

---

## Section 1: Import (lines 1–3)

```rust
//! Config structs matching design spec section 12.2.

use serde::Deserialize;
```

- `serde::Deserialize` — a **trait** from the `serde` crate. When you derive it on a struct, serde auto-generates code that can convert data formats (TOML, JSON, etc.) into that struct. Serde itself doesn't know about any specific format — it defines a generic data model, and format-specific crates (`toml`, `serde_json`) plug into it.

This is a key difference from many other languages: in Python you'd use `json.loads()` and get a dictionary; in Rust, you deserialize directly into a typed struct. If the data doesn't match the struct's shape, you get a compile-time type or a clear runtime error — not a `KeyError` three function calls later.

---

## Section 2: The top-level Config struct (lines 5–14)

```rust
/// Top-level application configuration.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct Config {
    pub appearance: AppearanceConfig,
    pub behavior: BehaviorConfig,
    pub keybindings: KeybindingConfig,
    pub git: GitConfig,
    pub export: ExportConfig,
}
```

### `#[derive(...)]` — generating trait implementations

You saw `Debug` and `Clone` in `repo.rs`. Two new ones here:

- `PartialEq` — lets you compare two `Config` values with `==`. The compiler generates field-by-field comparison. This is used in tests: `assert_eq!(config, Config::default())`.
- `Deserialize` — this is the serde magic. The derive macro generates a `Deserialize` implementation that knows how to build a `Config` from a TOML table (or JSON object, etc.). Each field name in the struct maps to a key in the TOML file:

```toml
[appearance]       # → Config.appearance
theme = "dark"     # → Config.appearance.theme

[behavior]         # → Config.behavior
auto_reload = true # → Config.behavior.auto_reload
```

### `#[serde(default)]` — the key design decision

This attribute tells serde: "if a field is missing from the input, use its `Default` value instead of returning an error."

Without `#[serde(default)]`, this TOML would fail to parse:

```toml
[appearance]
theme = "monokai"
# tab_width is missing — ERROR without serde(default)
```

With `#[serde(default)]`, the missing `tab_width` silently gets its default value (4). This applies at every level:

- The entire `[behavior]` section is missing? Use `BehaviorConfig::default()`.
- `[behavior]` exists but `reload_debounce_ms` is missing? Use the default for just that field.

This means the user's config file can contain only the settings they want to override. An empty file is valid and gives you all defaults.

### Where does `Default` come from?

`#[serde(default)]` calls the `Default` trait on each missing field. But `Default` isn't derived here — it's implemented manually in `defaults.rs`. We'll cover that in the `defaults.rs` explainer, but the key point is: **`#[serde(default)]` and the `Default` trait work as a pair.** Serde declares the *behavior* ("use defaults for missing fields"), and the `Default` impl declares the *values* ("tab_width defaults to 4").

### `pub` fields — contrast with `RepoInfo`

Notice all fields are `pub` here, unlike `RepoInfo` which had private fields with accessor methods. Why the difference?

- `RepoInfo` has **invariants** — its fields must satisfy guarantees (canonicalized path, valid SHA). Public fields would let anyone break those guarantees.
- `Config` is a **data bag** — it's loaded, validated once, then read. The fields are simple values with no cross-field invariants that the type itself enforces. Making them `pub` keeps access simple: `config.appearance.tab_width` instead of `config.appearance().tab_width()`.

This is a pragmatic trade-off: less encapsulation, but simpler code for a type where the added protection wouldn't prevent real bugs.

### Nested structs map to TOML sections

Each field in `Config` is another struct, and each struct maps to a `[section]` in TOML:

```
Config
├── appearance: AppearanceConfig    → [appearance]
├── behavior: BehaviorConfig        → [behavior]
├── keybindings: KeybindingConfig   → [keybindings]
├── git: GitConfig                  → [git]
└── export: ExportConfig            → [export]
```

Serde handles this nesting automatically — a struct field that is itself a struct becomes a TOML table.

---

## Section 3: The sub-structs (lines 16–53)

```rust
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct AppearanceConfig {
    pub theme: String,
    pub side_by_side: bool,
    pub line_numbers: bool,
    pub word_diff: bool,
    pub tab_width: u32,
}
```

### Primitive type mapping

Serde maps Rust types to TOML types automatically:

| Rust type | TOML type | Example |
|---|---|---|
| `String` | string | `theme = "default"` |
| `bool` | boolean | `side_by_side = true` |
| `u32` | integer | `tab_width = 4` |
| `u64` | integer | `reload_debounce_ms = 200` |

If the user writes `tab_width = "four"`, serde returns a parse error because `"four"` is a string, not an integer. Type safety from the file format all the way to your code.

### `u32` vs `u64`

```rust
pub tab_width: u32,          // max ~4 billion — more than enough
pub reload_debounce_ms: u64, // u64 for consistency with time APIs
```

Rust has explicit integer sizes (`u8`, `u16`, `u32`, `u64`, `i32`, etc.) — there's no ambiguous `int` that varies by platform. You pick the size that fits your data. `u32` means unsigned 32-bit (0 to ~4.2 billion). The `u` prefix means unsigned (no negative values); `i` would mean signed.

### Each sub-struct gets `#[serde(default)]` too

Every sub-struct has its own `#[serde(default)]`. This is important — without it, if the user provides a `[behavior]` section at all, they'd have to provide *every* field in it. With it, they can override just one field:

```toml
[behavior]
reload_debounce_ms = 500
# auto_reload and session_stale_days use their defaults
```

---

## Section 4: Enums (lines 55–70)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Text,
    Markdown,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportStatus {
    Open,
    Resolved,
    Updated,
    All,
}
```

### `enum` — Rust's most powerful feature

Rust enums are not like enums in C or Java (which are just named integers). Rust enums are **algebraic data types** — each variant can hold different data. These particular enums have no data in their variants (they're "unit variants"), but you'll see enums with data later in the project (e.g., `DiffSource::Git { base, head }`).

The key property: **the compiler forces you to handle every variant.** If you write a `match` on `ExportFormat` and forget `Json`, the code won't compile. This means adding a new variant later will cause compile errors everywhere it needs to be handled — no silent bugs from an unhandled case.

### Why enums instead of strings?

Earlier versions of this code used `String` for `default_format` and `default_status`, with validation checking against a list of valid values at runtime. The enum approach is better because:

1. **Invalid states are unrepresentable.** With `String`, you could have `default_format = "csv"` sitting in a `Config` struct. With `ExportFormat`, the only possible values are `Text`, `Markdown`, or `Json`. There is no way to construct an invalid value.
2. **Serde rejects unknown values at parse time.** If a user writes `default_format = "csv"` in their TOML, serde returns an error immediately — no separate validation step needed.
3. **Pattern matching is exhaustive.** `match format { Text => ..., Markdown => ..., Json => ... }` — the compiler guarantees you handled all cases.

### `#[serde(rename_all = "lowercase")]`

Rust convention is `PascalCase` for enum variants (`Text`, `Json`), but TOML convention is lowercase (`"text"`, `"json"`). This attribute tells serde to translate between them:

```toml
default_format = "json"    # lowercase in the file
```
```rust
ExportFormat::Json          # PascalCase in the code
```

Without this attribute, the TOML would need to match Rust's casing: `default_format = "Json"` — ugly for a config file.

### `Copy` — a new derive

The enums derive `Copy`, which the structs above don't. `Copy` means the value can be duplicated by just copying its bytes — no heap allocation, no `.clone()` needed. `Copy` types are passed by value automatically:

```rust
let fmt = config.export.default_format;  // copies the enum value
// config.export.default_format is still valid — it was copied, not moved
```

Only types that are small and don't own heap memory can be `Copy`: integers, bools, and simple enums like these. `String` and `Vec` cannot be `Copy` because they own heap-allocated data — copying them would need a deep copy (that's what `Clone` is for, and it requires an explicit `.clone()` call).

---

## Section 5: ExportConfig struct (lines 72–77)

```rust
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct ExportConfig {
    pub default_format: ExportFormat,
    pub default_status: ExportStatus,
}
```

This struct uses the enums as field types. From serde's perspective, it works seamlessly — serde sees the string `"json"` in the TOML, knows the target type is `ExportFormat`, and uses `ExportFormat`'s `Deserialize` impl (which knows about `#[serde(rename_all = "lowercase")]`) to convert it.

The `#[serde(default)]` here calls `ExportConfig::default()`, which is defined in `defaults.rs` to return `ExportFormat::Text` and `ExportStatus::Open`.

---

## Key Rust concepts summary

| Concept | Where in this file | Quick explanation |
|---|---|---|
| `Deserialize` derive | Every struct and enum | Auto-generates code to parse from TOML/JSON/etc. |
| `#[serde(default)]` | Every struct | Use `Default` values for missing fields |
| `#[serde(rename_all)]` | `ExportFormat`, `ExportStatus` | Translate between Rust naming and file format naming |
| `PartialEq` derive | Every struct | Enables `==` comparison (used in tests) |
| `Copy` trait | Enums only | Cheap bitwise copy — no `.clone()` needed |
| `enum` | `ExportFormat`, `ExportStatus` | Fixed set of values enforced at the type level |
| `pub` fields | All config structs | Public access — contrast with `RepoInfo`'s private fields |
| `u32` / `u64` | `tab_width`, `reload_debounce_ms` | Explicit integer sizes — no ambiguous `int` |
| Nested structs | `Config` contains `AppearanceConfig`, etc. | Maps to TOML `[sections]` automatically |
| Derive macros | `#[derive(...)]` on every type | Compiler generates trait impls from the struct definition |
