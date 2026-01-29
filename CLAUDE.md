# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Context

This is a learning project implementing an S3-like object store in Rust. The owner prefers to be advised first rather than having code implemented directly unless explicitly requested. Consult `design_doc.md` before making significant architectural changes.

## Build Commands

```bash
cargo build          # Build the project
cargo run            # Run the HTTP server (listens on 127.0.0.1:3000)
cargo test           # Run all tests
cargo test <name>    # Run a specific test by name
cargo check          # Type-check without building
```

## Architecture

### Storage Strategy

The system uses two storage backends based on file size:
- **SegmentStore**: For small files (≤30KB), packs multiple files into segment files to reduce inode overhead
- **StandaloneStore**: For larger files, stores each file individually with auto-incrementing numeric names (`0.store`, `1.store`, etc.)

Storage paths:
- Segment files: `./store/segment/`
- Standalone files: `./store/standalone/`

### Core Components

**ObjectStore trait** (`src/store/object_store.rs`): Common interface for storage backends with `save()` and `open()` methods.

**StoreType enum** (`src/common/store_type.rs`): Discriminates between Packed (segment offset/length) and Standalone (file path) storage.

**Metadata** (`src/store/metadata.rs`): Tracks object info (UUID, bucket, name, prefix, store type). Planned to use sled for persistence.

**AppError** (`src/store/app_error.rs`): Wraps `anyhow::Error` for Axum HTTP responses, converting errors to 500 status.

### HTTP API

- `PUT /object/:bucket/*key` - Upload object (handler in `src/http/object.rs`)
- GET endpoints for download and listing are planned

### Write Path Flow

HTTP request → `put_object` handler → size-based routing → ObjectStore::save() → returns StoreType → (metadata save - not yet implemented)

## Testing

Uses `rstest` for parameterized tests. Test modules are co-located with implementation files using `#[cfg(test)]`.
