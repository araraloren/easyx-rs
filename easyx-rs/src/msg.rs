//! 消息处理相关定义

use easyx_sys::*;

use crate::keycode::KeyCode;

/// 表示 EasyX 图形库中的各种消息类型
///
/// 该枚举包含了 EasyX 图形库中可能产生的所有消息类型，
/// 包括鼠标消息、键盘消息、字符消息和窗口消息。
///
/// # 变体说明
/// - `Mouse`: 鼠标消息，包含鼠标位置、按键状态和滚轮信息
/// - `KeyBoard`: 键盘消息，包含虚拟按键码、扫描码和按键状态
/// - `Char`: 字符消息，包含输入的字符
/// - `Window`: 窗口消息，包含原始的 Windows 消息参数
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Message {
    /// 鼠标消息
    ///
    /// 包含鼠标位置、按键状态和滚轮信息
    Mouse {
        /// Ctrl 键是否按下
        ctrl: bool,
        /// Shift 键是否按下
        shift: bool,
        /// 左键是否按下
        lbutton: bool,
        /// 中键是否按下
        mbutton: bool,
        /// 右键是否按下
        rbutton: bool,
        /// 鼠标 X 坐标
        x: u16,
        /// 鼠标 Y 坐标
        y: u16,
        /// 鼠标滚轮值
        wheel: u16,
    },
    /// 键盘消息
    ///
    /// 包含虚拟按键码、扫描码和按键状态
    KeyBoard {
        /// 虚拟按键码
        vkcode: KeyCode,
        /// 扫描码
        scancode: u8,
        /// 是否为扩展键
        extended: bool,
        /// 之前是否按下
        prevdown: bool,
    },
    /// 字符消息
    ///
    /// 包含输入的字符
    Char(TCHAR),
    /// 窗口消息
    ///
    /// 包含原始的 Windows 消息参数
    Window {
        /// Windows 消息的 wParam 参数
        wparam: WPARAM,
        /// Windows 消息的 lParam 参数
        lparam: LPARAM,
    },
}

/// 扩展消息类型枚举
///
/// 表示 EasyX 图形库中可能产生的各种扩展消息类型，
/// 基于 Windows API 的消息类型。
///
/// # 变体说明
/// - `KeyDown`: 按键按下消息
/// - `KeyUp`: 按键释放消息
/// - `Char`: 字符输入消息
/// - `Activate`: 窗口激活消息
/// - `Move`: 窗口移动消息
/// - `Size`: 窗口大小改变消息
/// - `MouseMove`: 鼠标移动消息
/// - `MouseWheel`: 鼠标滚轮消息
/// - `LButtonDown`: 鼠标左键按下消息
/// - `LButtonUp`: 鼠标左键释放消息
/// - `LButtonDBLck`: 鼠标左键双击消息
/// - `MButtonDown`: 鼠标中键按下消息
/// - `MButtonUp`: 鼠标中键释放消息
/// - `MButtonDBLck`: 鼠标中键双击消息
/// - `RButtonDown`: 鼠标右键按下消息
/// - `RButtonUp`: 鼠标右键释放消息
/// - `RButtonDBLck`: 鼠标右键双击消息
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExMessageType {
    /// 按键按下消息
    KeyDown = WM_KEYDOWN as isize,
    /// 按键释放消息
    KeyUp = WM_KEYUP as isize,
    /// 字符输入消息
    Char = WM_CHAR as isize,
    /// 窗口激活消息
    Activate = WM_ACTIVATE as isize,
    /// 窗口移动消息
    Move = WM_MOVE as isize,
    /// 窗口大小改变消息
    Size = WM_SIZE as isize,
    /// 鼠标移动消息
    MouseMove = WM_MOUSEMOVE as isize,
    /// 鼠标滚轮消息
    MouseWheel = WM_MOUSEWHEEL as isize,
    /// 鼠标左键按下消息
    LButtonDown = WM_LBUTTONDOWN as isize,
    /// 鼠标左键释放消息
    LButtonUp = WM_LBUTTONUP as isize,
    /// 鼠标左键双击消息
    LButtonDBLck = WM_LBUTTONDBLCLK as isize,
    /// 鼠标中键按下消息
    MButtonDown = WM_MBUTTONDOWN as isize,
    /// 鼠标中键释放消息
    MButtonUp = WM_MBUTTONUP as isize,
    /// 鼠标中键双击消息
    MButtonDBLck = WM_MBUTTONDBLCLK as isize,
    /// 鼠标右键按下消息
    RButtonDown = WM_RBUTTONDOWN as isize,
    /// 鼠标右键释放消息
    RButtonUp = WM_RBUTTONUP as isize,
    /// 鼠标右键双击消息
    RButtonDBLck = WM_RBUTTONDBLCLK as isize,
}

/// 消息过滤器枚举
///
/// 用于指定要获取的消息类型范围。
///
/// # 变体说明
/// - `All`: 获取所有类型的消息
/// - `Mouse`: 只获取鼠标消息
/// - `KeyBoard`: 只获取键盘消息
/// - `Char`: 只获取字符消息
/// - `Window`: 只获取窗口消息
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageFilter {
    /// 获取所有类型的消息
    All = -1,
    /// 只获取鼠标消息
    Mouse = EASYX_EX_MOUSE as isize,
    /// 只获取键盘消息
    KeyBoard = EASYX_EX_KEY as isize,
    /// 只获取字符消息
    Char = EASYX_EX_CHAR as isize,
    /// 只获取窗口消息
    Window = EASYX_EX_WINDOW as isize,
}

/// 扩展消息结构体
///
/// 包含消息类型和具体的消息内容，用于 EasyX 图形库中的消息处理。
///
/// # 字段说明
/// - `ty`: 消息类型
/// - `msg`: 具体的消息内容
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExMessage {
    /// 消息类型
    pub ty: ExMessageType,
    /// 具体的消息内容
    pub msg: Message,
}

impl ExMessage {
    /// 从 C++ CExMessage 结构体转换为 Rust ExMessage
    ///
    /// 将 EasyX 内部使用的 C++ 消息结构体转换为 Rust 消息结构体。
    ///
    /// # 参数
    /// - `c_msg`: C++ CExMessage 结构体引用
    ///
    /// # 返回值
    /// 返回转换后的 Rust ExMessage 实例
    pub fn from_c_message(c_msg: &CExMessage) -> Self {
        let ty = match c_msg.message as u32 {
            WM_KEYDOWN => ExMessageType::KeyDown,
            WM_KEYUP => ExMessageType::KeyUp,
            WM_CHAR => ExMessageType::Char,
            WM_ACTIVATE => ExMessageType::Activate,
            WM_MOVE => ExMessageType::Move,
            WM_SIZE => ExMessageType::Size,
            WM_MOUSEMOVE => ExMessageType::MouseMove,
            WM_MOUSEWHEEL => ExMessageType::MouseWheel,
            WM_LBUTTONDOWN => ExMessageType::LButtonDown,
            WM_LBUTTONUP => ExMessageType::LButtonUp,
            WM_LBUTTONDBLCLK => ExMessageType::LButtonDBLck,
            WM_MBUTTONDOWN => ExMessageType::MButtonDown,
            WM_MBUTTONUP => ExMessageType::MButtonUp,
            WM_MBUTTONDBLCLK => ExMessageType::MButtonDBLck,
            WM_RBUTTONDOWN => ExMessageType::RButtonDown,
            WM_RBUTTONUP => ExMessageType::RButtonUp,
            WM_RBUTTONDBLCLK => ExMessageType::RButtonDBLck,
            _ => panic!("Unknown message type: {}", c_msg.message),
        };

        // 注意：CExMessage结构体中的位域字段被转换为了方法，需要调用方法来访问
        let msg = unsafe {
            match ty {
                ExMessageType::MouseMove
                | ExMessageType::MouseWheel
                | ExMessageType::LButtonDown
                | ExMessageType::LButtonUp
                | ExMessageType::LButtonDBLck
                | ExMessageType::MButtonDown
                | ExMessageType::MButtonUp
                | ExMessageType::MButtonDBLck
                | ExMessageType::RButtonDown
                | ExMessageType::RButtonUp
                | ExMessageType::RButtonDBLck => Message::Mouse {
                    ctrl: c_msg.__bindgen_anon_1.__bindgen_anon_1.ctrl() != 0,
                    shift: c_msg.__bindgen_anon_1.__bindgen_anon_1.shift() != 0,
                    lbutton: c_msg.__bindgen_anon_1.__bindgen_anon_1.lbutton() != 0,
                    mbutton: c_msg.__bindgen_anon_1.__bindgen_anon_1.mbutton() != 0,
                    rbutton: c_msg.__bindgen_anon_1.__bindgen_anon_1.rbutton() != 0,
                    x: c_msg.__bindgen_anon_1.__bindgen_anon_1.x as u16,
                    y: c_msg.__bindgen_anon_1.__bindgen_anon_1.y as u16,
                    wheel: c_msg.__bindgen_anon_1.__bindgen_anon_1.wheel as u16,
                },
                ExMessageType::KeyDown | ExMessageType::KeyUp => Message::KeyBoard {
                    vkcode: KeyCode::from(c_msg.__bindgen_anon_1.__bindgen_anon_2.vkcode),
                    scancode: c_msg.__bindgen_anon_1.__bindgen_anon_2.scancode,
                    extended: c_msg.__bindgen_anon_1.__bindgen_anon_2.extended() != 0,
                    prevdown: c_msg.__bindgen_anon_1.__bindgen_anon_2.prevdown() != 0,
                },
                ExMessageType::Char => Message::Char(c_msg.__bindgen_anon_1.ch),
                ExMessageType::Activate | ExMessageType::Move | ExMessageType::Size => {
                    Message::Window {
                        wparam: c_msg.__bindgen_anon_1.__bindgen_anon_3.wParam,
                        lparam: c_msg.__bindgen_anon_1.__bindgen_anon_3.lParam,
                    }
                }
            }
        };

        ExMessage { ty, msg }
    }
}

impl ExMessage {
    /// 获取一个消息。如果当前消息队列中没有，就一直等待。
    ///
    /// 从消息队列中获取一个消息，如果当前没有消息，就一直等待直到有消息为止。
    ///
    /// # 参数
    /// - `filter`: 指定要获取的消息范围
    ///
    /// # 返回值
    /// 返回获取到的消息
    ///
    /// # 示例
    /// ```no_run
    /// use easyx::msg::{ExMessage, MessageFilter};
    ///
    /// // 循环获取并处理消息
    /// loop {
    ///     let msg = ExMessage::get_message(MessageFilter::All);
    ///     
    ///     // 处理消息
    ///     match msg.ty {
    ///         // 处理鼠标左键按下事件
    ///         _ => {}
    ///     }
    /// }
    /// ```
    pub fn get_message(filter: MessageFilter) -> Self {
        let mut c_msg = unsafe { std::mem::zeroed::<CExMessage>() };

        unsafe {
            easyx_getmessage(&mut c_msg, filter as u8);
        }

        Self::from_c_message(&c_msg)
    }

    /// 获取一个消息。如果当前消息队列中没有，就立即返回。
    ///
    /// 从消息队列中获取一个消息，如果当前没有消息，就立即返回 None。
    ///
    /// # 参数
    /// - `filter`: 指定要获取的消息范围
    /// - `removemsg`: 是否从消息队列中移除获取到的消息
    ///
    /// # 返回值
    /// 如果获取到消息，返回 Some(ExMessage)，否则返回 None
    ///
    /// # 示例
    /// ```no_run
    /// use easyx::msg::{ExMessage, MessageFilter};
    ///
    /// // 非阻塞方式获取消息
    /// if let Some(msg) = ExMessage::peek_message(MessageFilter::All, true) {
    ///     // 处理消息
    ///     println!("获取到消息: {:?}", msg);
    /// }
    /// ```
    pub fn peek_message(filter: MessageFilter, removemsg: bool) -> Option<Self> {
        let mut c_msg = unsafe { std::mem::zeroed::<CExMessage>() };

        let result = unsafe { easyx_peekmessage(&mut c_msg, filter as u8, removemsg as i32) };

        if result != 0 {
            Some(Self::from_c_message(&c_msg))
        } else {
            None
        }
    }
}
