# rev

A TUI-native code review tool for AI agent coding workflows.

`rev` renders diffs with syntax highlighting and provides a structured annotation layer (inline and global comments) that can be exported in a format AI agents understand and act on.

## Status

Work in progress. Not yet ready for public use.

## Requirements

- Rust 1.75+
- macOS system deps: `brew install cmake pkg-config libgit2 oniguruma`

## Build

```bash
cargo build
cargo run
```

## License

MIT
