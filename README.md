# Globex

Monorepo for Globex.

## Structure

- `globex-backend`: Rust (Axum)
- `globex-frontend`: Next.js (App Router)

## Development Commands

This monorepo uses [`just`](https://github.com/casey/just) to orchestrate tasks.
Run `just --list` or `just` from the project root to see all available commands.

### Setup

- `just install` - Installs `pnpm` dependencies for the frontend.

### Running Local Servers

- `just dev-backend` - Runs the Axum backend (`cargo run`).
- `just dev-frontend` - Runs the Next.js frontend (`pnpm dev`).

### Building and Linting

- `just build` - Builds both backend (release mode) and frontend.
- `just build-backend` / `just build-frontend` - Build individually.
- `just lint` - Lints both backend (`cargo clippy`) and frontend (`eslint`).
- `just fmt` - Runs `cargo fmt` and `prettier` to format code.

