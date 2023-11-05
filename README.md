# img_stegano
Image steganography with Rust

## Introduction

This Rust program allows you to hide a text message within an image using the Least Significant Bit (LSB) replacement technique. The LSB replacement technique involves altering the least significant bits of the image's pixels to encode the hidden message. This technique is a form of steganography, which is the practice of concealing one piece of information within another.

See [this whitepaper](https://core.ac.uk/download/pdf/235050007.pdf) for reference

## Features

- **Encoding Functions**: You can use the `encode_from_u8_array`, `encode_from_path` or `encode_from_image` functions to embed a text message into an image using LSB replacement. The output image remains visually similar, with the hidden message stored in the least significant bits of the red channel.

- **Decoding Functions**: The `decode_from_u8_array`, `decode_from_path`, `decode_from_image` function allows you to extract the hidden message from an encoded image.

## Usage

1. Add `img_stegano` as a `git` dependency in your `Cargo.toml`.

```toml
[dependencies]
img_stegano = {git = "https://github.com/collinsmuriuki/img_stegano.git"}
```

2. Import the `ImgStegano` to use as demonstrated in the following example:

`encode_from_image`: Encode and decode from an `Image` struct

```rust,no_run
use img_stegano::{ImgStegano, Image, image::ImageFormat};

fn main() {
    // open image
    let image = Image::open("dice.png").expect("Failed to open image");

    // encode message to image
    let encoded = ImgStegano::encode_from_image(image, "foo bar");

    // save text encoded image
    encoded.save("out.png", ImageFormat::Png).expect("Failed to save out.png");

    // open text encoded image
    let encoded = Image::open("out.png").expect("Failed to open encoded out.png file");

    // decode text from image
    let decoded_text = ImgStegano::decode_from_image(&encoded);
    println!("{decoded_text}");
    assert_eq!(decoded_text, "foo bar".to_string());
}
```

`encode_from_u8_array`: Encode and decode from `u8` array

```rust,no_run
use std::{fs::File, io::Read};
use img_stegano::{ImgStegano, Image, image::ImageFormat};

fn main() {
    // Load the input image
    let mut file = File::open("dice.png").expect("failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    // encode from buffer
    let encoded = ImgStegano::encode_from_u8_array(&buffer, "png", "foo bar").expect("Failed to encode message to image");
    
    // save text encoded image
    let encoded = Image::open_from_u8_array(&encoded, ImageFormat::Png).expect("Failed to load image");
    encoded
        .save("out2.png", ImageFormat::Png)
        .expect("Failed to save out2.png");

    // open saved text encoded image
    let mut decoded = File::open("out2.png").expect("Failed to open input file");
    let mut decoded_buffer = Vec::new();
    decoded.read_to_end(&mut decoded_buffer).unwrap();

    // decode encoded text from image buffer
    let decoded_text = ImgStegano::decode_from_u8_array(&decoded_buffer).expect("Failed to decode image");
    println!("{decoded_text}");
    assert_eq!(decoded_text, "foo bar".to_string());
}
```

`encode_from_path`: Encode and decode from path

```rust,no_run
use img_stegano::{ImgStegano, Image, image::ImageFormat};

fn main() {
    // encode from file path
    let encoded = ImgStegano::encode_from_path("dice.png", "foo bar").expect("Failed to encode text to image");

    // save text encoded image
    encoded
        .save("out3.png", ImageFormat::Png)
        .expect("Failed to save image");

    // decode saved text encoded image from its path
    let decoded_text = ImgStegano::decode_from_path("out3.png").expect("Failed to decode text from image");
    println!("{decoded_text}");
    assert_eq!(decoded_text, "foo bar".to_string());
}
```

## cli

You can interact with this program via your command line by installing via cargo:

```sh
cargo install --git https://github.com/collinsmuriuki/img_stegano.git
```

Encode:

```sh
img_stegano_cli encode --input "dice.png" --output "out.png" --message "foo bar" --fmt "png"
```

Decode:

```sh
img_stegano_cli decode --input "out.png"
```

## License
This library is licensed under the [MIT License](./LICENSE). Feel free to contribute, report issues, or suggest improvements. Enjoy using ImgStegano for image steganography!