# Renderer

A Rust crate for rendering SVG graphics with a simple API.

## Features

- SVG rendering backend
- Multiple shape support (rectangle, circle, line, text, ellipse, polygon)
- Configurable colors and strokes
- Optional logging functionality
- Builder pattern for easy shape construction

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
renderer = { path = "." }
```

### Basic Example

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

## Running Examples

```bash
# Run the SVG example
cargo run --example svg

# Run the logger example
cargo run --example logger_demo
```

## Development

### Git Hooks

This project uses Git hooks to ensure code quality. The pre-commit hook runs:

1. `cargo fmt --check` - Checks code formatting
2. `cargo clippy -- -D warnings` - Checks for linting issues
3. `cargo test` - Runs unit tests

### Running Checks Manually

```bash
# Format code
cargo fmt

# Check for linting issues
cargo clippy -- -D warnings

# Run tests
cargo test
```

## License

MIT