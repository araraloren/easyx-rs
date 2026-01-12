# EasyX-RS

Rust绑定的EasyX图形库，提供了简单易用的2D图形绘制API，适用于Windows平台的图形应用开发、游戏开发和可视化编程。

[English Version](README-en.md)

## 项目概述

EasyX-RS是一个基于C++ EasyX图形库的Rust绑定，提供了高性能、跨版本兼容的2D图形绘制功能。EasyX是一个专为Windows平台设计的简单易用的图形库，广泛应用于教学、游戏开发和可视化编程领域。

### 核心特性

- 简单易用的API设计
- 高性能图形绘制
- 支持多种图形基本元素
- 支持文本渲染
- 支持鼠标和键盘事件
- 支持图像加载和显示
- 支持批处理绘图优化
- 支持多种颜色模型

## 快速开始

### 安装

在你的Rust项目中添加easyx-rs依赖：

```toml
[dependencies]
easyx-rs = "0.1.0"
```

### 第一个示例

```rust
use easyx::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化图形窗口，大小为800x600
    run(800, 600, |app| {
        // 设置文本样式
        app.set_textstyle(30, 0, "微软雅黑");
        
        // 设置文本颜色为红色
        app.set_textcolor(&Color::RED);
        
        // 输出文本
        app.out_text(100, 100, "Hello, EasyX-RS!");
        
        // 绘制直线
        app.set_linecolor(&Color::BLUE);
        app.line(100, 200, 700, 200);
        
        // 绘制矩形
        app.set_linecolor(&Color::GREEN);
        app.rectangle(100, 300, 300, 400);
        
        // 绘制填充圆形
        app.set_fillcolor(&Color::YELLOW);
        app.fill_circle(500, 350, 50);
        
        // 等待用户按键
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

## 核心功能

### 1. 应用程序管理

- 窗口创建和初始化
- 运行模式控制
- 窗口属性获取
- 版本信息查询

### 2. 图形绘制

#### 基本图形
- 点绘制 (`put_pixel`)
- 直线绘制 (`line`)
- 矩形绘制 (`rectangle`, `fill_rectangle`)
- 圆形绘制 (`circle`, `fill_circle`)
- 椭圆绘制 (`ellipse`, `fill_ellipse`)
- 圆角矩形 (`roundrect`, `fill_roundrect`)

#### 高级图形
- 圆弧和扇形 (`arc`, `pie`)
- 多边形 (`polygon`, `fill_polygon`)
- 贝塞尔曲线 (`poly_bezier`)
- 区域填充 (`flood_fill`)

### 3. 颜色处理

- RGB颜色创建 (`Color::new(r, g, b)`)
- HSL/HSV颜色转换
- 常用颜色常量 (`Color::RED`, `Color::BLUE`, 等)
- 颜色分量访问

### 4. 文本处理

- 文本输出 (`out_text`)
- 文本样式设置 (`set_textstyle`)
- 文本尺寸获取 (`text_width`, `text_height`)
- 文本对齐方式

### 5. 事件处理

- 鼠标事件监听
- 键盘事件监听
- 消息过滤机制
- 非阻塞消息获取

### 6. 性能优化

- 批处理绘图 (`begin_batch_draw`, `end_batch_draw`)
- 局部刷新 (`flush_batch_draw_rect`)

## 项目结构

```
easyx-rs/
├── easyx-example/        # 示例代码
├── easyx-rs/             # 主要Rust绑定库
│   ├── src/
│   │   ├── app.rs        # 应用程序管理
│   │   ├── color.rs      # 颜色处理
│   │   ├── enums.rs      # 枚举定义
│   │   ├── fillstyle.rs  # 填充样式
│   │   ├── image.rs      # 图像处理
│   │   ├── input.rs      # 输入处理
│   │   ├── keycode.rs    # 键盘码定义
│   │   ├── linestyle.rs  # 线条样式
│   │   ├── logfont.rs    # 字体设置
│   │   └── msg.rs        # 消息处理
│   └── lib.rs            # 库入口
└── easyx-sys/            # 原始C绑定
```

## 使用指南

### 初始化窗口

```rust
// 基本初始化
run(800, 600, |app| {
    // 你的绘图代码
    Ok(())
})

// 使用初始化标志
run_flags(800, 600, InitFlags::ShowConsole, |app| {
    // 你的绘图代码
    Ok(())
})
```

### 绘图示例

```rust
// 绘制直线
app.line(100, 100, 300, 300);

// 绘制矩形
app.rectangle(100, 100, 300, 200);

// 绘制填充矩形
app.set_fillcolor(&Color::BLUE);
app.fill_rectangle(100, 100, 300, 200);

// 绘制圆形
app.circle(200, 200, 50);

// 绘制填充圆形
app.set_fillcolor(&Color::RED);
app.fill_circle(200, 200, 50);
```

### 文本渲染

```rust
// 设置文本样式
app.set_textstyle(24, 0, "Arial");

// 设置文本颜色
app.set_textcolor(&Color::BLACK);

// 输出文本
app.out_text(100, 100, "Hello, EasyX-RS!");
```

### 事件处理

```rust
loop {
    // 非阻塞获取鼠标消息
    if let Some(msg) = app.peek_message(MessageFilter::Mouse, true) {
        match msg.msg {
            Message::Mouse { x, y, button, .. } => {
                // 处理鼠标事件
                println!("Mouse: x={}, y={}, button={:?}", x, y, button);
            }
            _ => {}
        }
    }
    
    // 非阻塞获取键盘消息
    if let Some(msg) = app.peek_message(MessageFilter::KeyBoard, true) {
        match msg.msg {
            Message::KeyBoard { vkcode, .. } => {
                // 处理键盘事件
                if vkcode == KeyCode::Escape {
                    break;
                }
            }
            _ => {}
        }
    }
}
```

### 批处理绘图

```rust
// 开始批处理绘图
app.begin_batch_draw();

// 绘制大量图形
for i in 0..1000 {
    app.put_pixel(i % 800, i % 600, &Color::new(
        (i * 3) as u8,
        (i * 5) as u8,
        (i * 7) as u8
    ));
}

// 刷新批处理绘图
app.flush_batch_draw();

// 结束批处理绘图
app.end_batch_draw();
```

## 最佳实践

1. **使用批处理绘图**：对于大量图形绘制，使用批处理可以显著提高性能
2. **合理设置绘图样式**：在绘制前设置好线条样式、填充样式和颜色
3. **使用事件驱动编程**：采用非阻塞消息获取方式，提高程序响应性
4. **资源管理**：确保图形资源正确释放
5. **错误处理**：合理处理可能的错误情况

## 示例代码

更多示例代码请查看 `easyx-example` 目录：

- 基础绘图示例
- 鼠标事件示例
- 键盘事件示例
- 动画效果示例
- 文本渲染示例

## 依赖要求

- Windows 操作系统
- Rust 1.60+ 编译环境
- Visual Studio 或 MinGW 编译工具链

## 许可证

本项目采用 MIT 许可证，详情请查看 LICENSE 文件。

## 贡献

欢迎提交 Issue 和 Pull Request！

## 致谢

感谢 EasyX 团队提供的优秀图形库！

## 更新日志

### v0.1.0
- 初始版本发布
- 实现基本图形绘制功能
- 支持文本渲染
- 支持鼠标和键盘事件
- 支持批处理绘图

---

**EasyX-RS** - 让Rust图形编程变得简单！
