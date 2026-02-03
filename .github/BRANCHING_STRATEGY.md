# Git Workflow Strategy

## Strategy: GitHub Flow (Simplified)

We use **GitHub Flow** instead of Git Flow because:
- ✅ Simpler (only main + feature branches)
- ✅ Better for continuous deployment
- ✅ Faster releases
- ✅ Less overhead

## Branch Structure

### Main Branch
- **`master`** - Production-ready code
- Always deployable
- Protected branch
- All changes via PR

### Feature Branches
- **`feature/feature-name`** - New features
- **`fix/bug-name`** - Bug fixes
- **`docs/topic`** - Documentation
- **`refactor/component`** - Code refactoring
- **`perf/optimization`** - Performance improvements

## Workflow

### 1. Create Feature Branch
```bash
git checkout master
git pull origin master
git checkout -b feature/my-feature
```

### 2. Develop & Commit
```bash
# Make changes
git add .
git commit -m "feat: add new feature"

# Follow conventional commits
# feat: new feature
# fix: bug fix
# docs: documentation
# refactor: code refactoring
# perf: performance improvement
# test: add tests
# chore: maintenance
```

### 3. Push & Create PR
```bash
git push origin feature/my-feature
gh pr create --title "feat: add new feature" --body "Description"
```

### 4. Review & Merge
- CI must pass
- Code review required
- Squash and merge
- Delete branch after merge

### 5. Release
```bash
# On master
git tag v0.3.1
git push origin v0.3.1
# GitHub Actions auto-builds & publishes
```

## Commit Message Convention

### Format
```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types
- **feat**: New feature
- **fix**: Bug fix
- **docs**: Documentation
- **style**: Formatting
- **refactor**: Code restructuring
- **perf**: Performance
- **test**: Tests
- **chore**: Maintenance

### Examples
```bash
feat(mcp): add tool discovery
fix(context): resolve memory leak
docs(readme): update installation steps
refactor(tools): simplify tool execution
perf(streaming): optimize buffer size
```

## Branch Protection Rules

### Master Branch
- ✅ Require PR reviews (1 reviewer)
- ✅ Require status checks (CI must pass)
- ✅ Require branches up to date
- ✅ No direct pushes
- ✅ No force pushes

### Setup
```bash
gh api repos/wiseowltechnet/ollama-ocli/branches/master/protection \
  -X PUT \
  -f required_status_checks[strict]=true \
  -f required_pull_request_reviews[required_approving_review_count]=1 \
  -f enforce_admins=true
```

## Release Process

### Semantic Versioning
- **Major** (v1.0.0): Breaking changes
- **Minor** (v0.3.0): New features
- **Patch** (v0.3.1): Bug fixes

### Release Steps
1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Commit: `chore: bump version to v0.3.1`
4. Create tag: `git tag v0.3.1`
5. Push: `git push origin v0.3.1`
6. GitHub Actions auto-releases

## Hotfix Process

### For Critical Bugs
```bash
# Create hotfix branch from master
git checkout master
git checkout -b fix/critical-bug

# Fix and commit
git commit -m "fix: resolve critical bug"

# Push and create PR
git push origin fix/critical-bug
gh pr create --title "fix: critical bug" --label "priority:high"

# After merge, tag immediately
git tag v0.3.1
git push origin v0.3.1
```

## Best Practices

### DO
- ✅ Keep branches short-lived (< 1 week)
- ✅ Commit often, push daily
- ✅ Write descriptive commit messages
- ✅ Rebase before merging
- ✅ Delete merged branches
- ✅ Tag all releases

### DONT
