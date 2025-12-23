# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Docker Stack is a two-part project:

1. **dctl CLI** (`cli/`): A Rust-based Docker Compose wrapper that manages multiple projects from anywhere in the terminal. See [cli/CLAUDE.md](cli/CLAUDE.md) for detailed documentation.

2. **Compose Files Collection** (`collection/`): Pre-configured Docker Compose files for local development stacks. See [collection/CLAUDE.md](collection/CLAUDE.md) for detailed documentation.

## Quick Reference

### CLI (in `cli/` directory)
```bash
cargo build              # Build
cargo test               # Run tests
cargo run -- <args>      # Run locally
```

### Collection (in `collection/` directory)
```bash
docker network create stack_dev  # Create shared network
cp .env.dist .env                # Setup environment
```

## CI/CD

GitHub Actions workflow (`.github/workflows/dctl_cli.yml`):
- Runs tests on push/PR to main
- Code coverage via grcov + Codecov
- Multi-platform release builds on tags (Linux, macOS, Windows; x86_64 and aarch64)
