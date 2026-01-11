# 图形绘制

EasyX-RS提供了丰富的图形绘制功能，支持多种基本图形元素、填充样式和高级绘图技术。通过App实例的绘图方法，你可以轻松创建各种2D图形效果。

## 基本图形绘制

### 点绘制

使用`put_pixel`方法可以绘制单个像素点：

```rust
// 在坐标(100, 100)处绘制一个红色像素
app.set_linecolor(&Color::RED);
app.put_pixel(100, 100, &Color::RED);
```

### 直线绘制

使用`line`方法可以绘制直线：

```rust
// 绘制从(100, 100)到(300, 200)的蓝色直线
app.set_linecolor(&Color::BLUE);
app.line(100, 100, 300, 200);
```

### 矩形绘制

EasyX-RS提供了多种矩形绘制方法：

| 方法 | 描述 |
|------|------|
| `rectangle(left, top, right, bottom)` | 绘制矩形边框 |
| `fill_rectangle(left, top, right, bottom)` | 绘制填充矩形 |
| `solid_rectangle(left, top, right, bottom)` | 绘制实心矩形 |
| `clear_rectangle(left, top, right, bottom)` | 清除矩形区域 |

```rust
// 绘制矩形边框
app.set_linecolor(&Color::RED);
app.rectangle(100, 100, 300, 200);

// 绘制填充矩形
app.set_fillcolor(&Color::BLUE);
app.set_linecolor(&Color::BLACK);
app.fill_rectangle(100, 250, 300, 350);

// 绘制实心矩形
app.set_linecolor(&Color::GREEN);
app.solid_rectangle(100, 400, 300, 500);
```

### 圆形绘制

EasyX-RS提供了多种圆形绘制方法：

| 方法 | 描述 |
|------|------|
| `circle(x, y, radius)` | 绘制圆形边框 |
| `fill_circle(x, y, radius)` | 绘制填充圆形 |
| `solid_circle(x, y, radius)` | 绘制实心圆形 |
| `clear_circle(x, y, radius)` | 清除圆形区域 |

```rust
// 绘制圆形边框
app.set_linecolor(&Color::RED);
app.circle(200, 150, 50);

// 绘制填充圆形
app.set_fillcolor(&Color::BLUE);
app.set_linecolor(&Color::BLACK);
app.fill_circle(200, 300, 50);

// 绘制实心圆形
app.set_linecolor(&Color::GREEN);
app.solid_circle(200, 450, 50);
```

### 椭圆绘制

EasyX-RS提供了多种椭圆绘制方法：

| 方法 | 描述 |
|------|------|
| `ellipse(x, y, rx, ry)` | 绘制椭圆边框 |
| `fill_ellipse(left, top, right, bottom)` | 绘制填充椭圆 |
| `solid_ellipse(left, top, right, bottom)` | 绘制实心椭圆 |
| `clear_ellipse(left, top, right, bottom)` | 清除椭圆区域 |

```rust
// 绘制椭圆边框
app.set_linecolor(&Color::RED);
app.ellipse(200, 150, 100, 50);

// 绘制填充椭圆
app.set_fillcolor(&Color::BLUE);
app.set_linecolor(&Color::BLACK);
app.fill_ellipse(100, 250, 300, 350);
```

### 圆角矩形绘制

使用`roundrect`方法可以绘制圆角矩形：

| 方法 | 描述 |
|------|------|
| `roundrect(left, top, right, bottom, ellipse_width, ellipse_height)` | 绘制圆角矩形边框 |
| `fill_roundrect(left, top, right, bottom, ellipse_width, ellipse_height)` | 绘制填充圆角矩形 |
| `solid_roundrect(left, top, right, bottom, ellipse_width, ellipse_height)` | 绘制实心圆角矩形 |
| `clear_roundrect(left, top, right, bottom, ellipse_width, ellipse_height)` | 清除圆角矩形区域 |

```rust
// 绘制圆角矩形边框
app.set_linecolor(&Color::RED);
app.roundrect(100, 100, 300, 200, 20, 20);

// 绘制填充圆角矩形
app.set_fillcolor(&Color::BLUE);
app.set_linecolor(&Color::BLACK);
app.fill_roundrect(100, 250, 300, 350, 30, 30);
```

## 高级图形绘制

### 圆弧和扇形绘制

| 方法 | 描述 |
|------|------|
| `arc(left, top, right, bottom, start_angle, end_angle)` | 绘制圆弧 |
| `pie(left, top, right, bottom, start_angle, end_angle)` | 绘制扇形边框 |
| `fill_pie(left, top, right, bottom, start_angle, end_angle)` | 绘制填充扇形 |
| `solid_pie(left, top, right, bottom, start_angle, end_angle)` | 绘制实心扇形 |
| `clear_pie(left, top, right, bottom, start_angle, end_angle)` | 清除扇形区域 |

```rust
// 绘制圆弧
app.set_linecolor(&Color::RED);
app.arc(100, 100, 300, 300, 0.0, 180.0);

// 绘制填充扇形
app.set_fillcolor(&Color::BLUE);
app.set_linecolor(&Color::BLACK);
app.fill_pie(100, 350, 300, 550, 0.0, 120.0);
```

### 多边形绘制

| 方法 | 描述 |
|------|------|
| `polyline(points)` | 绘制多边形边框 |
| `polygon(points)` | 绘制闭合多边形边框 |
| `fill_polygon(points)` | 绘制填充多边形 |
| `solid_polygon(points)` | 绘制实心多边形 |
| `clear_polygon(points)` | 清除多边形区域 |

```rust
// 定义多边形顶点
let points = [
    POINT { x: 200, y: 100 },
    POINT { x: 300, y: 200 },
    POINT { x: 200, y: 300 },
    POINT { x: 100, y: 200 },
];

// 绘制多边形边框
app.set_linecolor(&Color::RED);
app.polygon(&points);

// 绘制填充多边形
let points2 = [
    POINT { x: 200, y: 400 },
    POINT { x: 300, y: 500 },
    POINT { x: 200, y: 600 },
    POINT { x: 100, y: 500 },
];

app.set_fillcolor(&Color::BLUE);
app.set_linecolor(&Color::BLACK);
app.fill_polygon(&points2);
```

### 贝塞尔曲线绘制

使用`poly_bezier`方法可以绘制贝塞尔曲线：

```rust
// 定义贝塞尔曲线控制点
let points = [
    POINT { x: 100, y: 200 },  // 起点
    POINT { x: 200, y: 100 },  // 控制点1
    POINT { x: 300, y: 300 },  // 控制点2
    POINT { x: 400, y: 200 },  // 终点
];

app.set_linecolor(&Color::RED);
app.poly_bezier(&points);
```

### 区域填充

使用`flood_fill`方法可以进行区域填充：

```rust
// 绘制一个圆
app.set_linecolor(&Color::RED);
app.circle(200, 200, 100);

// 在圆内填充蓝色
app.flood_fill(200, 200, Color::BLUE, FloodFillType::Border);
```

`FloodFillType`枚举有两个值：
- `Border`: 边界填充，填充到指定颜色边界为止
- `Surface`: 表面填充，填充所有连通的相同颜色区域

## 线条样式

在绘制图形之前，可以设置线条样式来改变线条的外观：

```rust
// 创建线条样式
let mut line_style =