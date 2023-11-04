# `img_stegano_rs`: Image steganography with Rust

## Introduction

This Rust program allows you to hide a text message within an image using the Least Significant Bit (LSB) replacement technique. The LSB replacement technique involves altering the least significant bits of the image's pixels to encode the hidden message. This technique is a form of steganography, which is the practice of concealing one piece of information within another.

See [this whitepaper](https://core.ac.uk/download/pdf/235050007.pdf) for reference

## Features

- **Encoding Function**: You can use the `encode_text` function to embed a text message into an image using LSB replacement. The input image remains visually similar, with the hidden message stored in the least significant bits of the red channel.

- **Decoding Function**: The `decode_text` function allows you to extract the hidden message from an encoded image.

## Usage

1. Add `img_stegano_rs` and `image` as a `git` dependency in your `Cargo.toml`.

```toml
[dependencies]
img_stegano_rs = {git = "https://github.com/collinsmuriuki/img_stegano_rs.git"}
image = "0.24.7"
```

2. Import the required libraries and use the functions `encode_text` and `decode_text` as demonstrated in the following example:

```rust
use img_stegano_rs::{encode_text, decode_text};

fn main() {
    // Load the input image
    let input_image = image::open("input.png").unwrap();
    
    // Encode a message and save the encoded image
    let message = "This is a secret message!";
    let encoded_image = encode_text(&input_image, message);
    encoded_image.save("encoded.png").unwrap();

    // Decode the hidden message from the encoded image
    let decoded_image = image::open("encoded.png").unwrap();
    let decoded_message = decode_text(&decoded_image);

    println!("Decoded Message: {}", decoded_message);
}
