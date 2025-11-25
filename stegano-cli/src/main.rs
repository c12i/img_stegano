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
    /// Encode text into an image (outputs PNG format)
    Encode {
        /// Input image path
        #[arg(short, long)]
        input: PathBuf,

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
        Commands::Encode { input, message } => {
            println!("Encoding message into image...");
            let encoded_image = encode_from_path(&input, &message)?;

            let output = input
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map(|stem| {
                    let mut path = input.clone();
                    path.set_file_name(format!("{stem}-encoded.png"));
                    path
                })
                .unwrap_or_else(|| PathBuf::from("encoded.png"));

            encoded_image.save(&output, ImageFormat::Png)?;
            println!(
                "✓ Text encoded image saved to: {} (PNG format)",
                output.display()
            );
        }
        Commands::Decode { input } => {
            println!("Decoding message from image...");
            let decoded = decode_from_path(input)?;
            println!("✓ Decoded Text:");
            println!("{decoded}");
        }
        Commands::Capacity { input } => {
            let image = Image::open(input)?;
            let capacity = image.capacity();
            println!("✓ Image capacity: {capacity} bytes (~{capacity} characters)");
        }
    }
    Ok(())
}
