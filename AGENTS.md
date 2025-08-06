<system_context>
This repository is an AI coding assistant tool (`zenocode`) with CLI and TUI interfaces supporting multiple LLM providers. It uses a Rust workspace with separate crates for core functionality, interfaces, and provider integrations.
</system_context>

<file_map>
## FILE MAP
- `Cargo.toml` - Workspace configuration
- `crates/core/` - Shared logic (traits, utilities, config)
- `crates/interface/cli/` - Command-line interface
- `crates/interface/tui/` - Terminal user interface, initialised by the cli
- `crates/completion` - Completion logic for LLM responses
- `crates/providers/` - LLM provider implementations (sub-crates)
- `bin` - Binary entry point
</file_map>

<paved_path>
## BUILD/LINT/TEST COMMANDS
- Build all: `cargo build --workspace`
- Build specific crate: `cargo build -p <crate-name>`
- Run CLI: `cargo run -p zenocode-cli -- [args]`
- Run TUI: `cargo run -p zenocode-tui`
- Test all: `cargo test --workspace`
- Single test: `cargo test -p <crate-name> -- test_name`
- Lint: `cargo clippy --workspace -- -D warnings`
- Format: `cargo fmt --all`
</paved_path>

<patterns>
## CODE STYLE GUIDELINES
- **Imports**: Group by stdlib > external crates > local modules
- **Formatting**: Follow `cargo fmt` defaults
- **Types**: Explicit types for public interfaces, inference allowed internally
- **Naming**: `snake_case` (vars/fns), `PascalCase` (types/traits), `SCREAMING_SNAKE_CASE` (consts)
- **Error Handling**: Use `anyhow`/`thiserror`; avoid `unwrap()` in production code
- **Documentation**: Doc comments (`///`) for all public items
- **Providers**: Isolate behind `core::LLMProvider` trait
</patterns>

<workflow>
## WORKFLOW FOR NEW FEATURES
1. **Add Provider**: 
   - Create `crates/providers/<name>/` 
   - Implement `core::LLMProvider` trait
   - Add to workspace in root `Cargo.toml`
2. **Add CLI/TUI Command**:
   - Extend `cli` or `tui` crate
   - Use `core` types for consistency
3. **Testing**:
   - Unit tests in each crate
   - Integration tests in `tests/` directory
</workflow>

<critical_notes>
## CRITICAL NOTES
- **Core Independence**: `core` crate MUST NOT depend on provider implementations
- **Lightweight Dependencies**: Keep `core` minimal; providers handle LLM-specific deps
- **Async Patterns**: Use `tokio` for async; avoid blocking calls in TUI
- **Versioning**: Synchronize crate versions in workspace
</critical_notes>
