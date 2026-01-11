use easyx_sys::*;

/// Binary raster operation modes for EasyX graphics library.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolyFillMode {
    /// 绘制出的像素颜色 = 黑色
    Alternate = ALTERNATE as isize,
    /// 绘制出的像素颜色 = 当前颜色（默认）
    Winding = WINDING as isize,
}

impl PolyFillMode {
    pub fn alternate() -> Self {
        Self::Alternate
    }

    pub fn winding() -> Self {
        Self::Winding
    }
}

impl PolyFillMode {
    /// 应用当前的二元光栅操作模式
    pub fn apply(&self) {
        unsafe {
            easyx_setpolyfillmode(*self as i32);
        }
    }

    /// 获取当前设备的二元光栅操作模式
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BkMode {
    Opaque = OPAQUE as isize,

    Transparent = TRANSPARENT as isize,
}

impl BkMode {
    pub fn opaque() -> Self {
        Self::Opaque
    }

    pub fn transparent() -> Self {
        Self::Transparent
    }
}

impl BkMode {
    pub fn apply(&self) {
        unsafe {
            easyx_setbkmode(*self as i32);
        }
    }

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
    /// DrawText 格式化输出文字的方法
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DrawTextFormat: u32 {
        /// 调整文字位置到矩形底部，仅当和 DT_SINGLELINE 一起使用时有效。
        const Bottom = DT_BOTTOM;
        /// 检测矩形的宽高。如果有多行文字，drawtext 使用 pRect 指定的宽度，并且扩展矩形的底部以容纳每一行文字。如果只有一行文字，drawtext 修改 pRect 的右边以容纳最后一个文字。无论哪种情况，drawtext 都返回格式化后的文字高度，并且不输出文字。
        const CalcRect = DT_CALCRECT;
        /// 文字水平居中。
        const Center = DT_CENTER;
        /// 以单行编辑的方式复制可见文本。具体的说，就是以字符的平均宽度为计算依据，同时用这个方式应用于编辑控制，并且这种方式不显示可见部分的最后一行。
        const EditControl = DT_EDITCONTROL;
        /// 对于文本显示，如果字符串的末字符不在矩形内，它会被截断并以省略号标识。如果是一个单词而不是一个字符，其末尾超出了矩形范围，它不会被截断。字符串不会被修改，除非指定了 DT_MODIFYSTRING 标志。
        const EndEllipsis = DT_END_ELLIPSIS;
        /// 展开 TAB 符号。默认每个 TAB 占 8 个字符位置。注意，DT_WORD_ELLIPSIS、DT_PATH_ELLIPSIS 和 DT_END_ELLIPSIS 不能和 DT_EXPANDTABS 一起用。
        const ExpandTabs = DT_EXPANDTABS;
        /// 在行高里包含字体的行间距。通常情况下，行间距不被包含在正文的行高里。
        const ExternalLeading = DT_EXTERNALLEADING;
        /// 忽略文字中的前缀字符(&)，并且前缀字符后面的字符不会出现下划线。其他前缀字符仍会被处理。
        const HidePrefix = DT_HIDEPREFIX;
        /// 使用系统字体计算文字的宽高等属性。
        const Internal = DT_INTERNAL;
        /// 文字左对齐。
        const Left = DT_LEFT;
        /// 修改指定字符串为显示出的正文。仅当和 DT_END_ELLIPSIS 或 DT_PATH_ELLIPSIS 标志同时使用时有效。
        const ModifyString = DT_MODIFYSTRING;
        /// 使输出文字不受 pRect 裁剪限制。使用 DT_NOCLIP 会使 drawtext 执行稍快一些。
        const NoClip = DT_NOCLIP;
        /// 防止换行符插入到 DBCS (double-wide character string，即宽字符串)，换行规则相当于 SBCS 字符串。仅当和 DT_WORDBREAK 一起使用时有效。例如，汉字就是宽字符，设置该标志后，连续的汉字会像英文单词一样不被换行符中断。
        const NoFullWidthCharBreak = DT_NOFULLWIDTHCHARBREAK;
        /// 关闭前缀字符的处理。通常，DrawText 解释前缀转义符 & 为其后的字符加下划线，解释 && 为显示单个 &。指定 DT_NOPREFIX，这种处理被关闭。
        const NoPrefix = DT_NOPREFIX;
        /// 对于显示的文字，用省略号替换字符串中间的字符以便容纳于矩形内。如果字符串包含反斜杠()，DT_PATH_ELLIPSIS 尽可能的保留最后一个反斜杠后面的文字。字符串不会被修改，除非指定了 DT_MODIFYSTRING 标志。
        const PathEllipsis = DT_PATH_ELLIPSIS;
        /// 仅仅在(&)前缀字符的位置下绘制一个下划线。不绘制字符串中的任何其他字符。
        const PrefixOnly = DT_PREFIXONLY;
        /// 文字右对齐。
        const Right = DT_RIGHT;
        /// 设置从右向左的阅读顺序（当文字是希伯来文或阿拉伯文时）。默认的阅读顺序是从左向右。
        const RtlReading = DT_RTLREADING;
        /// 使文字显示在一行。回车和换行符都无效。
        const SingleLine = DT_SINGLELINE;
        /// 设置 TAB 制表位。uFormat 的 15–8 位指定 TAB 的字符宽度。默认 TAB 表示 8 个字符宽度。注意，DT_CALCRECT、DT_EXTERNALLEADING、DT_INTERNAL、DT_NOCLIP 和 DT_NOPREFIX 不能和 DT_TABSTOP 一起用。
        const TabStop = DT_TABSTOP;
        /// 文字顶部对齐。
        const Top = DT_TOP;
        /// 文字垂直居中。仅当和 DT_SINGLELINE 一起使用时有效。
        const VCenter = DT_VCENTER;
        /// 自动换行。当文字超过右边界时会自动换行(不拆开单词)。回车符同样可以换行。
        const WordBreak = DT_WORDBREAK;
        /// 截去无法容纳的文字，并在末尾增加省略号。
        const WordEllipsis = DT_WORD_ELLIPSIS;
    }
}

/// Binary raster operation modes for EasyX graphics library.
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
    pub fn black() -> Self {
        Self::Black
    }

    pub fn copy_pen() -> Self {
        Self::CopyPen
    }

    pub fn mask_not_pen() -> Self {
        Self::MaskNotPen
    }

    pub fn mask_pen() -> Self {
        Self::MaskPen
    }

    pub fn mask_pen_not() -> Self {
        Self::MaskPenNot
    }

    pub fn merge_not_pen() -> Self {
        Self::MergeNotPen
    }

    pub fn merge_pen() -> Self {
        Self::MergePen
    }

    pub fn merge_pen_not() -> Self {
        Self::MergePenNot
    }

    pub fn nop() -> Self {
        Self::Nop
    }

    pub fn not() -> Self {
        Self::Not
    }

    pub fn not_copy_pen() -> Self {
        Self::NotCopyPen
    }

    pub fn not_mask_pen() -> Self {
        Self::NotMaskPen
    }

    pub fn not_merge_pen() -> Self {
        Self::NotMergePen
    }

    pub fn not_xor_pen() -> Self {
        Self::NotXorPen
    }

    pub fn white() -> Self {
        Self::White
    }

    pub fn xor_pen() -> Self {
        Self::XorPen
    }
}

impl Rop2 {
    /// 应用当前的二元光栅操作模式
    pub fn apply(&self) {
        unsafe {
            easyx_setrop2(*self as i32);
        }
    }

    /// 获取当前设备的二元光栅操作模式
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
