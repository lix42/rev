# Breakdown: `src/git/repo.rs`

This file has the real logic for git repo detection. It covers a lot of Rust ground: structs, `impl` blocks, ownership/borrowing, error handling, `Option`, traits, and testing.

---

## Section 1: Imports (lines 1–4)

```rust
//! Git repository detection and metadata extraction.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
```

- `//!` — module-level doc comment (same as in `mod.rs`)
- `use` brings names into scope so you don't have to write the full path every time
- `anyhow::Result` — a type alias for `Result<T, anyhow::Error>`. It replaces the standard `Result<T, E>` so you don't need to specify the error type everywhere. This is for **application-level** code where you just want to propagate errors with context, not match on specific error variants.
- `anyhow::Context` — a **trait** that adds the `.context("message")` method to `Result` and `Option` types. More on this below.
- `Path` vs `PathBuf` — think of these like `str` vs `String`:
  - `Path` is a **borrowed** reference to a path. You can read it but don't own it. It's always seen as `&Path`.
  - `PathBuf` is an **owned** path. You can store it in a struct, modify it, etc.
  - The pattern: functions *accept* `&Path` (flexible), structs *store* `PathBuf` (owned).

---

## Section 2: The struct (lines 6–12)

```rust
/// Metadata about the current git repository.
#[derive(Debug, Clone)]
pub struct RepoInfo {
    repo_root: PathBuf,
    head_sha: String,
    branch: Option<String>,
}
```

### `struct`

A struct is Rust's primary way to group related data. Similar to a class in other languages, but with no inheritance and no built-in methods — behavior is added separately via `impl` blocks.

### `#[derive(Debug, Clone)]`

This is an **attribute** that asks the compiler to auto-generate trait implementations:

- `Debug` — lets you print the struct with `{:?}` format (e.g., `println!("{:?}", info)`). Essential for debugging.
- `Clone` — lets you call `.clone()` to make a deep copy. Without this, the struct can only be *moved*, not copied.

Rust has many derivable traits. Common ones you'll see: `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`, `Serialize`, `Deserialize`.

### Private fields

Notice the fields have **no `pub`**. This is deliberate:

```rust
    repo_root: PathBuf,    // no pub = private
    head_sha: String,      // no pub = private
    branch: Option<String>, // no pub = private
```

Only code inside the `git` module can access these fields directly. Outside code must use the accessor methods (next section). This means you **can't** do this from `main.rs`:

```rust
let info = git::open_repo(&path)?;
let root = info.repo_root;  // ERROR: field is private
```

Why? Because `open_repo` guarantees that `repo_root` is canonicalized and `head_sha` is a valid 40-char hex SHA. If the fields were public, any code could construct a `RepoInfo` with garbage values and break those guarantees.

### `Option<String>`

`Option` is Rust's way of saying "this value might not exist." There's no `null` in Rust. Instead:

- `Some("main".to_string())` — the value exists
- `None` — no value

Here, `branch` is `None` when HEAD is detached (pointing at a commit directly, not a branch). Every piece of code that uses `branch` is *forced* by the compiler to handle the `None` case. This prevents null pointer bugs at compile time.

---

## Section 3: The `impl` block (lines 14–29)

```rust
impl RepoInfo {
    pub fn repo_root(&self) -> &Path {
        &self.repo_root
    }

    pub fn head_sha(&self) -> &str {
        &self.head_sha
    }

    pub fn branch(&self) -> Option<&str> {
        self.branch.as_deref()
    }
}
```

### `impl RepoInfo { ... }`

This is where you attach methods to a struct. It's separate from the struct definition — you can even have multiple `impl` blocks for the same struct.

### `&self`

The `&self` parameter means "I'm borrowing a reference to the struct, read-only." This is the most common method receiver. The three options are:

| Receiver | Meaning |
|---|---|
| `&self` | Borrow — read access, struct still usable after the call |
| `&mut self` | Mutable borrow — can modify fields, struct still usable after |
| `self` | Move — takes ownership, struct is *consumed* (can't use it after) |

### Return types and borrowing

```rust
pub fn repo_root(&self) -> &Path {
    &self.repo_root
}
```

This returns a `&Path` — a **borrowed reference** to the data inside the struct. The caller can look at the path but doesn't own it. The data stays inside `RepoInfo`.

Notice `self.repo_root` is a `PathBuf` but the return type is `&Path`. Rust automatically converts `&PathBuf` to `&Path` through a trait called `Deref`. Same idea as how `&String` auto-converts to `&str`:

```rust
pub fn head_sha(&self) -> &str {
    &self.head_sha  // &String → &str automatically
}
```

### `as_deref()`

```rust
pub fn branch(&self) -> Option<&str> {
    self.branch.as_deref()
}
```

This one is trickier. `self.branch` is `Option<String>`. We want to return `Option<&str>`. The method `as_deref()` converts:
- `Some(String)` → `Some(&str)` (borrows the string inside)
- `None` → `None`

Without `as_deref()`, you'd need more verbose code like `self.branch.as_ref().map(|s| s.as_str())`.

---

## Section 4: The `open_repo` function (lines 31–63)

```rust
pub fn open_repo(path: &Path) -> Result<RepoInfo> {
```

This is a **free function** (not a method — no `self`). The project convention prefers free functions over methods when the function doesn't need `self`. It accepts a borrowed path and returns a `Result<RepoInfo>` — either `Ok(RepoInfo)` on success or `Err(...)` on failure.

### The `?` operator and `.context()`

```rust
let repo = git2::Repository::discover(path)
    .context("not a git repository (or any parent up to mount point /)")?;
```

This is the heart of Rust error handling. Let's unpack it:

1. `Repository::discover(path)` returns `Result<Repository, git2::Error>`
2. `.context("...")` wraps the error with a human-readable message (this is why we imported `anyhow::Context`). It converts the git2 error into an `anyhow::Error` with the context layered on top.
3. `?` is the **try operator**. It means:
   - If the Result is `Ok(value)` → unwrap it and continue
   - If the Result is `Err(e)` → **return early** from the function with that error

Without `?`, you'd write:

```rust
let repo = match git2::Repository::discover(path).context("...") {
    Ok(r) => r,
    Err(e) => return Err(e),
};
```

The `?` operator saves you from this boilerplate on every fallible call.

### `Option` with `.context()`

```rust
let workdir = repo
    .workdir()
    .context("bare repositories are not supported")?;
```

`repo.workdir()` returns `Option<&Path>`, not a `Result`. But `.context()` works on `Option` too — it converts:
- `Some(value)` → `Ok(value)`
- `None` → `Err(anyhow!("bare repositories are not supported"))`

Then `?` handles it the same way. This is a clean pattern for turning "missing value" into a descriptive error.

### Chaining methods

```rust
let head_sha = head
    .target()              // Option<Oid>
    .context("HEAD does not point to a valid commit")?  // Result<Oid>
    .to_string();          // String
```

Rust encourages chaining. Read it top to bottom:
1. Get the target OID (might be `None`)
2. Convert `None` to an error with context
3. `?` — bail if it was None
4. Convert the OID to a hex string

### The `if` expression

```rust
let branch = if head.is_branch() {
    head.shorthand().map(String::from)
} else {
    None
};
```

In Rust, `if` is an **expression** — it returns a value. Both arms must return the same type (`Option<String>`).

`head.shorthand()` returns `Option<&str>`. The `.map(String::from)` converts the inner `&str` to an owned `String`:
- `Some("main")` → `Some(String::from("main"))` → `Some("main".to_owned())`
- `None` → `None`

### Struct construction with field shorthand

```rust
Ok(RepoInfo {
    repo_root,
    head_sha,
    branch,
})
```

When a variable has the same name as the struct field, Rust lets you skip the `field: value` syntax. This is equivalent to:

```rust
Ok(RepoInfo {
    repo_root: repo_root,
    head_sha: head_sha,
    branch: branch,
})
```

Wrapped in `Ok(...)` because the function returns `Result<RepoInfo>`.

---

## Section 5: Tests (lines 65–119)

```rust
#[cfg(test)]
mod tests {
    use super::*;
```

### `#[cfg(test)]`

This is a **conditional compilation** attribute. The `tests` module is only compiled when running `cargo test` — it's completely stripped from the release binary. This means test-only dependencies (like `tempfile`) add zero overhead to the final program.

### `mod tests`

A nested module. By convention, Rust puts unit tests in a `tests` module inside the same file as the code they test.

### `use super::*;`

`super` means "the parent module" (i.e., `repo`). The `*` glob import brings everything from the parent into scope, so the tests can use `open_repo`, `RepoInfo`, etc. directly.

### Test: happy path (lines 70–82)

```rust
#[test]
fn open_repo_succeeds_in_project_dir() {
    let info = open_repo(&PathBuf::from(".")).expect("should detect git repo");

    assert!(info.repo_root().is_absolute());
    assert!(info.repo_root().exists());
    assert_eq!(info.head_sha().len(), 40);
    assert!(info.head_sha().chars().all(|c| c.is_ascii_hexdigit()));
    if let Some(branch) = info.branch() {
        assert!(!branch.is_empty());
    }
}
```

- `#[test]` — marks this function as a test case for `cargo test`
- `.expect("message")` — like `.unwrap()` but with a custom panic message. Both are fine in test code (the project only bans `.unwrap()` in production code).
- `assert!` — panics (fails the test) if the condition is false
- `assert_eq!` — panics if the two values aren't equal, and prints both values on failure
- `if let Some(branch) = info.branch()` — **pattern matching** that only enters the block if `branch()` returns `Some`. This is a concise alternative to `match` when you only care about one variant.
- `.chars().all(|c| c.is_ascii_hexdigit())` — iterator chain with a **closure** (`|c|` is the parameter). Checks that every character is a hex digit (0-9, a-f).

### Test: failure path (lines 84–89)

```rust
#[test]
fn open_repo_fails_outside_git_repo() {
    let tmp = tempfile::tempdir().expect("should create temp dir");
    let result = open_repo(tmp.path());
    assert!(result.is_err());
}
```

`tempfile::tempdir()` creates a temporary directory that is **automatically deleted** when `tmp` goes out of scope (at the end of the function). This is Rust's **RAII pattern** — resources are cleaned up by the `Drop` trait when the owner is dropped. No manual cleanup needed.

### Test: detached HEAD (lines 98–118)

```rust
let sig = git2::Signature::now("test", "test@test.com").unwrap();
let tree_id = repo.index().unwrap().write_tree().unwrap();
let tree = repo.find_tree(tree_id).unwrap();
let commit_oid = repo
    .commit(Some("HEAD"), &sig, &sig, "initial", &tree, &[])
    .unwrap();

repo.set_head_detached(commit_oid).unwrap();
```

This test creates a real git repo in a temp dir, makes a commit, then detaches HEAD. Notable Rust syntax:

- `&[]` — an empty slice. The `commit` function expects a slice of parent commits (`&[&Commit]`). For the initial commit, there are no parents.
- `.unwrap()` everywhere — acceptable in tests. If any step fails, the test panics with a clear stack trace.

---

## Key Rust concepts summary

| Concept | Where in this file | Quick explanation |
|---|---|---|
| Owned vs borrowed | `PathBuf` vs `&Path`, `String` vs `&str` | Owned types store data; borrowed types reference someone else's data |
| `struct` | `RepoInfo` | Groups related data together |
| `impl` block | `impl RepoInfo { ... }` | Attaches methods to a struct |
| `&self` | All accessor methods | Borrows the struct for read access |
| `derive` | `#[derive(Debug, Clone)]` | Auto-generates trait implementations |
| `Option<T>` | `branch: Option<String>` | Rust's null-safe "might not exist" type |
| `Result<T>` | Return type of `open_repo` | Success-or-error type, Rust's primary error handling |
| `?` operator | Every fallible call in `open_repo` | Early return on error, unwrap on success |
| `.context()` | Every `?` call | Adds a human-readable message to errors |
| `if let` | `if let Some(branch) = ...` | Pattern match on one variant |
| Closures | `\|c\| c.is_ascii_hexdigit()` | Anonymous functions, used with iterators |
| `#[cfg(test)]` | Test module | Conditional compilation — tests only |
| RAII / `Drop` | `tempfile::tempdir()` | Automatic cleanup when variable goes out of scope |
| Field shorthand | `RepoInfo { repo_root, ... }` | Skip `field: value` when names match |
| `as_deref()` | `self.branch.as_deref()` | Convert `Option<String>` to `Option<&str>` |
