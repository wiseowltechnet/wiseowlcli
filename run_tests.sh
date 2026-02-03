#!/bin/bash
set -e

echo "🧪 OCLI Test Suite Runner"
echo "=========================="
echo ""

# Build release binary first
echo "📦 Building release binary..."
~/.cargo/bin/cargo build --release
echo "✅ Build complete"
echo ""

# 1. Rust Integration Tests
echo "🦀 Running Rust Integration Tests..."
~/.cargo/bin/cargo test --test integration_test --release
echo ""

# 2. Expect Scripts (if expect is installed)
if command -v expect &> /dev/null; then
    echo "🎭 Running Expect Scripts..."
    for script in tests/expect/*.exp; do
        echo "  Running $(basename $script)..."
        $script || echo "  ⚠️  Test failed"
    done
    echo ""
else
    echo "⚠️  Expect not installed, skipping expect tests"
    echo ""
fi

# 3. BATS Tests (if bats is installed)
if command -v bats &> /dev/null; then
    echo "🦇 Running BATS Tests..."
    bats tests/bats/functional.bats
    echo ""
else
    echo "⚠️  BATS not installed, skipping BATS tests"
    echo ""
fi

# 4. Snapshot Tests
echo "📸 Running Snapshot Tests..."
~/.cargo/bin/cargo test --test snapshot_tests --release
echo ""

# 5. Benchmarks (optional, takes longer)
if [ "$1" == "--bench" ]; then
    echo "⚡ Running Benchmarks..."
    ~/.cargo/bin/cargo bench
    echo ""
fi

echo "✅ All tests complete!"
