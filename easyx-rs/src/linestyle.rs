use easyx_sys::*;

bitflags::bitflags! {
    /// 线帽样式标志
    ///
    /// 线帽样式决定了线条端点的外观。这些标志用于设置线条绘制时端点的处理方式。
    ///
    /// # 变体说明
    /// - `EndCapRound`: 圆形线帽 - 端点是一个半圆，直径等于线条宽度
    /// - `EndCapSquare`: 方形线帽 - 端点是一个正方形，边长等于线条宽度
    /// - `EndCapFlat`: 平形线帽 - 端点是线条的实际端点，没有延伸
    ///
    /// # 示例
    /// ```rust
    /// use easyx::linestyle::EndCapStyle;
    ///
    /// // 设置圆形线帽
    /// let round_cap = EndCapStyle::EndCapRound;
    ///
    /// // 组合样式（虽然通常只使用一种线帽样式）
    /// let combined = EndCapStyle::EndCapRound | EndCapStyle::EndCapSquare;
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct EndCapStyle: u32 {
        /// 圆形线帽 - 端点是一个半圆
        const EndCapRound = PS_ENDCAP_ROUND;
        /// 方形线帽 - 端点是一个正方形
        const EndCapSquare = PS_ENDCAP_SQUARE;
        /// 平形线帽 - 端点是线条的实际端点
        const EndCapFlat = PS_ENDCAP_FLAT;
    }
}

bitflags::bitflags! {
    /// 线条连接样式标志
    ///
    /// 连接样式决定了两条线条相交处的外观。这些标志用于设置线条绘制时连接点的处理方式。
    ///
    /// # 变体说明
    /// - `JoinRound`: 圆形连接 - 连接点是一个平滑的圆弧
    /// - `JoinBevel`: 斜角连接 - 连接点是一个斜角
    /// - `JoinMiter`: 尖角连接 - 连接点是一个尖角
    ///
    /// # 示例
    /// ```rust
    /// use easyx::linestyle::JoinStyle;
    ///
    /// // 设置圆形连接
    /// let round_join = JoinStyle::JoinRound;
    ///
    /// // 设置斜角连接
    /// let bevel_join = JoinStyle::JoinBevel;
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct JoinStyle: u32 {
        /// 圆形连接 - 连接点是一个平滑的圆弧
        const JoinRound = PS_JOIN_ROUND;
        /// 斜角连接 - 连接点是一个斜角
        const JoinBevel = PS_JOIN_BEVEL;
        /// 尖角连接 - 连接点是一个尖角
        const JoinMiter = PS_JOIN_MITER;
    }
}

/// 内部线条样式枚举
///
/// 表示线条的具体绘制样式，包括实线、虚线、点线等预设样式，以及自定义样式。
///
/// # 变体说明
/// - `Solid`: 实线样式
/// - `Dashed`: 虚线样式
/// - `Dotted`: 点线样式
/// - `DashDot`: 点划线样式（-.-.-）
/// - `DashDotDot`: 双点划线样式（-..-..）
/// - `Null`: 不可见线条
/// - `User`: 用户自定义样式，包含样式标志和用户定义的样式数组
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InnerStyle {
    /// 实线样式
    Solid(u32),
    /// 虚线样式
    Dashed(u32),
    /// 点线样式
    Dotted(u32),
    /// 点划线样式（-.-.-）
    DashDot(u32),
    /// 双点划线样式（-..-..）
    DashDotDot(u32),
    /// 不可见线条
    Null(u32),
    /// 用户自定义样式
    User {
        /// 样式标志
        style: u32,
        /// 用户定义的样式数组，用于描述自定义线条样式的图案
        user_style: Vec<i32>,
    },
}

/// 线条样式结构体
///
/// 完整的线条样式配置，包含线条类型、粗细、线帽样式和连接样式。
/// 用于设置和管理 EasyX 图形库中的线条绘制样式。
///
/// # 字段说明
/// - `style`: 内部线条样式，决定线条的类型（实线、虚线等）
/// - `thickness`: 线条粗细，单位为像素
///
/// # 示例
/// ```rust
/// use easyx::linestyle::LineStyle;
///
/// // 创建一个2像素宽的实线样式
/// let solid_line = LineStyle::solid(2);
///
/// // 创建一个3像素宽的虚线样式
/// let dashed_line = LineStyle::dashed(3);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LineStyle {
    style: InnerStyle,
    thickness: i32,
}

impl LineStyle {
    /// 创建实线样式
    ///
    /// # 参数
    /// - `thickness`: 线条粗细，单位为像素
    ///
    /// # 返回值
    /// 返回一个实线样式的 `LineStyle` 实例
    ///
    /// # 示例
    /// ```rust
    /// use easyx::linestyle::LineStyle;
    ///
    /// let solid_line = LineStyle::solid(2);
    /// assert_eq!(solid_line.thickness(), 2);
    /// ```
    pub fn solid(thickness: i32) -> Self {
        Self {
            style: InnerStyle::Solid(0),
            thickness,
        }
    }

    /// 创建虚线样式
    ///
    /// # 参数
    /// - `thickness`: 线条粗细，单位为像素
    ///
    /// # 返回值
    /// 返回一个虚线样式的 `LineStyle` 实例
    ///
    /// # 示例
    /// ```rust
    /// use easyx::linestyle::LineStyle;
    ///
    /// let dashed_line = LineStyle::dashed(3);
    /// ```
    pub fn dashed(thickness: i32) -> Self {
        Self {
            style: InnerStyle::Dashed(0),
            thickness,
        }
    }

    /// 创建点线样式
    ///
    /// # 参数
    /// - `thickness`: 线条粗细，单位为像素
    ///
    /// # 返回值
    /// 返回一个点线样式的 `LineStyle` 实例
    ///
    /// # 示例
    /// ```rust
    /// use easyx::linestyle::LineStyle;
    ///
    /// let dotted_line = LineStyle::dotted(1);
    /// ```
    pub fn dotted(thickness: i32) -> Self {
        Self {
            style: InnerStyle::Dotted(0),
            thickness,
        }
    }

    /// 创建点划线样式（-.-.-）
    ///
    /// # 参数
    /// - `thickness`: 线条粗细，单位为像素
    ///
    /// # 返回值
    /// 返回一个点划线样式的 `LineStyle` 实例
    ///
    /// # 示例
    /// ```rust
    /// use easyx::linestyle::LineStyle;
    ///
    /// let dash_dot_line = LineStyle::dash_dot(2);
    /// ```
    pub fn dash_dot(thickness: i32) -> Self {
        Self {
            style: InnerStyle::DashDot(0),
            thickness,
        }
    }

    /// 创建双点划线样式（-..-..）
    ///
    /// # 参数
    /// - `thickness`: 线条粗细，单位为像素
    ///
    /// # 返回值
    /// 返回一个双点划线样式的 `LineStyle` 实例
    ///
    /// # 示例
    /// ```rust
    /// use easyx::linestyle::LineStyle;
    ///
    /// let dash_dot_dot_line = LineStyle::dash_dot_dot(2);
    /// ```
    pub fn dash_dot_dot(thickness: i32) -> Self {
        Self {
            style: InnerStyle::DashDotDot(0),
            thickness,
        }
    }

    /// 创建不可见线条样式
    ///
    /// # 参数
    /// - `thickness`: 线条粗细，单位为像素（尽管线条不可见）
    ///
    /// # 返回值
    /// 返回一个不可见线条样式的 `LineStyle` 实例
    ///
    /// # 示例
    /// ```rust
    /// use easyx::linestyle::LineStyle;
    ///
    /// let null_line = LineStyle::null(1);
    /// ```
    pub fn null(thickness: i32) -> Self {
        Self {
            style: InnerStyle::Null(0),
            thickness,
        }
    }

    /// 创建用户自定义线条样式
    ///
    /// # 参数
    /// - `thickness`: 线条粗细，单位为像素
    /// - `user_style`: 用户定义的样式数组，用于描述自定义线条的图案
    ///
    /// # 返回值
    /// 返回一个用户自定义样式的 `LineStyle` 实例
    ///
    /// # 示例
    /// ```rust
    /// use easyx::linestyle::LineStyle;
    ///
    /// // 创建一个自定义样式：10像素实线，5像素空白，循环
    /// let custom_style = vec![10, 5];
    /// let user_line = LineStyle::user(2, custom_style);
    /// ```
    pub fn user(thickness: i32, user_style: Vec<i32>) -> Self {
        Self {
            style: InnerStyle::User {
                style: 0,
                user_style,
            },
            thickness,
        }
    }

    /// 获取线条样式的整数值表示
    ///
    /// 将线条样式转换为 EasyX 内部使用的整数值，包含线条类型、线帽样式和连接样式。
    ///
    /// # 返回值
    /// 返回线条样式的整数值表示
    pub fn style_value(&self) -> i32 {
        (match &self.style {
            InnerStyle::Solid(val) => *val | PS_SOLID,
            InnerStyle::Dashed(val) => *val | PS_DASH,
            InnerStyle::Dotted(val) => *val | PS_DOT,
            InnerStyle::DashDot(val) => *val | PS_DASHDOT,
            InnerStyle::DashDotDot(val) => *val | PS_DASHDOTDOT,
            InnerStyle::Null(val) => *val | PS_NULL,
            InnerStyle::User {
                style: val,
                user_style: _,
            } => *val | PS_USERSTYLE,
        }) as i32
    }

    /// 设置线帽样式
    ///
    /// # 参数
    /// - `cap_style`: 要设置的线帽样式
    ///
    /// # 示例
    /// ```rust
    /// use easyx::linestyle::{LineStyle, EndCapStyle};
    ///
    /// let mut line = LineStyle::solid(3);
    /// line.set_cap_style(EndCapStyle::EndCapRound);
    /// ```
    pub fn set_cap_style(&mut self, cap_style: EndCapStyle) {
        match &mut self.style {
            InnerStyle::Solid(val) => {
                *val = (*val & !PS_ENDCAP_MASK) | cap_style.bits();
            }
            InnerStyle::Dashed(val) => {
                *val = (*val & !PS_ENDCAP_MASK) | cap_style.bits();
            }
            InnerStyle::Dotted(val) => {
                *val = (*val & !PS_ENDCAP_MASK) | cap_style.bits();
            }
            InnerStyle::DashDot(val) => {
                *val = (*val & !PS_ENDCAP_MASK) | cap_style.bits();
            }
            InnerStyle::DashDotDot(val) => {
                *val = (*val & !PS_ENDCAP_MASK) | cap_style.bits();
            }
            InnerStyle::Null(val) => {
                *val = (*val & !PS_ENDCAP_MASK) | cap_style.bits();
            }
            InnerStyle::User {
                style: val,
                user_style: _,
            } => {
                *val = (*val & !PS_ENDCAP_MASK) | cap_style.bits();
            }
        }
    }

    /// 获取当前线帽样式
    ///
    /// # 返回值
    /// 返回当前的线帽样式
    ///
    /// # 示例
    /// ```rust
    /// use easyx::linestyle::{LineStyle, EndCapStyle};
    ///
    /// let mut line = LineStyle::solid(3);
    /// line.set_cap_style(EndCapStyle::EndCapRound);
    /// assert_eq!(line.get_cap_style(), EndCapStyle::EndCapRound);
    /// ```
    pub fn get_cap_style(&self) -> EndCapStyle {
        match &self.style {
            InnerStyle::Solid(val) => EndCapStyle::from_bits_truncate(*val & PS_ENDCAP_MASK),
            InnerStyle::Dashed(val) => EndCapStyle::from_bits_truncate(*val & PS_ENDCAP_MASK),
            InnerStyle::Dotted(val) => EndCapStyle::from_bits_truncate(*val & PS_ENDCAP_MASK),
            InnerStyle::DashDot(val) => EndCapStyle::from_bits_truncate(*val & PS_ENDCAP_MASK),
            InnerStyle::DashDotDot(val) => EndCapStyle::from_bits_truncate(*val & PS_ENDCAP_MASK),
            InnerStyle::Null(val) => EndCapStyle::from_bits_truncate(*val & PS_ENDCAP_MASK),
            InnerStyle::User {
                style: val,
                user_style: _,
            } => EndCapStyle::from_bits_truncate(*val & PS_ENDCAP_MASK),
        }
    }

    /// 设置线条连接样式
    ///
    /// # 参数
    /// - `join_style`: 要设置的连接样式
    ///
    /// # 示例
    /// ```rust
    /// use easyx::linestyle::{LineStyle, JoinStyle};
    ///
    /// let mut line = LineStyle::solid(3);
    /// line.set_join_style(JoinStyle::JoinRound);
    /// ```
    pub fn set_join_style(&mut self, join_style: JoinStyle) {
        match &mut self.style {
            InnerStyle::Solid(val) => {
                *val = (*val & !PS_JOIN_MASK) | join_style.bits();
            }
            InnerStyle::Dashed(val) => {
                *val = (*val & !PS_JOIN_MASK) | join_style.bits();
            }
            InnerStyle::Dotted(val) => {
                *val = (*val & !PS_JOIN_MASK) | join_style.bits();
            }
            InnerStyle::DashDot(val) => {
                *val = (*val & !PS_JOIN_MASK) | join_style.bits();
            }
            InnerStyle::DashDotDot(val) => {
                *val = (*val & !PS_JOIN_MASK) | join_style.bits();
            }
            InnerStyle::Null(val) => {
                *val = (*val & !PS_JOIN_MASK) | join_style.bits();
            }
            InnerStyle::User {
                style: val,
                user_style: _,
            } => {
                *val = (*val & !PS_JOIN_MASK) | join_style.bits();
            }
        }
    }

    /// 获取当前线条连接样式
    ///
    /// # 返回值
    /// 返回当前的线条连接样式
    ///
    /// # 示例
    /// ```rust
    /// use easyx::linestyle::{LineStyle, JoinStyle};
    ///
    /// let mut line = LineStyle::solid(3);
    /// line.set_join_style(JoinStyle::JoinRound);
    /// assert_eq!(line.get_join_style(), JoinStyle::JoinRound);
    /// ```
    pub fn get_join_style(&self) -> JoinStyle {
        match &self.style {
            InnerStyle::Solid(val) => JoinStyle::from_bits_truncate(*val & PS_JOIN_MASK),
            InnerStyle::Dashed(val) => JoinStyle::from_bits_truncate(*val & PS_JOIN_MASK),
            InnerStyle::Dotted(val) => JoinStyle::from_bits_truncate(*val & PS_JOIN_MASK),
            InnerStyle::DashDot(val) => JoinStyle::from_bits_truncate(*val & PS_JOIN_MASK),
            InnerStyle::DashDotDot(val) => JoinStyle::from_bits_truncate(*val & PS_JOIN_MASK),
            InnerStyle::Null(val) => JoinStyle::from_bits_truncate(*val & PS_JOIN_MASK),
            InnerStyle::User {
                style: val,
                user_style: _,
            } => JoinStyle::from_bits_truncate(*val & PS_JOIN_MASK),
        }
    }

    /// 获取线条粗细
    ///
    /// # 返回值
    /// 返回线条的粗细，单位为像素
    ///
    /// # 示例
    /// ```rust
    /// use easyx::linestyle::LineStyle;
    ///
    /// let line = LineStyle::solid(3);
    /// assert_eq!(line.thickness(), 3);
    /// ```
    pub fn thickness(&self) -> i32 {
        self.thickness
    }

    /// 设置线条粗细
    ///
    /// # 参数
    /// - `thickness`: 线条粗细，单位为像素
    ///
    /// # 示例
    /// ```rust
    /// use easyx::linestyle::LineStyle;
    ///
    /// let mut line = LineStyle::solid(2);
    /// line.set_thickness(5);
    /// assert_eq!(line.thickness(), 5);
    /// ```
    pub fn set_thickness(&mut self, thickness: i32) {
        self.thickness = thickness;
    }

    /// 获取用户自定义样式
    ///
    /// # 返回值
    /// 如果是用户自定义样式，返回 Some(&[i32])，否则返回 None
    ///
    /// # 示例
    /// ```rust
    /// use easyx::linestyle::LineStyle;
    ///
    /// let custom_style = vec![10, 5];
    /// let user_line = LineStyle::user(2, custom_style.clone());
    /// assert_eq!(user_line.user_style(), Some(&custom_style[..]));
    ///
    /// let solid_line = LineStyle::solid(2);
    /// assert_eq!(solid_line.user_style(), None);
    /// ```
    pub fn user_style(&self) -> Option<&[i32]> {
        match &self.style {
            InnerStyle::User {
                style: _,
                user_style,
            } => Some(user_style.as_slice()),
            _ => None,
        }
    }

    /// 设置用户自定义样式
    ///
    /// # 参数
    /// - `style`: 由(实线长度, 空白长度)对组成的向量，用于定义自定义线条样式
    ///
    /// # 示例
    /// ```rust
    /// use easyx::linestyle::LineStyle;
    ///
    /// let mut user_line = LineStyle::user(2, vec![5, 3]);
    /// // 重新设置样式：10像素实线，5像素空白，3像素实线，2像素空白
    /// user_line.set_user_style(vec![(10, 5), (3, 2)]);
    /// ```
    pub fn set_user_style(&mut self, style: Vec<(i32, i32)>) {
        if let InnerStyle::User {
            style: _,
            user_style,
        } = &mut self.style
        {
            *user_style = style.into_iter().flat_map(|(a, b)| [a, b]).collect();
        }
    }
}

impl LineStyle {
    /// 应用当前线条样式到 EasyX 图形库
    ///
    /// 将当前的线条样式设置为 EasyX 图形库的活动线条样式，
    /// 后续绘制的所有线条都将使用此样式。
    ///
    /// # 示例
    /// ```rust
    /// use easyx::{App, linestyle::LineStyle};
    ///
    /// let mut app = App::init().unwrap();
    /// let solid_line = LineStyle::solid(2);
    ///
    /// // 应用线条样式
    /// solid_line.apply();
    ///
    /// // 后续绘制的线条将使用此样式
    /// app.line((10, 10), (100, 100));
    /// ```
    pub fn apply(&self) {
        match &self.style {
            InnerStyle::User {
                style: _,
                user_style,
            } => unsafe {
                easyx_setlinestyle(
                    self.style_value(),
                    self.thickness,
                    user_style.as_ptr().cast(),
                    user_style.len() as u32,
                );
            },
            _ => unsafe {
                easyx_setlinestyle(self.style_value(), self.thickness, std::ptr::null_mut(), 0);
            },
        }
    }

    /// 获取当前 EasyX 图形库使用的线条样式
    ///
    /// 从 EasyX 图形库获取当前活动的线条样式，并转换为 `LineStyle` 实例。
    ///
    /// # 返回值
    /// 返回当前活动的线条样式
    ///
    /// # 示例
    /// ```rust
    /// use easyx::{App, linestyle::LineStyle};
    ///
    /// let mut app = App::init().unwrap();
    /// let solid_line = LineStyle::solid(2);
    /// solid_line.apply();
    ///
    /// // 获取当前线条样式
    /// let current_style = LineStyle::current();
    /// assert_eq!(current_style.thickness(), 2);
    /// ```
    pub fn current() -> Self {
        let mut style_bits = 0;
        let mut thickness = 0;

        // 首先获取 user_style 的长度
        let mut user_style_len = unsafe { easyx_getlinestyle_len() };

        // 然后创建足够大小的向量
        let mut user_style_u32 = vec![0u32; user_style_len as usize];

        // 获取实际的 style, thickness 和 user_style 数据
        unsafe {
            easyx_getlinestyle(
                &mut style_bits,
                &mut thickness,
                user_style_u32.as_mut_ptr(),
                &mut user_style_len,
            );
        }

        // 转换为 Vec<i32>
        let user_style: Vec<i32> = user_style_u32.iter().map(|&x| x as i32).collect();
        let major_style = style_bits & PS_STYLE_MASK;
        let style = style_bits & !PS_STYLE_MASK;

        Self {
            style: match major_style {
                PS_SOLID => InnerStyle::Solid(style),
                PS_DASH => InnerStyle::Dashed(style),
                PS_DOT => InnerStyle::Dotted(style),
                PS_DASHDOT => InnerStyle::DashDot(style),
                PS_DASHDOTDOT => InnerStyle::DashDotDot(style),
                PS_NULL => InnerStyle::Null(style),
                PS_USERSTYLE => InnerStyle::User { style, user_style },
                _ => {
                    unreachable!("Invalid line style");
                }
            },
            thickness,
        }
    }
}
