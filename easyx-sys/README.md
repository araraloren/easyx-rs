# easyx-sys

原始C绑定的EasyX图形库，为Rust提供底层FFI支持。

[English Version](README-en.md)

## 项目概述

easyx-sys是EasyX-RS项目的底层组件，提供了对EasyX C图形库的原始FFI（Foreign Function Interface）绑定。它作为中间层，连接上层Rust API和底层C实现，为`easyx-rs`库提供基础支持。

## 核心功能

- 自动生成的C到Rust FFI绑定
- 包含EasyX库的头文件和二进制文件
- 支持Windows平台
- 与EasyX 26.1.1版本兼容
- 提供基础的图形绘制、窗口管理、事件处理等底层API

## 项目结构

```
easyx-sys/
├── build.rs           # 构建脚本，用于生成绑定
├── Cargo.toml         # 项目配置
├── cpp/               # C++辅助代码
├── EasyX_26.1.1/      # 包含的EasyX库文件
└── src/
    └── lib.rs         # 库入口，导出生成的绑定
```

## 技术实现

1. **绑定生成**：使用`bindgen`工具自动从EasyX头文件生成Rust绑定
2. **构建系统**：使用`cc` crate编译C/C++代码
3. **静态链接**：包含EasyX库文件，实现静态链接
4. **安全封装**：提供原始C API的安全封装（在上层`easyx-rs`库中实现）

## 作为依赖使用

通常情况下，您不需要直接使用`easyx-sys`，而是使用上层的`easyx-rs`库。但如果您需要直接访问底层C API，可以在项目中添加依赖：

```toml
[dependencies]
easyx-sys = "0.1.3"
```

## 构建要求

- Windows 操作系统
- Rust 1.60+ 编译环境
- Visual Studio 或 MinGW 编译工具链

## 构建过程

1. 克隆仓库
2. 运行`cargo build`命令
3. 构建脚本会自动生成绑定并编译代码

## 许可证

本项目采用 MIT 许可证，详情请查看项目根目录的 LICENSE 文件。

## 相关项目

- [easyx-rs](https://github.com/araraloren/easyx-rs)：上层Rust API库
- [easyx-example](https://github.com/araraloren/easyx-rs/tree/main/easyx-example)：示例代码集
- [tetris](https://github.com/araraloren/easyx-rs/tree/main/tetris)：俄罗斯方块示例游戏

## 项目链接

- 项目主页：https://araraloren.github.io/easyx-rs/
- 文档：https://araraloren.github.io/easyx-rs/

## 致谢

感谢 EasyX 团队提供的优秀图形库！
