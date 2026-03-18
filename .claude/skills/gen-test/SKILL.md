---
name: gen-test
description: Generate idiomatic Rust unit test scaffolding for a source file
disable-model-invocation: true
---

# Generate Tests

Generate a `#[cfg(test)] mod tests` block for the specified Rust source file.

## Arguments

- `$ARGUMENTS` — path to the Rust source file (e.g., `src/diff/engine.rs`)

## Instructions

1. Read the target file at `$ARGUMENTS`.
2. Identify all public functions, methods, and key logic branches.
3. Generate a `#[cfg(test)] mod tests` block at the bottom of the file with:
   - `use super::*;`
   - One `#[test]` function per public function/method covering the happy path.
   - Additional tests for edge cases (empty input, error paths) where obvious.
4. Use `assert_eq!`, `assert!`, or `matches!()` as appropriate. Do not use `assert_matches!` (unstable).
5. Use descriptive test names in snake_case: `test_<function_name>_<scenario>`.
6. If the file already has a `mod tests` block, add new tests to it rather than creating a duplicate.
7. Run `cargo test` to verify the new tests compile and pass.
