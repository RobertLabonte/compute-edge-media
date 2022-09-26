//! FFMPEG low-level bindings for Rust, precompiled for WebAssembly/WASI.
//!
//! This crate bundles FFMPEG's `avcodec` and `avformat` libraries, precompiled for WebAssembly. No native installation required.
//!
//! Compatible with Fastly's Compute@Edge.
//!
//! These are *low-level* bindings, directly exposing the original C functions to Rust.
//!
//! ## Usage
//!
//! ```toml
//! [dependencies]
//! media_sdk = "0"
//! ```

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(
    non_camel_case_types,
    clashing_extern_declarations,
    non_upper_case_globals,
    non_snake_case,
    improper_ctypes
)]

pub mod ffmpeg {
    pub mod avcodec;
    pub mod avfilter;
    pub mod avformat;
    pub mod avutil;
    pub mod error;
    pub mod imgutils;
    pub mod pixdesc;
    pub mod swscale;
}

use std::alloc::{GlobalAlloc, Layout};

/// Allocator backed by the ffmpeg memory allocation functions.
///
/// Must be set as a global allocator with:
///
/// ```rust
/// #[global_allocator]
/// static ALLOCATOR: FFMpegAllocator = FFMpegAllocator;
/// ```
pub struct FFMpegAllocator;

unsafe impl GlobalAlloc for FFMpegAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ffmpeg::avutil::av_malloc(layout.size()) as *mut _
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        ffmpeg::avutil::av_free(ptr as *mut _)
    }
}

#[global_allocator]
static ALLOCATOR: FFMpegAllocator = FFMpegAllocator;

#[test]
pub fn test_ffmpeg() {
    unsafe {
        avformat::avformat_version();
    }
}
