.PHONY: all build test lint fmt check audit clean install

# Default target
all: fmt lint test build

# Build release binary
build:
	@echo "🔨 Building release..."
	@~/.cargo/bin/cargo build --release

# Run tests
test:
	@echo "🧪 Running tests..."
	@~/.cargo/bin/cargo test

# Lint with clippy
lint:
	@echo "🔍 Running clippy..."
	@~/.cargo/bin/cargo clippy -- -D warnings

# Format code
fmt:
	@echo "✨ Formatting code..."
	@~/.cargo/bin/cargo fmt

# Check formatting
check:
	@echo "📋 Checking format..."
	@~/.cargo/bin/cargo fmt -- --check

# Security audit
audit:
	@echo "🔒 Security audit..."
	@~/.cargo/bin/cargo audit

# Clean build artifacts
clean:
	@echo "🧹 Cleaning..."
	@~/.cargo/bin/cargo clean

# Install locally
install:
	@echo "📦 Installing..."
	@~/.cargo/bin/cargo install --path .

# Full QA pipeline
qa: check lint test audit
	@echo "✅ QA pipeline complete!"

# CI pipeline
ci: check lint test build
	@echo "✅ CI pipeline complete!"

# Testing targets
.PHONY: test test-all test-rust test-expect test-bats test-snapshot bench

test: test-rust

test-all:
	./run_tests.sh

test-rust:
	~/.cargo/bin/cargo test --release

test-expect:
	@for script in tests/expect/*.exp; do \
		echo "Running $$script..."; \
		$$script || echo "Failed"; \
	done

test-bats:
	bats tests/bats/functional.bats

test-snapshot:
	~/.cargo/bin/cargo test --test snapshot_tests --release

bench:
	~/.cargo/bin/cargo bench
