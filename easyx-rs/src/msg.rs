use easyx_sys::*;

use crate::keycode::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Message {
    Mouse {
        ctrl: bool,
        shift: bool,
        lbutton: bool,
        mbutton: bool,
        rbutton: bool,
        x: u16,
        y: u16,
        wheel: u16,
    },
    KeyBoard {
        vkcode: KeyCode,
        scancode: u8,
        extended: bool,
        prevdown: bool,
    },
    Char(TCHAR),
    Window {
        wparam: WPARAM,
        lparam: LPARAM,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExMessageType {
    KeyDown = WM_KEYDOWN as isize,
    KeyUp = WM_KEYUP as isize,
    Char = WM_CHAR as isize,
    Activate = WM_ACTIVATE as isize,
    Move = WM_MOVE as isize,
    Size = WM_SIZE as isize,
    MouseMove = WM_MOUSEMOVE as isize,
    MouseWheel = WM_MOUSEWHEEL as isize,
    LButtonDown = WM_LBUTTONDOWN as isize,
    LButtonUp = WM_LBUTTONUP as isize,
    LButtonDBLck = WM_LBUTTONDBLCLK as isize,
    MButtonDown = WM_MBUTTONDOWN as isize,
    MButtonUp = WM_MBUTTONUP as isize,
    MButtonDBLck = WM_MBUTTONDBLCLK as isize,
    RButtonDown = WM_RBUTTONDOWN as isize,
    RButtonUp = WM_RBUTTONUP as isize,
    RButtonDBLck = WM_RBUTTONDBLCLK as isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageFilter {
    All = -1,
    Mouse = EASYX_EX_MOUSE as isize,
    KeyBoard = EASYX_EX_KEY as isize,
    Char = EASYX_EX_CHAR as isize,
    Window = EASYX_EX_WINDOW as isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExMessage {
    pub ty: ExMessageType,
    pub msg: Message,
}

impl ExMessage {
    /// 从C++ CExMessage转换为Rust ExMessage
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
    /// filter: 指定要获取的消息范围，默认 -1 获取所有类别的消息。
    pub fn get_message(filter: MessageFilter) -> Self {
        let mut c_msg = unsafe { std::mem::zeroed::<CExMessage>() };

        unsafe {
            easyx_getmessage(&mut c_msg, filter as u8);
        }

        Self::from_c_message(&c_msg)
    }

    /// 获取一个消息。如果当前消息队列中没有，就立即返回。
    /// filter: 指定要获取的消息范围，默认 -1 获取所有类别的消息。
    /// removemsg: 是否从消息队列中移除获取到的消息，true 表示移除，false 表示不移除。
    /// 返回值: 如果获取到消息，返回 Some(ExMessage)，否则返回 None。
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
