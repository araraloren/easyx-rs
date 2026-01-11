# 颜色处理 (Color)

Color模块提供了强大的颜色处理功能，支持多种颜色创建方式、颜色转换和颜色操作。EasyX-RS使用RGB颜色模型作为主要的颜色表示方式，同时支持HSL和HSV颜色模型。

## 颜色创建

### RGB颜色创建

使用`Color::new`方法可以创建RGB颜色：

```rust
use easyx::prelude::*;

// 创建红色 (R: 255, G: 0, B: 0)
let red = Color::new(255, 0, 0);

// 创建绿色 (R: 0, G: 255, B: 0)
let green = Color::new(0, 255, 0);

// 创建蓝色 (R: 0, G: 0, B: 255)
let blue = Color::new(0, 0, 255);

// 创建白色 (R: 255, G: 255, B: 255)
let white = Color::new(255, 255, 255);

// 创建黑色 (R: 0, G: 0, B: 0)
let black = Color::new(0, 0, 0);

// 创建灰色 (R: 128, G: 128, B: 128)
let gray = Color::new(128, 128, 128);
```

### HSL颜色创建

使用`Color::from_hsl`方法可以从HSL颜色模型创建颜色：

```rust
// 创建红色 (H: 0, S: 1.0, L: 0.5)
let red = Color::from_hsl(0.0, 1.0, 0.5);

// 创建绿色 (H: 120, S: 1.0, L: 0.5)
let green = Color::from_hsl(120.0, 1.0, 0.5);

// 创建蓝色 (H: 240, S: 1.0, L: 0.5)
let blue = Color::from_hsl(240.0, 1.0, 0.5);

// 创建白色 (H: 0, S: 0, L: 1.0)
let white = Color::from_hsl(0.0, 0.0, 1.0);

// 创建黑色 (H: 0, S: 0, L: 0.0)
let black = Color::from_hsl(0.0, 0.0, 0.0);
```

### HSV颜色创建

使用`Color::from_hsv`方法可以从HSV颜色模型创建颜色：

```rust
// 创建红色 (H: 0, S: 1.0, V: 1.0)
let red = Color::from_hsv(0.0, 1.0, 1.0);

// 创建绿色 (H: 120, S: 1.0, V: 1.0)
let green = Color::from_hsv(120.0, 1.0, 1.0);

// 创建蓝色 (H: 240, S: 1.0, V: 1.0)
let blue = Color::from_hsv(240.0, 1.0, 1.0);

// 创建白色 (H: 0, S: 0, V: 1.0)
let white = Color::from_hsv(0.0, 0.0, 1.0);

// 创建黑色 (H: 0, S: 0, V: 0.0)
let black = Color::from_hsv(0.0, 0.0, 0.0);
```

## 常用颜色常量

EasyX-RS提供了一系列常用颜色常量，方便直接使用：

| 颜色常量 | RGB值 | 描述 |
|----------|-------|------|
| `Color::BLACK` | (0, 0, 0) | 黑色 |
| `Color::BLUE` | (0, 0, 255) | 蓝色 |
| `Color::GREEN` | (0, 255, 0) | 绿色 |
| `Color::CYAN` | (0, 255, 255) | 青色 |
| `Color::RED` | (255, 0, 0) | 红色 |
| `Color::MAGENTA` | (255, 0, 255) | 品红色 |
| `Color::BROWN` | (128, 0, 0) | 棕色 |
| `Color::LIGHTGRAY` | (192, 192, 192) | 浅灰色 |
| `Color::DARKGRAY` | (128, 128, 128) | 深灰色 |
| `Color::LIGHTBLUE` | (0, 128, 255) | 浅蓝色 |
| `Color::LIGHTGREEN` | (0, 255, 128) | 浅绿色 |
| `Color::LIGHTCYAN` | (0, 255, 255) | 浅青色 |
| `Color::LIGHTRED` | (255, 128, 128) | 浅红色 |
| `Color::LIGHTMAGENTA` | (255, 128, 255) | 浅品红色 |
| `Color::YELLOW` | (255, 255, 0) | 黄色 |
| `Color::WHITE` | (255, 255, 255) | 白色 |

## 颜色转换

### RGB转HSL

```rust
let color = Color::new(255, 0, 0);
let (h, s, l) = color.to_hsl();
println!("HSL: {}, {}, {}", h, s, l);
```

### RGB转HSV

```rust
let color = Color::new(255, 0, 0);
let (h, s, v) = color.to_hsv();
println!("HSV: {}, {}, {}", h, s, v);
```

### 转为灰度

```rust
let color = Color::new(255, 0, 0);
let gray = color.to_gray();
println!("灰度: {}, {}, {}", gray.r(), gray.g(), gray.b());
```

## 颜色分量访问

可以直接访问颜色的RGB分量：

```rust
let color = Color::new(255, 128, 64);

println!("红色分量: {}", color.r()); // 输出: 255
println!("绿色分量: {}", color.g()); // 输出: 128
println!("蓝色分量: {}", color.b()); // 输出: 64
```

## 颜色操作

Color类型支持按位与、或、异或操作：

```rust
let color1 = Color::new(255, 0, 0);
let color2 = Color::new(0, 255, 0);

// 按位与操作
let and_color = color1 & color2; // 结果: (0, 0, 0)

// 按位或操作
let or_color = color1 | color2;  // 结果: (255, 255, 0)

// 按位异或操作
let xor_color = color1 ^ color2; // 结果: (255, 255, 0)
```

## 颜色使用示例

```rust
use easyx::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run(800, 600, |app| {
        // 设置文本样式
        app.set_textstyle(20, 0, "微软雅黑");
        
        // 使用颜色常量绘制矩形
        app.set_fillcolor(&Color::RED);
        app.fill_rectangle(100, 100, 200, 200);
        
        app.set_fillcolor(&Color::GREEN);
        app.fill_rectangle(250, 100, 350, 200);
        
        app.set_fillcolor(&Color::BLUE);
        app.fill_rectangle(400, 100, 500, 200);
        
        // 使用RGB创建颜色
        let purple = Color::new(128, 0, 128);
        app.set_fillcolor(&purple);
        app.fill_rectangle(550, 100, 650, 200);
        
        // 使用HSL创建颜色
        let orange = Color::from_hsl(30.0, 1.0, 0.5);
        app.set_fillcolor(&orange);
        app.fill_rectangle(100, 250, 200, 350);
        
        // 使用HSV创建颜色
        let pink = Color::from_hsv(330.0, 1.0, 1.0);
        app.set_fillcolor(&pink);
        app.fill_rectangle(250, 250, 350, 350);
        
        // 颜色转换示例
        let color = Color::new(255, 128, 64);
        let gray = color.to_gray();
        app.set_fillcolor(&gray);
        app.fill_rectangle(400, 250, 500, 350);
        
        // 输出颜色信息
        let (h, s, l) = color.to_hsl();
        let (hsv_h, hsv_s, hsv_v) = color.to_hsv();
        
        app.set_textcolor(&Color::WHITE);
        app.out_text(100, 400, format!("RGB: {}, {}, {}", color.r(), color.g(), color.b()).as_str());
        app.out_text(100, 430, format!("HSL: {:.1}, {:.2}, {:.2}", h, s, l).as_str());
        app.out_text(100, 460, format!("HSV: {:.1}, {:.2}, {:.2}", hsv_h, hsv_s, hsv_v).as_str());
        
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

## 颜色管理

### 设置绘图颜色

```rust
// 设置线条颜色
app.set_linecolor(&Color::RED);

// 设置填充颜色
app.set_fillcolor(&Color::BLUE);

// 设置文本颜色
app.set_textcolor(&Color::GREEN);

// 设置背景颜色
app.set_bkcolor(&Color::WHITE);
```

### 获取当前颜色

```rust
// 获取当前线条颜色
let line_color = Color::get_linecolor();

// 获取当前填充颜色
let fill_color = Color::get_fillcolor();

// 获取当前文本颜色
let text_color = Color::get_textcolor();

// 获取当前背景颜色
let bk_color = Color::get_bkcolor();
```

### 像素操作

```rust
// 获取指定位置的像素颜色
let pixel_color = Color::get_pixel(x, y);

// 设置指定位置的像素颜色
let color = Color::new(255, 0, 0);
color.put_pixel(x, y);
```

## 最佳实践

1. **使用颜色常量**：对于常用颜色，建议使用预定义的颜色常量，提高代码可读性和一致性。
2. **合理使用颜色模型**：根据需求选择合适的颜色模型，RGB适合直接颜色控制，HSL适合调整亮度和饱和度，HSV适合调整色调。
3. **考虑颜色对比度**：确保文本和背景颜色有足够的对比度，提高可读性。
4. **使用颜色渐变**：可以通过颜色插值创建平滑的颜色渐变效果。
5. **注意性能**：频繁的颜色转换可能会影响性能，对于大量图形绘制，建议预先计算好颜色。

## 常见问题

### Q: 如何创建随机颜色？
A: 可以使用rand库生成随机的RGB分量：

```rust
use rand::Rng;

let mut rng = rand::thread_rng();
let r = rng.gen_range(0..=255);
let g = rng.gen_range(0..=255);
let b = rng.gen_range(0..=255);
let random_color = Color::new(r, g, b);
```

### Q: 如何创建颜色渐变？
A: 可以通过线性插值计算两个颜色之间的中间色：

```rust
fn lerp_color(color1: Color, color2: Color, t: f32) -> Color {
    let r = ((color1.r() as f32) * (1.0 - t) + (color2.r() as f32) * t) as u8;
    let g = ((color1.g() as f32) * (1.0 - t) + (color2.g() as f32) * t) as u8;
    let b = ((color1.b() as f32) * (1.0 - t) + (color2.b() as f32) * t) as u8;
    Color::new(r, g, b)
}
```

### Q: 如何将颜色转换为十六进制字符串？
A: 可以使用格式化字符串：

```rust
let color = Color::new(255, 128, 64);
let hex = format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b());
// 结果: "#FF8040"
```

## 总结

Color模块是EasyX-RS的重要组成部分，提供了丰富的颜色处理功能。通过Color模块，你可以轻松创建和管理各种颜色，支持多种颜色模型转换，以及进行颜色操作。合理使用Color模块可以让你的图形应用更加生动和美观。