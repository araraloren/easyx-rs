use easyx_sys::*;
use std::fmt;

/// 颜色结构体，用于包装 EasyX 的 COLORREF 类型
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Color {
    pub(crate) value: u32,
}

impl Color {
    /// 创建一个新的颜色
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            value: ((r as u32) << 16) | ((g as u32) << 8) | (b as u32),
        }
    }

    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        Self::from_colorref(unsafe { easyx_hsl_to_rgb(h, s, l) })
    }

    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        Self::from_colorref(unsafe { easyx_hsv_to_rgb(h, s, v) })
    }

    pub fn to_gray(&self) -> Self {
        Self::from_colorref(unsafe { easyx_rgb_to_gray(self.value) })
    }

    pub fn to_hsl(&self) -> (f32, f32, f32) {
        let (mut h, mut s, mut l) = (0.0, 0.0, 0.0);

        unsafe { easyx_rgb_to_hsl(self.value, &mut h, &mut s, &mut l) };
        (h, s, l)
    }

    pub fn to_hsv(&self) -> (f32, f32, f32) {
        let (mut h, mut s, mut v) = (0.0, 0.0, 0.0);

        unsafe { easyx_rgb_to_hsv(self.value, &mut h, &mut s, &mut v) };
        (h, s, v)
    }

    /// 从 COLORREF 创建颜色
    pub const fn from_colorref(value: u32) -> Self {
        Self { value }
    }

    /// 获取颜色的 COLORREF 值
    pub const fn as_colorref(&self) -> u32 {
        self.value
    }

    /// 获取红色分量
    pub const fn r(&self) -> u8 {
        ((self.value >> 16) & 0xFF) as u8
    }

    /// 获取绿色分量
    pub const fn g(&self) -> u8 {
        ((self.value >> 8) & 0xFF) as u8
    }

    /// 获取蓝色分量
    pub const fn b(&self) -> u8 {
        (self.value & 0xFF) as u8
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color(r={}, g={}, b={})", self.r(), self.g(), self.b())
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self::from_colorref(value)
    }
}

impl From<Color> for u32 {
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

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::from_colorref(self.value & rhs.value)
    }
}

impl std::ops::BitOr for Color {
    type Output = Color;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::from_colorref(self.value | rhs.value)
    }
}

impl std::ops::BitXor for Color {
    type Output = Color;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::from_colorref(self.value ^ rhs.value)
    }
}

impl Color {
    pub fn get_linecolor() -> Self {
        Self::from_colorref(unsafe { easyx_getlinecolor() })
    }

    pub fn set_linecolor(&self) {
        unsafe { easyx_setlinecolor(self.value) };
    }

    pub fn get_textcolor() -> Self {
        Self::from_colorref(unsafe { easyx_gettextcolor() })
    }

    pub fn set_textcolor(&self) {
        unsafe { easyx_settextcolor(self.value) };
    }

    pub fn get_fillcolor() -> Self {
        Self::from_colorref(unsafe { easyx_getfillcolor() })
    }

    pub fn set_fillcolor(&self) {
        unsafe { easyx_setfillcolor(self.value) };
    }

    pub fn get_bkcolor() -> Self {
        Self::from_colorref(unsafe { easyx_getbkcolor() })
    }

    pub fn set_bkcolor(&self) {
        unsafe { easyx_setbkcolor(self.value) };
    }

    pub fn get_pixel(x: i32, y: i32) -> Self {
        Self::from_colorref(unsafe { easyx_getpixel(x, y) })
    }

    pub fn put_pixel(&self, x: i32, y: i32) {
        unsafe { easyx_putpixel(x, y, self.value) };
    }
}
