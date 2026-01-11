use easyx_sys::*;
use std::error::Error;
use std::ffi::CString;
use std::fmt;
use std::ptr;

use crate::color::Color;

/// 图像绘制操作码枚举
/// 
/// 定义了图像绘制时的各种操作码，控制图像如何与目标区域组合
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Rop {
    /// 黑色填充
    /// 
    /// 将目标区域填充为黑色
    Blackness,
    /// 反转源并擦除目标
    /// 
    /// 将源图像反转后与目标区域进行擦除操作
    NotSrcErase,
    /// 反转源复制
    /// 
    /// 将源图像反转后复制到目标区域
    NotSrcCopy,
    /// 反转目标
    /// 
    /// 将目标区域的颜色反转
    DstInvert,
    /// 反转源与目标的组合
    /// 
    /// 将源图案与目标区域的颜色反转组合
    PatInvert,
    /// 源与目标的异或
    /// 
    /// 将源图像与目标区域进行异或操作
    SrcInvert,
    /// 源与目标的与
    /// 
    /// 将源图像与目标区域进行与操作
    SrcAnd,
    /// 合并绘制
    /// 
    /// 将源图像与目标区域进行合并绘制
    MergePaint,
    /// 合并复制
    /// 
    /// 将源图像与目标区域进行合并复制
    MergeCopy,
    /// 源复制（默认）
    /// 
    /// 将源图像直接复制到目标区域（默认操作）
    SrcCopy,
    /// 源绘制
    /// 
    /// 将源图像绘制到目标区域
    SrcPaint,
    /// 图案复制
    /// 
    /// 将源图案复制到目标区域
    PatCopy,
    /// 其他操作码
    /// 
    /// 自定义操作码
    Other(u32),
}

impl Rop {
    /// 将 Rop 转换为 u32
    /// 
    /// # 返回值
    /// 对应的操作码数值
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
    /// 从 u32 转换为 Rop
    /// 
    /// # 参数
    /// - `code`: 操作码数值
    /// 
    /// # 返回值
    /// 对应的 Rop 枚举值
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
/// 
/// 定义了图像操作过程中可能发生的各种错误
#[derive(Debug, PartialEq, Eq)]
pub enum ImageError {
    /// 文件未找到
    /// 
    /// 当尝试从不存在的文件加载图像时返回此错误
    FileNotFound,
    /// 资源未找到
    /// 
    /// 当尝试从不存在的资源加载图像时返回此错误
    ResourceNotFound,
    /// 未知错误
    /// 
    /// 其他未分类的错误，包含错误码
    Unknown(i32),
}

impl fmt::Display for ImageError {
    /// 格式化图像错误为字符串
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
    /// 从错误码转换为 ImageError
    /// 
    /// # 参数
    /// - `code`: 错误码
    /// 
    /// # 返回值
    /// 对应的 ImageError 枚举值
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
/// 
/// Image 结构体是 EasyX-RS 中表示图像的核心类型，
/// 支持图像的创建、加载、绘制、保存等操作
#[derive(Debug, PartialEq)]
pub struct Image {
    ptr: *mut std::os::raw::c_void,
}

impl Image {
    /// 创建一个新的图像
    /// 
    /// # 参数
    /// - `width`: 图像宽度
    /// - `height`: 图像高度
    /// 
    /// # 返回值
    /// 新创建的 Image 对象
    pub fn new(width: i32, height: i32) -> Self {
        let ptr = unsafe { easyx_create_image(width, height) };
        Self { ptr }
    }

    /// 获取图像宽度
    /// 
    /// # 返回值
    /// 图像宽度（像素）
    pub fn width(&self) -> i32 {
        unsafe { easyx_image_getwidth(self.ptr) }
    }

    /// 获取图像高度
    /// 
    /// # 返回值
    /// 图像高度（像素）
    pub fn height(&self) -> i32 {
        unsafe { easyx_image_getheight(self.ptr) }
    }

    /// 调整图像大小
    /// 
    /// # 参数
    /// - `width`: 新的宽度
    /// - `height`: 新的高度
    pub fn resize(&mut self, width: i32, height: i32) {
        unsafe {
            easyx_image_resize(self.ptr, width, height);
        }
    }

    /// 从文件加载图像
    /// 
    /// # 参数
    /// - `path`: 图像文件路径
    /// - `width`: 图像宽度，0表示使用原始宽度
    /// - `height`: 图像高度，0表示使用原始高度
    /// - `resize`: 是否调整图像大小以适应指定的宽高
    /// 
    /// # 返回值
    /// 成功返回 Image 对象，失败返回 ImageError
    pub fn load_file(
        path: &str,
        width: i32,
        height: i32,
        resize: bool,
    ) -> Result<Self, ImageError> {
        let c_path = CString::new(path).map_err(|_| ImageError::Unknown(-1))?;
        let img = Self::new(width, height);
        let result = unsafe {
            easyx_loadimage_file(img.ptr, c_path.as_ptr(), width, height, resize as i32)
        };
        if result == 0 {
            Ok(img)
        } else {
            Err(result.into())
        }
    }

    /// 从资源加载图像
    /// 
    /// # 参数
    /// - `ty`: 资源类型
    /// - `name`: 资源名称
    /// - `width`: 图像宽度，0表示使用原始宽度
    /// - `height`: 图像高度，0表示使用原始高度
    /// - `resize`: 是否调整图像大小以适应指定的宽高
    /// 
    /// # 返回值
    /// 成功返回 Image 对象，失败返回 ImageError
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
    /// 
    /// # 参数
    /// - `path`: 保存路径
    /// 
    /// # 返回值
    /// 成功返回 Ok(())，失败返回 ImageError
    pub fn save(&self, path: &str) -> Result<(), ImageError> {
        let c_path = CString::new(path).map_err(|_| ImageError::Unknown(-1))?;
        unsafe {
            easyx_saveimage(c_path.as_ptr(), self.ptr);
        }
        Ok(())
    }

    /// 从屏幕获取图像
    /// 
    /// # 参数
    /// - `x`: 屏幕起始x坐标
    /// - `y`: 屏幕起始y坐标
    /// - `width`: 图像宽度
    /// - `height`: 图像高度
    /// 
    /// # 返回值
    /// 从屏幕获取的 Image 对象
    pub fn get_image(x: i32, y: i32, width: i32, height: i32) -> Self {
        let img = Self::new(width, height);
        unsafe {
            easyx_getimage(img.ptr, x, y, width, height);
        }
        img
    }

    /// 绘制图像
    /// 
    /// 使用默认的 SrcCopy 操作码绘制图像
    /// 
    /// # 参数
    /// - `x`: 目标位置x坐标
    /// - `y`: 目标位置y坐标
    pub fn put_image(&self, x: i32, y: i32) {
        self.put_image_rop(x, y, Rop::SrcCopy);
    }

    /// 使用指定操作码绘制图像
    /// 
    /// # 参数
    /// - `x`: 目标位置x坐标
    /// - `y`: 目标位置y坐标
    /// - `rop`: 绘制操作码
    pub fn put_image_rop(&self, x: i32, y: i32, rop: impl Into<Rop>) {
        unsafe {
            easyx_putimage(x, y, self.ptr, rop.into().as_u32());
        }
    }

    /// 绘制图像的一部分
    /// 
    /// 使用默认的 SrcCopy 操作码绘制图像的一部分
    /// 
    /// # 参数
    /// - `x`: 目标位置x坐标
    /// - `y`: 目标位置y坐标
    /// - `width`: 绘制宽度
    /// - `height`: 绘制高度
    /// - `src_x`: 源图像起始x坐标
    /// - `src_y`: 源图像起始y坐标
    #[allow(clippy::too_many_arguments)]
    pub fn put_image_part(&self, x: i32, y: i32, width: i32, height: i32, src_x: i32, src_y: i32) {
        self.put_image_part_rop(x, y, width, height, src_x, src_y, Rop::SrcCopy);
    }

    /// 使用指定操作码绘制图像的一部分
    /// 
    /// # 参数
    /// - `x`: 目标位置x坐标
    /// - `y`: 目标位置y坐标
    /// - `width`: 绘制宽度
    /// - `height`: 绘制高度
    /// - `src_x`: 源图像起始x坐标
    /// - `src_y`: 源图像起始y坐标
    /// - `rop`: 绘制操作码
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
    /// 
    /// # 参数
    /// - `radian`: 旋转角度（弧度）
    /// - `bkcolor`: 背景颜色
    /// - `autosize`: 是否自动调整图像大小以容纳旋转后的图像
    /// - `highquality`: 是否使用高质量旋转
    /// 
    /// # 返回值
    /// 旋转后的新 Image 对象
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
    /// 
    /// # 返回值
    /// 指向图像像素数据的指针，每个像素为32位RGBA格式
    pub fn buffer(&self) -> *mut u32 {
        unsafe { easyx_getimagebuffer(self.ptr) }
    }

    /// 获取当前工作图像
    /// 
    /// # 返回值
    /// 如果有当前工作图像，返回 Some(Image)，否则返回 None
    pub fn working_image() -> Option<Self> {
        let ptr = unsafe { easyx_getworkingimage() };
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// 设置当前工作图像
    /// 
    /// 将当前图像设置为工作图像，后续的绘图操作将作用于该图像
    pub fn set_working_image(&self) {
        unsafe {
            easyx_setworkingimage(self.ptr);
        }
    }

    /// 重置当前工作图像
    /// 
    /// 将当前工作图像重置为默认值，后续的绘图操作将作用于屏幕
    pub fn reset_working_image() {
        unsafe {
            easyx_setworkingimage(ptr::null_mut());
        }
    }

    /// 直接赋值图像，仅拷贝源图像的内容，不拷贝绘图环境
    /// 
    /// # 参数
    /// - `src`: 源图像
    pub fn assign(&mut self, src: &Self) {
        unsafe {
            easyx_copy_image(self.ptr, src.ptr);
        }
    }

    /// 获取图像的原始指针
    /// 
    /// # 返回值
    /// 指向底层 IMAGE 结构的指针
    pub fn as_mut_ptr(&self) -> *mut std::os::raw::c_void {
        self.ptr
    }

    /// 从原始指针创建 Image 对象
    /// 
    /// # Safety
    /// 确保 `ptr` 指向一个有效的 `IMAGE` 结构体实例。
    /// 
    /// # 参数
    /// - `ptr`: 指向 IMAGE 结构的指针
    /// 
    /// # 返回值
    /// 新的 Image 对象
    pub unsafe fn from_raw(ptr: *mut std::os::raw::c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for Image {
    /// 释放图像资源
    /// 
    /// 当 Image 对象被销毁时，自动释放底层的 IMAGE 结构资源
    fn drop(&mut self) {
        unsafe {
            easyx_destroy_image(self.ptr);
        }
    }
}