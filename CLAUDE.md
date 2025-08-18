# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based poster generation tool that creates poster images with text and image elements using Skia Safe. The project supports both command-line interface and HTTP API, with additional Vue.js component for frontend integration.

## Architecture

The codebase follows a modular architecture:

- **Core Library** (`src/lib.rs`): Contains the main `PosterGenerator` struct and all rendering logic using Skia Safe
- **CLI Binary** (`src/main.rs`): Command-line interface for generating posters from JSON config files
- **API Server** (`src/bin/server.rs`): HTTP API server using Axum framework
- **Example Binary** (`src/bin/example.rs`): Demonstrates programmatic usage with Chinese and Arabic text examples
- **Vue Component** (`Poster.vue`): Frontend component for canvas-based poster generation (likely for mobile/web)

### Key Components

1. **Element System**: Three element types with z-index layering:
   - `BackgroundElement`: Canvas background with optional image and rounded corners
   - `ImageElement`: Images with positioning, scaling (cover/contain/stretch), and rounded corners
   - `TextElement`: Text with multi-line support, RTL/LTR direction, custom backgrounds, and styling

2. **Configuration**: JSON-based configuration system with serde serialization
3. **Rendering Pipeline**: Skia Safe-based rendering with proper z-index sorting and clipping for rounded corners

## Common Commands

### Build and Development
```bash
# Build the project
cargo build --release

# Run CLI poster generator (default binary due to default-run setting)
cargo run --release -- -c example_config.json -o output_poster.png

# Or explicitly specify the binary
cargo run --release --bin poster_generator -- -c example_config.json -o output_poster.png

# Generate base64 output to stdout (output path ignored when --base64 is used)
cargo run --release -- -c example_config.json -o output.png --base64

# Run the example (creates example_output.png)
cargo run --release --bin example

# Start API server (default port 3000)
cargo run --release --bin server

# Start API server on custom port
cargo run --release --bin server -- -p 8080
```

### Testing
```bash
# Run tests (no tests currently defined in the project)
cargo test

# Check code compilation
cargo check
```

## Key Features & Implementation Details

### Text Rendering
- **RTL Support**: Uses Skia Safe's `TextDirection` for proper Arabic/Hebrew text rendering
- **Multi-line Text**: Custom text wrapping with configurable line height and max lines
- **Text Backgrounds**: Optional colored backgrounds with padding and border radius
- **Font Styling**: Support for bold text and custom font sizes

### Image Processing
- **Object Fit Modes**: Cover (crop to fill), Contain (fit within bounds), Stretch (distort to fill)
- **Rounded Corners**: Uses Skia Safe paths with clipping for smooth rounded corners
- **Base64 Support**: Can load images from base64 data URLs
- **Z-Index Layering**: Proper element stacking with configurable z-index values

### Configuration System
- **JSON-based**: Uses serde for serialization/deserialization
- **Flexible Radius**: Single value or array of 4 values for different corner radii
- **Color Parsing**: Supports hex colors (#RRGGBB and #RRGGBBAA formats)
- **Element Composition**: Background, image, and text elements can be freely combined

## API Usage

### HTTP Endpoints
- `POST /generate`: Generate poster from JSON configuration
  - Request body: `{"config": PosterConfig, "format": "base64"|"file"}`
  - Response: `{"success": bool, "data": string|null, "error": string|null}`

### Error Handling
The project uses custom `PosterError` enum with three categories:
- `ImageLoadError`: Issues loading/decoding images
- `RenderError`: Problems during Skia Safe rendering
- `OutputError`: Failures in PNG encoding or file operations

## Development Notes

### Dependencies
- **skia-safe**: Primary rendering engine (same as Chrome/Android) - requires system dependencies
- **axum**: Web framework for API server (v0.6.20 with legacy Server API)
- **image**: Image processing utilities
- **serde/serde_json**: JSON serialization
- **clap**: CLI argument parsing
- **tokio**: Async runtime for server
- **anyhow/thiserror**: Error handling
- **base64**: Base64 encoding/decoding
- **chrono**: Date/time utilities for temporary file naming

### Vue.js Integration
The included `Poster.vue` component provides similar functionality using HTML5 Canvas API, designed for uni-app/mobile environments. Key differences from Rust version:
- Uses HTML5 Canvas instead of Skia Safe
- Designed for mobile/uni-app framework
- Supports base64 image handling for mobile environments
- Same element types and configuration structure

### Text Direction Handling
RTL text requires:
1. Set `direction: "rtl"` in text element config
2. Usually pair with `align: "right"` for proper display
3. The system automatically handles text direction in Skia Safe rendering

### Image Loading
The Rust implementation supports:
- Local file paths
- Base64 data URLs (with `data:image/` prefix)

The Vue.js component additionally supports:
- Base64 conversion and temporary file handling for mobile environments
- Network image loading through uni-app APIs