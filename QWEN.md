# Qwen Code Project Context

## Project Overview

This is a Rust crate called "Renderer" that provides multiple graphics rendering capabilities with a simple API. It supports SVG, PPM, and PNG formats, includes shape rendering (rectangle, circle, line, text, ellipse, polygon), and has optional logging functionality.

## Key Features

- Multiple rendering backends (SVG, PPM, PNG)
- Multiple shape support (rectangle, circle, line, text, ellipse, polygon)
- Configurable colors and strokes
- Optional logging functionality
- Font rendering capabilities
- Builder pattern for easy shape construction

## Project Structure

```
renderer/
├── Cargo.toml          # Rust project configuration and dependencies
├── README.md           # Project documentation and usage examples
├── AGENT.md            # Guidelines for AI agents working on this codebase
├── src/
│   ├── lib.rs          # Main library entry point
│   ├── color.rs        # Color definitions and handling
│   ├── image.rs        # Image representation and rendering interface
│   ├── stroke.rs       # Stroke definitions
│   ├── logger.rs       # Optional logging functionality
│   ├── backend/        # Backend implementations
│   │   ├── mod.rs      # Backend module definitions
│   │   ├── svg.rs      # SVG backend implementation
│   │   ├── ppm.rs      # PPM backend implementation
│   │   ├── ascii_ppm.rs# ASCII PPM backend implementation
│   │   ├── png.rs      # PNG backend implementation
│   │   └── png_tests.rs# PNG backend tests
│   ├── shape/          # Shape definitions
│   │   ├── mod.rs      # Shape module definitions
│   │   ├── rect.rs     # Rectangle shape
│   │   ├── circle.rs   # Circle shape
│   │   ├── line.rs     # Line shape
│   │   ├── text.rs     # Text shape
│   │   ├── ellipse.rs  # Ellipse shape
│   │   ├── polygon.rs  # Polygon shape
│   │   ├── tests.rs    # Shape tests
│   ├── rasterizer/     # Rasterization components
│   │   ├── mod.rs      # Rasterizer module definitions
│   │   └── font.rs     # Font rendering
├── examples/
│   ├── svg.rs          # Example demonstrating SVG rendering
│   ├── ppm.rs          # Example demonstrating PPM rendering
│   ├── ascii_ppm.rs    # Example demonstrating ASCII PPM rendering
│   ├── png.rs          # Example demonstrating PNG rendering
│   ├── logger_demo.rs  # Example demonstrating logging functionality
│   ├── rasterizer.rs   # Example demonstrating rasterization
│   ├── font.rs         # Example demonstrating font rendering
│   └── ppm_with_font.rs# Example demonstrating PPM with font rendering
└── scripts/
    └── pre-commit      # Git pre-commit hook for code quality checks
```

## Main Components

### 1. Core Rendering System

The renderer provides a simple API for creating graphics in multiple formats:

- `Image` - Represents an image that can contain multiple shapes
- `Shape` enum - Contains all supported shape types (Rectangle, Circle, Line, Text, Ellipse, Polygon)
- `Color` - Color definitions with support for RGB and RGBA values
- `Stroke` - Stroke properties (size, color) for shapes

### 2. Backend System

The renderer supports multiple backends for different output formats:

- `SVGBackend` - Renders images to SVG format
- `PPMBackend` - Renders images to PPM format
- `PNGBackend` - Renders images to PNG format
- `ASCIIPPMBackend` - Renders images to ASCII PPM format

Each backend implements the `Backend` trait from the `image` module to provide a consistent interface for rendering.

### 3. Shape System

The project includes multiple shape types with builder patterns:

- `Rectangle` - Rectangular shapes with position, size, fill, and stroke
- `Circle` - Circular shapes with center position, radius, fill, and stroke
- `Line` - Line shapes with start/end points and stroke
- `Text` - Text elements with position, content, font, and fill
- `Ellipse` - Elliptical shapes with center position, radii, fill, and stroke
- `Polygon` - Polygonal shapes with a series of points, fill, and stroke

### 4. Rasterization System

The project includes rasterization capabilities for font rendering:

- `PixelImage` - Represents a raster image as a grid of pixels
- `Pixel` - Represents a single pixel with RGBA values
- Font rendering using the ab_glyph library

### 5. Logging Module (Optional)

The project includes an optional logging module that:

- Provides colored console output for log messages
- Can optionally show file and line number information for each log message
- Integrates with the standard `log` crate

## Agent Guidelines

This project includes specific guidelines for AI agents in `AGENT.md` that cover:

- **Identity & Scope**: Expectations for Rust expertise and edition preferences
- **Priorities**: Emphasis on correctness, minimal compiling diffs, performance, and maintainability
- **Style Requirements**: Code formatting with `cargo fmt`, documentation standards, error handling patterns
- **Unsafe & FFI**: Requirements for safety comments, abstraction preferences, and validation practices
- **Concurrency & Async**: Guidelines for choosing between threads/channels and async patterns
- **Testing & Benchmarking**: Requirements for unit tests, property-based testing, and performance validation
- **Examples**: Standards for runnable examples in the `examples/` directory
- **Validation**: Required commands for validation (`cargo check`, `cargo test`, etc.)
- **Libraries & Patterns**: Specific requirements for logging using only the `log` crate
- **Security & Safety**: Input validation, error handling, and memory safety practices

AI agents working on this codebase should follow these guidelines to ensure consistency with the project's standards and practices.

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

### Rasterizer with Font Rendering

```rust
use renderer::{
    color::Color,
    rasterizer::{Pixel, PixelImage},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut image = PixelImage::new(400, 300);
    
    // Render text with a font
    image.render_text(
        "Hello, Renderer!",
        50.0, 100.0,
        24.0,
        Color::BLACK,
    )?;
    
    // Save as PNG
    image.save("output.png")?;
    
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

# Build with all features
cargo build --features all

# Build with specific features (e.g., SVG and PNG support)
cargo build --features "svg png"
```

### Running Examples

```bash
# Run the SVG example
cargo run --example svg

# Run the PNG example
cargo run --example png

# Run the logger example
cargo run --example logger_demo

# Run the rasterizer example
cargo run --example rasterizer

# Run the font example
cargo run --example font
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
- `ppm` - Enables PPM rendering backend
- `png` - Enables PNG rendering backend
- `logger` - Enables logging functionality
- `rasterizer` - Enables rasterization functionality
- `all` - Enables all features
- `default` - No features enabled by default

### Testing

The project includes unit tests for:
- SVG text escaping
- Shape builders (rectangle, circle, ellipse, polygon)
- Backend rendering functionality
- PNG encoding/decoding
- Rasterization functionality

## License

MIT