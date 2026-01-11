use easyx_sys::*;
use std::error::Error;
use std::ffi::CString;
use std::fmt;
use std::ptr;

use crate::color::Color;

/// 图像绘制操作码枚举
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Rop {
    /// 黑色填充
    Blackness,
    /// 反转源并擦除目标
    NotSrcErase,
    /// 反转源复制
    NotSrcCopy,
    /// 反转目标
    DstInvert,
    /// 反转源与目标的组合
    PatInvert,
    /// 源与目标的异或
    SrcInvert,
    /// 源与目标的与
    SrcAnd,
    /// 合并绘制
    MergePaint,
    /// 合并复制
    MergeCopy,
    /// 源复制（默认）
    SrcCopy,
    /// 源绘制
    SrcPaint,
    /// 图案复制
    PatCopy,
    /// 其他操作码
    Other(u32),
}

impl Rop {
    /// 将 Rop 转换为 u32
    pub fn as_u32(&self) -> u32 {
        match self {
            Rop::Blackness => 0x00000042,
            Rop::NotSrcErase => 0x001100A6,
            Rop::NotSrcCopy => 0x00330008,
            Rop::DstInvert => 0x00550009,
            Rop::PatInvert => 0x005A0049,
            Rop::SrcInvert => 0x00660046,
            Rop::SrcAnd => 0x008800C6,
            Rop::MergePaint => 0x00BB0226,
            Rop::MergeCopy => 0x00C000CA,
            Rop::SrcCopy => 0x00CC0020,
            Rop::SrcPaint => 0x00EE0086,
            Rop::PatCopy => 0x00F00021,
            Rop::Other(code) => *code,
        }
    }
}

impl From<u32> for Rop {
    fn from(code: u32) -> Self {
        match code {
            0x00000042 => Rop::Blackness,
            0x001100A6 => Rop::NotSrcErase,
            0x00330008 => Rop::NotSrcCopy,
            0x00550009 => Rop::DstInvert,
            0x005A0049 => Rop::PatInvert,
            0x00660046 => Rop::SrcInvert,
            0x008800C6 => Rop::SrcAnd,
            0x00BB0226 => Rop::MergePaint,
            0x00C000CA => Rop::MergeCopy,
            0x00CC0020 => Rop::SrcCopy,
            0x00EE0086 => Rop::SrcPaint,
            0x00F00021 => Rop::PatCopy,
            other => Rop::Other(other),
        }
    }
}

/// 图像操作相关错误
#[derive(Debug, PartialEq, Eq)]
pub enum ImageError {
    /// 文件未找到
    FileNotFound,
    /// 资源未找到
    ResourceNotFound,
    /// 未知错误
    Unknown(i32),
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageError::FileNotFound => write!(f, "文件未找到"),
            ImageError::ResourceNotFound => write!(f, "资源未找到"),
            ImageError::Unknown(code) => write!(f, "未知错误，错误码: {}", code),
        }
    }
}

impl Error for ImageError {}

impl From<i32> for ImageError {
    fn from(code: i32) -> Self {
        match code {
            0 => unreachable!("成功不应转换为错误"),
            2 => ImageError::FileNotFound,
            5007 => ImageError::ResourceNotFound,
            other => ImageError::Unknown(other),
        }
    }
}

/// 图像结构体，用于包装 EasyX 的 IMAGE 类型
#[derive(Debug, PartialEq)]
pub struct Image {
    ptr: *mut std::os::raw::c_void,
}

impl Image {
    /// 创建一个新的图像
    pub fn new(width: i32, height: i32) -> Self {
        let ptr = unsafe { easyx_create_image(width, height) };
        Self { ptr }
    }

    /// 获取图像宽度
    pub fn width(&self) -> i32 {
        unsafe { easyx_image_getwidth(self.ptr) }
    }

    /// 获取图像高度
    pub fn height(&self) -> i32 {
        unsafe { easyx_image_getheight(self.ptr) }
    }

    /// 调整图像大小
    pub fn resize(&mut self, width: i32, height: i32) {
        unsafe {
            easyx_image_resize(self.ptr, width, height);
        }
    }

    /// 从文件加载图像
    pub fn load_file(
        path: &str,
        width: i32,
        height: i32,
        resize: bool,
    ) -> Result<Self, ImageError> {
        let c_path = CString::new(path).map_err(|_| ImageError::Unknown(-1))?;
        let img = Self::new(width, height);
        let result =
            unsafe { easyx_loadimage_file(img.ptr, c_path.as_ptr(), width, height, resize as i32) };
        if result == 0 {
            Ok(img)
        } else {
            Err(result.into())
        }
    }

    /// 从资源加载图像
    pub fn load_resource(
        ty: &str,
        name: &str,
        width: i32,
        height: i32,
        resize: bool,
    ) -> Result<Self, ImageError> {
        let c_res_type = CString::new(ty).map_err(|_| ImageError::Unknown(-1))?;
        let c_res_name = CString::new(name).map_err(|_| ImageError::Unknown(-1))?;
        let img = Self::new(width, height);
        let result = unsafe {
            easyx_loadimage_resource(
                img.ptr,
                c_res_type.as_ptr(),
                c_res_name.as_ptr(),
                width,
                height,
                resize as i32,
            )
        };
        if result == 0 {
            Ok(img)
        } else {
            Err(result.into())
        }
    }

    /// 保存图像到文件
    pub fn save(&self, path: &str) -> Result<(), ImageError> {
        let c_path = CString::new(path).map_err(|_| ImageError::Unknown(-1))?;
        unsafe {
            easyx_saveimage(c_path.as_ptr(), self.ptr);
        }
        Ok(())
    }

    /// 从屏幕获取图像
    pub fn get_image(x: i32, y: i32, width: i32, height: i32) -> Self {
        let img = Self::new(width, height);
        unsafe {
            easyx_getimage(img.ptr, x, y, width, height);
        }
        img
    }

    /// 绘制图像
    pub fn put_image(&self, x: i32, y: i32) {
        self.put_image_rop(x, y, Rop::SrcCopy);
    }

    /// 绘制图像
    pub fn put_image_rop(&self, x: i32, y: i32, rop: impl Into<Rop>) {
        unsafe {
            easyx_putimage(x, y, self.ptr, rop.into().as_u32());
        }
    }

    /// 绘制图像的一部分
    #[allow(clippy::too_many_arguments)]
    pub fn put_image_part(&self, x: i32, y: i32, width: i32, height: i32, src_x: i32, src_y: i32) {
        self.put_image_part_rop(x, y, width, height, src_x, src_y, Rop::SrcCopy);
    }

    /// 绘制图像的一部分
    #[allow(clippy::too_many_arguments)]
    pub fn put_image_part_rop(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        src_x: i32,
        src_y: i32,
        rop: impl Into<Rop>,
    ) {
        unsafe {
            easyx_putimage_part(
                x,
                y,
                width,
                height,
                self.ptr,
                src_x,
                src_y,
                rop.into().as_u32(),
            );
        }
    }

    /// 旋转图像
    pub fn rotate(
        &self,
        radian: f64,
        bkcolor: impl Into<Color>,
        autosize: bool,
        highquality: bool,
    ) -> Self {
        let img = Self::new(self.width(), self.height());
        let bkcolor = bkcolor.into();
        unsafe {
            easyx_rotateimage(
                img.ptr,
                self.ptr,
                radian,
                bkcolor.as_colorref(),
                autosize as i32,
                highquality as i32,
            );
        }
        img
    }

    /// 获取图像缓冲区
    pub fn buffer(&self) -> *mut u32 {
        unsafe { easyx_getimagebuffer(self.ptr) }
    }

    /// 获取当前工作图像
    pub fn working_image() -> Option<Self> {
        let ptr = unsafe { easyx_getworkingimage() };
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// 设置当前工作图像
    pub fn set_working_image(&self) {
        unsafe {
            easyx_setworkingimage(self.ptr);
        }
    }

    /// 重置当前工作图像
    pub fn reset_working_image() {
        unsafe {
            easyx_setworkingimage(ptr::null_mut());
        }
    }

    /// 直接赋值图像，仅拷贝源图像的内容，不拷贝绘图环境
    pub fn assign(&mut self, src: &Self) {
        unsafe {
            easyx_copy_image(self.ptr, src.ptr);
        }
    }

    pub fn as_mut_ptr(&self) -> *mut std::os::raw::c_void {
        self.ptr
    }

    /// # Safety
    /// 确保 `ptr` 指向一个有效的 `IMAGE` 结构体实例。
    pub unsafe fn from_raw(ptr: *mut std::os::raw::c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for Image {
    /// 释放图像资源
    fn drop(&mut self) {
        unsafe {
            easyx_destroy_image(self.ptr);
        }
    }
}
