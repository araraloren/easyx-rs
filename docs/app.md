# 应用程序管理 (App)

App模块是EasyX-RS的核心，负责图形窗口的创建、初始化和管理。通过App模块，你可以控制图形应用的生命周期、窗口属性和运行模式。

## 初始化窗口

EasyX-RS提供了两种方式来初始化图形窗口：

### 基本初始化

使用`run`函数初始化窗口，这是最简单的方式：

```rust
use easyx_rs::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化800x600大小的图形窗口
    run(800, 600, |app| {
        // 你的绘图代码
        Ok(())
    })
}
```

### 带初始化标志的初始化

使用`run_flags`函数可以设置窗口的初始化标志：

```rust
use easyx_rs::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化窗口并显示控制台
    run_flags(800, 600, InitFlags::ShowConsole, |app| {
        // 你的绘图代码
        Ok(())
    })
}
```

## 初始化标志

InitFlags是一个位标志枚举，用于设置窗口的初始化选项：

| 标志 | 描述 |
|------|------|
| `None` | 无特殊标志 |
| `ShowConsole` | 显示控制台窗口 |
| `NoClose` | 禁用关闭按钮 |
| `NoMinimize` | 禁用最小化按钮 |
| `DblClks` | 启用双击事件 |

多个标志可以组合使用：

```rust
run_flags(800, 600, InitFlags::ShowConsole | InitFlags::NoClose, |app| {
    // 你的绘图代码
    Ok(())
})
```

## App结构体

App结构体提供了丰富的方法来管理图形窗口和绘图上下文：

### 窗口属性

| 方法 | 描述 |
|------|------|
| `width()` | 获取窗口宽度 |
| `height()` | 获取窗口高度 |
| `hwnd()` | 获取窗口句柄 |
| `graphics_hwnd()` | 获取图形窗口句柄 |
| `version()` | 获取EasyX库版本 |

### 坐标变换

| 方法 | 描述 |
|------|------|
| `set_origin(x, y)` | 设置坐标原点 |
| `set_aspectratio(width, height)` | 设置宽高比 |
| `get_aspectratio()` | 获取宽高比 |

### 剪贴区域

| 方法 | 描述 |
|------|------|
| `set_cliprgn(x, y, width, height)` | 设置剪贴区域 |
| `reset_cliprgn()` | 重置剪贴区域 |

### 批处理绘图

| 方法 | 描述 |
|------|------|
| `begin_batch_draw()` | 开始批处理绘图 |
| `flush_batch_draw()` | 刷新批处理绘图 |
| `flush_batch_draw_rect(left, top, right, bottom)` | 刷新指定区域的批处理绘图 |
| `end_batch_draw()` | 结束批处理绘图 |

### 绘图状态重置

| 方法 | 描述 |
|------|------|
| `reset_graph_defaults()` | 重置绘图默认值 |
| `clear_device()` | 清除整个设备屏幕 |

### 事件处理

| 方法 | 描述 |
|------|------|
| `get_message(filter)` | 获取消息（阻塞） |
| `peek_message(filter, removemsg)` | 查看消息（非阻塞） |
| `flush_messages(filter)` | 刷新消息队列 |
| `set_capture()` | 设置鼠标捕获 |
| `release_capture()` | 释放鼠标捕获 |

### 输入框

| 方法 | 描述 |
|------|------|
| `input_box(max, prompt)` | 创建输入框 |

## 窗口创建示例

```rust
use easyx_rs::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化窗口，大小为800x600，显示控制台
    run_flags(800, 600, InitFlags::ShowConsole, |app| {
        // 获取窗口属性
        println!("窗口尺寸: {}x{}", app.width(), app.height());
        println!("EasyX版本: {}", app.version());
        
        // 设置坐标原点到窗口中心
        app.set_origin(app.width() / 2, app.height() / 2);
        
        // 绘制一个十字线
        app.set_linecolor(&Color::GRAY);
        app.line(-app.width() / 2, 0, app.width() / 2, 0);
        app.line(0, -app.height() / 2, 0, app.height() / 2);
        
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

## 生命周期管理

App实例在创建时会自动初始化图形窗口，在实例被销毁时会自动关闭图形窗口。使用`run`或`run_flags`函数可以确保App实例的正确创建和销毁。

## 最佳实践

1. **使用run或run_flags函数**：这些函数会自动处理窗口的初始化和关闭，确保资源正确释放。
2. **避免在多个线程中使用同一个App实例**：EasyX-RS不是线程安全的，同一时间只能在一个线程中使用App实例。
3. **合理设置初始化标志**：根据应用需求选择合适的初始化标志，例如在调试时显示控制台。
4. **使用批处理绘图**：对于大量图形绘制，使用批处理可以显著提高性能。

## 常见问题

### Q: 为什么窗口无法显示？
A: 可能是因为你没有使用`run`或`run_flags`函数来初始化窗口，或者窗口被其他窗口遮挡。

### Q: 如何获取窗口句柄？
A: 使用`app.hwnd()`方法可以获取窗口句柄，`app.graphics_hwnd()`方法可以获取图形窗口句柄。

### Q: 如何设置窗口标题？
A: 目前EasyX-RS不直接支持设置窗口标题，你可以通过获取窗口句柄，然后使用Windows API来设置标题。

### Q: 如何处理窗口关闭事件？
A: 你可以监听键盘事件，当用户按下ESC键时退出应用，或者使用Windows API来处理窗口关闭事件。

## 总结

App模块是EasyX-RS的核心，负责图形窗口的创建、初始化和管理。通过App模块，你可以控制图形应用的生命周期、窗口属性和运行模式。使用`run`或`run_flags`函数可以方便地初始化窗口，然后使用App实例的各种方法来进行绘图和事件处理。