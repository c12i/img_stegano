use image::{DynamicImage, Pixel, Rgba};

pub fn encode_text<'a>(
    image: &'a mut DynamicImage,
    text: &'a str,
) -> Result<&'a DynamicImage, String> {
    let (height, width) = (image.height(), image.width());
    let pixels = image.as_mut_rgba8().unwrap();
    let bytes = text.as_bytes();
    let mut data_index = 0;

    for y in 0..height {
        for x in 0..width {
            if data_index < bytes.len() {
                let pixel = pixels.get_pixel(x, y);
                let (r, g, b, mut a) = pixel.channels4();

                let bit = (bytes[data_index] >> 7) & 1;
                a = (a & 0xFE) | bit;

                let new_pixel = Rgba([r, g, b, a]);
                pixels.put_pixel(x, y, new_pixel);

                data_index += 1;
            } else {
                break;
            }
        }
    }

    if data_index < bytes.len() {
        return Err("Insufficient image size to encode the data.".to_string());
    }

    Ok(image)
}

pub fn decode_text(image: &DynamicImage) -> String {
    let pixels = image.as_rgba8().unwrap();
    let mut decoded_data = Vec::new();
    let mut byte = 0;
    let mut data_index = 0;

    for y in 0..image.height() {
        for x in 0..image.width() {
            if data_index < 8 {
                let pixel = pixels.get_pixel(x, y);
                let (_, _, _, a) = pixel.channels4();
                byte = (byte << 1) | (a & 1);

                if x % 8 == 7 {
                    decoded_data.push(byte);
                    byte = 0;
                    data_index += 1;
                }
            } else {
                break;
            }
        }
        if data_index >= 8 {
            break;
        }
    }

    let decoded_text = String::from_utf8_lossy(&decoded_data).into_owned();
    decoded_text
}
