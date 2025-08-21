# Qwen Code Project Context

## Project Overview

This is a Rust crate called "Renderer" that provides SVG graphics rendering capabilities with a simple API. It supports multiple shape types (rectangle, circle, line, text, ellipse, polygon) and includes optional logging functionality.

## Key Features

- SVG rendering backend
- Multiple shape support (rectangle, circle, line, text, ellipse, polygon)
- Configurable colors and strokes
- Optional logging functionality
- Builder pattern for easy shape construction

## Project Structure

```
renderer/
├── Cargo.toml          # Rust project configuration and dependencies
├── README.md           # Project documentation and usage examples
├── src/
│   ├── lib.rs          # Main library entry point
│   ├── backend.rs      # SVG backend implementation
│   ├── color.rs        # Color definitions and handling
│   ├── image.rs        # Image representation and rendering interface
│   ├── shape.rs        # Shape definitions (rectangle, circle, etc.)
│   ├── stroke.rs       # Stroke definitions
│   └── logger.rs       # Optional logging functionality
├── examples/
│   ├── svg.rs          # Example demonstrating SVG rendering
│   └── logger_demo.rs  # Example demonstrating logging functionality
└── scripts/
    └── pre-commit      # Git pre-commit hook for code quality checks
```

## Main Components

### 1. Core Rendering System

The renderer provides a simple API for creating SVG graphics:

- `Image` - Represents an image that can contain multiple shapes
- `Shape` enum - Contains all supported shape types (Rectangle, Circle, Line, Text, Ellipse, Polygon)
- `Color` - Color definitions with support for RGB and RGBA values
- `Stroke` - Stroke properties (size, color) for shapes

### 2. SVG Backend

The `SVGBackend` is responsible for rendering the image to an SVG file:

- Implements the `Backend` trait from the `image` module
- Generates valid SVG XML with proper formatting
- Supports text escaping to handle special characters in text content

### 3. Logging Module (Optional)

The project includes an optional logging module that:

- Provides colored console output for log messages
- Can optionally show file and line number information for each log message
- Integrates with the standard `log` crate

## Usage Examples

### Basic SVG Rendering

```rust
use renderer::{
    backend::SVGBackend,
    color::Color,
    image::Image,
    shape::{self, Shape},
    stroke,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut image = Image::new().with_width(800.0).with_height(600.0);

    image.add(Shape::Rectangle(
        shape::Rectangle::new()
            .with_pos(20.0, 40.0)
            .with_size(50.0, 50.0)
            .with_fill_color(Color::RED)
            .with_stroke(
                stroke::Stroke::new()
                    .with_size(1.0)
                    .with_color(Color::rgb8(0x20, 0x20, 0x20)),
            ),
    ));

    let mut svg = SVGBackend::init();
    let path = std::path::Path::new("output.svg");
    image.save(path, &mut svg)?;

    Ok(())
}
```

### Logger Usage

```rust
use log::{debug, error, info, trace, warn};
use renderer::{Level, init_logger};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger with location display
    init_logger(Level::Trace, true)?;

    info!("This is an info message with location");
    warn!("This is a warning message with location");
    
    Ok(())
}
```

## Building and Running

### Prerequisites
- Rust (1.70 or later) - https://rustup.rs/

### Building

```bash
# Build the project
cargo build

# Build with specific features (e.g., only SVG support)
cargo build --features svg
```

### Running Examples

```bash
# Run the SVG example
cargo run --example svg

# Run the logger example
cargo run --example logger_demo
```

### Development Checks

```bash
# Format code
cargo fmt

# Check for linting issues
cargo clippy -- -D warnings

# Run tests
cargo test

# Run all checks (formatting, linting, tests)
cargo fmt && cargo clippy -- -D warnings && cargo test
```

## Development Conventions

### Git Hooks

This project uses Git hooks to ensure code quality:

1. `cargo fmt --check` - Checks code formatting
2. `cargo clippy -- -D warnings` - Checks for linting issues
3. `cargo test` - Runs unit tests

### Feature Flags

The project uses feature flags to control which components are included:

- `svg` - Enables SVG rendering backend
- `logger` - Enables logging functionality
- `default` - Includes both SVG and logger features

### Testing

The project includes unit tests for:
- SVG text escaping
- Shape builders (rectangle, circle, ellipse, polygon)
- Backend rendering functionality

## License

MIT