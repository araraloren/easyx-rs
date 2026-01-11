use easyx_sys::*;

use crate::image::Image;

/// 填充图案样式
/// 
/// 定义了不同类型的填充图案，用于当填充样式为图案填充时使用
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Hatch {
    /// 水平线条图案
    Horizontal = 0,
    /// 垂直线条图案
    Vertical = 1,
    /// 正向对角线图案
    FDiagonal = 2,
    /// 反向对角线图案
    BDiagonal = 3,
    /// 十字线条图案
    Cross = 4,
    /// 对角线十字图案
    DiagCross = 5,
}

/// 填充样式
/// 
/// 定义了不同类型的填充样式，用于图形绘制时的填充效果
#[derive(Debug)]
pub enum FillStyle {
    /// 实色填充
    /// 
    /// 使用当前填充颜色进行实色填充
    Solid,
    
    /// 无填充
    /// 
    /// 不进行任何填充
    Null,
    
    /// 图案填充
    /// 
    /// 使用指定的图案样式进行填充
    Hatched(Hatch),
    
    /// 图像填充
    /// 
    /// 使用指定的图像进行填充
    Pattern(Image),
    
    /// DIB图像填充
    /// 
    /// 使用指定的DIB图像进行填充
    DibPattern(Image),
    
    /// 8x8位图填充
    /// 
    /// 使用8x8的位图进行填充，每个字节的每一位代表一个像素，1表示使用当前填充颜色，0表示透明
    Pattern8x8([u8; 8]),
}

impl FillStyle {
    /// 创建实色填充样式
    pub fn solid() -> Self {
        Self::Solid
    }

    /// 创建无填充样式
    pub fn null() -> Self {
        Self::Null
    }

    /// 创建图案填充样式
    pub fn hatched(hatch: Hatch) -> Self {
        Self::Hatched(hatch)
    }

    /// 创建图像填充样式
    pub fn pattern(img: Image) -> Self {
        Self::Pattern(img)
    }

    /// 创建DIB图像填充样式
    pub fn dib_pattern(img: Image) -> Self {
        Self::DibPattern(img)
    }

    /// 创建8x8位图填充样式
    pub fn pattern8x8(pattern8x8: [u8; 8]) -> Self {
        Self::Pattern8x8(pattern8x8)
    }

    /// 获取填充样式类型代码
    /// 
    /// 内部使用，返回填充样式对应的整数类型代码
    pub fn get_style(&self) -> i32 {
        match self {
            Self::Solid => 0,
            Self::Null => 1,
            Self::Hatched(_) => 2,
            Self::Pattern(_) => 3,
            Self::DibPattern(_) => 5,
            Self::Pattern8x8(_) => 3,
        }
    }

    /// 获取图案填充的图案类型
    /// 
    /// 如果当前填充样式是图案填充，则返回图案类型；否则返回None
    pub fn as_hatch(&self) -> Option<&Hatch> {
        match self {
            Self::Hatched(hatch) => Some(hatch),
            _ => None,
        }
    }

    /// 设置图案填充的图案类型
    /// 
    /// 如果当前填充样式是图案填充，则更新图案类型；否则不做任何操作
    pub fn set_hatch(&mut self, hatch: Hatch) {
        if let Self::Hatched(h) = self {
            *h = hatch;
        }
    }

    /// 获取图像填充的图像
    /// 
    /// 如果当前填充样式是图像填充，则返回图像引用；否则返回None
    pub fn as_pattern(&self) -> Option<&Image> {
        match self {
            Self::Pattern(img) => Some(img),
            Self::DibPattern(img) => Some(img),
            _ => None,
        }
    }

    /// 设置图像填充的图像
    /// 
    /// 如果当前填充样式是图像填充，则更新图像；否则不做任何操作
    pub fn set_pattern(&mut self, pattern: Image) {
        match self {
            Self::Pattern(img) => *img = pattern,
            Self::DibPattern(img) => *img = pattern,
            _ => {}        }
    }
}

impl FillStyle {
    /// 应用填充样式
    /// 
    /// 将当前填充样式应用到设备上下文
    pub fn apply(&self) {
        let style = self.get_style();

        match self {
            Self::Solid => unsafe { easyx_setfillstyle(style, 0, std::ptr::null_mut()) },
            Self::Null => unsafe { easyx_setfillstyle(style, 0, std::ptr::null_mut()) },
            Self::Hatched(hatch) => unsafe {
                easyx_setfillstyle(style, *hatch as i32, std::ptr::null_mut())
            },
            Self::Pattern(img) => unsafe { easyx_setfillstyle(style, 0, img.as_mut_ptr()) },
            Self::DibPattern(img) => unsafe { easyx_setfillstyle(style, 0, img.as_mut_ptr()) },
            Self::Pattern8x8(pattern8x8) => unsafe {
                easyx_setfillstyle_pattern(pattern8x8.as_ptr());
            },
        }
    }

    /// 获取当前设备上下文的填充样式
    /// 
    /// # 返回值
    /// 当前设备上下文的填充样式
    pub fn current() -> Self {
        let mut style = 0;
        let mut hatch = 0;
        let mut ppattern = std::ptr::null_mut();

        unsafe {
            // 调用C++函数获取当前填充样式
            easyx_getfillstyle(&mut style, &mut hatch, &mut ppattern);
        }

        // 根据样式类型构造对应的FillStyle变体
        match style {
            0 => Self::Solid,
            1 => Self::Null,
            2 => {
                // 图案填充，转换hatch值为Hatch枚举
                let hatch_enum = match hatch {
                    0 => Hatch::Horizontal,
                    1 => Hatch::Vertical,
                    2 => Hatch::FDiagonal,
                    3 => Hatch::BDiagonal,
                    4 => Hatch::Cross,
                    5 => Hatch::DiagCross,
                    _ => unreachable!("无效的图案填充值: {}", hatch),
                };
                Self::Hatched(hatch_enum)
            }
            3 => {
                // 图像填充，转换ppattern为Image
                if ppattern.is_null() {
                    unreachable!("图像指针为空")
                } else {
                    // 使用Image::from_raw从原始指针创建Image
                    unsafe { Self::Pattern(Image::from_raw(ppattern)) }
                }
            }
            5 => {
                // DIB图像填充，转换ppattern为Image
                if ppattern.is_null() {
                    unreachable!("DIB图像指针为空")
                } else {
                    // 使用Image::from_raw从原始指针创建Image
                    unsafe { Self::DibPattern(Image::from_raw(ppattern)) }
                }
            }
            _ => unreachable!("无效的填充样式值: {}", style),
        }
    }
}