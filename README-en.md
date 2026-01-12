# EasyX-RS

Rust bindings for the EasyX graphics library, providing simple and easy-to-use 2D graphics drawing APIs for Windows platform graphics application development, game development, and visual programming.

## Project Overview

EasyX-RS is a Rust binding based on the C++ EasyX graphics library, providing high-performance, cross-version compatible 2D graphics drawing capabilities. EasyX is a simple and easy-to-use graphics library designed specifically for Windows platforms, widely used in education, game development, and visual programming fields.

### Core Features

- Simple and easy-to-use API design
- High-performance graphics drawing
- Support for multiple basic graphics elements
- Support for text rendering
- Support for mouse and keyboard events
- Support for image loading and display
- Support for batch drawing optimization
- Support for multiple color models

## Quick Start

### Installation

Add easyx-rs dependency to your Rust project:

```toml
[dependencies]
easysql-rs = "0.1.0"
```

### First Example

```rust
use easyx::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize graphics window with size 800x600
    run(800, 600, |app| {
        // Set text style
        app.set_textstyle(30, 0, "Microsoft YaHei");
        
        // Set text color to red
        app.set_textcolor(&Color::RED);
        
        // Output text
        app.out_text(100, 100, "Hello, EasyX-RS!");
        
        // Draw line
        app.set_linecolor(&Color::BLUE);
        app.line(100, 200, 700, 200);
        
        // Draw rectangle
        app.set_linecolor(&Color::GREEN);
        app.rectangle(100, 300, 300, 400);
        
        // Draw filled circle
        app.set_fillcolor(&Color::YELLOW);
        app.fill_circle(500, 350, 50);
        
        // Wait for user key press
        loop {
            if let Some(msg) = app.peek_message(MessageFilter::KeyBoard, true) {
                if let Message::KeyBoard { vkcode, .. } = msg.msg {
                    if vkcode == KeyCode::Escape {
                        break;
                    }
                }
            }
        }
        
        Ok(())
    })
}
```

## Core Features

### 1. Application Management

- Window creation and initialization
- Run mode control
- Window property retrieval
- Version information query

### 2. Graphics Drawing

#### Basic Graphics
- Point drawing (`put_pixel`)
- Line drawing (`line`)
- Rectangle drawing (`rectangle`, `fill_rectangle`)
- Circle drawing (`circle`, `fill_circle`)
- Ellipse drawing (`ellipse`, `fill_ellipse`)
- Rounded rectangle (`roundrect`, `fill_roundrect`)

#### Advanced Graphics
- Arc and sector (`arc`, `pie`)
- Polygon (`polygon`, `fill_polygon`)
- Bezier curve (`poly_bezier`)
- Area filling (`flood_fill`)

### 3. Color Processing

- RGB color creation (`Color::new(r, g, b)`)
- HSL/HSV color conversion
- Common color constants (`Color::RED`, `Color::BLUE`, etc.)
- Color component access

### 4. Text Processing

- Text output (`out_text`)
- Text style setting (`set_textstyle`)
- Text size retrieval (`text_width`, `text_height`)
- Text alignment

### 5. Event Handling

- Mouse event listening
- Keyboard event listening
- Message filtering mechanism
- Non-blocking message retrieval

### 6. Performance Optimization

- Batch drawing (`begin_batch_draw`, `end_batch_draw`)
- Partial refresh (`flush_batch_draw_rect`)

## Project Structure

```
easysql-rs/
├── easyx-example/        # Example code
├── easyx-rs/             # Main Rust binding library
│   ├── src/
│   │   ├── app.rs        # Application management
│   │   ├── color.rs      # Color processing
│   │   ├── enums.rs      # Enumeration definitions
│   │   ├── fillstyle.rs  # Fill style
│   │   ├── image.rs      # Image processing
│   │   ├── input.rs      # Input processing
│   │   ├── keycode.rs    # Keyboard code definitions
│   │   ├── linestyle.rs  # Line style
│   │   ├── logfont.rs    # Font setting
│   │   └── msg.rs        # Message processing
│   └── lib.rs            # Library entry
└── easyx-sys/            # Raw C bindings
```

## Usage Guide

### Initializing Window

```rust
// Basic initialization
run(800, 600, |app| {
    // Your drawing code
    Ok(())
})

// Using initialization flags
run_flags(800, 600, InitFlags::ShowConsole, |app| {
    // Your drawing code
    Ok(())
})
```

### Drawing Examples

```rust
// Draw line
app.line(100, 100, 300, 300);

// Draw rectangle
app.rectangle(100, 100, 300, 200);

// Draw filled rectangle
app.set_fillcolor(&Color::BLUE);
app.fill_rectangle(100, 100, 300, 200);

// Draw circle
app.circle(200, 200, 50);

// Draw filled circle
app.set_fillcolor(&Color::RED);
app.fill_circle(200, 200, 50);
```

### Text Rendering

```rust
// Set text style
app.set_textstyle(24, 0, "Arial");

// Set text color
app.set_textcolor(&Color::BLACK);

// Output text
app.out_text(100, 100, "Hello, EasyX-RS!");
```

### Event Handling

```rust
loop {
    // Non-blocking get mouse message
    if let Some(msg) = app.peek_message(MessageFilter::Mouse, true) {
        match msg.msg {
            Message::Mouse { x, y, button, .. } => {
                // Handle mouse event
                println!("Mouse: x={}, y={}, button={:?}", x, y, button);
            }
            _ => {}
        }
    }
    
    // Non-blocking get keyboard message
    if let Some(msg) = app.peek_message(MessageFilter::KeyBoard, true) {
        match msg.msg {
            Message::KeyBoard { vkcode, .. } => {
                // Handle keyboard event
                if vkcode == KeyCode::Escape {
                    break;
                }
            }
            _ => {}
        }
    }
}
```

### Batch Drawing

```rust
// Start batch drawing
app.begin_batch_draw();

// Draw a large number of graphics
for i in 0..1000 {
    app.put_pixel(i % 800, i % 600, &Color::new(
        (i * 3) as u8,
        (i * 5) as u8,
        (i * 7) as u8
    ));
}

// Flush batch drawing
app.flush_batch_draw();

// End batch drawing
app.end_batch_draw();
```

## Best Practices

1. **Use batch drawing**: For a large number of graphics drawing, using batch drawing can significantly improve performance
2. **Set drawing styles reasonably**: Set line styles, fill styles, and colors before drawing
3. **Use event-driven programming**: Adopt non-blocking message retrieval to improve program responsiveness
4. **Resource management**: Ensure proper release of graphics resources
5. **Error handling**: Reasonably handle possible error situations

## Example Code

More example code can be found in the `easyx-example` directory:

- Basic drawing examples
- Mouse event examples
- Keyboard event examples
- Animation effect examples
- Text rendering examples

## Dependencies

- Windows operating system
- Rust 1.60+ compilation environment
- Visual Studio or MinGW compilation toolchain

## License

This project adopts the MIT license. For details, please see the LICENSE file.

## Contribution

Welcome to submit Issues and Pull Requests!

## Acknowledgments

Thanks to the EasyX team for providing this excellent graphics library!

## Changelog

### v0.1.0
- Initial version release
- Implemented basic graphics drawing functionality
- Support for text rendering
- Support for mouse and keyboard events
- Support for batch drawing

---

**EasyX-RS** - Making Rust graphics programming simple!
