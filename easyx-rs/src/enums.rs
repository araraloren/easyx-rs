use easyx_sys::*;

/// 多边形填充模式
///
/// 定义多边形填充的两种模式：交替模式和环绕模式
///
/// # 示例
/// ```no_run
/// // 设置多边形填充模式为交替模式
/// let mode = PolyFillMode::alternate();
/// mode.apply();
///
/// // 获取当前多边形填充模式
/// let current_mode = PolyFillMode::current();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolyFillMode {
    /// 交替填充模式
    ///
    /// 交替模式是通过奇数/偶数规则来确定多边形的内部区域。
    /// 从任意点向无穷远画一条射线，统计与多边形边界的交点个数，
    /// 如果是奇数则在内部，偶数则在外部。
    Alternate = ALTERNATE as isize,
    /// 环绕填充模式（默认）
    ///
    /// 环绕模式是通过环绕数来确定多边形的内部区域。
    /// 环绕数是射线穿过多边形边界的净次数，
    /// 如果环绕数不为零则在内部，否则在外部。
    Winding = WINDING as isize,
}

impl PolyFillMode {
    /// 创建交替填充模式
    pub fn alternate() -> Self {
        Self::Alternate
    }

    /// 创建环绕填充模式
    pub fn winding() -> Self {
        Self::Winding
    }
}

impl PolyFillMode {
    /// 应用当前的多边形填充模式
    ///
    /// 将当前多边形填充模式应用到设备上下文
    pub fn apply(&self) {
        unsafe {
            easyx_setpolyfillmode(*self as i32);
        }
    }

    /// 获取当前设备的多边形填充模式
    ///
    /// # 返回值
    /// 当前设备上下文的多边形填充模式
    pub fn current() -> Self {
        unsafe {
            let mode = easyx_getpolyfillmode() as u32;
            match mode {
                ALTERNATE => Self::Alternate,
                WINDING => Self::Winding,
                _ => {
                    unreachable!("Invalid PolyFillMode value: {}", mode)
                }
            }
        }
    }
}

/// 背景模式
///
/// 定义文本和图形的背景模式
///
/// # 示例
/// ```no_run
/// // 设置背景模式为透明
/// let mode = BkMode::transparent();
/// mode.apply();
///
/// // 获取当前背景模式
/// let current_mode = BkMode::current();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BkMode {
    /// 不透明背景模式
    ///
    /// 在绘制文本或图形时，会使用当前背景色填充背景区域
    Opaque = OPAQUE as isize,

    /// 透明背景模式
    ///
    /// 在绘制文本或图形时，不会填充背景区域，保持原有像素不变
    Transparent = TRANSPARENT as isize,
}

impl BkMode {
    /// 创建不透明背景模式
    pub fn opaque() -> Self {
        Self::Opaque
    }

    /// 创建透明背景模式
    pub fn transparent() -> Self {
        Self::Transparent
    }
}

impl BkMode {
    /// 应用当前的背景模式
    ///
    /// 将当前背景模式应用到设备上下文
    pub fn apply(&self) {
        unsafe {
            easyx_setbkmode(*self as i32);
        }
    }

    /// 获取当前设备的背景模式
    ///
    /// # 返回值
    /// 当前设备上下文的背景模式
    pub fn current() -> Self {
        unsafe {
            let mode = easyx_getbkmode() as u32;
            match mode {
                OPAQUE => Self::Opaque,
                TRANSPARENT => Self::Transparent,
                _ => {
                    unreachable!("Invalid BkMode value: {}", mode)
                }
            }
        }
    }
}

bitflags::bitflags! {
    /// 文本绘制格式
    ///
    /// 用于控制文本绘制的各种格式选项，可以通过位或操作组合多个选项
    ///
    /// # 示例
    /// ```no_run
    /// // 设置文本居中对齐并自动换行
    /// let format = DrawTextFormat::Center | DrawTextFormat::WordBreak;
    /// app.draw_text("Hello, World!", rect, format);
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DrawTextFormat: u32 {
        /// 文字底部对齐
        ///
        /// 调整文字位置到矩形底部，仅当和 SingleLine 一起使用时有效
        const Bottom = DT_BOTTOM;
        /// 计算文本矩形
        ///
        /// 检测矩形的宽高。如果有多行文字，使用指定的宽度，扩展矩形底部以容纳每一行文字。
        /// 如果只有一行文字，修改矩形右边以容纳最后一个文字。
        /// 无论哪种情况，都返回格式化后的文字高度，并且不输出文字
        const CalcRect = DT_CALCRECT;
        /// 文字水平居中
        const Center = DT_CENTER;
        /// 编辑控件模式
        ///
        /// 以单行编辑的方式复制可见文本，使用字符的平均宽度为计算依据
        const EditControl = DT_EDITCONTROL;
        /// 末尾省略号
        ///
        /// 对于文本显示，如果字符串的末字符不在矩形内，它会被截断并以省略号标识
        const EndEllipsis = DT_END_ELLIPSIS;
        /// 展开制表符
        ///
        /// 展开 TAB 符号。默认每个 TAB 占 8 个字符位置
        const ExpandTabs = DT_EXPANDTABS;
        /// 包含外部行间距
        ///
        /// 在行高里包含字体的行间距。通常情况下，行间距不被包含在正文的行高里
        const ExternalLeading = DT_EXTERNALLEADING;
        /// 隐藏前缀
        ///
        /// 忽略文字中的前缀字符(&)，并且前缀字符后面的字符不会出现下划线
        const HidePrefix = DT_HIDEPREFIX;
        /// 使用内部字体
        ///
        /// 使用系统字体计算文字的宽高等属性
        const Internal = DT_INTERNAL;
        /// 文字左对齐
        const Left = DT_LEFT;
        /// 修改字符串
        ///
        /// 修改指定字符串为显示出的正文。仅当和 EndEllipsis 或 PathEllipsis 标志同时使用时有效
        const ModifyString = DT_MODIFYSTRING;
        /// 无裁剪
        ///
        /// 使输出文字不受矩形裁剪限制。使用此标志会使 drawtext 执行稍快一些
        const NoClip = DT_NOCLIP;
        /// 无全宽字符换行
        ///
        /// 防止换行符插入到宽字符串，换行规则相当于单字节字符串。仅当和 WordBreak 一起使用时有效
        const NoFullWidthCharBreak = DT_NOFULLWIDTHCHARBREAK;
        /// 无前缀
        ///
        /// 关闭前缀字符的处理。通常，DrawText 解释前缀转义符 & 为其后的字符加下划线，解释 && 为显示单个 &
        const NoPrefix = DT_NOPREFIX;
        /// 路径省略号
        ///
        /// 对于显示的文字，用省略号替换字符串中间的字符以便容纳于矩形内。如果字符串包含反斜杠，尽可能保留最后一个反斜杠后面的文字
        const PathEllipsis = DT_PATH_ELLIPSIS;
        /// 仅前缀下划线
        ///
        /// 仅仅在(&)前缀字符的位置下绘制一个下划线。不绘制字符串中的任何其他字符
        const PrefixOnly = DT_PREFIXONLY;
        /// 文字右对齐
        const Right = DT_RIGHT;
        /// 从右向左阅读
        ///
        /// 设置从右向左的阅读顺序（当文字是希伯来文或阿拉伯文时）。默认的阅读顺序是从左向右
        const RtlReading = DT_RTLREADING;
        /// 单行显示
        ///
        /// 使文字显示在一行。回车和换行符都无效
        const SingleLine = DT_SINGLELINE;
        /// 设置制表位
        ///
        /// 设置 TAB 制表位。格式的 15–8 位指定 TAB 的字符宽度。默认 TAB 表示 8 个字符宽度
        const TabStop = DT_TABSTOP;
        /// 文字顶部对齐
        const Top = DT_TOP;
        /// 文字垂直居中
        ///
        /// 仅当和 SingleLine 一起使用时有效
        const VCenter = DT_VCENTER;
        /// 自动换行
        ///
        /// 当文字超过右边界时会自动换行(不拆开单词)。回车符同样可以换行
        const WordBreak = DT_WORDBREAK;
        /// 单词省略号
        ///
        /// 截去无法容纳的文字，并在末尾增加省略号
        const WordEllipsis = DT_WORD_ELLIPSIS;
    }
}

/// 二元光栅操作模式
///
/// 定义绘图操作的二元光栅操作模式，控制源像素和目标像素如何组合
///
/// # 示例
/// ```no_run
/// // 设置光栅操作模式为异或模式
/// let mode = Rop2::xor_pen();
/// mode.apply();
///
/// // 获取当前光栅操作模式
/// let current_mode = Rop2::current();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rop2 {
    /// 绘制出的像素颜色 = 黑色
    Black = R2_BLACK as isize,
    /// 绘制出的像素颜色 = 当前颜色（默认）
    CopyPen = R2_COPYPEN as isize,
    /// 绘制出的像素颜色 = 屏幕颜色 AND (NOT 当前颜色)
    MaskNotPen = R2_MASKNOTPEN as isize,
    /// 绘制出的像素颜色 = 屏幕颜色 AND 当前颜色
    MaskPen = R2_MASKPEN as isize,
    /// 绘制出的像素颜色 = (NOT 屏幕颜色) AND 当前颜色
    MaskPenNot = R2_MASKPENNOT as isize,
    /// 绘制出的像素颜色 = 屏幕颜色 OR (NOT 当前颜色)
    MergeNotPen = R2_MERGENOTPEN as isize,
    /// 绘制出的像素颜色 = 屏幕颜色 OR 当前颜色
    MergePen = R2_MERGEPEN as isize,
    /// 绘制出的像素颜色 = (NOT 屏幕颜色) OR 当前颜色
    MergePenNot = R2_MERGEPENNOT as isize,
    /// 绘制出的像素颜色 = 屏幕颜色
    Nop = R2_NOP as isize,
    /// 绘制出的像素颜色 = NOT 屏幕颜色
    Not = R2_NOT as isize,
    /// 绘制出的像素颜色 = NOT 当前颜色
    NotCopyPen = R2_NOTCOPYPEN as isize,
    /// 绘制出的像素颜色 = NOT (屏幕颜色 AND 当前颜色)
    NotMaskPen = R2_NOTMASKPEN as isize,
    /// 绘制出的像素颜色 = NOT (屏幕颜色 OR 当前颜色)
    NotMergePen = R2_NOTMERGEPEN as isize,
    /// 绘制出的像素颜色 = NOT (屏幕颜色 XOR 当前颜色)
    NotXorPen = R2_NOTXORPEN as isize,
    /// 绘制出的像素颜色 = 白色
    White = R2_WHITE as isize,
    /// 绘制出的像素颜色 = 屏幕颜色 XOR 当前颜色
    XorPen = R2_XORPEN as isize,
}

impl Rop2 {
    /// 创建黑色模式
    pub fn black() -> Self {
        Self::Black
    }

    /// 创建复制模式
    pub fn copy_pen() -> Self {
        Self::CopyPen
    }

    /// 创建遮罩非笔模式
    pub fn mask_not_pen() -> Self {
        Self::MaskNotPen
    }

    /// 创建遮罩笔模式
    pub fn mask_pen() -> Self {
        Self::MaskPen
    }

    /// 创建遮罩笔非模式
    pub fn mask_pen_not() -> Self {
        Self::MaskPenNot
    }

    /// 创建合并非笔模式
    pub fn merge_not_pen() -> Self {
        Self::MergeNotPen
    }

    /// 创建合并笔模式
    pub fn merge_pen() -> Self {
        Self::MergePen
    }

    /// 创建合并笔非模式
    pub fn merge_pen_not() -> Self {
        Self::MergePenNot
    }

    /// 创建无操作模式
    pub fn nop() -> Self {
        Self::Nop
    }

    /// 创建非模式
    pub fn not() -> Self {
        Self::Not
    }

    /// 创建非复制笔模式
    pub fn not_copy_pen() -> Self {
        Self::NotCopyPen
    }

    /// 创建非遮罩笔模式
    pub fn not_mask_pen() -> Self {
        Self::NotMaskPen
    }

    /// 创建非合并笔模式
    pub fn not_merge_pen() -> Self {
        Self::NotMergePen
    }

    /// 创建非异或笔模式
    pub fn not_xor_pen() -> Self {
        Self::NotXorPen
    }

    /// 创建白色模式
    pub fn white() -> Self {
        Self::White
    }

    /// 创建异或笔模式
    pub fn xor_pen() -> Self {
        Self::XorPen
    }
}

impl Rop2 {
    /// 应用当前的二元光栅操作模式
    ///
    /// 将当前二元光栅操作模式应用到设备上下文
    pub fn apply(&self) {
        unsafe {
            easyx_setrop2(*self as i32);
        }
    }

    /// 获取当前设备的二元光栅操作模式
    ///
    /// # 返回值
    /// 当前设备上下文的二元光栅操作模式
    pub fn current() -> Self {
        unsafe {
            let mode = easyx_getrop2() as u32;
            match mode {
                R2_BLACK => Rop2::Black,
                R2_COPYPEN => Rop2::CopyPen,
                R2_MASKNOTPEN => Rop2::MaskNotPen,
                R2_MASKPEN => Rop2::MaskPen,
                R2_MASKPENNOT => Rop2::MaskPenNot,
                R2_MERGENOTPEN => Rop2::MergeNotPen,
                R2_MERGEPEN => Rop2::MergePen,
                R2_MERGEPENNOT => Rop2::MergePenNot,
                R2_NOP => Rop2::Nop,
                R2_NOT => Rop2::Not,
                R2_NOTCOPYPEN => Rop2::NotCopyPen,
                R2_NOTMASKPEN => Rop2::NotMaskPen,
                R2_NOTMERGEPEN => Rop2::NotMergePen,
                R2_NOTXORPEN => Rop2::NotXorPen,
                R2_WHITE => Rop2::White,
                R2_XORPEN => Rop2::XorPen,
                _ => {
                    unreachable!("Invalid ROP2 value: {}", mode)
                }
            }
        }
    }
}
