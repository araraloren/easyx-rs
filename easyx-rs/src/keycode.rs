//! 按键代码定义

/// 表示键盘和鼠标按键的代码枚举
///
/// 该枚举包含了所有常见的键盘按键、鼠标按键和游戏手柄按键的代码值，
/// 用于 EasyX 图形库中的输入处理。按键代码基于 Windows API 的虚拟按键码 (VK_*)。
///
/// # 按键分类
/// - **鼠标按键**: LButton, RButton, MButton, XButton1, XButton2
/// - **基本按键**: Back, Tab, Return, Escape, Space 等
/// - **方向键**: Up, Down, Left, Right, Home, End, Prior, Next
/// - **功能键**: F1-F24
/// - **字母数字键**: A-Z, D0-D9
/// - **数字小键盘**: NumPad0-NumPad9, Multiply, Add, Subtract, Decimal, Divide
/// - **修饰键**: Shift, Control, Menu (Alt), LShift, RShift, LControl, RControl, LMenu, RMenu
/// - **Windows 键**: LWin, RWin, Apps, Sleep
/// - **媒体控制键**: VolumeMute, VolumeDown, VolumeUp, MediaNextTrack, MediaPrevTrack, MediaPlayPause, MediaStop
/// - **浏览器控制键**: BrowserBack, BrowserForward, BrowserRefresh, BrowserStop, BrowserSearch, BrowserFavorites, BrowserHome
/// - **游戏手柄键**: GamepadA, GamepadB, GamepadX, GamepadY, GamepadDpadUp-Down-Left-Right 等
/// - **其他按键**: 各种 OEM 按键和特殊功能按键
///
/// # 示例
/// ```rust
/// use easyx::keycode::KeyCode;
///
/// // 检查按键是否为方向键
/// fn is_direction_key(key: KeyCode) -> bool {
///     matches!(key, KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right)
/// }
///
/// // 检查按键是否为字母键
/// fn is_letter_key(key: KeyCode) -> bool {
///     matches!(key, 
///         KeyCode::A | KeyCode::B | KeyCode::C | KeyCode::D | KeyCode::E | 
///         KeyCode::F | KeyCode::G | KeyCode::H | KeyCode::I | KeyCode::J | 
///         KeyCode::K | KeyCode::L | KeyCode::M | KeyCode::N | KeyCode::O | 
///         KeyCode::P | KeyCode::Q | KeyCode::R | KeyCode::S | KeyCode::T | 
///         KeyCode::U | KeyCode::V | KeyCode::W | KeyCode::X | KeyCode::Y | KeyCode::Z
///     )
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum KeyCode {
    /// 鼠标左键
    LButton = 0x01,
    /// 鼠标右键
    RButton = 0x02,
    /// Break 键
    Cancel = 0x03,
    /// 鼠标中键
    MButton = 0x04,
    /// 鼠标 X1 键（通常为后退键）
    XButton1 = 0x05,
    /// 鼠标 X2 键（通常为前进键）
    XButton2 = 0x06,
    /// Backspace 键
    Back = 0x08,
    /// Tab 键
    Tab = 0x09,
    /// Clear 键
    Clear = 0x0C,
    /// Enter 键
    Return = 0x0D,
    /// Shift 键（左或右）
    Shift = 0x10,
    /// Control 键（左或右）
    Control = 0x11,
    /// Menu 键（Alt 键）
    Menu = 0x12,
    /// Pause 键
    Pause = 0x13,
    /// Caps Lock 键
    Capital = 0x14,
    /// Kana 键
    Kana = 0x15,
    /// IME On 键
    ImeOn = 0x16,
    /// Junja 键
    Junja = 0x17,
    /// Final 键
    Final = 0x18,
    /// Hanja 键
    Hanja = 0x19,
    /// IME Off 键
    ImeOff = 0x1A,
    /// Escape 键
    Escape = 0x1B,
    /// Convert 键
    Convert = 0x1C,
    /// NonConvert 键
    NonConvert = 0x1D,
    /// Accept 键
    Accept = 0x1E,
    /// ModeChange 键
    ModeChange = 0x1F,
    /// Space 键
    Space = 0x20,
    /// Page Up 键
    Prior = 0x21,
    /// Page Down 键
    Next = 0x22,
    /// End 键
    End = 0x23,
    /// Home 键
    Home = 0x24,
    /// 左箭头键
    Left = 0x25,
    /// 上箭头键
    Up = 0x26,
    /// 右箭头键
    Right = 0x27,
    /// 下箭头键
    Down = 0x28,
    /// Select 键
    Select = 0x29,
    /// Print 键
    Print = 0x2A,
    /// Execute 键
    Execute = 0x2B,
    /// Print Screen 键
    Snapshot = 0x2C,
    /// Insert 键
    Insert = 0x2D,
    /// Delete 键
    Delete = 0x2E,
    /// Help 键
    Help = 0x2F,
    /// 数字键 0
    D0 = 0x30,
    /// 数字键 1
    D1 = 0x31,
    /// 数字键 2
    D2 = 0x32,
    /// 数字键 3
    D3 = 0x33,
    /// 数字键 4
    D4 = 0x34,
    /// 数字键 5
    D5 = 0x35,
    /// 数字键 6
    D6 = 0x36,
    /// 数字键 7
    D7 = 0x37,
    /// 数字键 8
    D8 = 0x38,
    /// 数字键 9
    D9 = 0x39,
    /// 字母键 A
    A = 0x41,
    /// 字母键 B
    B = 0x42,
    /// 字母键 C
    C = 0x43,
    /// 字母键 D
    D = 0x44,
    /// 字母键 E
    E = 0x45,
    /// 字母键 F
    F = 0x46,
    /// 字母键 G
    G = 0x47,
    /// 字母键 H
    H = 0x48,
    /// 字母键 I
    I = 0x49,
    /// 字母键 J
    J = 0x4A,
    /// 字母键 K
    K = 0x4B,
    /// 字母键 L
    L = 0x4C,
    /// 字母键 M
    M = 0x4D,
    /// 字母键 N
    N = 0x4E,
    /// 字母键 O
    O = 0x4F,
    /// 字母键 P
    P = 0x50,
    /// 字母键 Q
    Q = 0x51,
    /// 字母键 R
    R = 0x52,
    /// 字母键 S
    S = 0x53,
    /// 字母键 T
    T = 0x54,
    /// 字母键 U
    U = 0x55,
    /// 字母键 V
    V = 0x56,
    /// 字母键 W
    W = 0x57,
    /// 字母键 X
    X = 0x58,
    /// 字母键 Y
    Y = 0x59,
    /// 字母键 Z
    Z = 0x5A,
    /// 左 Windows 键
    LWin = 0x5B,
    /// 右 Windows 键
    RWin = 0x5C,
    /// 应用程序键（右键菜单）
    Apps = 0x5D,
    /// 睡眠键
    Sleep = 0x5F,
    /// 数字小键盘 0
    NumPad0 = 0x60,
    /// 数字小键盘 1
    NumPad1 = 0x61,
    /// 数字小键盘 2
    NumPad2 = 0x62,
    /// 数字小键盘 3
    NumPad3 = 0x63,
    /// 数字小键盘 4
    NumPad4 = 0x64,
    /// 数字小键盘 5
    NumPad5 = 0x65,
    /// 数字小键盘 6
    NumPad6 = 0x66,
    /// 数字小键盘 7
    NumPad7 = 0x67,
    /// 数字小键盘 8
    NumPad8 = 0x68,
    /// 数字小键盘 9
    NumPad9 = 0x69,
    /// 数字小键盘乘号
    Multiply = 0x6A,
    /// 数字小键盘加号
    Add = 0x6B,
    /// 数字小键盘分隔符
    Separator = 0x6C,
    /// 数字小键盘减号
    Subtract = 0x6D,
    /// 数字小键盘小数点
    Decimal = 0x6E,
    /// 数字小键盘除号
    Divide = 0x6F,
    /// 功能键 F1
    F1 = 0x70,
    /// 功能键 F2
    F2 = 0x71,
    /// 功能键 F3
    F3 = 0x72,
    /// 功能键 F4
    F4 = 0x73,
    /// 功能键 F5
    F5 = 0x74,
    /// 功能键 F6
    F6 = 0x75,
    /// 功能键 F7
    F7 = 0x76,
    /// 功能键 F8
    F8 = 0x77,
    /// 功能键 F9
    F9 = 0x78,
    /// 功能键 F10
    F10 = 0x79,
    /// 功能键 F11
    F11 = 0x7A,
    /// 功能键 F12
    F12 = 0x7B,
    /// 功能键 F13
    F13 = 0x7C,
    /// 功能键 F14
    F14 = 0x7D,
    /// 功能键 F15
    F15 = 0x7E,
    /// 功能键 F16
    F16 = 0x7F,
    /// 功能键 F17
    F17 = 0x80,
    /// 功能键 F18
    F18 = 0x81,
    /// 功能键 F19
    F19 = 0x82,
    /// 功能键 F20
    F20 = 0x83,
    /// 功能键 F21
    F21 = 0x84,
    /// 功能键 F22
    F22 = 0x85,
    /// 功能键 F23
    F23 = 0x86,
    /// 功能键 F24
    F24 = 0x87,
    /// Num Lock 键
    NumLock = 0x90,
    /// Scroll Lock 键
    Scroll = 0x91,
    /// 左 Shift 键
    LShift = 0xA0,
    /// 右 Shift 键
    RShift = 0xA1,
    /// 左 Control 键
    LControl = 0xA2,
    /// 右 Control 键
    RControl = 0xA3,
    /// 左 Alt 键
    LMenu = 0xA4,
    /// 右 Alt 键
    RMenu = 0xA5,
    /// 浏览器后退键
    BrowserBack = 0xA6,
    /// 浏览器前进键
    BrowserForward = 0xA7,
    /// 浏览器刷新键
    BrowserRefresh = 0xA8,
    /// 浏览器停止键
    BrowserStop = 0xA9,
    /// 浏览器搜索键
    BrowserSearch = 0xAA,
    /// 浏览器收藏夹键
    BrowserFavorites = 0xAB,
    /// 浏览器主页键
    BrowserHome = 0xAC,
    /// 音量静音键
    VolumeMute = 0xAD,
    /// 音量减小键
    VolumeDown = 0xAE,
    /// 音量增大键
    VolumeUp = 0xAF,
    /// 媒体下一曲键
    MediaNextTrack = 0xB0,
    /// 媒体上一曲键
    MediaPrevTrack = 0xB1,
    /// 媒体停止键
    MediaStop = 0xB2,
    /// 媒体播放/暂停键
    MediaPlayPause = 0xB3,
    /// 启动邮件键
    LaunchMail = 0xB4,
    /// 启动媒体选择键
    LaunchMediaSelect = 0xB5,
    /// 启动应用程序 1 键
    LaunchApp1 = 0xB6,
    /// 启动应用程序 2 键
    LaunchApp2 = 0xB7,
    /// OEM 按键 1（通常是 ;: 键）
    Oem1 = 0xBA,
    /// OEM 加号键（=+）
    OemPlus = 0xBB,
    /// OEM 逗号键（,<）
    OemComma = 0xBC,
    /// OEM 减号键（-_）
    OemMinus = 0xBD,
    /// OEM 句号键（.>）
    OemPeriod = 0xBE,
    /// OEM 按键 2（通常是 /? 键）
    Oem2 = 0xBF,
    /// OEM 按键 3（通常是 `~ 键）
    Oem3 = 0xC0,
    /// 游戏手柄 A 键
    GamepadA = 0xC3,
    /// 游戏手柄 B 键
    GamepadB = 0xC4,
    /// 游戏手柄 X 键
    GamepadX = 0xC5,
    /// 游戏手柄 Y 键
    GamepadY = 0xC6,
    /// 游戏手柄右肩键
    GamepadRightShoulder = 0xC7,
    /// 游戏手柄左肩键
    GamepadLeftShoulder = 0xC8,
    /// 游戏手柄左扳机键
    GamepadLeftTrigger = 0xC9,
    /// 游戏手柄右扳机键
    GamepadRightTrigger = 0xCA,
    /// 游戏手柄方向键上
    GamepadDpadUp = 0xCB,
    /// 游戏手柄方向键下
    GamepadDpadDown = 0xCC,
    /// 游戏手柄方向键左
    GamepadDpadLeft = 0xCD,
    /// 游戏手柄方向键右
    GamepadDpadRight = 0xCE,
    /// 游戏手柄菜单键
    GamepadMenu = 0xCF,
    /// 游戏手柄视图键
    GamepadView = 0xD0,
    /// 游戏手柄左摇杆按键
    GamepadLeftThumbstickButton = 0xD1,
    /// 游戏手柄右摇杆按键
    GamepadRightThumbstickButton = 0xD2,
    /// 游戏手柄左摇杆上
    GamepadLeftThumbstickUp = 0xD3,
    /// 游戏手柄左摇杆下
    GamepadLeftThumbstickDown = 0xD4,
    /// 游戏手柄左摇杆右
    GamepadLeftThumbstickRight = 0xD5,
    /// 游戏手柄左摇杆左
    GamepadLeftThumbstickLeft = 0xD6,
    /// 游戏手柄右摇杆上
    GamepadRightThumbstickUp = 0xD7,
    /// 游戏手柄右摇杆下
    GamepadRightThumbstickDown = 0xD8,
    /// 游戏手柄右摇杆右
    GamepadRightThumbstickRight = 0xD9,
    /// 游戏手柄右摇杆左
    GamepadRightThumbstickLeft = 0xDA,
    /// OEM 按键 4（通常是 [{ 键）
    Oem4 = 0xDB,
    /// OEM 按键 5（通常是 \\| 键）
    Oem5 = 0xDC,
    /// OEM 按键 6（通常是 ]} 键）
    Oem6 = 0xDD,
    /// OEM 按键 7（通常是 '" 键）
    Oem7 = 0xDE,
    /// OEM 按键 8
    Oem8 = 0xDF,
    /// OEM 按键 102（通常是 <> 键）
    Oem102 = 0xE2,
    /// Process Key
    ProcessKey = 0xE5,
    /// Packet 键
    Packet = 0xE7,
    /// Attn 键
    Attn = 0xF6,
    /// CrSel 键
    CrSel = 0xF7,
    /// ExSel 键
    ExSel = 0xF8,
    /// Eof 键
    Eof = 0xF9,
    /// Play 键
    Play = 0xFA,
    /// Zoom 键
    Zoom = 0xFB,
    /// Pa1 键
    Pa1 = 0xFD,
    /// OemClear 键
    OemClear = 0xFE,
    /// 其他未定义的按键代码
    Other(u8),
}

/// 从 u8 转换为 KeyCode
///
/// 将虚拟按键码 (VK_*) 转换为 KeyCode 枚举值。
/// 对于未定义的按键码，返回 KeyCode::Other(code)。
///
/// # 示例
/// ```rust
/// use easyx::keycode::KeyCode;
///
/// // 将虚拟按键码 0x41 (VK_A) 转换为 KeyCode::A
/// let key_code = KeyCode::from(0x41);
/// assert_eq!(key_code, KeyCode::A);
///
/// // 将未定义的按键码转换为 KeyCode::Other
/// let unknown_key = KeyCode::from(0xFF);
/// assert!(matches!(unknown_key, KeyCode::Other(0xFF)));
/// ```
impl From<u8> for KeyCode {
    fn from(vkcode: u8) -> Self {
        match vkcode {
            0x01 => KeyCode::LButton,
            0x02 => KeyCode::RButton,
            0x03 => KeyCode::Cancel,
            0x04 => KeyCode::MButton,
            0x05 => KeyCode::XButton1,
            0x06 => KeyCode::XButton2,
            0x08 => KeyCode::Back,
            0x09 => KeyCode::Tab,
            0x0C => KeyCode::Clear,
            0x0D => KeyCode::Return,
            0x10 => KeyCode::Shift,
            0x11 => KeyCode::Control,
            0x12 => KeyCode::Menu,
            0x13 => KeyCode::Pause,
            0x14 => KeyCode::Capital,
            0x15 => KeyCode::Kana,
            0x16 => KeyCode::ImeOn,
            0x17 => KeyCode::Junja,
            0x18 => KeyCode::Final,
            0x19 => KeyCode::Hanja,
            0x1A => KeyCode::ImeOff,
            0x1B => KeyCode::Escape,
            0x1C => KeyCode::Convert,
            0x1D => KeyCode::NonConvert,
            0x1E => KeyCode::Accept,
            0x1F => KeyCode::ModeChange,
            0x20 => KeyCode::Space,
            0x21 => KeyCode::Prior,
            0x22 => KeyCode::Next,
            0x23 => KeyCode::End,
            0x24 => KeyCode::Home,
            0x25 => KeyCode::Left,
            0x26 => KeyCode::Up,
            0x27 => KeyCode::Right,
            0x28 => KeyCode::Down,
            0x29 => KeyCode::Select,
            0x2A => KeyCode::Print,
            0x2B => KeyCode::Execute,
            0x2C => KeyCode::Snapshot,
            0x2D => KeyCode::Insert,
            0x2E => KeyCode::Delete,
            0x2F => KeyCode::Help,
            0x30 => KeyCode::D0,
            0x31 => KeyCode::D1,
            0x32 => KeyCode::D2,
            0x33 => KeyCode::D3,
            0x34 => KeyCode::D4,
            0x35 => KeyCode::D5,
            0x36 => KeyCode::D6,
            0x37 => KeyCode::D7,
            0x38 => KeyCode::D8,
            0x39 => KeyCode::D9,
            0x41 => KeyCode::A,
            0x42 => KeyCode::B,
            0x43 => KeyCode::C,
            0x44 => KeyCode::D,
            0x45 => KeyCode::E,
            0x46 => KeyCode::F,
            0x47 => KeyCode::G,
            0x48 => KeyCode::H,
            0x49 => KeyCode::I,
            0x4A => KeyCode::J,
            0x4B => KeyCode::K,
            0x4C => KeyCode::L,
            0x4D => KeyCode::M,
            0x4E => KeyCode::N,
            0x4F => KeyCode::O,
            0x50 => KeyCode::P,
            0x51 => KeyCode::Q,
            0x52 => KeyCode::R,
            0x53 => KeyCode::S,
            0x54 => KeyCode::T,
            0x55 => KeyCode::U,
            0x56 => KeyCode::V,
            0x57 => KeyCode::W,
            0x58 => KeyCode::X,
            0x59 => KeyCode::Y,
            0x5A => KeyCode::Z,
            0x5B => KeyCode::LWin,
            0x5C => KeyCode::RWin,
            0x5D => KeyCode::Apps,
            0x5F => KeyCode::Sleep,
            0x60 => KeyCode::NumPad0,
            0x61 => KeyCode::NumPad1,
            0x62 => KeyCode::NumPad2,
            0x63 => KeyCode::NumPad3,
            0x64 => KeyCode::NumPad4,
            0x65 => KeyCode::NumPad5,
            0x66 => KeyCode::NumPad6,
            0x67 => KeyCode::NumPad7,
            0x68 => KeyCode::NumPad8,
            0x69 => KeyCode::NumPad9,
            0x6A => KeyCode::Multiply,
            0x6B => KeyCode::Add,
            0x6C => KeyCode::Separator,
            0x6D => KeyCode::Subtract,
            0x6E => KeyCode::Decimal,
            0x6F => KeyCode::Divide,
            0x70 => KeyCode::F1,
            0x71 => KeyCode::F2,
            0x72 => KeyCode::F3,
            0x73 => KeyCode::F4,
            0x74 => KeyCode::F5,
            0x75 => KeyCode::F6,
            0x76 => KeyCode::F7,
            0x77 => KeyCode::F8,
            0x78 => KeyCode::F9,
            0x79 => KeyCode::F10,
            0x7A => KeyCode::F11,
            0x7B => KeyCode::F12,
            0x7C => KeyCode::F13,
            0x7D => KeyCode::F14,
            0x7E => KeyCode::F15,
            0x7F => KeyCode::F16,
            0x80 => KeyCode::F17,
            0x81 => KeyCode::F18,
            0x82 => KeyCode::F19,
            0x83 => KeyCode::F20,
            0x84 => KeyCode::F21,
            0x85 => KeyCode::F22,
            0x86 => KeyCode::F23,
            0x87 => KeyCode::F24,
            0x90 => KeyCode::NumLock,
            0x91 => KeyCode::Scroll,
            0xA0 => KeyCode::LShift,
            0xA1 => KeyCode::RShift,
            0xA2 => KeyCode::LControl,
            0xA3 => KeyCode::RControl,
            0xA4 => KeyCode::LMenu,
            0xA5 => KeyCode::RMenu,
            0xA6 => KeyCode::BrowserBack,
            0xA7 => KeyCode::BrowserForward,
            0xA8 => KeyCode::BrowserRefresh,
            0xA9 => KeyCode::BrowserStop,
            0xAA => KeyCode::BrowserSearch,
            0xAB => KeyCode::BrowserFavorites,
            0xAC => KeyCode::BrowserHome,
            0xAD => KeyCode::VolumeMute,
            0xAE => KeyCode::VolumeDown,
            0xAF => KeyCode::VolumeUp,
            0xB0 => KeyCode::MediaNextTrack,
            0xB1 => KeyCode::MediaPrevTrack,
            0xB2 => KeyCode::MediaStop,
            0xB3 => KeyCode::MediaPlayPause,
            0xB4 => KeyCode::LaunchMail,
            0xB5 => KeyCode::LaunchMediaSelect,
            0xB6 => KeyCode::LaunchApp1,
            0xB7 => KeyCode::LaunchApp2,
            0xBA => KeyCode::Oem1,
            0xBB => KeyCode::OemPlus,
            0xBC => KeyCode::OemComma,
            0xBD => KeyCode::OemMinus,
            0xBE => KeyCode::OemPeriod,
            0xBF => KeyCode::Oem2,
            0xC0 => KeyCode::Oem3,
            0xC3 => KeyCode::GamepadA,
            0xC4 => KeyCode::GamepadB,
            0xC5 => KeyCode::GamepadX,
            0xC6 => KeyCode::GamepadY,
            0xC7 => KeyCode::GamepadRightShoulder,
            0xC8 => KeyCode::GamepadLeftShoulder,
            0xC9 => KeyCode::GamepadLeftTrigger,
            0xCA => KeyCode::GamepadRightTrigger,
            0xCB => KeyCode::GamepadDpadUp,
            0xCC => KeyCode::GamepadDpadDown,
            0xCD => KeyCode::GamepadDpadLeft,
            0xCE => KeyCode::GamepadDpadRight,
            0xCF => KeyCode::GamepadMenu,
            0xD0 => KeyCode::GamepadView,
            0xD1 => KeyCode::GamepadLeftThumbstickButton,
            0xD2 => KeyCode::GamepadRightThumbstickButton,
            0xD3 => KeyCode::GamepadLeftThumbstickUp,
            0xD4 => KeyCode::GamepadLeftThumbstickDown,
            0xD5 => KeyCode::GamepadLeftThumbstickRight,
            0xD6 => KeyCode::GamepadLeftThumbstickLeft,
            0xD7 => KeyCode::GamepadRightThumbstickUp,
            0xD8 => KeyCode::GamepadRightThumbstickDown,
            0xD9 => KeyCode::GamepadRightThumbstickRight,
            0xDA => KeyCode::GamepadRightThumbstickLeft,
            0xDB => KeyCode::Oem4,
            0xDC => KeyCode::Oem5,
            0xDD => KeyCode::Oem6,
            0xDE => KeyCode::Oem7,
            0xDF => KeyCode::Oem8,
            0xE2 => KeyCode::Oem102,
            0xE5 => KeyCode::ProcessKey,
            0xE7 => KeyCode::Packet,
            0xF6 => KeyCode::Attn,
            0xF7 => KeyCode::CrSel,
            0xF8 => KeyCode::ExSel,
            0xF9 => KeyCode::Eof,
            0xFA => KeyCode::Play,
            0xFB => KeyCode::Zoom,
            0xFD => KeyCode::Pa1,
            0xFE => KeyCode::OemClear,
            _ => KeyCode::Other(vkcode),
        }
    }
}

/// 从 KeyCode 转换为 u8
///
/// 将 KeyCode 枚举值转换为对应的虚拟按键码 (VK_*)。
/// 对于 KeyCode::Other(code)，返回原始的 code 值。
///
/// # 示例
/// ```rust
/// use easyx::keycode::KeyCode;
///
/// // 将 KeyCode::A 转换为虚拟按键码 0x41 (VK_A)
/// let vk_code = u8::from(KeyCode::A);
/// assert_eq!(vk_code, 0x41);
///
/// // 将 KeyCode::Other 转换为原始的按键码
/// let vk_code = u8::from(KeyCode::Other(0xFF));
/// assert_eq!(vk_code, 0xFF);
/// ```
impl From<KeyCode> for u8 {
    fn from(key_code: KeyCode) -> Self {
        match key_code {
            KeyCode::LButton => 0x01,
            KeyCode::RButton => 0x02,
            KeyCode::Cancel => 0x03,
            KeyCode::MButton => 0x04,
            KeyCode::XButton1 => 0x05,
            KeyCode::XButton2 => 0x06,
            KeyCode::Back => 0x08,
            KeyCode::Tab => 0x09,
            KeyCode::Clear => 0x0C,
            KeyCode::Return => 0x0D,
            KeyCode::Shift => 0x10,
            KeyCode::Control => 0x11,
            KeyCode::Menu => 0x12,
            KeyCode::Pause => 0x13,
            KeyCode::Capital => 0x14,
            KeyCode::Kana => 0x15,
            KeyCode::ImeOn => 0x16,
            KeyCode::Junja => 0x17,
            KeyCode::Final => 0x18,
            KeyCode::Hanja => 0x19,
            KeyCode::ImeOff => 0x1A,
            KeyCode::Escape => 0x1B,
            KeyCode::Convert => 0x1C,
            KeyCode::NonConvert => 0x1D,
            KeyCode::Accept => 0x1E,
            KeyCode::ModeChange => 0x1F,
            KeyCode::Space => 0x20,
            KeyCode::Prior => 0x21,
            KeyCode::Next => 0x22,
            KeyCode::End => 0x23,
            KeyCode::Home => 0x24,
            KeyCode::Left => 0x25,
            KeyCode::Up => 0x26,
            KeyCode::Right => 0x27,
            KeyCode::Down => 0x28,
            KeyCode::Select => 0x29,
            KeyCode::Print => 0x2A,
            KeyCode::Execute => 0x2B,
            KeyCode::Snapshot => 0x2C,
            KeyCode::Insert => 0x2D,
            KeyCode::Delete => 0x2E,
            KeyCode::Help => 0x2F,
            KeyCode::D0 => 0x30,
            KeyCode::D1 => 0x31,
            KeyCode::D2 => 0x32,
            KeyCode::D3 => 0x33,
            KeyCode::D4 => 0x34,
            KeyCode::D5 => 0x35,
            KeyCode::D6 => 0x36,
            KeyCode::D7 => 0x37,
            KeyCode::D8 => 0x38,
            KeyCode::D9 => 0x39,
            KeyCode::A => 0x41,
            KeyCode::B => 0x42,
            KeyCode::C => 0x43,
            KeyCode::D => 0x44,
            KeyCode::E => 0x45,
            KeyCode::F => 0x46,
            KeyCode::G => 0x47,
            KeyCode::H => 0x48,
            KeyCode::I => 0x49,
            KeyCode::J => 0x4A,
            KeyCode::K => 0x4B,
            KeyCode::L => 0x4C,
            KeyCode::M => 0x4D,
            KeyCode::N => 0x4E,
            KeyCode::O => 0x4F,
            KeyCode::P => 0x50,
            KeyCode::Q => 0x51,
            KeyCode::R => 0x52,
            KeyCode::S => 0x53,
            KeyCode::T => 0x54,
            KeyCode::U => 0x55,
            KeyCode::V => 0x56,
            KeyCode::W => 0x57,
            KeyCode::X => 0x58,
            KeyCode::Y => 0x59,
            KeyCode::Z => 0x5A,
            KeyCode::LWin => 0x5B,
            KeyCode::RWin => 0x5C,
            KeyCode::Apps => 0x5D,
            KeyCode::Sleep => 0x5F,
            KeyCode::NumPad0 => 0x60,
            KeyCode::NumPad1 => 0x61,
            KeyCode::NumPad2 => 0x62,
            KeyCode::NumPad3 => 0x63,
            KeyCode::NumPad4 => 0x64,
            KeyCode::NumPad5 => 0x65,
            KeyCode::NumPad6 => 0x66,
            KeyCode::NumPad7 => 0x67,
            KeyCode::NumPad8 => 0x68,
            KeyCode::NumPad9 => 0x69,
            KeyCode::Multiply => 0x6A,
            KeyCode::Add => 0x6B,
            KeyCode::Separator => 0x6C,
            KeyCode::Subtract => 0x6D,
            KeyCode::Decimal => 0x6E,
            KeyCode::Divide => 0x6F,
            KeyCode::F1 => 0x70,
            KeyCode::F2 => 0x71,
            KeyCode::F3 => 0x72,
            KeyCode::F4 => 0x73,
            KeyCode::F5 => 0x74,
            KeyCode::F6 => 0x75,
            KeyCode::F7 => 0x76,
            KeyCode::F8 => 0x77,
            KeyCode::F9 => 0x78,
            KeyCode::F10 => 0x79,
            KeyCode::F11 => 0x7A,
            KeyCode::F12 => 0x7B,
            KeyCode::F13 => 0x7C,
            KeyCode::F14 => 0x7D,
            KeyCode::F15 => 0x7E,
            KeyCode::F16 => 0x7F,
            KeyCode::F17 => 0x80,
            KeyCode::F18 => 0x81,
            KeyCode::F19 => 0x82,
            KeyCode::F20 => 0x83,
            KeyCode::F21 => 0x84,
            KeyCode::F22 => 0x85,
            KeyCode::F23 => 0x86,
            KeyCode::F24 => 0x87,
            KeyCode::NumLock => 0x90,
            KeyCode::Scroll => 0x91,
            KeyCode::LShift => 0xA0,
            KeyCode::RShift => 0xA1,
            KeyCode::LControl => 0xA2,
            KeyCode::RControl => 0xA3,
            KeyCode::LMenu => 0xA4,
            KeyCode::RMenu => 0xA5,
            KeyCode::BrowserBack => 0xA6,
            KeyCode::BrowserForward => 0xA7,
            KeyCode::BrowserRefresh => 0xA8,
            KeyCode::BrowserStop => 0xA9,
            KeyCode::BrowserSearch => 0xAA,
            KeyCode::BrowserFavorites => 0xAB,
            KeyCode::BrowserHome => 0xAC,
            KeyCode::VolumeMute => 0xAD,
            KeyCode::VolumeDown => 0xAE,
            KeyCode::VolumeUp => 0xAF,
            KeyCode::MediaNextTrack => 0xB0,
            KeyCode::MediaPrevTrack => 0xB1,
            KeyCode::MediaStop => 0xB2,
            KeyCode::MediaPlayPause => 0xB3,
            KeyCode::LaunchMail => 0xB4,
            KeyCode::LaunchMediaSelect => 0xB5,
            KeyCode::LaunchApp1 => 0xB6,
            KeyCode::LaunchApp2 => 0xB7,
            KeyCode::Oem1 => 0xBA,
            KeyCode::OemPlus => 0xBB,
            KeyCode::OemComma => 0xBC,
            KeyCode::OemMinus => 0xBD,
            KeyCode::OemPeriod => 0xBE,
            KeyCode::Oem2 => 0xBF,
            KeyCode::Oem3 => 0xC0,
            KeyCode::GamepadA => 0xC3,
            KeyCode::GamepadB => 0xC4,
            KeyCode::GamepadX => 0xC5,
            KeyCode::GamepadY => 0xC6,
            KeyCode::GamepadRightShoulder => 0xC7,
            KeyCode::GamepadLeftShoulder => 0xC8,
            KeyCode::GamepadLeftTrigger => 0xC9,
            KeyCode::GamepadRightTrigger => 0xCA,
            KeyCode::GamepadDpadUp => 0xCB,
            KeyCode::GamepadDpadDown => 0xCC,
            KeyCode::GamepadDpadLeft => 0xCD,
            KeyCode::GamepadDpadRight => 0xCE,
            KeyCode::GamepadMenu => 0xCF,
            KeyCode::GamepadView => 0xD0,
            KeyCode::GamepadLeftThumbstickButton => 0xD1,
            KeyCode::GamepadRightThumbstickButton => 0xD2,
            KeyCode::GamepadLeftThumbstickUp => 0xD3,
            KeyCode::GamepadLeftThumbstickDown => 0xD4,
            KeyCode::GamepadLeftThumbstickRight => 0xD5,
            KeyCode::GamepadLeftThumbstickLeft => 0xD6,
            KeyCode::GamepadRightThumbstickUp => 0xD7,
            KeyCode::GamepadRightThumbstickDown => 0xD8,
            KeyCode::GamepadRightThumbstickRight => 0xD9,
            KeyCode::GamepadRightThumbstickLeft => 0xDA,
            KeyCode::Oem4 => 0xDB,
            KeyCode::Oem5 => 0xDC,
            KeyCode::Oem6 => 0xDD,
            KeyCode::Oem7 => 0xDE,
            KeyCode::Oem8 => 0xDF,
            KeyCode::Oem102 => 0xE2,
            KeyCode::ProcessKey => 0xE5,
            KeyCode::Packet => 0xE7,
            KeyCode::Attn => 0xF6,
            KeyCode::CrSel => 0xF7,
            KeyCode::ExSel => 0xF8,
            KeyCode::Eof => 0xF9,
            KeyCode::Play => 0xFA,
            KeyCode::Zoom => 0xFB,
            KeyCode::Pa1 => 0xFD,
            KeyCode::OemClear => 0xFE,
            KeyCode::Other(code) => code,
        }
    }
}
