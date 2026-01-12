# easyx-sys

Raw C bindings for EasyX graphics library, providing low-level FFI support for Rust.

## Project Overview

easyx-sys is the underlying component of the EasyX-RS project, providing raw FFI (Foreign Function Interface) bindings to the EasyX C graphics library. It serves as an intermediate layer, connecting the upper-level Rust API with the lower-level C implementation, and providing basic support for the `easyx-rs` library.

## Core Features

- Auto-generated C to Rust FFI bindings
- Includes EasyX library headers and binary files
- Supports Windows platform
- Compatible with EasyX 26.1.1 version
- Provides basic graphics drawing, window management, event handling, and other low-level APIs

## Project Structure

```
easyx-sys/
├── build.rs           # Build script for generating bindings
├── Cargo.toml         # Project configuration
├── cpp/               # C++ auxiliary code
├── EasyX_26.1.1/      # Included EasyX library files
└── src/
    └── lib.rs         # Library entry point, exporting generated bindings
```

## Technical Implementation

1. **Binding Generation**: Uses `bindgen` tool to automatically generate Rust bindings from EasyX header files
2. **Build System**: Uses `cc` crate to compile C/C++ code
3. **Static Linking**: Includes EasyX library files to achieve static linking
4. **Safe Encapsulation**: Provides safe encapsulation of raw C APIs (implemented in the upper-level `easyx-rs` library)

## Usage as a Dependency

In most cases, you don't need to use `easyx-sys` directly. Instead, you should use the upper-level `easyx-rs` library. However, if you need direct access to the low-level C API, you can add a dependency in your project:

```toml
[dependencies]
easyx-sys = "0.1.3"
```

## Build Requirements

- Windows operating system
- Rust 1.60+ compilation environment
- Visual Studio or MinGW compilation toolchain

## Build Process

1. Clone the repository
2. Run `cargo build` command
3. The build script will automatically generate bindings and compile the code

## License

This project is licensed under the MIT License. For details, please see the LICENSE file in the project root directory.

## Related Projects

- [easyx-rs](https://github.com/araraloren/easyx-rs)：Upper-level Rust API library
- [easyx-example](https://github.com/araraloren/easyx-rs/tree/main/easyx-example)：Example code collection
- [tetris](https://github.com/araraloren/easyx-rs/tree/main/tetris)：Tetris example game

## Project Links

- Project Home: https://araraloren.github.io/easyx-rs/
- Documentation: https://araraloren.github.io/easyx-rs/
- Repository: https://github.com/araraloren/easyx-rs/
- Issue Tracker: https://github.com/araraloren/easyx-rs/issues

## Acknowledgments

Thanks to the EasyX team for providing this excellent graphics library!
