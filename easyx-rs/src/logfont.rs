//! 文本字体样式定义

use easyx_sys::{easyx_gettextstyle, easyx_settextstyle_logfont};

/// Windows API LOGFONT 结构体的 Rust 包装
///
/// 该结构体用于定义文本的字体样式，包括字体名称、大小、粗细、倾斜度等属性。
/// 它是 Windows API 中 LOGFONT 结构体的直接映射，用于 EasyX 图形库中的文本绘制。
///
/// # 字段说明
/// - `logfont`: 内部的 Windows API LOGFONT 结构体，包含了字体的各种属性
///
/// # 示例
/// ```no_run
/// use easyx::logfont::LogFont;
/// 
/// // 获取当前字体样式
/// let current_font = LogFont::current();
/// 
/// // 应用字体样式（假设已修改）
/// // current_font.apply();
/// ```
#[repr(C)]
pub struct LogFont {
    /// 内部的 Windows API LOGFONT 结构体
    ///
    /// 这个字段包含了字体的完整属性，包括：
    /// - `lfHeight`: 字体高度（逻辑单位）
    /// - `lfWidth`: 字体宽度（逻辑单位）
    /// - `lfEscapement`: 文本的书写角度（0-3600，每10度为一个单位）
    /// - `lfOrientation`: 每个字符的旋转角度（0-3600，每10度为一个单位）
    /// - `lfWeight`: 字体粗细（0-1000，400为正常，700为粗体）
    /// - `lfItalic`: 是否斜体（0为否，非0为是）
    /// - `lfUnderline`: 是否下划线（0为否，非0为是）
    /// - `lfStrikeOut`: 是否删除线（0为否，非0为是）
    /// - `lfCharSet`: 字符集
    /// - `lfOutPrecision`: 输出精度
    /// - `lfClipPrecision`: 裁剪精度
    /// - `lfQuality`: 输出质量
    /// - `lfPitchAndFamily`: 字体间距和族
    /// - `lfFaceName`: 字体名称（最大32个字符）
    pub(crate) logfont: easyx_sys::LOGFONT,
}

impl LogFont {
    /// 将当前字体样式应用到 EasyX 绘图上下文
    ///
    /// 将 LogFont 中定义的字体样式设置为 EasyX 图形库的活动字体样式，
    /// 后续绘制的所有文本都将使用此样式。
    ///
    /// # 示例
    /// ```no_run
    /// use easyx::logfont::LogFont;
    /// 
    /// let font = LogFont::current();
    /// 
    /// // 应用字体样式
    /// font.apply();
    /// ```
    pub fn apply(&self) {
        unsafe {
            easyx_settextstyle_logfont(&self.logfont as *const _ as *mut ::std::os::raw::c_void);
        }
    }

    /// 获取当前 EasyX 绘图上下文中使用的字体样式
    ///
    /// 从 EasyX 图形库获取当前活动的字体样式，并转换为 `LogFont` 实例。
    ///
    /// # 返回值
    /// 返回当前活动的字体样式
    ///
    /// # 示例
    /// ```no_run
    /// use easyx::logfont::LogFont;
    /// 
    /// // 获取当前字体样式
    /// let current_font = LogFont::current();
    /// 
    /// // 应用当前字体样式
    /// current_font.apply();
    /// ```
    pub fn current() -> Self {
        unsafe {
            let mut text_style = Self {
                logfont: std::mem::zeroed(),
            };

            easyx_gettextstyle(&mut text_style.logfont as *mut _ as *mut ::std::os::raw::c_void);

            text_style
        }
    }
}
