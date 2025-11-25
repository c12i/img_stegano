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
use img_stegano::{encode_from_image, decode_from_image, Image, image::ImageFormat};

fn main() {
    let image = Image::open("dice.png").expect("Failed to open image");
    let encoded = encode_from_image(image, "foo bar");
    encoded.save("out.png", ImageFormat::Png).expect("Failed to save image");

    let encoded = Image::open("out.png").expect("Failed to open image");
    let decoded_text = decode_from_image(&encoded);
    assert_eq!(decoded_text, "foo bar");
}
```

`encode_from_u8_array`: Encode and decode from `u8` array

```rust,no_run
use std::{fs::File, io::Read};
use img_stegano::{encode_from_u8_array, decode_from_u8_array, Image, image::ImageFormat};

fn main() {
    let mut file = File::open("dice.png").expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let encoded = encode_from_u8_array(&buffer, "png", "foo bar").expect("Failed to encode");
    let image = Image::open_from_u8_array(&encoded, ImageFormat::Png).expect("Failed to load image");
    image.save("out2.png", ImageFormat::Png).expect("Failed to save image");

    let mut file = File::open("out2.png").expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let decoded_text = decode_from_u8_array(&buffer).expect("Failed to decode");
    assert_eq!(decoded_text, "foo bar");
}
```

`encode_from_path`: Encode and decode from path

```rust,no_run
use img_stegano::{encode_from_path, decode_from_path, image::ImageFormat};

fn main() {
    let encoded = encode_from_path("dice.png", "foo bar").expect("Failed to encode");
    encoded.save("out3.png", ImageFormat::Png).expect("Failed to save image");

    let decoded_text = decode_from_path("out3.png").expect("Failed to decode");
    assert_eq!(decoded_text, "foo bar");
}
```

## cli

You can interact with this program in your command line by installing via cargo:

```sh
cargo install --git https://github.com/c12i/img_stegano.git
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
This project is licensed under the [MIT License](./LICENSE). Feel free to contribute, report issues, or suggest improvements. Enjoy using ImgStegano for image steganography!