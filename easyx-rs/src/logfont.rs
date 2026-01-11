use easyx_sys::{easyx_gettextstyle, easyx_settextstyle_logfont};

/// Windows API LOGFONT 结构体的包装
#[repr(C)]
pub struct LogFont {
    /// 内部 LOGFONT 结构体
    pub(crate) logfont: easyx_sys::LOGFONT,
}

impl LogFont {
    /// 将 TextStyle 应用到当前绘图上下文
    pub fn apply(&self) {
        unsafe {
            easyx_settextstyle_logfont(&self.logfont as *const _ as *mut ::std::os::raw::c_void);
        }
    }

    /// 获取当前的文本样式
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
