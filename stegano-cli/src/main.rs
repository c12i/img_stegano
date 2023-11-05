mod error;

use std::path::PathBuf;

use img_stegano::{image::ImageFormat, ImgStegano, ImgSteganoError};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "image steganography cli")]
enum ImgSteganoCliArgs {
    #[structopt(about = "encode text to image")]
    Encode {
        #[structopt(name = "input", long, short, parse(from_os_str))]
        input_path: PathBuf,
        #[structopt(name = "output", long, short, parse(from_os_str))]
        output_path: PathBuf,
        #[structopt(name = "fmt", long, short)]
        output_format: String,
        #[structopt(long, short)]
        message: String,
    },
    #[structopt(about = "decode text from image")]
    Decode {
        #[structopt(name = "input", long, short, parse(from_os_str))]
        input_path: PathBuf,
    },
}

fn main() -> Result<(), ImgSteganoError> {
    let args = ImgSteganoCliArgs::from_args();
    match args {
        ImgSteganoCliArgs::Encode {
            input_path,
            output_path,
            output_format,
            message,
        } => {
            let image = ImgStegano::encode_from_path(input_path, &message)?;
            image.save(
                output_path,
                ImageFormat::from_extension(output_format)
                    .ok_or(ImgSteganoError::InvalidImageFormat)?,
            )?;
            println!("Text encoded image saved.");
        }
        ImgSteganoCliArgs::Decode { input_path } => {
            let decoded = ImgStegano::decode_from_path(input_path)?;
            println!("Decoded Text:");
            println!("{decoded}");
        }
    }
    Ok(())
}
