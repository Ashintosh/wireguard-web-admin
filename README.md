# wireguard-web-admin
A simple web interface to manage WireGuard clients, built with Rust

## Status

Early development - not production ready

This project is still incomplete and experimental.
Expect missing features, breaking changes, and bugs. The codebase and architecture are still evolving and will likely undergo major changes.

It is **not** ready for real-world or security-sensitive use yet.

## Goals

- Simple web interface for managing clients
- Server-rendered UI
- Minimal dependencies
- Easy self-hosting
- Configurable for a variety of setups

## Development Setup

This project uses `mise` as a small task runner for development and build scripts. Some steps (like moving compiled assets into `dist/`) are handled automatically.

```bash
# Running (development only)
mise dev

# Building release binaries
mise build

# Running tests (none at the moment)
mise test

# Running tests on specific crates
mise web:test    # web-app
mise admin:test  # wg-admin
```