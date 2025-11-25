mod error;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use img_stegano::{decode_from_path, encode_from_path, image::ImageFormat, Image, ImgSteganoError};

#[derive(Parser, Debug)]
#[command(name = "img_stegano_cli")]
#[command(about = "Image steganography CLI - hide and reveal text in images", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Encode text into an image
    Encode {
        /// Input image path
        #[arg(short, long)]
        input: PathBuf,

        /// Output image path
        #[arg(short, long)]
        output: PathBuf,

        /// Output image format (e.g., png, jpg)
        #[arg(short, long)]
        format: String,

        /// Text message to encode
        #[arg(short, long)]
        message: String,
    },
    /// Decode text from an image
    Decode {
        /// Input image path
        #[arg(short, long)]
        input: PathBuf,
    },
    /// Get the maximum message capacity for an image
    Capacity {
        /// Input image path
        #[arg(short, long)]
        input: PathBuf,
    },
}

fn main() -> Result<(), ImgSteganoError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode {
            input,
            output,
            format,
            message,
        } => {
            let image_format =
                ImageFormat::from_extension(&format).ok_or(ImgSteganoError::InvalidImageFormat)?;

            println!("Encoding message into image...");
            let encoded_image = encode_from_path(input, &message)?;
            encoded_image.save(output.clone(), image_format)?;
            println!("✓ Text encoded image saved to: {}", output.display());
        }
        Commands::Decode { input } => {
            println!("Decoding message from image...");
            let decoded = decode_from_path(input)?;
            println!("✓ Decoded Text:");
            println!("{}", decoded);
        }
        Commands::Capacity { input } => {
            let image = Image::open(input)?;
            let capacity = image.capacity();
            println!(
                "✓ Image capacity: {} bytes (~{} characters)",
                capacity, capacity
            );
        }
    }
    Ok(())
}
