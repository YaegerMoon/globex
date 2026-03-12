# Globex Justfile

# Default command
default:
    @just --list

# Install all dependencies
install:
    @echo "Installing frontend dependencies..."
    cd globex-frontend && pnpm install

# Run backend development server
dev-backend:
    @echo "Starting backend..."
    cd globex-backend && cargo run

# Run frontend development server
dev-frontend:
    @echo "Starting frontend..."
    cd globex-frontend && pnpm dev

# Build both applications
build: build-backend build-frontend

# Build the backend
build-backend:
    @echo "Building backend..."
    cd globex-backend && cargo build --release

# Build the frontend
build-frontend:
    @echo "Building frontend..."
    cd globex-frontend && pnpm build

# Lint both applications
lint:
    @echo "Linting backend..."
    cd globex-backend && cargo clippy -- -D warnings
    @echo "Linting frontend..."
    cd globex-frontend && pnpm lint

# Format code
fmt:
    @echo "Formatting backend..."
    cd globex-backend && cargo fmt
    @echo "Formatting frontend..."
    cd globex-frontend && pnpm exec prettier --write .
