# OCLI Usage Analytics

## 1. GitHub Metrics (Built-in, Free)

### Check Current Stats
```bash
# Stars
gh api repos/wiseowltechnet/ollama-ocli --jq .stargazers_count

# Clones (last 14 days)
gh api repos/wiseowltechnet/ollama-ocli/traffic/clones

# Views (last 14 days)  
gh api repos/wiseowltechnet/ollama-ocli/traffic/views
```

### Web Dashboard
https://github.com/wiseowltechnet/ollama-ocli/graphs/traffic

**Tracks:**
- Unique visitors
- Page views
- Git clones
- Traffic sources

## 2. Crates.io (After Publishing)

Once published:
- Total downloads
- Recent downloads (90 days)
- Dependent crates

View at: https://crates.io/crates/ocli

## 3. GitHub Releases

Track binary downloads:
```bash
gh api repos/wiseowltechnet/ollama-ocli/releases
```

## 4. Optional: Anonymous Telemetry

Add to OCLI (opt-in only):
- Version usage
- OS distribution
- Feature usage
- No personal data

## Recommended: Start Simple

**Now:** GitHub metrics (free, automatic)
**Week 1:** Publish to crates.io (download counts)
**v0.3.0:** Consider opt-in telemetry

---

Privacy-first approach: Public metrics only, opt-in for anything else.
