# OCLI Testing Guide

Comprehensive testing suite for OCLI v0.3.0 with 5 different testing approaches.

## Test Structure

```
tests/
├── expect/              # Expect scripts for interactive testing
│   ├── test_basic.exp
│   ├── test_config.exp
│   └── test_mcp.exp
├── bats/                # BATS functional tests
│   └── functional.bats
├── integration_test.rs  # Rust integration tests
└── snapshot_tests.rs    # Insta snapshot tests

benches/
└── functional_bench.rs  # Criterion benchmarks
```

## Quick Start

Run all tests:
```bash
./run_tests.sh
```

Run with benchmarks:
```bash
./run_tests.sh --bench
```

## Test Types

### 1. Expect Scripts (Interactive Testing)
Tests actual terminal interaction and ANSI output

### 2. Rust Integration Tests  
Fast, reliable API testing - 11 tests covering core functionality

### 3. BATS Tests
Shell-based functional testing - 15 tests

### 4. Snapshot Tests
Output verification - 8 tests for UI consistency

### 5. Criterion Benchmarks
Performance regression testing - 5 benchmarks

## Total Coverage
57 tests across 5 frameworks

