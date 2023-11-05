#![doc = include_str!("../README.md")]

pub use img_stegano_core::{Image, ImgStegano, ImgSteganoError};

pub mod image {
    pub use img_stegano_core::ImageFormat;
}
