# GitHub Setup Guide

## 1. Create Repository

```bash
# On GitHub, create new repo: ollama-ocli
# Description: 🦉 AI coding assistant with self-improvement, LCARS styling, and MCP support
# Topics: rust, ai, ollama, cli, mcp, lcars, coding-assistant
```

## 2. Push Code

```bash
cd ~/projects/ollama-ocli
git remote add origin https://github.com/yourusername/ollama-ocli.git
git push -u origin master
```

## 3. Enable GitHub Pages

1. Go to **Settings** → **Pages**
2. Source: **GitHub Actions**
3. Pages will auto-deploy on push to master

## 4. Enable GitHub Actions

Actions are already configured:
- ✅ `.github/workflows/qa.yml` - QA pipeline (format, lint, test)
- ✅ `.github/workflows/pages.yml` - Deploy docs to Pages
- ✅ `.github/workflows/release.yml` - Build releases on tags

## 5. Create First Release

```bash
git tag v0.2.0
git push origin v0.2.0
```

This triggers:
- Build for Linux and macOS
- Create GitHub release
- Upload binaries

## 6. Configure Repository Settings

### Branch Protection
- Settings → Branches → Add rule
- Branch name: `master`
- ✅ Require status checks (QA must pass)
- ✅ Require branches to be up to date

### Topics
Add these topics for discoverability:
- `rust`
- `ai`
- `ollama`
- `cli`
- `mcp`
- `lcars`
- `coding-assistant`
- `self-improving`

### About Section
- Description: "🦉 AI coding assistant with self-improvement, LCARS styling, and MCP support"
- Website: https://yourusername.github.io/ollama-ocli
- Topics: (as above)

## 7. Verify Workflows

After pushing, check:
1. **Actions** tab - QA workflow should run
2. **Pages** - Docs should deploy
3. **Releases** - Create tag to test release workflow

## 8. Update URLs

Replace `yourusername` in these files:
- README.md
- CONTRIBUTING.md
- docs/_config.yml
- docs/index.html (in pages.yml)
- ocli.rb (Homebrew formula)

## 9. Post-Publish

1. **Star your own repo** ⭐
2. **Share on**:
   - Hacker News
   - Reddit (r/rust, r/programming)
   - Twitter/X
   - Dev.to
3. **Add demo GIF** to README
4. **Create v0.2.0 release notes**

## GitHub Pages URL

After setup, docs will be at:
```
https://yourusername.github.io/ollama-ocli
```

## Badges for README

Add these to top of README.md:

```markdown
[![CI](https://github.com/yourusername/ollama-ocli/workflows/QA/badge.svg)](https://github.com/yourusername/ollama-ocli/actions)
[![Release](https://img.shields.io/github/v/release/yourusername/ollama-ocli)](https://github.com/yourusername/ollama-ocli/releases)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
```

---

**Ready to publish!** 🚀
