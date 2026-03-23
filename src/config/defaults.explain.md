# Breakdown: `src/config/defaults.rs`

This file manually implements the `Default` trait for every config struct. It's short and repetitive on purpose — but it introduces important concepts: traits, manual vs derived trait implementations, `Self`, and why `Default` matters in the larger config loading system.

---

## Section 1: Import (lines 1–3)

```rust
//! Default values for all config structs, matching design spec section 12.2.

use super::types::*;
```

### `use super::types::*;`

`super` means "parent module" — in this case, `config`. So this imports everything public from `config::types`: all the structs (`Config`, `AppearanceConfig`, etc.) and enums (`ExportFormat`, `ExportStatus`).

The `*` is a **glob import**. It pulls in every public item from the module. Glob imports are generally discouraged in Rust because they make it unclear where a name comes from — but within the same module family (files that are part of the same `config` module), they're common and accepted.

---

## Section 2: The `Default` trait (lines 5–15)

```rust
impl Default for Config {
    fn default() -> Self {
        Self {
            appearance: AppearanceConfig::default(),
            behavior: BehaviorConfig::default(),
            keybindings: KeybindingConfig::default(),
            git: GitConfig::default(),
            export: ExportConfig::default(),
        }
    }
}
```

### What is a trait?

A trait is Rust's version of an interface — it defines a set of methods that a type must implement. `Default` is a standard library trait with one required method:

```rust
pub trait Default {
    fn default() -> Self;
}
```

Any type that implements `Default` promises: "I can create a meaningful value from nothing." You've already seen traits in this project:

| Trait | What it means | How you've seen it |
|---|---|---|
| `Debug` | "I can be printed with `{:?}`" | `#[derive(Debug)]` on structs |
| `Clone` | "I can be deeply copied" | `#[derive(Clone)]` on structs |
| `PartialEq` | "I can be compared with `==`" | `#[derive(PartialEq)]` on structs |
| `Deserialize` | "I can be built from data formats" | `#[derive(Deserialize)]` on structs |
| `Default` | "I have a sensible default value" | **Manually implemented here** |

### `impl Default for Config`

This is a **trait implementation** — it says "the type `Config` implements the `Default` trait, and here's how." The syntax is always:

```rust
impl TraitName for TypeName {
    // implement the trait's required methods
}
```

Compare with a regular `impl` block (like `impl RepoInfo { ... }` in `repo.rs`), which adds *inherent* methods — methods that belong to the type itself, not to any trait.

### `Self`

Inside an `impl` block, `Self` (capital S) is an alias for the type being implemented. So inside `impl Default for Config`, `Self` means `Config`. These are equivalent:

```rust
fn default() -> Self {      // Self = Config
    Self { ... }
}

fn default() -> Config {    // explicit type name
    Config { ... }
}
```

`Self` is preferred because if you rename the type, you don't have to update every method body. Note the difference from `self` (lowercase), which you saw in `repo.rs` — `self` is an instance of the type, `Self` is the type itself:

```rust
impl RepoInfo {
    fn head_sha(&self) -> &str {  // self = an instance of RepoInfo
        &self.head_sha
    }
}

impl Default for Config {
    fn default() -> Self {        // Self = the type Config (no instance yet)
        Self { ... }
    }
}
```

`default()` has no `self` parameter because there's no existing instance — the whole point is to *create* one.

### Why call `.default()` on each field?

```rust
appearance: AppearanceConfig::default(),
```

Each field is itself a struct with its own `Default` impl (defined later in this file). `Config::default()` delegates to each sub-struct's `default()`. This is the **composition pattern** — the top-level default is built from its parts' defaults.

---

## Section 3: Manual impl vs `#[derive(Default)]` (lines 17–27)

```rust
impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            theme: "default".into(),
            side_by_side: true,
            line_numbers: true,
            word_diff: true,
            tab_width: 4,
        }
    }
}
```

### Why not `#[derive(Default)]`?

You *can* derive `Default` — the compiler will auto-generate it. But derived `Default` uses each field type's own default:

| Type | Derived default |
|---|---|
| `bool` | `false` |
| `u32`, `u64` | `0` |
| `String` | `""` (empty string) |

That would give us `theme = ""`, `side_by_side = false`, `tab_width = 0` — not what we want. Our design spec says the defaults should be `theme = "default"`, `side_by_side = true`, `tab_width = 4`. So we implement `Default` manually to choose meaningful values.

If the derived defaults *happened* to match your spec, `#[derive(Default)]` would be fine and preferable — less code to maintain. Manual impls exist for when you need custom values.

### `.into()` again

```rust
theme: "default".into(),
```

Same pattern you saw earlier — converts `&str` to `String`. The compiler knows `theme` is a `String` field, so `.into()` picks the `From<&str> for String` conversion.

### Literal values don't need `.into()`

```rust
side_by_side: true,
tab_width: 4,
```

`true` is already a `bool`, and `4` is already a number that fits `u32`. No conversion needed. `.into()` is only needed when the literal type doesn't match the field type — which in practice means string literals (`&str` → `String`).

---

## Section 4: Enum defaults (lines 64–71)

```rust
impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            default_format: ExportFormat::Text,
            default_status: ExportStatus::Open,
        }
    }
}
```

Notice there's no `.into()` here. `ExportFormat::Text` is already the right type — it's an `ExportFormat` value going into an `ExportFormat` field. Enum variants are constructed directly, not converted from strings.

Compare the two styles in this file:

```rust
theme: "default".into(),           // &str → String (conversion needed)
default_format: ExportFormat::Text, // ExportFormat → ExportFormat (already correct)
```

---

## How `Default` connects to the config system

This file doesn't exist in isolation. Here's how `Default` flows through the config loading pipeline:

```
User's config.toml          defaults.rs              types.rs
─────────────────          ───────────              ────────
[appearance]          ──►  serde sees missing    ──►  #[serde(default)]
theme = "monokai"          fields, calls              triggers Default
# tab_width missing        AppearanceConfig           for missing fields
                           ::default()
                           → tab_width = 4
```

1. User writes a partial config file (only `theme`, no `tab_width`)
2. Serde parses it and hits a missing field (`tab_width`)
3. `#[serde(default)]` on the struct (in `types.rs`) tells serde to call `Default`
4. Serde calls `AppearanceConfig::default()` (from this file) to fill in the gap
5. Result: `theme = "monokai"` (from file) + `tab_width = 4` (from default)

If the entire `[appearance]` section is missing, serde calls `AppearanceConfig::default()` for the whole thing. If the entire file is missing, `load_config()` in `load.rs` returns `Config::default()` directly — no serde involved.

---

## Key Rust concepts summary

| Concept | Where in this file | Quick explanation |
|---|---|---|
| Trait | `Default` | An interface — defines methods a type must implement |
| Trait implementation | `impl Default for Config` | Provides the trait's methods for a specific type |
| `Self` (capital) | `fn default() -> Self` | Alias for the implementing type — `Config`, `AppearanceConfig`, etc. |
| `self` vs `Self` | `default()` has no `self` | `Self` = the type, `self` = an instance (not needed here) |
| Manual vs derive | All impls in this file | Manual when derived defaults don't match your needs |
| `.into()` | `"default".into()` | Type conversion — `&str` to `String` here |
| Glob import | `use super::types::*` | Import everything from a sibling module |
| Composition | `Config::default()` calls sub-struct defaults | Build complex defaults from simpler defaults |
