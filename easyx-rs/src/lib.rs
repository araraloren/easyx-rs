#![cfg(windows)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! EasyX-RS
//!
//! Rust绑定的EasyX图形库，提供了简单易用的2D图形绘制API，适用于Windows平台的图形应用开发、游戏开发和可视化编程。
//!
//! ## 项目概述
//!
//! EasyX-RS是一个基于C++ EasyX图形库的Rust绑定，提供了高性能、跨版本兼容的2D图形绘制功能。
//! EasyX是一个专为Windows平台设计的简单易用的图形库，广泛应用于教学、游戏开发和可视化编程领域。
//!
//! ### 核心特性
//!
//! - 简单易用的API设计
//! - 高性能图形绘制
//! - 支持多种图形基本元素
//! - 支持文本渲染
//! - 支持鼠标和键盘事件
//! - 支持图像加载和显示
//! - 支持批处理绘图优化
//! - 支持多种颜色模型
//!
//! ## 快速开始
//!
//! ### 安装
//!
//! 在你的Rust项目中添加easyx-rs依赖：
//!
//! ```toml
//! [dependencies]
//! easyx-rs = "0.1.0"
//! ```
//!
//! ### 第一个示例
//!
//! ```no_run
//! use easyx::prelude::*;
//! use easyx::run;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 初始化图形窗口，大小为800x600
//!     run(800, 600, |app| {
//!         // 设置文本样式
//!         app.set_textstyle(30, 0, "微软雅黑");
//!         
//!         // 设置文本颜色为红色
//!         app.set_textcolor(&Color::RED);
//!         
//!         // 输出文本
//!         app.out_text(100, 100, "Hello, EasyX-RS!");
//!         
//!         // 绘制直线
//!         app.set_linecolor(&Color::BLUE);
//!         app.line(100, 200, 700, 200);
//!         
//!         // 绘制矩形
//!         app.set_linecolor(&Color::GREEN);
//!         app.rectangle(100, 300, 300, 400);
//!         
//!         // 绘制填充圆形
//!         app.set_fillcolor(&Color::YELLOW);
//!         app.fill_circle(500, 350, 50);
//!         
//!         // 等待用户按键
//!         loop {
//!             if let Some(msg) = app.peek_message(MessageFilter::KeyBoard, true) {
//!                 if let Message::KeyBoard { vkcode, .. } = msg.msg {
//!                     if vkcode == KeyCode::Escape {
//!                         break;
//!                     }
//!                 }
//!             }
//!         }
//!         
//!         Ok(())
//!     })
//! }
//! ```
//!
//! ## 核心功能
//!
//! ### 1. 应用程序管理 (App)
//! - 窗口创建和初始化
//! - 运行模式控制
//! - 窗口属性获取
//! - 版本信息查询
//!
//! ### 2. 图形绘制
//! - 基本图形 (点、线、矩形、圆形等)
//! - 填充图形
//! - 曲线绘制
//! - 多边形绘制
//!
//! ### 3. 颜色处理 (Color)
//! - RGB颜色创建
//! - HSL/HSV颜色转换
//! - 常用颜色常量
//! - 颜色分量访问
//!
//! ### 4. 文本处理
//! - 文本输出
//! - 文本样式设置
//! - 文本尺寸获取
//! - 文本对齐
//!
//! ### 5. 事件处理
//! - 鼠标事件监听
//! - 键盘事件监听
//! - 消息过滤机制
//! - 非阻塞消息获取
//!
//! ### 6. 性能优化
//! - 批处理绘图
//! - 局部刷新
//!
//! ## 使用指南
//!
//! ### 初始化窗口
//!
//! ```no_run
//! use easyx::*;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 基本初始化
//!     run(800, 600, |app| {
//!         // 你的绘图代码
//!         Ok(())
//!     })
//! }
//! ```
//!
//! ### 绘图示例
//!
//! ```no_run
//! use easyx::prelude::*;
//! use easyx::run;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     run(800, 600, |app| {
//!         // 绘制直线
//!         app.line(100, 100, 300, 300);
//!         
//!         // 绘制矩形
//!         app.rectangle(100, 100, 300, 200);
//!         
//!         // 绘制填充矩形
//!         app.set_fillcolor(&Color::BLUE);
//!         app.fill_rectangle(100, 100, 300, 200);
//!         
//!         // 绘制圆形
//!         app.circle(200, 200, 50);
//!         
//!         // 绘制填充圆形
//!         app.set_fillcolor(&Color::RED);
//!         app.fill_circle(200, 200, 50);
//!         
//!         Ok(())
//!     })
//! }
//! ```
//!
//! ### 事件处理
//!
//! ```no_run
//! use easyx::prelude::*;
//! use easyx::run;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     run(800, 600, |app| {
//!         loop {
//!             // 非阻塞获取鼠标消息
//!             if let Some(msg) = app.peek_message(MessageFilter::Mouse, true) {
//!                 match msg.msg {
//!                     Message::Mouse { x, y, lbutton, mbutton, rbutton, .. } => {
//!                         // 处理鼠标事件
//!                         println!("Mouse: x={}, y={}, lbutton={}, mbutton={}, rbutton={}", x, y, lbutton, mbutton, rbutton);
//!                     }
//!                     _ => {}
//!                 }
//!             }
//!             
//!             // 非阻塞获取键盘消息
//!             if let Some(msg) = app.peek_message(MessageFilter::KeyBoard, true) {
//!                 match msg.msg {
//!                     Message::KeyBoard { vkcode, .. } => {
//!                         // 处理键盘事件
//!                         if vkcode == KeyCode::Escape {
//!                             break;
//!                         }
//!                     }
//!                     _ => {}
//!                 }
//!             }
//!         }
//!         
//!         Ok(())
//!     })
//! }
//! ```
//!
//! ## 模块说明
//!
//! - **app**: 应用程序管理，负责窗口创建和初始化
//! - **color**: 颜色处理，支持多种颜色模型
//! - **enums**: 通用枚举定义
//! - **fillstyle**: 填充样式设置
//! - **image**: 图像处理，支持图像加载和显示
//! - **input**: 输入处理，支持输入框
//! - **keycode**: 键盘码定义
//! - **linestyle**: 线条样式设置
//! - **logfont**: 字体设置
//! - **msg**: 消息处理，支持事件监听
//!
//! ## 最佳实践
//!
//! 1. **使用run或run_flags函数**: 这些函数会自动处理窗口的初始化和关闭，确保资源正确释放。
//! 2. **使用批处理绘图**: 对于大量图形绘制，使用批处理可以显著提高性能。
//! 3. **合理设置绘图样式**: 在绘制前设置好线条样式、填充样式和颜色。
//! 4. **使用事件驱动编程**: 采用非阻塞消息获取方式，提高程序响应性。
//! 5. **资源管理**: 确保图形资源正确释放。

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

/// 预导入模块，包含常用的类型和函数
///
/// 通过导入此模块，可以方便地使用EasyX-RS的核心功能，无需逐个导入各个类型。
/// # 示例
/// ```no_run
/// use easyx::prelude::*;
/// use easyx::run;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     run(800, 600, |app| {
///         // 可以直接使用Color、LineStyle等类型
///         app.set_textcolor(&Color::RED);
///         Ok(())
///     })
/// }
/// ```
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

/// 使用初始化标志运行图形应用程序
///
/// 此函数创建一个新的图形窗口，使用指定的宽度、高度和初始化标志，
/// 然后执行提供的闭包。闭包执行完毕后，窗口会自动关闭。
///
/// # 参数
///
/// * `width` - 图形窗口的宽度
/// * `height` - 图形窗口的高度
/// * `flags` - 窗口初始化标志
/// * `f` - 要执行的闭包，接收App实例作为参数
///
/// # 返回值
///
/// 执行结果，成功返回Ok(())，失败返回Err
///
/// # 示例
///
/// ```no_run
/// use easyx::prelude::*;
/// use easyx::run_flags;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // 初始化窗口并显示控制台
///     run_flags(800, 600, InitFlags::ShowConsole, |app| {
///         // 你的绘图代码
///         app.out_text(100, 100, "Hello, EasyX-RS!");
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

/// 运行图形应用程序
///
/// 此函数创建一个新的图形窗口，使用指定的宽度和高度，
/// 然后执行提供的闭包。闭包执行完毕后，窗口会自动关闭。
/// 此函数使用默认的初始化标志（InitFlags::None）。
///
/// # 参数
///
/// * `width` - 图形窗口的宽度
/// * `height` - 图形窗口的高度
/// * `f` - 要执行的闭包，接收App实例作为参数
///
/// # 返回值
///
/// 执行结果，成功返回Ok(())，失败返回Err
///
/// # 示例
///
/// ```no_run
/// use easyx::prelude::*;
/// use easyx::run;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // 初始化800x600大小的图形窗口
///     run(800, 600, |app| {
///         // 你的绘图代码
///         app.out_text(100, 100, "Hello, EasyX-RS!");
///         Ok(())
///     })
/// }
/// ```
pub fn run<F>(width: i32, height: i32, f: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnOnce(&App) -> Result<(), Box<dyn std::error::Error>> + std::panic::UnwindSafe,
{
    run_flags(width, height, InitFlags::None, f)
}
