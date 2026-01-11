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
    /// Window initialization flags for EasyX graphics library.
    ///
    /// These flags can be combined using the `|` operator to set multiple options.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct InitFlags: u32 {
        /// No flags set.
        const None = 0;
        /// Shows the console window.
        const ShowConsole = EASYX_EX_SHOWCONSOLE;
        /// Disables the close button.
        const NoClose = EASYX_EX_NOCLOSE;
        /// Disables the minimize button.
        const NoMinimize = EASYX_EX_NOMINIMIZE;
        /// Enables double-click events.
        const DblClks = EASYX_EX_DBLCLKS;
    }
}

pub struct App {
    width: i32,
    height: i32,
    hwnd: HWND,
}

impl App {
    /// Creates a new EasyX graphics application instance.
    ///
    /// # Parameters
    ///
    /// * `width` - The width of the graphics window.
    /// * `height` - The height of the graphics window.
    /// * `flags` - The initialization flags for the graphics window.
    ///
    /// # Returns
    ///
    /// A new `App` instance.
    pub fn new(width: i32, height: i32, flags: InitFlags) -> Self {
        let hwnd = unsafe { easyx_initgraph(width, height, flags.bits() as i32) };

        Self {
            width,
            height,
            hwnd: hwnd as HWND,
        }
    }

    /// Initializes the graphics window and runs the provided closure.
    ///
    /// This method initializes the graphics window, runs the provided closure,
    /// and ensures that the window is properly closed when done.
    /// The closure is passed a reference to the App instance.
    ///
    /// # Parameters
    ///
    /// * `f` - A closure that will be executed with the graphics context.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
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

    /// Gets the width of the graphics window.
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Gets the height of the graphics window.
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Gets the initialization flags of the graphics window.
    pub fn hwnd(&self) -> &HWND {
        &self.hwnd
    }

    pub fn graphics_hwnd(&self) -> HWND {
        unsafe { easyx_gethwnd() as _ }
    }

    pub fn version(&self) -> &'static str {
        let ver = unsafe { easyx_geteasyxver() };

        unsafe { std::ffi::CStr::from_ptr(ver).to_str().unwrap() }
    }
}

impl App {
    pub fn set_origin(&self, x: i32, y: i32) {
        unsafe {
            easyx_setorigin(x, y);
        }
    }

    pub fn set_cliprgn(&self, x: i32, y: i32, width: i32, height: i32) {
        let rgn = unsafe { CreateRectRgn(x, y, x + width, y + height) };

        unsafe {
            easyx_setcliprgn(rgn as _);
        }
    }

    pub fn reset_cliprgn(&self) {
        unsafe {
            easyx_clearcliprgn();
        }
    }

    pub fn set_aspectratio(&self, width: f32, height: f32) {
        unsafe {
            easyx_setaspectratio(width, height);
        }
    }

    pub fn get_aspectratio(&self) -> (f32, f32) {
        let mut width = 0.0;
        let mut height = 0.0;

        unsafe {
            easyx_getaspectratio(&mut width, &mut height);
        }
        (width, height)
    }

    pub fn reset_graph_defaults(&self) {
        unsafe {
            easyx_graphdefaults();
        }
    }

    /// Clears the device screen.
    pub fn clear_device(&self) {
        unsafe {
            easyx_cleardevice();
        }
    }
}

impl App {
    pub fn get_linestyle(&self) -> LineStyle {
        LineStyle::current()
    }

    pub fn set_linestyle(&self, linestyle: &LineStyle) {
        linestyle.apply();
    }

    pub fn get_fillstyle(&self) -> FillStyle {
        FillStyle::current()
    }

    pub fn set_fillstyle(&self, fillstyle: &FillStyle) {
        fillstyle.apply();
    }
}

impl App {
    pub fn get_linecolor(&self) -> Color {
        Color::get_linecolor()
    }

    pub fn set_linecolor(&self, color: &Color) {
        color.set_linecolor();
    }

    pub fn get_textcolor(&self) -> Color {
        Color::get_textcolor()
    }

    pub fn set_textcolor(&self, color: &Color) {
        color.set_textcolor();
    }

    pub fn get_fillcolor(&self) -> Color {
        Color::get_fillcolor()
    }

    pub fn set_fillcolor(&self, color: &Color) {
        color.set_fillcolor();
    }

    pub fn get_bkmode(&self) -> BkMode {
        BkMode::current()
    }

    pub fn set_bkmode(&self, bkmode: &BkMode) {
        bkmode.apply();
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Color {
        Color::get_pixel(x, y)
    }

    pub fn put_pixel(&self, x: i32, y: i32, color: &Color) {
        color.put_pixel(x, y);
    }
}

impl App {
    pub fn line(&self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe {
            easyx_line(left, top, right, bottom);
        }
    }

    pub fn rectangle(&self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe {
            easyx_rectangle(left, top, right, bottom);
        }
    }

    pub fn fill_rectangle(&self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe {
            easyx_fillrectangle(left, top, right, bottom);
        }
    }

    pub fn solid_rectangle(&self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe {
            easyx_solidrectangle(left, top, right, bottom);
        }
    }

    pub fn clear_rectangle(&self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe {
            easyx_clearrectangle(left, top, right, bottom);
        }
    }

    pub fn circle(&self, x: i32, y: i32, radius: i32) {
        unsafe {
            easyx_circle(x, y, radius);
        }
    }

    pub fn fill_circle(&self, x: i32, y: i32, radius: i32) {
        unsafe {
            easyx_fillcircle(x, y, radius);
        }
    }

    pub fn solid_circle(&self, x: i32, y: i32, radius: i32) {
        unsafe {
            easyx_solidcircle(x, y, radius);
        }
    }

    pub fn clear_circle(&self, x: i32, y: i32, radius: i32) {
        unsafe {
            easyx_clearcircle(x, y, radius);
        }
    }

    pub fn ellipse(&self, x: i32, y: i32, rx: i32, ry: i32) {
        unsafe {
            easyx_ellipse(x, y, rx, ry);
        }
    }

    pub fn fill_ellipse(&self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe {
            easyx_fillellipse(left, top, right, bottom);
        }
    }

    pub fn solid_ellipse(&self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe {
            easyx_solidellipse(left, top, right, bottom);
        }
    }

    pub fn clear_ellipse(&self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe {
            easyx_clearellipse(left, top, right, bottom);
        }
    }

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

    pub fn arc(&self, left: i32, top: i32, right: i32, bottom: i32, stange: f64, endangle: f64) {
        unsafe {
            easyx_arc(left, top, right, bottom, stange, endangle);
        }
    }

    pub fn pie(&self, left: i32, top: i32, right: i32, bottom: i32, stange: f64, endangle: f64) {
        unsafe {
            easyx_pie(left, top, right, bottom, stange, endangle);
        }
    }

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

    pub fn poly_line(&self, points: &[POINT]) {
        unsafe {
            easyx_polyline(points.as_ptr() as _, points.len() as i32);
        }
    }

    pub fn poly_gon(&self, points: &[POINT]) {
        unsafe {
            easyx_polygon(points.as_ptr() as _, points.len() as i32);
        }
    }

    pub fn fill_polygon(&self, points: &[POINT]) {
        unsafe {
            easyx_fillpolygon(points.as_ptr() as _, points.len() as i32);
        }
    }

    pub fn solid_polygon(&self, points: &[POINT]) {
        unsafe {
            easyx_solidpolygon(points.as_ptr() as _, points.len() as i32);
        }
    }

    pub fn clear_polygon(&self, points: &[POINT]) {
        unsafe {
            easyx_clearpolygon(points.as_ptr() as _, points.len() as i32);
        }
    }

    pub fn poly_bezier(&self, points: &[POINT]) {
        unsafe {
            easyx_polybezier(points.as_ptr() as _, points.len() as i32);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloodFillType {
    Border = FLOODFILLBORDER as isize,

    Surface = FLOODFILLSURFACE as isize,
}

impl App {
    pub fn flood_fill(&self, x: i32, y: i32, color: Color, fill_type: FloodFillType) {
        unsafe {
            easyx_floodfill(x, y, color.value, fill_type as i32);
        }
    }
}

impl App {
    pub fn out_text(&self, x: i32, y: i32, text: &str) {
        use std::ffi::CString;
        unsafe {
            let c_str = CString::new(text).expect("Failed to create C string");
            easyx_outtextxy(x, y, c_str.as_ptr());
        }
    }

    pub fn out_text_char(&self, x: i32, y: i32, c: char) {
        unsafe {
            easyx_outtextxy_char(x, y, c as i8);
        }
    }

    pub fn text_width(&self, text: &str) -> i32 {
        use std::ffi::CString;
        unsafe {
            let c_str = CString::new(text).expect("Failed to create C string");
            easyx_textwidth(c_str.as_ptr())
        }
    }

    pub fn text_width_char(&self, c: char) -> i32 {
        unsafe { easyx_textwidth_char(c as i8) }
    }

    pub fn text_height(&self, text: &str) -> i32 {
        use std::ffi::CString;
        unsafe {
            let c_str = CString::new(text).expect("Failed to create C string");
            easyx_textheight(c_str.as_ptr())
        }
    }

    pub fn text_height_char(&self, c: char) -> i32 {
        unsafe { easyx_textheight_char(c as i8) }
    }

    /// 在指定区域内以指定格式输出字符串
    pub fn draw_text(&self, str: &str, mut rect: RECT, format: DrawTextFormat) -> i32 {
        use std::ffi::CString;

        unsafe {
            let c_str = CString::new(str).expect("Failed to create C string");

            easyx_drawtext(c_str.as_ptr(), &mut rect as *mut _ as *mut _, format.bits())
        }
    }

    /// 在指定区域内以指定格式输出字符
    pub fn draw_text_char(&self, c: char, mut rect: RECT, format: DrawTextFormat) -> i32 {
        unsafe { easyx_drawtext_char(c as i8, &mut rect as *mut _ as *mut _, format.bits()) }
    }

    /// 设置文本样式
    pub fn set_textstyle_font(&self, text_style: &LogFont) {
        text_style.apply();
    }

    /// 获取当前文本样式
    pub fn get_textstyle_font(&self) -> LogFont {
        LogFont::current()
    }

    pub fn set_textstyle(&self, height: i32, width: i32, face: &str) {
        use std::ffi::CString;

        unsafe {
            let c_str = CString::new(face).expect("Failed to create C string");
            easyx_settextstyle(height, width, c_str.as_ptr());
        }
    }

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
    /// // 开始批处理绘图
    /// app.begin_batch_draw();
    /// 
    /// // 绘制大量图形
    /// for i in 0..1000 {
    ///     app.line(0, i, 800, i);
    /// }
    /// 
    /// // 刷新绘制结果
    /// app.flush_batch_draw();
    /// 
    /// // 结束批处理绘图
    /// app.end_batch_draw();
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
