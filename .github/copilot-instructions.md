# Copilot instructions for task_now_service

- Project type: Rust service crate (binary) declared in `Cargo.toml`.
- Entry point: `src/main.rs` (currently minimal - prints "Hello, world!").

What the repo expects from AI edits
- Keep changes minimal and idiomatic Rust 2024 edition.
- Prefer edits to `src/` modules; avoid changing `Cargo.toml` version numbers unless explicitly asked.
- Diesel is used (see `Cargo.toml`). Use Diesel patterns already in the repo if adding DB code: connection pooling via `r2d2`, feature flags `chrono` are available.

Architecture and conventions
- Single-binary crate. If adding new modules, place them under `src/` and import in `main.rs`.
- Database access should use Diesel + r2d2. Create a `db` module with a `Pool` type alias and connection helpers.
- Use `chrono` for timestamps (enabled via Diesel features).

Build / test / debug
- Build and check: `cargo build` / `cargo check`.
- Run: `cargo run`.
- Tests: `cargo test`.
- Formatting & linting: follow default `rustfmt` and `clippy` conventions; run `cargo fmt` and `cargo clippy`.

Patterns and examples
- For new database models, create `src/schema.rs` (via Diesel `infer_schema!` or using migration-generated schema) and `src/models.rs`.
- When creating async or background workers, prefer using `tokio` runtime and document in `Cargo.toml` if added.

Integration points
- Diesel is the main external integration; no other external services discovered in the repo.

Notes for AI code generators
- Avoid introducing large new frameworks or build tooling unless the user requests it.
- When generating DB migrations, explain the migration steps and add files under `migrations/` if requested.

If you need more context
- Ask to inspect additional files or describe intended feature (API endpoints, schemas, services) before implementing.

Please review and tell me where you'd like more detail or extra examples specific to a feature.