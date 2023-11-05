# img_stegano
Image steganography with Rust

## Introduction

This Rust program allows you to hide a text message within an image using the Least Significant Bit (LSB) replacement technique. The LSB replacement technique involves altering the least significant bits of the image's pixels to encode the hidden message. This technique is a form of steganography, which is the practice of concealing one piece of information within another.

See [this whitepaper](https://core.ac.uk/download/pdf/235050007.pdf) for reference

## Features

- **Encoding Functions**: You can use the `encode_from_u8_array`, `encode_from_path` or `encode_from_image` functions to embed a text message into an image using LSB replacement. The output image remains visually similar, with the hidden message stored in the least significant bits of the red channel.

- **Decoding Functions**: The `decode_from_u8_array`, `decode_from_path`, `decode_from_image` function allows you to extract the hidden message from an encoded image.

## Usage

1. Add `img_stegano`, `git` dependency in your `Cargo.toml`.

```toml
[dependencies]
img_stegano = {git = "https://github.com/collinsmuriuki/img_stegano.git"}
```

2. Import the `ImgStegano` to use as demonstrated in the following example:

`encode_from_image`: Encode and decode from [`DynamicImage`](https://creative-coding-the-hard-way.github.io/Agents/image/enum.DynamicImage.html)

```rust,no_run
use img_stegano::ImgStegano;

fn main() {
    let image = img_stegano::image::open("dice.png").expect("failed to open image");
    let encoded = ImgStegano::encode_from_image(&image, "foo bar");
    encoded
        .save_with_format("out.png", img_stegano::image::ImageFormat::Png)
        .expect("Failed to save out.png");
    let encoded = img_stegano::image::open("out.png").expect("Failed to open encoded out.png file");
    let decoded_text = ImgStegano::decode_from_image(&encoded);
    println!("{decoded_text}");
    assert_eq!(decoded_text, "foo bar".to_string());
}
```

`encode_from_u8_array`: Encode and decode from `u8` array

```rust,no_run
use std::{fs::File, io::Read};
use img_stegano::ImgStegano;

fn main() {
    // Load the input image
    let mut file = File::open("dice.png").expect("failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    // encode from buffer
    let encoded = ImgStegano::encode_from_u8_array(&buffer, "png", "foo bar")
        .expect("Failed to encode message to image");
    let encoded = img_stegano::image::load_from_memory_with_format(&encoded, img_stegano::image::ImageFormat::Png)
        .expect("Failed to load image");
    
    // save encoded file
    encoded
        .save_with_format("out2.png", img_stegano::image::ImageFormat::Png)
        .expect("Failed to save out2.png");

    // decode saved encoded file
    let mut decoded = File::open("out2.png").expect("Failed to open input file");
    let mut decoded_buffer = Vec::new();
    decoded.read_to_end(&mut decoded_buffer).unwrap();

    // decode from buffer
    let decoded_text = ImgStegano::decode_from_u8_array(&decoded_buffer).expect("Failed to decode image");
    println!("{decoded_text}");
    assert_eq!(decoded_text, "foo bar".to_string());
}
```

`encode_from_path`: Encode and decode from path

```rust,no_run
use img_stegano::ImgStegano;

fn main() {
    // encode from file path
    let encoded = ImgStegano::encode_from_path("dice.png", "foo bar")
        .expect("Failed to encode text to image");
    // save encoded file
    encoded
        .save_with_format("out3.png", img_stegano::image::ImageFormat::Png)
        .expect("Failed to save image");
    // decode from saved encoded file path
    let decoded_text =
        ImgStegano::decode_from_path("out3.png").expect("Failed to decode text from image");
    println!("{decoded_text}");
    assert_eq!(decoded_text, "foo bar".to_string());
}
```

## License
This library is licensed under the [MIT License](./LICENSE). Feel free to contribute, report issues, or suggest improvements. Enjoy using ImgStegano for image steganography!