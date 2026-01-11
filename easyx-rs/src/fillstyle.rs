use easyx_sys::*;

use crate::image::Image;

/// Hatch styles for EasyX graphics library.
///
/// These values represent different hatch patterns that can be used when the fill style is Hatched.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Hatch {
    /// Horizontal hatch style.
    Horizontal = 0,
    /// Vertical hatch style.
    Vertical = 1,
    /// Forward diagonal hatch style.
    FDiagonal = 2,
    /// Backward diagonal hatch style.
    BDiagonal = 3,
    /// Cross hatch style.
    Cross = 4,
    /// Diagonal cross hatch style.
    DiagCross = 5,
}

#[derive(Debug)]
pub enum FillStyle {
    Solid,

    Null,

    Hatched(Hatch),

    Pattern(Image),

    DibPattern(Image),

    Pattern8x8([u8; 8]),
}

impl FillStyle {
    pub fn solid() -> Self {
        Self::Solid
    }

    pub fn null() -> Self {
        Self::Null
    }

    pub fn hatched(hatch: Hatch) -> Self {
        Self::Hatched(hatch)
    }

    pub fn pattern(img: Image) -> Self {
        Self::Pattern(img)
    }

    pub fn dib_pattern(img: Image) -> Self {
        Self::DibPattern(img)
    }

    pub fn pattern8x8(pattern8x8: [u8; 8]) -> Self {
        Self::Pattern8x8(pattern8x8)
    }

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

    pub fn as_hatch(&self) -> Option<&Hatch> {
        match self {
            Self::Hatched(hatch) => Some(hatch),
            _ => None,
        }
    }

    pub fn set_hatch(&mut self, hatch: Hatch) {
        if let Self::Hatched(h) = self {
            *h = hatch;
        }
    }

    pub fn as_pattern(&self) -> Option<&Image> {
        match self {
            Self::Pattern(img) => Some(img),
            Self::DibPattern(img) => Some(img),
            _ => None,
        }
    }

    pub fn set_pattern(&mut self, pattern: Image) {
        match self {
            Self::Pattern(img) => *img = pattern,
            Self::DibPattern(img) => *img = pattern,
            _ => {}
        }
    }
}

impl FillStyle {
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

    pub fn current() -> Self {
        let mut style = 0;
        let mut hatch = 0;
        let mut ppattern = std::ptr::null_mut();

        unsafe {
            // Call the C++ function to get the current fill style
            easyx_getfillstyle(&mut style, &mut hatch, &mut ppattern);
        }

        // Match the style and construct the appropriate FillStyle variant
        match style {
            0 => Self::Solid,
            1 => Self::Null,
            2 => {
                // Hatched style, convert hatch value to Hatch enum
                let hatch_enum = match hatch {
                    0 => Hatch::Horizontal,
                    1 => Hatch::Vertical,
                    2 => Hatch::FDiagonal,
                    3 => Hatch::BDiagonal,
                    4 => Hatch::Cross,
                    5 => Hatch::DiagCross,
                    _ => unreachable!("Invalid hatch value"),
                };
                Self::Hatched(hatch_enum)
            }
            3 => {
                // Pattern style, convert ppattern to Image
                if ppattern.is_null() {
                    unreachable!("Pattern pointer is null")
                } else {
                    // Use Image::from_raw to create an Image from the raw pointer
                    unsafe { Self::Pattern(Image::from_raw(ppattern)) }
                }
            }
            5 => {
                // DibPattern style, convert ppattern to Image
                if ppattern.is_null() {
                    unreachable!("Pattern pointer is null")
                } else {
                    // Use Image::from_raw to create an Image from the raw pointer
                    unsafe { Self::DibPattern(Image::from_raw(ppattern)) }
                }
            }
            _ => unreachable!("Invalid fill style value"),
        }
    }
}
