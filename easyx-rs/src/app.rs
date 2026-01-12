use easyx_sys::*;
use windows_sys::Win32::Foundation::HWND;

use crate::color::Color;
use crate::enums::BkMode;
use crate::enums::DrawTextFormat;
use crate::fillstyle::FillStyle;
use crate::input::InputBox;
use crate::linestyle::LineStyle;
use crate::logfont::LogFont;
use crate::msg::{ExMessage, MessageFilter};

/// RECT结构体，用于draw_text函数
#[repr(C)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

bitflags::bitflags! {
    /// EasyX图形库的窗口初始化标志。
    ///
    /// 这些标志可以使用 `|` 操作符组合，设置多个选项。
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct InitFlags: u32 {
        /// 未设置任何标志。
        const None = 0;
        /// 显示控制台窗口。
        const ShowConsole = EASYX_EX_SHOWCONSOLE;
        /// 禁用关闭按钮。
        const NoClose = EASYX_EX_NOCLOSE;
        /// 禁用最小化按钮。
        const NoMinimize = EASYX_EX_NOMINIMIZE;
        /// 启用双击事件。
        const DblClks = EASYX_EX_DBLCLKS;
    }
}

pub struct App {
    width: i32,
    height: i32,
    hwnd: HWND,
}

impl App {
    /// 创建一个新的 EasyX 图形应用实例。
    ///
    /// # 参数
    ///
    /// * `width` - 图形窗口的宽度。
    /// * `height` - 图形窗口的高度。
    /// * `flags` - 图形窗口的初始化标志。
    ///
    /// # 返回值
    ///
    /// 一个新的 `App` 实例。
    pub fn new(width: i32, height: i32, flags: InitFlags) -> Self {
        let hwnd = unsafe { easyx_initgraph(width, height, flags.bits() as i32) };

        Self {
            width,
            height,
            hwnd: hwnd as HWND,
        }
    }

    /// 初始化图形窗口并运行提供的闭包。
    ///
    /// 此方法初始化图形窗口，运行提供的闭包，并确保窗口在完成后正确关闭。
    /// 闭包会被传递一个 App 实例的引用。
    ///
    /// # 参数
    ///
    /// * `f` - 一个将在图形上下文中执行的闭包。
    ///
    /// # 返回值
    ///
    /// 一个表示成功或失败的 `Result`。
    pub fn run<F>(&self, f: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(&Self) -> Result<(), Box<dyn std::error::Error>> + std::panic::UnwindSafe,
    {
        // Ensure the closure is executed safely
        let result = std::panic::catch_unwind(|| f(self));

        // Handle the result from the closure
        match result {
            Ok(res) => res,
            Err(err) => {
                // Convert panic to error
                let panic_msg = if let Some(msg) = err.downcast_ref::<&str>() {
                    *msg
                } else if let Some(msg) = err.downcast_ref::<String>() {
                    msg.as_str()
                } else {
                    "Unknown panic occurred"
                };
                Err(panic_msg.into())
            }
        }
    }

    /// 获取图形窗口的宽度。
    pub fn width(&self) -> i32 {
        self.width
    }

    /// 获取图形窗口的高度。
    pub fn height(&self) -> i32 {
        self.height
    }

    /// 获取图形窗口的句柄。
    pub fn hwnd(&self) -> &HWND {
        &self.hwnd
    }

    /// 获取图形窗口的句柄。
    ///
    /// 此方法直接调用底层的easyx_gethwnd函数，获取当前图形窗口的句柄。
    pub fn graphics_hwnd(&self) -> HWND {
        unsafe { easyx_gethwnd() as _ }
    }

    /// 获取EasyX库的版本信息。
    ///
    /// 返回当前使用的EasyX库的版本字符串。
    pub fn version(&self) -> &'static str {
        let ver = unsafe { easyx_geteasyxver() };

        unsafe { std::ffi::CStr::from_ptr(ver).to_str().unwrap() }
    }
}

impl App {
    /// 设置坐标系原点。
    ///
    /// 将当前坐标系的原点设置到指定位置。
    ///
    /// # 参数
    /// * `x` - 新原点的x坐标。
    /// * `y` - 新原点的y坐标。
    pub fn set_origin(&self, x: i32, y: i32) {
        unsafe {
            easyx_setorigin(x, y);
        }
    }

    /// 设置裁剪区域。
    ///
    /// 设置当前绘图的裁剪区域，超出该区域的图形将不会被绘制。
    ///
    /// # 参数
    /// * `x` - 裁剪区域左上角的x坐标。
    /// * `y` - 裁剪区域左上角的y坐标。
    /// * `width` - 裁剪区域的宽度。
    /// * `height` - 裁剪区域的高度。
    pub fn set_cliprgn(&self, x: i32, y: i32, width: i32, height: i32) {
        let rgn = unsafe { CreateRectRgn(x, y, x + width, y + height) };

        unsafe {
            easyx_setcliprgn(rgn as _);
        }
    }

    /// 重置裁剪区域。
    ///
    /// 清除当前设置的裁剪区域，恢复为默认的整个窗口区域。
    pub fn reset_cliprgn(&self) {
        unsafe {
            easyx_clearcliprgn();
        }
    }

    /// 设置坐标系的纵横比。
    ///
    /// 设置当前坐标系的宽高比，用于控制图形的缩放比例。
    ///
    /// # 参数
    /// * `width` - 宽高比的宽度分量。
    /// * `height` - 宽高比的高度分量。
    pub fn set_aspectratio(&self, width: f32, height: f32) {
        unsafe {
            easyx_setaspectratio(width, height);
        }
    }

    /// 获取当前坐标系的纵横比。
    ///
    /// 返回当前坐标系的宽高比，用于控制图形的缩放比例。
    ///
    /// # 返回值
    /// 一个包含宽高比的元组，第一个元素是宽度分量，第二个元素是高度分量。
    pub fn get_aspectratio(&self) -> (f32, f32) {
        let mut width = 0.0;
        let mut height = 0.0;
        unsafe {
            easyx_getaspectratio(&mut width, &mut height);
        }

        (width, height)
    }

    /// 重置图形环境默认设置。
    ///
    /// 将当前图形环境的所有设置恢复为默认值，包括颜色、线条样式、填充样式等。
    pub fn reset_graph_defaults(&self) {
        unsafe {
            easyx_graphdefaults();
        }
    }

    /// 清屏。
    ///
    /// 清除当前设备屏幕，将其恢复为默认背景色。
    pub fn clear_device(&self) {
        unsafe {
            easyx_cleardevice();
        }
    }
}

impl App {
    /// 获取当前线条样式。
    ///
    /// 返回当前图形环境的线条样式设置。
    ///
    /// # 返回值
    /// 当前的线条样式对象。
    pub fn get_linestyle(&self) -> LineStyle {
        LineStyle::current()
    }

    /// 设置线条样式。
    ///
    /// 设置当前图形环境的线条样式。
    ///
    /// # 参数
    /// * `linestyle` - 要设置的线条样式对象。
    pub fn set_linestyle(&self, linestyle: &LineStyle) {
        linestyle.apply();
    }

    /// 获取当前填充样式。
    ///
    /// 返回当前图形环境的填充样式设置。
    ///
    /// # 返回值
    /// 当前的填充样式对象。
    pub fn get_fillstyle(&self) -> FillStyle {
        FillStyle::current()
    }

    /// 设置填充样式。
    ///
    /// 设置当前图形环境的填充样式。
    ///
    /// # 参数
    /// * `fillstyle` - 要设置的填充样式对象。
    pub fn set_fillstyle(&self, fillstyle: &FillStyle) {
        fillstyle.apply();
    }
}

impl App {
    /// 获取当前线条颜色。
    ///
    /// 返回当前图形环境的线条颜色设置。
    ///
    /// # 返回值
    /// 当前的线条颜色对象。
    pub fn get_linecolor(&self) -> Color {
        Color::get_linecolor()
    }

    /// 设置线条颜色。
    ///
    /// 设置当前图形环境的线条颜色。
    ///
    /// # 参数
    /// * `color` - 要设置的线条颜色对象。
    pub fn set_linecolor(&self, color: &Color) {
        color.set_linecolor();
    }

    /// 获取当前文本颜色。
    ///
    /// 返回当前图形环境的文本颜色设置。
    ///
    /// # 返回值
    /// 当前的文本颜色对象。
    pub fn get_textcolor(&self) -> Color {
        Color::get_textcolor()
    }

    /// 设置文本颜色。
    ///
    /// 设置当前图形环境的文本颜色。
    ///
    /// # 参数
    /// * `color` - 要设置的文本颜色对象。
    pub fn set_textcolor(&self, color: &Color) {
        color.set_textcolor();
    }

    /// 获取当前填充颜色。
    ///
    /// 返回当前图形环境的填充颜色设置。
    ///
    /// # 返回值
    /// 当前的填充颜色对象。
    pub fn get_fillcolor(&self) -> Color {
        Color::get_fillcolor()
    }

    /// 设置填充颜色。
    ///
    /// 设置当前图形环境的填充颜色。
    ///
    /// # 参数
    /// * `color` - 要设置的填充颜色对象。
    pub fn set_fillcolor(&self, color: &Color) {
        color.set_fillcolor();
    }

    /// 获取当前背景模式。
    ///
    /// 返回当前图形环境的背景模式设置。
    ///
    /// # 返回值
    /// 当前的背景模式对象。
    pub fn get_bkmode(&self) -> BkMode {
        BkMode::current()
    }

    /// 设置背景模式。
    ///
    /// 设置当前图形环境的背景模式。
    ///
    /// # 参数
    /// * `bkmode` - 要设置的背景模式对象。
    pub fn set_bkmode(&self, bkmode: &BkMode) {
        bkmode.apply();
    }

    /// 获取指定位置的像素颜色。
    ///
    /// 获取指定坐标位置的像素颜色。
    ///
    /// # 参数
    /// * `x` - 像素的x坐标。
    /// * `y` - 像素的y坐标。
    ///
    /// # 返回值
    /// 像素的颜色对象。
    pub fn get_pixel(&self, x: i32, y: i32) -> Color {
        Color::get_pixel(x, y)
    }

    /// 绘制点。
    ///
    /// 在指定位置绘制一个点。
    ///
    /// # 参数
    /// * `x` - 点的x坐标。
    /// * `y` - 点的y坐标。
    /// * `color` - 点的颜色。
    pub fn put_pixel(&self, x: i32, y: i32, color: &Color) {
        color.put_pixel(x, y);
    }

    /// 获取当前背景颜色
    ///
    /// # 返回值
    /// 当前背景颜色
    pub fn get_bkcolor(&self) -> Color {
        Color::get_bkcolor()
    }

    /// 设置当前背景颜色
    ///
    /// # 参数
    /// * `color` - 背景颜色。
    pub fn set_bkcolor(&self, color: &Color) {
        color.set_bkcolor();
    }
}

impl App {
    /// 绘制直线。
    ///
    /// 在指定的两个点之间绘制一条直线。
    ///
    /// # 参数
    /// * `left` - 直线起点的x坐标。
    /// * `top` - 直线起点的y坐标。
    /// * `right` - 直线终点的x坐标。
    /// * `bottom` - 直线终点的y坐标。
    pub fn line(&self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe {
            easyx_line(left, top, right, bottom);
        }
    }

    /// 绘制矩形。
    ///
    /// 绘制一个矩形边框。
    ///
    /// # 参数
    /// * `left` - 矩形左上角的x坐标。
    /// * `top` - 矩形左上角的y坐标。
    /// * `right` - 矩形右下角的x坐标。
    /// * `bottom` - 矩形右下角的y坐标。
    pub fn rectangle(&self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe {
            easyx_rectangle(left, top, right, bottom);
        }
    }

    /// 绘制填充矩形。
    ///
    /// 绘制一个填充的矩形。
    ///
    /// # 参数
    /// * `left` - 矩形左上角的x坐标。
    /// * `top` - 矩形左上角的y坐标。
    /// * `right` - 矩形右下角的x坐标。
    /// * `bottom` - 矩形右下角的y坐标。
    pub fn fill_rectangle(&self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe {
            easyx_fillrectangle(left, top, right, bottom);
        }
    }

    /// 绘制实心矩形。
    ///
    /// 绘制一个实心矩形，使用当前线条颜色作为填充颜色。
    ///
    /// # 参数
    /// * `left` - 矩形左上角的x坐标。
    /// * `top` - 矩形左上角的y坐标。
    /// * `right` - 矩形右下角的x坐标。
    /// * `bottom` - 矩形右下角的y坐标。
    pub fn solid_rectangle(&self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe {
            easyx_solidrectangle(left, top, right, bottom);
        }
    }

    /// 清除矩形区域。
    ///
    /// 使用当前背景色清除指定的矩形区域。
    ///
    /// # 参数
    /// * `left` - 矩形左上角的x坐标。
    /// * `top` - 矩形左上角的y坐标。
    /// * `right` - 矩形右下角的x坐标。
    /// * `bottom` - 矩形右下角的y坐标。
    pub fn clear_rectangle(&self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe {
            easyx_clearrectangle(left, top, right, bottom);
        }
    }

    /// 绘制圆形。
    ///
    /// 绘制一个圆形边框。
    ///
    /// # 参数
    /// * `x` - 圆心的x坐标。
    /// * `y` - 圆心的y坐标。
    /// * `radius` - 圆的半径。
    pub fn circle(&self, x: i32, y: i32, radius: i32) {
        unsafe {
            easyx_circle(x, y, radius);
        }
    }

    /// 绘制填充圆形。
    ///
    /// 绘制一个填充的圆形。
    ///
    /// # 参数
    /// * `x` - 圆心的x坐标。
    /// * `y` - 圆心的y坐标。
    /// * `radius` - 圆的半径。
    pub fn fill_circle(&self, x: i32, y: i32, radius: i32) {
        unsafe {
            easyx_fillcircle(x, y, radius);
        }
    }

    /// 绘制实心圆形。
    ///
    /// 绘制一个实心圆形，使用当前线条颜色作为填充颜色。
    ///
    /// # 参数
    /// * `x` - 圆心的x坐标。
    /// * `y` - 圆心的y坐标。
    /// * `radius` - 圆的半径。
    pub fn solid_circle(&self, x: i32, y: i32, radius: i32) {
        unsafe {
            easyx_solidcircle(x, y, radius);
        }
    }

    /// 清除圆形区域。
    ///
    /// 使用当前背景色清除指定的圆形区域。
    ///
    /// # 参数
    /// * `x` - 圆心的x坐标。
    /// * `y` - 圆心的y坐标。
    /// * `radius` - 圆的半径。
    pub fn clear_circle(&self, x: i32, y: i32, radius: i32) {
        unsafe {
            easyx_clearcircle(x, y, radius);
        }
    }

    /// 绘制椭圆。
    ///
    /// 绘制一个椭圆边框。
    ///
    /// # 参数
    /// * `x` - 椭圆中心的x坐标。
    /// * `y` - 椭圆中心的y坐标。
    /// * `rx` - 椭圆的x轴半径。
    /// * `ry` - 椭圆的y轴半径。
    pub fn ellipse(&self, x: i32, y: i32, rx: i32, ry: i32) {
        unsafe {
            easyx_ellipse(x, y, rx, ry);
        }
    }

    /// 绘制填充椭圆。
    ///
    /// 绘制一个填充的椭圆。
    ///
    /// # 参数
    /// * `left` - 椭圆外接矩形左上角的x坐标。
    /// * `top` - 椭圆外接矩形左上角的y坐标。
    /// * `right` - 椭圆外接矩形右下角的x坐标。
    /// * `bottom` - 椭圆外接矩形右下角的y坐标。
    pub fn fill_ellipse(&self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe {
            easyx_fillellipse(left, top, right, bottom);
        }
    }

    /// 绘制实心椭圆。
    ///
    /// 绘制一个实心椭圆，使用当前线条颜色作为填充颜色。
    ///
    /// # 参数
    /// * `left` - 椭圆外接矩形左上角的x坐标。
    /// * `top` - 椭圆外接矩形左上角的y坐标。
    /// * `right` - 椭圆外接矩形右下角的x坐标。
    /// * `bottom` - 椭圆外接矩形右下角的y坐标。
    pub fn solid_ellipse(&self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe {
            easyx_solidellipse(left, top, right, bottom);
        }
    }

    /// 清除椭圆区域。
    ///
    /// 使用当前背景色清除指定的椭圆区域。
    ///
    /// # 参数
    /// * `left` - 椭圆外接矩形左上角的x坐标。
    /// * `top` - 椭圆外接矩形左上角的y坐标。
    /// * `right` - 椭圆外接矩形右下角的x坐标。
    /// * `bottom` - 椭圆外接矩形右下角的y坐标。
    pub fn clear_ellipse(&self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe {
            easyx_clearellipse(left, top, right, bottom);
        }
    }

    /// 绘制圆角矩形。
    ///
    /// 绘制一个圆角矩形边框。
    ///
    /// # 参数
    /// * `left` - 圆角矩形左上角的x坐标。
    /// * `top` - 圆角矩形左上角的y坐标。
    /// * `right` - 圆角矩形右下角的x坐标。
    /// * `bottom` - 圆角矩形右下角的y坐标。
    /// * `ellipsewith` - 圆角的宽度（水平方向）。
    /// * `ellipseheight` - 圆角的高度（垂直方向）。
    pub fn roundrect(
        &self,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
        ellipsewith: i32,
        ellipseheight: i32,
    ) {
        unsafe {
            easyx_roundrect(left, top, right, bottom, ellipsewith, ellipseheight);
        }
    }

    /// 绘制填充圆角矩形。
    ///
    /// 绘制一个填充的圆角矩形。
    ///
    /// # 参数
    /// * `left` - 圆角矩形左上角的x坐标。
    /// * `top` - 圆角矩形左上角的y坐标。
    /// * `right` - 圆角矩形右下角的x坐标。
    /// * `bottom` - 圆角矩形右下角的y坐标。
    /// * `ellipsewith` - 圆角的宽度（水平方向）。
    /// * `ellipseheight` - 圆角的高度（垂直方向）。
    pub fn fill_roundrect(
        &self,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
        ellipsewith: i32,
        ellipseheight: i32,
    ) {
        unsafe {
            easyx_fillroundrect(left, top, right, bottom, ellipsewith, ellipseheight);
        }
    }

    /// 绘制实心圆角矩形。
    ///
    /// 绘制一个实心圆角矩形，使用当前线条颜色作为填充颜色。
    ///
    /// # 参数
    /// * `left` - 圆角矩形左上角的x坐标。
    /// * `top` - 圆角矩形左上角的y坐标。
    /// * `right` - 圆角矩形右下角的x坐标。
    /// * `bottom` - 圆角矩形右下角的y坐标。
    /// * `ellipsewith` - 圆角的宽度（水平方向）。
    /// * `ellipseheight` - 圆角的高度（垂直方向）。
    pub fn solid_roundrect(
        &self,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
        ellipsewith: i32,
        ellipseheight: i32,
    ) {
        unsafe {
            easyx_solidroundrect(left, top, right, bottom, ellipsewith, ellipseheight);
        }
    }

    /// 清除圆角矩形区域。
    ///
    /// 使用当前背景色清除指定的圆角矩形区域。
    ///
    /// # 参数
    /// * `left` - 圆角矩形左上角的x坐标。
    /// * `top` - 圆角矩形左上角的y坐标。
    /// * `right` - 圆角矩形右下角的x坐标。
    /// * `bottom` - 圆角矩形右下角的y坐标。
    /// * `ellipsewith` - 圆角的宽度（水平方向）。
    /// * `ellipseheight` - 圆角的高度（垂直方向）。
    pub fn clear_roundrect(
        &self,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
        ellipsewith: i32,
        ellipseheight: i32,
    ) {
        unsafe {
            easyx_clearroundrect(left, top, right, bottom, ellipsewith, ellipseheight);
        }
    }

    /// 绘制圆弧。
    ///
    /// 绘制椭圆的一段圆弧。
    ///
    /// # 参数
    /// * `left` - 椭圆外接矩形左上角的x坐标。
    /// * `top` - 椭圆外接矩形左上角的y坐标。
    /// * `right` - 椭圆外接矩形右下角的x坐标。
    /// * `bottom` - 椭圆外接矩形右下角的y坐标。
    /// * `stange` - 圆弧的起始角度（弧度）。
    /// * `endangle` - 圆弧的结束角度（弧度）。
    pub fn arc(&self, left: i32, top: i32, right: i32, bottom: i32, stange: f64, endangle: f64) {
        unsafe {
            easyx_arc(left, top, right, bottom, stange, endangle);
        }
    }

    /// 绘制扇形。
    ///
    /// 绘制椭圆的一段扇形。
    ///
    /// # 参数
    /// * `left` - 椭圆外接矩形左上角的x坐标。
    /// * `top` - 椭圆外接矩形左上角的y坐标。
    /// * `right` - 椭圆外接矩形右下角的x坐标。
    /// * `bottom` - 椭圆外接矩形右下角的y坐标。
    /// * `stange` - 扇形的起始角度（弧度）。
    /// * `endangle` - 扇形的结束角度（弧度）。
    pub fn pie(&self, left: i32, top: i32, right: i32, bottom: i32, stange: f64, endangle: f64) {
        unsafe {
            easyx_pie(left, top, right, bottom, stange, endangle);
        }
    }

    /// 绘制填充扇形。
    ///
    /// 绘制一个填充的扇形。
    ///
    /// # 参数
    /// * `left` - 椭圆外接矩形左上角的x坐标。
    /// * `top` - 椭圆外接矩形左上角的y坐标。
    /// * `right` - 椭圆外接矩形右下角的x坐标。
    /// * `bottom` - 椭圆外接矩形右下角的y坐标。
    /// * `stange` - 扇形的起始角度（弧度）。
    /// * `endangle` - 扇形的结束角度（弧度）。
    pub fn fill_pie(
        &self,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
        stange: f64,
        endangle: f64,
    ) {
        unsafe {
            easyx_fillpie(left, top, right, bottom, stange, endangle);
        }
    }

    /// 绘制实心扇形。
    ///
    /// 绘制一个实心扇形，使用当前线条颜色作为填充颜色。
    ///
    /// # 参数
    /// * `left` - 椭圆外接矩形左上角的x坐标。
    /// * `top` - 椭圆外接矩形左上角的y坐标。
    /// * `right` - 椭圆外接矩形右下角的x坐标。
    /// * `bottom` - 椭圆外接矩形右下角的y坐标。
    /// * `stange` - 扇形的起始角度（弧度）。
    /// * `endangle` - 扇形的结束角度（弧度）。
    pub fn solid_pie(
        &self,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
        stange: f64,
        endangle: f64,
    ) {
        unsafe {
            easyx_solidpie(left, top, right, bottom, stange, endangle);
        }
    }

    /// 清除扇形区域。
    ///
    /// 使用当前背景色清除指定的扇形区域。
    ///
    /// # 参数
    /// * `left` - 椭圆外接矩形左上角的x坐标。
    /// * `top` - 椭圆外接矩形左上角的y坐标。
    /// * `right` - 椭圆外接矩形右下角的x坐标。
    /// * `bottom` - 椭圆外接矩形右下角的y坐标。
    /// * `stange` - 扇形的起始角度（弧度）。
    /// * `endangle` - 扇形的结束角度（弧度）。
    pub fn clear_pie(
        &self,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
        stange: f64,
        endangle: f64,
    ) {
        unsafe {
            easyx_clearpie(left, top, right, bottom, stange, endangle);
        }
    }

    /// 绘制折线。
    ///
    /// 连接多个点绘制一条折线。
    ///
    /// # 参数
    /// * `points` - 折线的顶点数组。
    pub fn poly_line(&self, points: &[POINT]) {
        unsafe {
            easyx_polyline(points.as_ptr() as _, points.len() as i32);
        }
    }

    /// 绘制多边形。
    ///
    /// 绘制一个多边形边框。
    ///
    /// # 参数
    /// * `points` - 多边形的顶点数组。
    pub fn poly_gon(&self, points: &[POINT]) {
        unsafe {
            easyx_polygon(points.as_ptr() as _, points.len() as i32);
        }
    }

    /// 绘制填充多边形。
    ///
    /// 绘制一个填充的多边形。
    ///
    /// # 参数
    /// * `points` - 多边形的顶点数组。
    pub fn fill_polygon(&self, points: &[POINT]) {
        unsafe {
            easyx_fillpolygon(points.as_ptr() as _, points.len() as i32);
        }
    }

    /// 绘制实心多边形。
    ///
    /// 绘制一个实心多边形，使用当前线条颜色作为填充颜色。
    ///
    /// # 参数
    /// * `points` - 多边形的顶点数组。
    pub fn solid_polygon(&self, points: &[POINT]) {
        unsafe {
            easyx_solidpolygon(points.as_ptr() as _, points.len() as i32);
        }
    }

    /// 清除多边形区域。
    ///
    /// 使用当前背景色清除指定的多边形区域。
    ///
    /// # 参数
    /// * `points` - 多边形的顶点数组。
    pub fn clear_polygon(&self, points: &[POINT]) {
        unsafe {
            easyx_clearpolygon(points.as_ptr() as _, points.len() as i32);
        }
    }

    /// 绘制贝塞尔曲线。
    ///
    /// 绘制一条贝塞尔曲线。
    ///
    /// # 参数
    /// * `points` - 贝塞尔曲线的控制点数组。
    pub fn poly_bezier(&self, points: &[POINT]) {
        unsafe {
            easyx_polybezier(points.as_ptr() as _, points.len() as i32);
        }
    }
}

/// 区域填充类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloodFillType {
    /// 边界填充模式：从指定点开始，填充到指定边界颜色为止。
    Border = FLOODFILLBORDER as isize,

    /// 表面填充模式：从指定点开始，填充所有连接的同色区域。
    Surface = FLOODFILLSURFACE as isize,
}

impl App {
    /// 区域填充。
    ///
    /// 使用指定颜色填充一个区域。
    ///
    /// # 参数
    /// * `x` - 填充的起始点x坐标。
    /// * `y` - 填充的起始点y坐标。
    /// * `color` - 填充颜色。
    /// * `fill_type` - 填充类型，可以是Border或Surface。
    pub fn flood_fill(&self, x: i32, y: i32, color: Color, fill_type: FloodFillType) {
        unsafe {
            easyx_floodfill(x, y, color.value, fill_type as i32);
        }
    }
}

impl App {
    /// 在指定位置输出文本。
    ///
    /// 在指定坐标位置输出文本。
    ///
    /// # 参数
    /// * `x` - 文本输出的x坐标。
    /// * `y` - 文本输出的y坐标。
    /// * `text` - 要输出的文本。
    pub fn out_text(&self, x: i32, y: i32, text: &str) {
        use std::ffi::CString;
        unsafe {
            let c_str = CString::new(text).expect("Failed to create C string");
            easyx_outtextxy(x, y, c_str.as_ptr());
        }
    }

    /// 在指定位置输出单个字符。
    ///
    /// 在指定坐标位置输出单个字符。
    ///
    /// # 参数
    /// * `x` - 字符输出的x坐标。
    /// * `y` - 字符输出的y坐标。
    /// * `c` - 要输出的字符。
    pub fn out_text_char(&self, x: i32, y: i32, c: char) {
        unsafe {
            easyx_outtextxy_char(x, y, c as i8);
        }
    }

    /// 获取文本宽度。
    ///
    /// 计算指定文本在当前字体下的宽度。
    ///
    /// # 参数
    /// * `text` - 要计算宽度的文本。
    ///
    /// # 返回值
    /// 文本的宽度，以像素为单位。
    pub fn text_width(&self, text: &str) -> i32 {
        use std::ffi::CString;
        unsafe {
            let c_str = CString::new(text).expect("Failed to create C string");
            easyx_textwidth(c_str.as_ptr())
        }
    }

    /// 获取单个字符的宽度。
    ///
    /// 计算指定字符在当前字体下的宽度。
    ///
    /// # 参数
    /// * `c` - 要计算宽度的字符。
    ///
    /// # 返回值
    /// 字符的宽度，以像素为单位。
    pub fn text_width_char(&self, c: char) -> i32 {
        unsafe { easyx_textwidth_char(c as i8) }
    }

    /// 获取文本高度。
    ///
    /// 计算指定文本在当前字体下的高度。
    ///
    /// # 参数
    /// * `text` - 要计算高度的文本。
    ///
    /// # 返回值
    /// 文本的高度，以像素为单位。
    pub fn text_height(&self, text: &str) -> i32 {
        use std::ffi::CString;
        unsafe {
            let c_str = CString::new(text).expect("Failed to create C string");
            easyx_textheight(c_str.as_ptr())
        }
    }

    /// 获取单个字符的高度。
    ///
    /// 计算指定字符在当前字体下的高度。
    ///
    /// # 参数
    /// * `c` - 要计算高度的字符。
    ///
    /// # 返回值
    /// 字符的高度，以像素为单位。
    pub fn text_height_char(&self, c: char) -> i32 {
        unsafe { easyx_textheight_char(c as i8) }
    }

    /// 在指定区域内以指定格式输出字符串。
    ///
    /// 在指定的矩形区域内绘制文本，并应用指定的格式。
    ///
    /// # 参数
    /// * `str` - 要绘制的文本。
    /// * `rect` - 绘制文本的矩形区域。
    /// * `format` - 文本绘制格式。
    ///
    /// # 返回值
    /// 实际绘制的文本高度，以像素为单位。
    pub fn draw_text(&self, str: &str, mut rect: RECT, format: DrawTextFormat) -> i32 {
        use std::ffi::CString;

        unsafe {
            let c_str = CString::new(str).expect("Failed to create C string");

            easyx_drawtext(c_str.as_ptr(), &mut rect as *mut _ as *mut _, format.bits())
        }
    }

    /// 在指定区域内以指定格式输出字符。
    ///
    /// 在指定的矩形区域内绘制单个字符，并应用指定的格式。
    ///
    /// # 参数
    /// * `c` - 要绘制的字符。
    /// * `rect` - 绘制字符的矩形区域。
    /// * `format` - 文本绘制格式。
    ///
    /// # 返回值
    /// 实际绘制的字符高度，以像素为单位。
    pub fn draw_text_char(&self, c: char, mut rect: RECT, format: DrawTextFormat) -> i32 {
        unsafe { easyx_drawtext_char(c as i8, &mut rect as *mut _ as *mut _, format.bits()) }
    }

    /// 设置文本样式。
    ///
    /// 使用LogFont对象设置当前图形环境的文本样式。
    ///
    /// # 参数
    /// * `text_style` - 要设置的LogFont对象，包含字体的各种属性。
    pub fn set_textstyle_font(&self, text_style: &LogFont) {
        text_style.apply();
    }

    /// 获取当前文本样式。
    ///
    /// 返回当前图形环境的文本样式设置。
    ///
    /// # 返回值
    /// 当前的文本样式对象，包含字体的各种属性。
    pub fn get_textstyle_font(&self) -> LogFont {
        LogFont::current()
    }

    /// 设置文本样式。
    ///
    /// 使用指定的高度、宽度和字体名称设置当前文本样式。
    ///
    /// # 参数
    /// * `height` - 文本高度。
    /// * `width` - 文本宽度。
    /// * `face` - 字体名称。
    pub fn set_textstyle(&self, height: i32, width: i32, face: &str) {
        use std::ffi::CString;

        unsafe {
            let c_str = CString::new(face).expect("Failed to create C string");
            easyx_settextstyle(height, width, c_str.as_ptr());
        }
    }

    /// 完整设置文本样式。
    ///
    /// 使用详细的参数设置当前文本样式，包括字体的各种属性。
    ///
    /// # 参数
    /// * `height` - 文本高度。
    /// * `width` - 文本宽度。
    /// * `face` - 字体名称。
    /// * `escapement` - 文本的倾斜角度，以十分之一度为单位。
    /// * `orientation` - 文本的方向角度，以十分之一度为单位。
    /// * `weight` - 字体的粗细，范围为0-1000。
    /// * `italic` - 是否为斜体。
    /// * `underline` - 是否有下划线。
    /// * `strikeout` - 是否有删除线。
    #[allow(clippy::too_many_arguments)]
    pub fn set_textstyle_full(
        &self,
        height: i32,
        width: i32,
        face: &str,
        escapement: i32,
        orientation: i32,
        weight: i32,
        italic: bool,
        underline: bool,
        strikeout: bool,
    ) {
        use std::ffi::CString;

        unsafe {
            let c_str = CString::new(face).expect("Failed to create C string");
            easyx_settextstyle_full(
                height,
                width,
                c_str.as_ptr(),
                escapement,
                orientation,
                weight,
                italic as i32,
                underline as i32,
                strikeout as i32,
            );
        }
    }

    /// 高级完整设置文本样式。
    ///
    /// 使用最详细的参数设置当前文本样式，包括字体的所有属性。
    ///
    /// # 参数
    /// * `height` - 文本高度。
    /// * `width` - 文本宽度。
    /// * `face` - 字体名称。
    /// * `escapement` - 文本的倾斜角度，以十分之一度为单位。
    /// * `orientation` - 文本的方向角度，以十分之一度为单位。
    /// * `weight` - 字体的粗细，范围为0-1000。
    /// * `italic` - 是否为斜体。
    /// * `underline` - 是否有下划线。
    /// * `strikeout` - 是否有删除线。
    /// * `charset` - 字符集，用于指定字体的字符集。
    /// * `out_precision` - 输出精度，指定如何选择合适的字体。
    /// * `clip_precision` - 裁剪精度，指定如何裁剪超出裁剪区域的文本。
    /// * `quality` - 输出质量，指定文本的渲染质量。
    /// * `pitch_and_family` - 间距和字体系列，指定字体的间距和字体系列。
    #[allow(clippy::too_many_arguments)]
    pub fn set_textstyle_full_ex(
        &self,
        height: i32,
        width: i32,
        face: &str,
        escapement: i32,
        orientation: i32,
        weight: i32,
        italic: bool,
        underline: bool,
        strikeout: bool,
        charset: u8,
        out_precision: u8,
        clip_precision: u8,
        quality: u8,
        pitch_and_family: u8,
    ) {
        use std::ffi::CString;

        unsafe {
            let c_str = CString::new(face).expect("Failed to create C string");
            easyx_settextstyle_full_ex(
                height,
                width,
                c_str.as_ptr(),
                escapement,
                orientation,
                weight,
                italic as i32,
                underline as i32,
                strikeout as i32,
                charset,
                out_precision,
                clip_precision,
                quality,
                pitch_and_family,
            );
        }
    }
}

impl App {
    /// 开始批处理绘图
    ///
    /// 开始批处理绘图模式，后续的绘图操作不会立即显示在屏幕上，
    /// 需要调用`flush_batch_draw`或`end_batch_draw`才能显示。
    /// 批处理绘图可以显著提高大量图形绘制的性能。
    ///
    /// # 示例
    /// ```no_run
    /// use easyx::*;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // 初始化图形窗口
    ///     run(800, 600, |app| {
    ///         // 开始批处理绘图
    ///         app.begin_batch_draw();
    ///         
    ///         // 绘制大量图形
    ///         for i in 0..1000 {
    ///             app.line(0, i, 800, i);
    ///         }
    ///         
    ///         // 刷新绘制结果
    ///         app.flush_batch_draw();
    ///         
    ///         // 结束批处理绘图
    ///         app.end_batch_draw();
    ///         
    ///         Ok(())
    ///     })
    /// }
    /// ```
    pub fn begin_batch_draw(&self) {
        unsafe {
            easyx_beginbatchdraw();
        }
    }

    /// 刷新批处理绘图
    ///
    /// 将批处理绘图的结果刷新到屏幕上
    pub fn flush_batch_draw(&self) {
        unsafe {
            easyx_flushbatchdraw();
        }
    }

    /// 刷新指定区域的批处理绘图
    ///
    /// 将指定区域内的批处理绘图结果刷新到屏幕上
    ///
    /// # 参数
    /// - `left`: 区域左上角x坐标
    /// - `top`: 区域左上角y坐标
    /// - `right`: 区域右下角x坐标
    /// - `bottom`: 区域右下角y坐标
    pub fn flush_batch_draw_rect(&self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe {
            easyx_flushbatchdraw_rect(left, top, right, bottom);
        }
    }

    /// 结束批处理绘图
    ///
    /// 结束批处理绘图模式，并将所有绘图结果刷新到屏幕上
    pub fn end_batch_draw(&self) {
        unsafe {
            easyx_endbatchdraw();
        }
    }

    /// 结束指定区域的批处理绘图
    ///
    /// 结束批处理绘图模式，并将指定区域内的绘图结果刷新到屏幕上
    ///
    /// # 参数
    /// - `left`: 区域左上角x坐标
    /// - `top`: 区域左上角y坐标
    /// - `right`: 区域右下角x坐标
    /// - `bottom`: 区域右下角y坐标
    pub fn end_batch_draw_rect(&self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe {
            easyx_endbatchdraw_rect(left, top, right, bottom);
        }
    }
}

impl App {
    /// 创建输入框
    ///
    /// 创建一个输入框，用于获取用户输入
    ///
    /// # 参数
    /// - `max`: 最大输入字符数
    /// - `prompt`: 提示文本
    ///
    /// # 返回值
    /// 输入框对象
    pub fn input_box(&self, max: i32, prompt: &str) -> InputBox {
        InputBox::new(max, prompt)
    }
}

impl App {
    /// 设置鼠标捕获
    ///
    /// 设置鼠标捕获，使当前窗口接收所有鼠标事件
    pub fn set_capture(&self) {
        unsafe {
            easyx_setcapture();
        }
    }

    /// 释放鼠标捕获
    ///
    /// 释放之前设置的鼠标捕获
    pub fn release_capture(&self) {
        unsafe {
            easyx_releasecapture();
        }
    }

    /// 获取消息（阻塞）
    ///
    /// 获取消息队列中的消息，阻塞直到有消息可用
    ///
    /// # 参数
    /// - `filter`: 消息过滤类型
    ///
    /// # 返回值
    /// 捕获到的消息
    pub fn get_message(&self, filter: MessageFilter) -> ExMessage {
        ExMessage::get_message(filter)
    }

    /// 查看消息（非阻塞）
    ///
    /// 查看消息队列中的消息，非阻塞，如果没有消息则返回None
    ///
    /// # 参数
    /// - `filter`: 消息过滤类型
    /// - `removemsg`: 是否从消息队列中移除消息
    ///
    /// # 返回值
    /// 如果有消息则返回Some(ExMessage)，否则返回None
    pub fn peek_message(&self, filter: MessageFilter, removemsg: bool) -> Option<ExMessage> {
        ExMessage::peek_message(filter, removemsg)
    }

    /// 刷新消息队列
    ///
    /// 刷新指定类型的消息队列，处理所有待处理的消息
    ///
    /// # 参数
    /// - `filter`: 消息过滤类型
    pub fn flush_messages(&self, filter: MessageFilter) {
        unsafe {
            easyx_flushmessage(filter as u8);
        }
    }
}

impl Drop for App {
    /// App实例销毁时自动关闭图形窗口
    ///
    /// 当App实例被销毁时，会自动调用此方法关闭图形窗口，
    /// 确保资源正确释放。
    fn drop(&mut self) {
        unsafe {
            easyx_closegraph();
        }
    }
}
