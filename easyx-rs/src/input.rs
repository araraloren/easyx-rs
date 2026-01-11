use easyx_sys::*;
use std::ffi::{CStr, CString};

/// InputBox参数结构体，用于构造输入框的各种参数
pub struct InputBox {
    max_count: i32,
    prompt: Option<CString>,
    title: Option<CString>,
    default_text: Option<CString>,
    width: i32,
    height: i32,
    only_ok: bool,
}

impl InputBox {
    /// 创建InputBox参数实例，至少需要两个参数：max_count和prompt
    pub fn new(max_count: i32, prompt: &str) -> Self {
        Self {
            max_count,
            prompt: Some(CString::new(prompt).expect("Invalid C string")),
            title: None,
            default_text: None,
            width: 0,
            height: 0,
            only_ok: true,
        }
    }

    /// 设置输入框标题
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(CString::new(title).expect("Invalid C string"));
        self
    }

    /// 设置输入框默认文本
    pub fn with_default_text(mut self, default_text: &str) -> Self {
        self.default_text = Some(CString::new(default_text).expect("Invalid C string"));
        self
    }

    /// 设置输入框宽度
    pub fn with_width(mut self, width: i32) -> Self {
        self.width = width;
        self
    }

    /// 设置输入框高度
    pub fn with_height(mut self, height: i32) -> Self {
        self.height = height;
        self
    }

    /// 设置是否只显示OK按钮
    pub fn with_only_ok(mut self, only_ok: bool) -> Self {
        self.only_ok = only_ok;
        self
    }

    /// 调用底层sys函数，显示输入框并获取结果
    pub fn show(&self) -> Option<String> {
        // 创建足够大的缓冲区来存储输入结果
        let mut buffer = vec![0u8; self.max_count as usize];

        // 转换各参数为C类型
        let prompt_ptr = self
            .prompt
            .as_ref()
            .map_or(std::ptr::null(), |s| s.as_ptr());
        let title_ptr = self.title.as_ref().map_or(std::ptr::null(), |s| s.as_ptr());
        let default_ptr = self
            .default_text
            .as_ref()
            .map_or(std::ptr::null(), |s| s.as_ptr());

        // 调用底层C函数
        let result = unsafe {
            easyx_inputbox(
                buffer.as_mut_ptr() as *mut _,
                self.max_count,
                prompt_ptr,
                title_ptr,
                default_ptr,
                self.width,
                self.height,
                self.only_ok as i32,
            )
        };

        // 根据返回值判断是否成功
        if result != 0 {
            // 将缓冲区转换为Rust字符串，去除末尾的空字符
            let val = CStr::from_bytes_until_nul(&buffer).expect("Invalid C string");
            let s = val.to_string_lossy().to_string();
            Some(s)
        } else {
            None
        }
    }
}
