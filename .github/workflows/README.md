# CI/CD Workflows

This directory contains GitHub Actions workflows for continuous integration, deployment, and releases.

## Workflows

### 1. CI (`ci.yml`)

**Triggers:**
- Push to `main`, `master`, or `develop` branches
- Pull requests to `main`, `master`, or `develop` branches

**Jobs:**

#### Test Suite
- Runs all tests for `stegano-core` and `stegano-cli`
- Uses cargo caching for faster builds
- Runs on Ubuntu

#### Rustfmt
- Checks code formatting
- Fails if code is not properly formatted
- Run `cargo fmt --all` locally to fix

#### Clippy
- Runs Rust linter
- Treats warnings as errors
- Ensures code quality

#### Build CLI
- Builds CLI on Linux, macOS, and Windows
- Uploads artifacts for each platform
- Ensures cross-platform compatibility

#### Build WASM
- Builds WASM module using wasm-pack
- Uploads WASM package artifact
- Validates WASM compilation

#### Build Web App
- Builds the React web application
- Includes WASM module
- Uploads dist artifact

### 2. Deploy (`deploy.yml`)

**Triggers:**
- Push to `main` or `master` branch
- Manual workflow dispatch

**What it does:**
- Builds WASM module
- Builds React web app with GitHub Pages base path
- Deploys to GitHub Pages

**Setup Required:**

1. **Enable GitHub Pages:**
   - Go to repository Settings → Pages
   - Source: "GitHub Actions"

2. **Base Path Configuration:**
   - Edit `web-app/vite.config.ts`
   - Update `base` path to match your repository name
   - Current: `/img_stegano/`
   - For custom domain: use `/`

3. **Permissions:**
   - Workflow has required permissions configured
   - No additional setup needed

**Access deployed app:**
- `https://c12i.github.io/img_stegano/`

### 3. Release (`release.yml`)

**Triggers:**
- Push tags matching `v*.*.*` (e.g., `v1.0.0`)
- Manual workflow dispatch

**What it does:**
- Creates GitHub release
- Builds CLI binaries for multiple platforms:
  - Linux (GNU and musl)
  - macOS (Intel and ARM)
  - Windows
- Uploads binaries to release
- Optionally publishes to crates.io

**Creating a release:**

```bash
# Tag the release
git tag v1.0.0
git push origin v1.0.0

# Or create via GitHub UI
```

**Platforms built:**
- `x86_64-unknown-linux-gnu` - Linux (glibc)
- `x86_64-unknown-linux-musl` - Linux (musl, static)
- `x86_64-apple-darwin` - macOS Intel
- `aarch64-apple-darwin` - macOS ARM (M1/M2)
- `x86_64-pc-windows-msvc` - Windows

**Crates.io Publishing:**
- Requires `CARGO_TOKEN` secret
- Add in Settings → Secrets → Actions
- Get token from https://crates.io/settings/tokens

## Caching Strategy

All workflows use caching to speed up builds:

- **Cargo registry** - Downloaded crates
- **Cargo index** - Crate metadata
- **Cargo build** - Compiled dependencies
- **Yarn cache** - Node modules

## Local Testing

### Test what CI runs:

```bash
# Run tests
cd stegano-core && cargo test
cd stegano-cli && cargo test

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Build CLI
cd stegano-cli && cargo build --release

# Build WASM
cd stegano-wasm && wasm-pack build --target web

# Build web app
cd web-app && yarn build
```

### Test GitHub Pages build locally:

```bash
cd web-app
GITHUB_PAGES=true yarn build
yarn preview
```

## Troubleshooting

### CI failing on tests
- Run tests locally: `cargo test`
- Check for platform-specific issues
- Ensure all dependencies are in Cargo.toml

### Deploy failing
- Check GitHub Pages is enabled
- Verify base path in vite.config.ts
- Check workflow permissions

### Release failing
- Verify tag format: `v1.0.0`
- Check cross-compilation dependencies
- For crates.io: verify CARGO_TOKEN secret

### WASM build failing
- Ensure wasm32-unknown-unknown target installed
- Check wasm-pack version compatibility
- Verify wasm-bindgen versions match

## Status Badges

Add to your README.md:

```markdown
![CI](https://github.com/<username>/<repo>/workflows/CI/badge.svg)
![Deploy](https://github.com/<username>/<repo>/workflows/Deploy%20to%20GitHub%20Pages/badge.svg)
```

## Maintenance

### Updating dependencies
- Dependabot will create PRs for updates
- CI will run on all PRs
- Review and merge when tests pass

### Adding new platforms
Edit `release.yml` matrix to add targets:
```yaml
- os: ubuntu-latest
  target: aarch64-unknown-linux-gnu
  artifact_name: img_stegano_cli
  asset_name: img_stegano_cli-linux-arm64
```

