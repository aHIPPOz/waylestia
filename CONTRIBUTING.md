# Contributing to Waylestia

Thank you for your interest in contributing to Waylestia! This document provides guidelines and instructions for contributing.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Setup](#development-setup)
4. [Making Changes](#making-changes)
5. [Submitting Changes](#submitting-changes)
6. [Style Guidelines](#style-guidelines)
7. [Project Structure](#project-structure)

## Code of Conduct

We are committed to providing a welcoming and inspiring community for all. Please read and follow our Code of Conduct:

- Be respectful and constructive in all interactions
- Welcome diverse perspectives and experiences
- Focus on what is best for the community
- Show empathy towards other community members
- Report inappropriate behavior to [team email]

## Getting Started

### Prerequisites

- Rust 1.70+ (install from https://rustup.rs/)
- Cargo (comes with Rust)
- Git
- Linux (Ubuntu 20.04+ recommended, or Fedora 36+)
- Basic knowledge of Rust, TypeScript, and GTK

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/waylestia.git
   cd waylestia
   ```
3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/aHIPPOz/waylestia.git
   ```

## Development Setup

### Build from Source

```bash
# Install dependencies (Ubuntu)
sudo apt-get install build-essential pkg-config libgtk-4-dev libadwaita-1-dev

# Or Fedora
sudo dnf install gcc pkg-config gtk4-devel libadwaita-devel

# Build all components
make all

# Or individually
make core
make widgets
```

### Run Services

```bash
# Install to ~/.local (recommended for development)
INSTALL_PREFIX=$HOME/.local make install

# Or use systemd
./scripts/install.sh

# Start
systemctl --user start waylestia-core
systemctl --user start waylestia-widgets

# View logs
journalctl --user -u waylestia-core -f
```

### Development Workflow

```bash
# Make your changes
# Test locally
make test

# Check code quality
make lint
make fmt

# Rebuild after changes
make clean
make core  # or make widgets

# Restart services
systemctl --user restart waylestia-core
```

## Making Changes

### Choose an Issue

1. Look for issues labeled `good-first-issue` or `help-wanted`
2. Comment on the issue to let maintainers know you're working on it
3. Create a new branch for your work

### Create a Feature Branch

```bash
# Update main branch
git fetch upstream
git checkout main
git merge upstream/main

# Create feature branch
git checkout -b feature/your-feature-name
```

### Code Changes

#### For Rust (core, widgets)

- Follow Rust naming conventions
- Use `cargo fmt` before committing
- Run `cargo clippy` for linting
- Add documentation comments (`///`) for public items
- Write unit tests for new functionality

```bash
# Format
cargo fmt

# Lint
cargo clippy -- -D warnings

# Test
cargo test
```

#### For TypeScript/JavaScript (apps)

- Use explicit types (no `any` unless unavoidable)
- Follow Google TypeScript style guide
- Add JSDoc comments for functions
- Keep functions and classes focused and single-purpose

#### For Protocol Buffers

- Keep message definitions simple and focused
- Add meaningful comments to fields
- Use enums for fixed sets of values
- Maintain backward compatibility

### Commit Messages

Write clear, descriptive commit messages:

```
Short summary (50 chars max)

Longer description explaining the change, why it was made, and any
important details. Wrap at 72 characters. Reference related issues
with "Fixes #123" or "Related to #456".

- Bullet points are helpful
- For complex changes
- Explain the rationale
```

Examples:
```
Add perf monitoring to core daemon

Implement CPU/GPU/RAM monitoring via sysinfo crate.
Exposes metrics via protobuf IPC for shell consumption.

- Adds PerfStats protobuf message
- PerfMonitor struct with update() method
- Integration with IPC server

Fixes #42
```

## Submitting Changes

### Before Submitting

1. **Full build test**:
   ```bash
   make clean
   make all
   make test
   ```

2. **Code quality**:
   ```bash
   make lint
   make fmt
   cargo clippy
   ```

3. **Documentation**:
   - Update README.md if behavior changes
   - Update ARCHITECTURE.md if structure changes
   - Add code comments for complex logic
   - Add/update inline documentation

4. **Commit cleanup**:
   ```bash
   # Make sure commits are logical and ordered
   git log origin/main..HEAD
   
   # Squash fixup commits if needed
   git rebase -i origin/main
   ```

### Create a Pull Request

1. Push to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

2. Create a Pull Request on GitHub
3. Fill out the PR template completely
4. Link related issues
5. Describe your changes and reasoning

### PR Review Process

- Maintainers will review your PR
- Respond to feedback or requests for changes
- Keep commits logical during review
- Mark conversations as resolved once addressed
- A minimum of one approval is required to merge

## Style Guidelines

### Rust Code

```rust
// Use meaningful variable names
let widget_count = widgets.len();

// Use module organization
mod widget;
use widget::{Widget, WidgetState};

// Document public APIs
/// Creates a new widget instance with the given ID.
/// 
/// # Arguments
/// * `widget_id` - The unique widget identifier
/// 
/// # Returns
/// A new WidgetInstance with default state
pub fn create_instance(widget_id: String) -> WidgetInstance {
    // ...
}

// Use Result for fallible operations
pub fn load_manifest(path: &Path) -> Result<WidgetManifest> {
    // ...
}
```

### TypeScript/JavaScript

```typescript
// Use explicit types
function updateWidget(id: string, state: WidgetState): void {
    // ...
}

// Use const by default
const WIDGET_TIMEOUT = 5000;

// Document with JSDoc
/**
 * Initialize the widget application
 * @param elementId - ID of the DOM element to mount on
 * @returns Promise that resolves when ready
 */
async function initialize(elementId: string): Promise<void> {
    // ...
}
```

### Protocol Buffers

```protobuf
// Group related messages
message PerformanceMetrics {
  float cpu_usage = 1;      // 0-100 percentage
  float gpu_usage = 2;      // 0-100 percentage
  float ram_usage = 3;      // 0-100 percentage
  
  // Reserve fields for future expansion
  reserved 4, 5;
}
```

## Project Structure

```
waylestia/
├── core/              # Rust daemon - PR guidelines:
│                      # - Follow Rust conventions
│                      # - Add tests for new modules
│                      # - Update state.rs for new data
│
├── widgets/           # Widget engine - PR guidelines:
│                      # - Maintain loader backwards compat
│                      # - Test manifest parsing edge cases
│
├── apps/              # GJS applications - PR guidelines:
│                      # - Use GTK widgets properly
│                      # - Implement IPC patterns correctly
│
├── protobuf/          # Protocol definitions - PR guidelines:
│                      # - Don't change existing message IDs
│                      # - Add comments for new messages
│                      # - Consider forward compatibility
│
├── assets/            # Resources - PR guidelines:
│                      # - Optimize image sizes
│                      # - Keep manifests valid
│
└── scripts/           # Build/install tools - PR guidelines:
                       # - Test on clean system
                       # - Support INSTALL_PREFIX
```

## Useful Commands

```bash
# View file history
git log -p -- path/to/file.rs

# Find Contributors
git shortlog -sn

# Rebase on latest
git fetch upstream
git rebase upstream/main

# Interactive rebase (edit commits)
git rebase -i HEAD~3

# Squash commits
git rebase -i master  # mark 'squash' on commits to combine
```

## Getting Help

- **Questions**: Open an issue with label `question`
- **Discussion**: Use GitHub Discussions
- **Bugs**: File an issue with reproduction steps
- **Chat**: Join our community chat [link]

## Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md file
- Release notes
- GitHub contributors page

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (GPL-3.0).

---

Thank you for contributing to Waylestia! 🎉
