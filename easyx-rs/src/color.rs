use easyx_sys::*;
use std::fmt;

/// 颜色结构体，用于包装 EasyX 的 COLORREF 类型
///
/// Color 结构体是 EasyX-RS 中表示颜色的核心类型，
/// 支持多种颜色创建方式和颜色转换功能。
///
/// # 示例
/// ```
/// // 从 RGB 值创建颜色
/// let red = Color::new(255, 0, 0);
///
/// // 从 HSL 值创建颜色
/// let green = Color::from_hsl(120.0, 1.0, 0.5);
///
/// // 从 HSV 值创建颜色
/// let blue = Color::from_hsv(240.0, 1.0, 1.0);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Color {
    pub(crate) value: u32,
}

impl Color {
    /// 从 RGB 值创建一个新的颜色
    ///
    /// 创建一个基于 RGB 颜色模型的颜色对象
    ///
    /// # 参数
    /// - `r`: 红色分量 (0-255)
    /// - `g`: 绿色分量 (0-255)
    /// - `b`: 蓝色分量 (0-255)
    ///
    /// # 返回值
    /// 一个新的 Color 对象
    ///
    /// # 示例
    /// ```
    /// // 创建红色
    /// let red = Color::new(255, 0, 0);
    ///
    /// // 创建绿色
    /// let green = Color::new(0, 255, 0);
    ///
    /// // 创建蓝色
    /// let blue = Color::new(0, 0, 255);
    /// ```
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            value: ((r as u32) << 16) | ((g as u32) << 8) | (b as u32),
        }
    }

    /// 从 HSL 值创建颜色
    ///
    /// 创建一个基于 HSL 颜色模型的颜色对象
    ///
    /// # 参数
    /// - `h`: 色相 (0-360)
    /// - `s`: 饱和度 (0-1)
    /// - `l`: 亮度 (0-1)
    ///
    /// # 返回值
    /// 一个新的 Color 对象
    ///
    /// # 示例
    /// ```
    /// // 创建红色 (H: 0, S: 1.0, L: 0.5)
    /// let red = Color::from_hsl(0.0, 1.0, 0.5);
    /// ```
    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        Self::from_colorref(unsafe { easyx_hsl_to_rgb(h, s, l) })
    }

    /// 从 HSV 值创建颜色
    ///
    /// 创建一个基于 HSV 颜色模型的颜色对象
    ///
    /// # 参数
    /// - `h`: 色相 (0-360)
    /// - `s`: 饱和度 (0-1)
    /// - `v`: 明度 (0-1)
    ///
    /// # 返回值
    /// 一个新的 Color 对象
    ///
    /// # 示例
    /// ```
    /// // 创建红色 (H: 0, S: 1.0, V: 1.0)
    /// let red = Color::from_hsv(0.0, 1.0, 1.0);
    /// ```
    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        Self::from_colorref(unsafe { easyx_hsv_to_rgb(h, s, v) })
    }

    /// 将颜色转换为灰度
    ///
    /// 将当前颜色转换为灰度值
    ///
    /// # 返回值
    /// 灰度颜色对象
    ///
    /// # 示例
    /// ```
    /// let color = Color::new(255, 0, 0);
    /// let gray = color.to_gray();
    /// ```
    pub fn to_gray(&self) -> Self {
        Self::from_colorref(unsafe { easyx_rgb_to_gray(self.value) })
    }

    /// 将 RGB 颜色转换为 HSL 颜色
    ///
    /// 将当前 RGB 颜色转换为 HSL 颜色模型
    ///
    /// # 返回值
    /// 一个元组，包含 (色相, 饱和度, 亮度)
    ///
    /// # 示例
    /// ```
    /// let color = Color::new(255, 0, 0);
    /// let (h, s, l) = color.to_hsl();
    /// ```
    pub fn to_hsl(&self) -> (f32, f32, f32) {
        let (mut h, mut s, mut l) = (0.0, 0.0, 0.0);

        unsafe { easyx_rgb_to_hsl(self.value, &mut h, &mut s, &mut l) };
        (h, s, l)
    }

    /// 将 RGB 颜色转换为 HSV 颜色
    ///
    /// 将当前 RGB 颜色转换为 HSV 颜色模型
    ///
    /// # 返回值
    /// 一个元组，包含 (色相, 饱和度, 明度)
    ///
    /// # 示例
    /// ```
    /// let color = Color::new(255, 0, 0);
    /// let (h, s, v) = color.to_hsv();
    /// ```
    pub fn to_hsv(&self) -> (f32, f32, f32) {
        let (mut h, mut s, mut v) = (0.0, 0.0, 0.0);

        unsafe { easyx_rgb_to_hsv(self.value, &mut h, &mut s, &mut v) };
        (h, s, v)
    }

    /// 从 COLORREF 值创建颜色
    ///
    /// 从底层的 COLORREF 值创建一个 Color 对象
    ///
    /// # 参数
    /// - `value`: COLORREF 值
    ///
    /// # 返回值
    /// 一个新的 Color 对象
    ///
    /// # 注意
    /// COLORREF 是 EasyX 使用的颜色表示方式，通常不需要直接使用此方法
    pub const fn from_colorref(value: u32) -> Self {
        Self { value }
    }

    /// 获取颜色的 COLORREF 值
    ///
    /// 获取底层的 COLORREF 值
    ///
    /// # 返回值
    /// COLORREF 值
    ///
    /// # 注意
    /// COLORREF 是 EasyX 使用的颜色表示方式，通常不需要直接使用此方法
    pub const fn as_colorref(&self) -> u32 {
        self.value
    }

    /// 获取红色分量
    ///
    /// # 返回值
    /// 红色分量值 (0-255)
    pub const fn r(&self) -> u8 {
        ((self.value >> 16) & 0xFF) as u8
    }

    /// 获取绿色分量
    ///
    /// # 返回值
    /// 绿色分量值 (0-255)
    pub const fn g(&self) -> u8 {
        ((self.value >> 8) & 0xFF) as u8
    }

    /// 获取蓝色分量
    ///
    /// # 返回值
    /// 蓝色分量值 (0-255)
    pub const fn b(&self) -> u8 {
        (self.value & 0xFF) as u8
    }
}

impl fmt::Display for Color {
    /// 格式化颜色为字符串
    ///
    /// # 示例
    /// ```
    /// let color = Color::new(255, 0, 0);
    /// println!("{}", color); // 输出: Color(r=255, g=0, b=0)
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color(r={}, g={}, b={})", self.r(), self.g(), self.b())
    }
}

impl From<u32> for Color {
    /// 从 u32 值创建颜色
    ///
    /// # 参数
    /// - `value`: u32 值，格式为 0xRRGGBB
    ///
    /// # 返回值
    /// 一个新的 Color 对象
    ///
    /// # 示例
    /// ```
    /// let color: Color = 0xFF0000.into(); // 红色
    /// ```
    fn from(value: u32) -> Self {
        Self::from_colorref(value)
    }
}

impl From<Color> for u32 {
    /// 将颜色转换为 u32 值
    ///
    /// # 参数
    /// - `color`: Color 对象
    ///
    /// # 返回值
    /// u32 值，格式为 0xRRGGBB
    ///
    /// # 示例
    /// ```
    /// let color = Color::new(255, 0, 0);
    /// let value: u32 = color.into(); // 0xFF0000
    /// ```
    fn from(color: Color) -> Self {
        color.as_colorref()
    }
}

// 常用颜色常量
impl Color {
    /// 黑色
    pub const BLACK: Self = Self::from_colorref(EASYX_BLACK);
    /// 蓝色
    pub const BLUE: Self = Self::from_colorref(EASYX_BLUE);
    /// 绿色
    pub const GREEN: Self = Self::from_colorref(EASYX_GREEN);
    /// 青色
    pub const CYAN: Self = Self::from_colorref(EASYX_CYAN);
    /// 红色
    pub const RED: Self = Self::from_colorref(EASYX_RED);
    /// 品红色
    pub const MAGENTA: Self = Self::from_colorref(EASYX_MAGENTA);
    /// 棕色
    pub const BROWN: Self = Self::from_colorref(EASYX_BROWN);
    /// 浅灰色
    pub const LIGHTGRAY: Self = Self::from_colorref(EASYX_LIGHTGRAY);
    /// 深灰色
    pub const DARKGRAY: Self = Self::from_colorref(EASYX_DARKGRAY);
    /// 浅蓝色
    pub const LIGHTBLUE: Self = Self::from_colorref(EASYX_LIGHTBLUE);
    /// 浅绿色
    pub const LIGHTGREEN: Self = Self::from_colorref(EASYX_LIGHTGREEN);
    /// 浅青色
    pub const LIGHTCYAN: Self = Self::from_colorref(EASYX_LIGHTCYAN);
    /// 浅红色
    pub const LIGHTRED: Self = Self::from_colorref(EASYX_LIGHTRED);
    /// 浅品红色
    pub const LIGHTMAGENTA: Self = Self::from_colorref(EASYX_LIGHTMAGENTA);
    /// 黄色
    pub const YELLOW: Self = Self::from_colorref(EASYX_YELLOW);
    /// 白色
    pub const WHITE: Self = Self::from_colorref(EASYX_WHITE);
}

// 允许直接使用颜色值进行算术运算
impl std::ops::BitAnd for Color {
    type Output = Color;

    /// 颜色按位与操作
    ///
    /// # 参数
    /// - `rhs`: 右侧颜色值
    ///
    /// # 返回值
    /// 按位与结果
    fn bitand(self, rhs: Self) -> Self::Output {
        Self::from_colorref(self.value & rhs.value)
    }
}

impl std::ops::BitOr for Color {
    type Output = Color;

    /// 颜色按位或操作
    ///
    /// # 参数
    /// - `rhs`: 右侧颜色值
    ///
    /// # 返回值
    /// 按位或结果
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::from_colorref(self.value | rhs.value)
    }
}

impl std::ops::BitXor for Color {
    type Output = Color;

    /// 颜色按位异或操作
    ///
    /// # 参数
    /// - `rhs`: 右侧颜色值
    ///
    /// # 返回值
    /// 按位异或结果
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::from_colorref(self.value ^ rhs.value)
    }
}

impl Color {
    /// 获取当前线条颜色
    ///
    /// # 返回值
    /// 当前线条颜色
    pub fn get_linecolor() -> Self {
        Self::from_colorref(unsafe { easyx_getlinecolor() })
    }

    /// 设置当前线条颜色
    ///
    /// # 示例
    /// ```no_run
    /// let color = Color::RED;
    /// color.set_linecolor();
    /// ```
    pub fn set_linecolor(&self) {
        unsafe { easyx_setlinecolor(self.value) };
    }

    /// 获取当前文本颜色
    ///
    /// # 返回值
    /// 当前文本颜色
    pub fn get_textcolor() -> Self {
        Self::from_colorref(unsafe { easyx_gettextcolor() })
    }

    /// 设置当前文本颜色
    ///
    /// # 示例
    /// ```no_run
    /// let color = Color::BLUE;
    /// color.set_textcolor();
    /// ```
    pub fn set_textcolor(&self) {
        unsafe { easyx_settextcolor(self.value) };
    }

    /// 获取当前填充颜色
    ///
    /// # 返回值
    /// 当前填充颜色
    pub fn get_fillcolor() -> Self {
        Self::from_colorref(unsafe { easyx_getfillcolor() })
    }

    /// 设置当前填充颜色
    ///
    /// # 示例
    /// ```no_run
    /// let color = Color::GREEN;
    /// color.set_fillcolor();
    /// ```
    pub fn set_fillcolor(&self) {
        unsafe { easyx_setfillcolor(self.value) };
    }

    /// 获取当前背景颜色
    ///
    /// # 返回值
    /// 当前背景颜色
    pub fn get_bkcolor() -> Self {
        Self::from_colorref(unsafe { easyx_getbkcolor() })
    }

    /// 设置当前背景颜色
    ///
    /// # 示例
    /// ```no_run
    /// let color = Color::WHITE;
    /// color.set_bkcolor();
    /// ```
    pub fn set_bkcolor(&self) {
        unsafe { easyx_setbkcolor(self.value) };
    }

    /// 获取指定位置的像素颜色
    ///
    /// # 参数
    /// - `x`: 像素x坐标
    /// - `y`: 像素y坐标
    ///
    /// # 返回值
    /// 该位置的像素颜色
    pub fn get_pixel(x: i32, y: i32) -> Self {
        Self::from_colorref(unsafe { easyx_getpixel(x, y) })
    }

    /// 设置指定位置的像素颜色
    ///
    /// # 参数
    /// - `x`: 像素x坐标
    /// - `y`: 像素y坐标
    ///
    /// # 示例
    /// ```no_run
    /// let color = Color::RED;
    /// color.put_pixel(100, 100);
    /// ```
    pub fn put_pixel(&self, x: i32, y: i32) {
        unsafe { easyx_putpixel(x, y, self.value) };
    }
}
