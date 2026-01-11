use easyx_sys::*;

bitflags::bitflags! {
    /// Endcap styles for EasyX graphics library.
    ///
    /// These flags can be combined using the `|` operator to set multiple options.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct EndCapStyle: u32 {
        /// End cap style.
        const EndCapRound = PS_ENDCAP_ROUND;
        /// End cap style.
        const EndCapSquare = PS_ENDCAP_SQUARE;
        /// End cap style.
        const EndCapFlat = PS_ENDCAP_FLAT;
    }
}

bitflags::bitflags! {
    /// Join styles for EasyX graphics library.
    ///
    /// These flags can be combined using the `|` operator to set multiple options.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct JoinStyle: u32 {
        /// Line join style.
        const JoinRound = PS_JOIN_ROUND;
        /// Line join style.
        const JoinBevel = PS_JOIN_BEVEL;
        /// Line join style.
        const JoinMiter = PS_JOIN_MITER;
    }

}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InnerStyle {
    Solid(u32),
    Dashed(u32),
    Dotted(u32),
    DashDot(u32),
    DashDotDot(u32),
    Null(u32),
    User { style: u32, user_style: Vec<i32> },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LineStyle {
    style: InnerStyle,
    thickness: i32,
}

impl LineStyle {
    pub fn solid(thickness: i32) -> Self {
        Self {
            style: InnerStyle::Solid(0),
            thickness,
        }
    }

    pub fn dashed(thickness: i32) -> Self {
        Self {
            style: InnerStyle::Dashed(0),
            thickness,
        }
    }

    pub fn dotted(thickness: i32) -> Self {
        Self {
            style: InnerStyle::Dotted(0),
            thickness,
        }
    }

    pub fn dash_dot(thickness: i32) -> Self {
        Self {
            style: InnerStyle::DashDot(0),
            thickness,
        }
    }

    pub fn dash_dot_dot(thickness: i32) -> Self {
        Self {
            style: InnerStyle::DashDotDot(0),
            thickness,
        }
    }

    pub fn null(thickness: i32) -> Self {
        Self {
            style: InnerStyle::Null(0),
            thickness,
        }
    }

    pub fn user(thickness: i32, user_style: Vec<i32>) -> Self {
        Self {
            style: InnerStyle::User {
                style: 0,
                user_style,
            },
            thickness,
        }
    }

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

    pub fn thickness(&self) -> i32 {
        self.thickness
    }

    pub fn set_thickness(&mut self, thickness: i32) {
        self.thickness = thickness;
    }

    pub fn user_style(&self) -> Option<&[i32]> {
        match &self.style {
            InnerStyle::User {
                style: _,
                user_style,
            } => Some(user_style.as_slice()),
            _ => None,
        }
    }

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
