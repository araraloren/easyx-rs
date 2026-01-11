//! EasyX-RS
//!
//! A high-level Rust wrapper for the EasyX graphics library.

// Import the raw FFI bindings
pub use easyx_sys;

use crate::app::{App, InitFlags};

// High-level API implementation

// Module imports
pub mod app;
pub mod color;
pub mod enums;
pub mod fillstyle;
pub mod image;
pub mod input;
pub mod keycode;
pub mod linestyle;
pub mod logfont;
pub mod msg;

pub mod prelude {
    // Re-export the App struct from the app module
    pub use crate::app::*;
    // Re-export the LineStyle struct from the linestyle module
    pub use crate::linestyle::*;
    // Re-export the FillStyle struct from the fillstyle module
    pub use crate::fillstyle::*;
    // Re-export the Image struct from the image module
    pub use crate::image::*;
    // Re-export the Color struct from the color module
    pub use crate::color::*;
    // Re-export the Msg struct from the msg module
    pub use crate::msg::*;
    // Re-export the TextStyle struct from the textstyle module
    pub use crate::logfont::LogFont;
    // Re-export the InputBox related structs and functions
    pub use crate::input::*;
    // Re-export other types
    pub use crate::enums::*;
    // Re-export the KeyCode enum from the keycode module
    pub use crate::keycode::KeyCode;
}

/// Runs the graphics application with automatic initialization and cleanup,
/// using `InitFlags` for configuration.
///
/// This is a convenience function that creates an `App` instance and calls its
/// `run` method. It's the recommended way to use the library with the Rust enum flags.
///
/// # Parameters
///
/// * `width` - The width of the graphics window.
/// * `height` - The height of the graphics window.
/// * `flags` - The initialization flags for the graphics window.
/// * `f` - A closure that will be executed with the graphics context.
///
/// # Returns
///
/// A `Result` indicating success or failure.
///
/// # Example
///
/// ```no_run
/// use easyx_rs::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     run(800, 600, InitFlags::ShowConsole, |app| {
///         // Your graphics code here
///         // Use the app instance to draw
///         Ok(())
///     })
/// }
/// ```
pub fn run_flags<F>(
    width: i32,
    height: i32,
    flags: InitFlags,
    f: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnOnce(&App) -> Result<(), Box<dyn std::error::Error>> + std::panic::UnwindSafe,
{
    let app = App::new(width, height, flags);

    app.run(f)
}

pub fn run<F>(width: i32, height: i32, f: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnOnce(&App) -> Result<(), Box<dyn std::error::Error>> + std::panic::UnwindSafe,
{
    run_flags(width, height, InitFlags::None, f)
}
