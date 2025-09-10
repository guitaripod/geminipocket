# GeminiPocket Agent Guidelines

## Build Commands
- **Build all**: `cargo build --workspace`
- **Build release**: `cargo build --workspace --release`
- **Build CLI only**: `cargo build -p geminipocket`
- **Build backend**: `cd backend/worker && cargo build --release`

## Test Commands
- **Run all tests**: `cargo test --workspace`
- **Run CLI tests**: `cargo test -p geminipocket`
- **Run backend tests**: `cd backend/worker && cargo test`
- **Run single test**: `cargo test test_name` or `cargo test -- --exact test_name`
- **Run with output**: `cargo test -- --nocapture`

## Code Style Guidelines

### Imports
- Group imports: `std`, external crates, then local modules
- Use explicit imports, avoid globs except for prelude items
- Example:
```rust
use std::path::PathBuf;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::types::Config;
```

### Naming Conventions
- **Functions**: snake_case (e.g., `generate_image`, `handle_config`)
- **Types/Structs**: PascalCase (e.g., `GeminiClient`, `ApiResponse`)
- **Constants**: SCREAMING_SNAKE_CASE
- **Modules**: snake_case (e.g., `api.rs`, `types.rs`)

### Error Handling
- Use `anyhow::Result<T>` for all functions that can fail
- Prefer `?` operator for error propagation
- Use descriptive error messages with `anyhow::anyhow!`
- Example: `Err(anyhow::anyhow!("API request failed with status: {}", response.status()))`

### Types & Serialization
- Use `#[derive(Serialize, Deserialize)]` for API types
- Use `Option<T>` for optional fields
- Use `#[allow(dead_code)]` for unused fields in API responses

### Async Code
- Prefer async methods over blocking operations

### Documentation
- Use `///` for public API documentation
- Include examples in doc comments when helpful
- Keep comments concise and focused on "why" not "what"
- Do not add code comments.

### Dependencies
- Use workspace dependencies for shared crates (serde, anyhow, etc.)
- Check existing Cargo.toml files before adding new dependencies
- Prefer well-maintained crates with good documentation

### Database
- Do not create new database schema versions. We are in development phase. You can just overwrite what was there when we need new fields.
