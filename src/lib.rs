#![doc = include_str!("../README.md")]

pub use img_stegano_core::{
    decode_from_image, decode_from_path, decode_from_u8_array, encode_from_image, encode_from_path,
    encode_from_u8_array, Image, ImgSteganoError,
};

pub mod image {
    pub use img_stegano_core::ImageFormat;
}
