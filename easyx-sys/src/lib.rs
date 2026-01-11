// easyx-sys/lib.rs
// 原始 C 函数的 FFI 绑定

// 包含生成的绑定
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unnecessary_transmutes)]
#![allow(unsafe_op_in_unsafe_fn)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
