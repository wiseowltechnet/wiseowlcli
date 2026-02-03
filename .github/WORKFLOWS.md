# GitHub Actions Workflows

## Overview

OCLI uses 5 automated workflows for CI/CD, security, and maintenance.

## Workflows

### 1. CI (Continuous Integration)
**File**: `.github/workflows/ci.yml`  
**Triggers**: Push to master/main, Pull Requests

**Jobs:**
- **Test Suite** - Runs on Ubuntu & macOS
  - Format check (`cargo fmt`)
  - Linting (`cargo clippy`)
  - Tests (`cargo test`)
  - Release build
  - Dependency caching for faster builds

- **Security Audit** - Checks for vulnerabilities
  - Uses `rustsec/audit-check`
  - Scans dependencies

- **Benchmark** - Performance checks
  - Runs benchmarks
  - Tracks performance

**Features:**
- ✅ Matrix builds (Ubuntu, macOS)
- ✅ Dependency caching
- ✅ Parallel execution
- ✅ Security scanning

### 2. Release
**File**: `.github/workflows/release.yml`  
**Triggers**: Git tags (v*)

**Jobs:**
- **Create Release** - Creates GitHub release
- **Build** - Multi-platform binaries
  - Linux (amd64, arm64)
  - macOS (amd64, arm64)
  - Stripped binaries
  - Compressed archives

- **Publish to crates.io** - Automatic publishing

**Artifacts:**
- `ocli-linux-amd64.tar.gz`
- `ocli-linux-arm64.tar.gz`
- `ocli-macos-amd64.tar.gz`
- `ocli-macos-arm64.tar.gz`

### 3. Dependencies
**File**: `.github/workflows/dependencies.yml`  
**Triggers**: Weekly (Sunday), Manual

**Jobs:**
- **Update** - Auto-update dependencies
  - Runs `cargo update`
  - Tests changes
  - Creates PR if successful

- **Audit** - Security check
  - Weekly vulnerability scan

**Benefits:**
- Automated dependency updates
- Security monitoring
- No manual intervention needed

### 4. Performance
**File**: `.github/workflows/performance.yml`  
**Triggers**: Push, Pull Requests

**Jobs:**
- **Benchmark** - Performance tracking
  - Runs benchmarks
  - Checks binary size
  - Comments on PRs

**Metrics:**
- Binary size
- Build time
- Benchmark results

### 5. Pages (Documentation)
**File**: `.github/workflows/pages.yml`  
**Triggers**: Push to master/main

**Jobs:**
- **Deploy** - Updates GitHub Pages
  - Builds documentation
  - Deploys to Pages
  - Updates site

## Setup Requirements

### Secrets Needed
- `GITHUB_TOKEN` - Automatic (provided by GitHub)
- `CARGO_TOKEN` - For crates.io publishing (optional)

### Permissions
- `contents: write` - For releases
- `pages: write` - For documentation
- `id-token: write` - For Pages deployment

## Usage

### Trigger Release
```bash
git tag v0.3.0
git push origin v0.3.0
```

### Manual Dependency Update
```bash
gh workflow run dependencies.yml
```

### View Workflow Status
```bash
gh run list
gh run view <run-id>
```

## Workflow Status Badges

Add to README.md:
```markdown
[![CI](https://github.com/wiseowltechnet/ollama-ocli/workflows/CI/badge.svg)](https://github.com/wiseowltechnet/ollama-ocli/actions)
[![Release](https://github.com/wiseowltechnet/ollama-ocli/workflows/Release/badge.svg)](https://github.com/wiseowltechnet/ollama-ocli/actions)
[![Security](https://github.com/wiseowltechnet/ollama-ocli/workflows/Dependencies/badge.svg)](https://github.com/wiseowltechnet/ollama-ocli/actions)
```

## Performance

**Build Times:**
- CI: ~5 minutes (with cache)
- Release: ~15 minutes (4 platforms)
- Dependencies: ~3 minutes

**Caching:**
- Cargo registry
- Cargo git
- Build artifacts
- ~80% faster builds

## Best Practices

1. **Always run CI before merging**
2. **Review security audit results**
3. **Check binary size changes**
4. **Update dependencies weekly**
5. **Test releases on all platforms**

---

**All workflows are production-ready!** 🚀
