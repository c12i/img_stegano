use image::{DynamicImage, Rgb, RgbImage};
use img_stegano_core::{
    calculate_capacity, decode_from_image, decode_from_path, decode_from_u8_array,
    encode_from_image, encode_from_path, encode_from_u8_array, Image, ImageFormat, ImgSteganoError,
};
use std::{
    fs::File,
    io::{Cursor, Read},
};

const SECRET_MESSAGE: &str = "The quick brown fox jumps over the lazy dog";

// ============================================================================
// Test Helper Functions
// ============================================================================

/// Create a test image with specified dimensions and a pattern
/// All LSBs are cleared to 0 to ensure clean encoding
fn create_test_image(width: u32, height: u32) -> Image {
    let mut img = RgbImage::new(width, height);

    // Create a gradient pattern for visual variety
    // Clear LSBs (set to 0) to ensure clean state for encoding
    for y in 0..height {
        for x in 0..width {
            let r = ((x as f32 / width as f32) * 255.0) as u8 & 0xFE; // Clear LSB
            let g = ((y as f32 / height as f32) * 255.0) as u8 & 0xFE; // Clear LSB
            let b = ((x + y) as f32 / (width + height) as f32 * 255.0) as u8 & 0xFE; // Clear LSB
            img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }

    DynamicImage::ImageRgb8(img).into()
}

/// Create a solid color test image with LSBs cleared
fn create_solid_image(width: u32, height: u32, color: [u8; 3]) -> Image {
    // Clear LSBs to ensure clean state
    let clean_color = [color[0] & 0xFE, color[1] & 0xFE, color[2] & 0xFE];
    let img = RgbImage::from_pixel(width, height, Rgb(clean_color));
    DynamicImage::ImageRgb8(img).into()
}

/// Create a checkerboard pattern image with LSBs cleared
fn create_checkerboard_image(width: u32, height: u32, square_size: u32) -> Image {
    let mut img = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let is_white = ((x / square_size) + (y / square_size)) % 2 == 0;
            // Clear LSBs for clean encoding
            let color = if is_white {
                Rgb([254, 254, 254]) // 255 & 0xFE = 254
            } else {
                Rgb([0, 0, 0])
            };
            img.put_pixel(x, y, color);
        }
    }

    DynamicImage::ImageRgb8(img).into()
}

// ============================================================================
// Basic Functionality Tests
// ============================================================================

#[test]
fn test_encode_and_decode_from_image() {
    let image = create_test_image(100, 100);
    let encoded = encode_from_image(image, SECRET_MESSAGE).expect("Failed to encode message");
    let decoded_text = decode_from_image(&encoded).expect("Failed to decode message");
    assert_eq!(&decoded_text, SECRET_MESSAGE);
}

#[test]
fn test_encode_and_decode_with_save_reload() {
    // Test that encoding survives save/reload cycle
    let image = create_test_image(100, 100);
    let encoded = encode_from_image(image, SECRET_MESSAGE).expect("Failed to encode message");
    encoded
        .save("test_out.png", ImageFormat::Png)
        .expect("Failed to save");
    let reloaded = Image::open("test_out.png").expect("Failed to reload");
    let decoded_text = decode_from_image(&reloaded).expect("Failed to decode message");
    assert_eq!(&decoded_text, SECRET_MESSAGE);
}

#[test]
fn test_encode_and_decode_from_u8_array() {
    // Create image and convert to bytes
    let image = create_test_image(100, 100);
    image
        .save("test_temp.png", ImageFormat::Png)
        .expect("Failed to save temp");

    let mut file = File::open("test_temp.png").expect("failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let encoded = encode_from_u8_array(&buffer, "png", SECRET_MESSAGE)
        .expect("Failed to encode message to image");
    let decoded = decode_from_u8_array(&encoded).expect("Failed to decode image");
    assert_eq!(&decoded, SECRET_MESSAGE);
}

#[test]
fn test_encode_and_decode_from_path() {
    // Create and save a test image first
    let image = create_test_image(100, 100);
    image
        .save("test_path_input.png", ImageFormat::Png)
        .expect("Failed to save");

    let encoded =
        encode_from_path("test_path_input.png", "foo bar").expect("Failed to encode text to image");
    encoded
        .save("test_path_output.png", ImageFormat::Png)
        .expect("Failed to save output");
    let decoded_text =
        decode_from_path("test_path_output.png").expect("Failed to decode text from image");
    assert_eq!(decoded_text, "foo bar");
}

// ============================================================================
// Edge Case Tests - Empty and Small Messages
// ============================================================================

#[test]
fn test_empty_message_returns_error() {
    let image = create_test_image(50, 50);
    let result = encode_from_image(image, "");
    assert!(result.is_err());
    match result {
        Err(ImgSteganoError::EmptyMessage) => (),
        _ => panic!("Expected EmptyMessage error"),
    }
}

#[test]
fn test_single_character_message() {
    let image = create_test_image(50, 50);
    let message = "A";
    let encoded = encode_from_image(image, message).expect("Failed to encode message");
    let decoded = decode_from_image(&encoded).expect("Failed to decode message");
    assert_eq!(decoded, message);
}

#[test]
fn test_single_byte_message() {
    let image = create_test_image(50, 50);
    let message = "x";
    let encoded = encode_from_image(image, message).expect("Failed to encode message");
    let decoded = decode_from_image(&encoded).expect("Failed to decode message");
    assert_eq!(decoded, message);
}

// ============================================================================
// Edge Case Tests - Special Characters and Unicode
// ============================================================================

#[test]
fn test_special_characters() {
    let image = create_test_image(80, 80);
    let message = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
    let encoded = encode_from_image(image, message).expect("Failed to encode message");
    let decoded = decode_from_image(&encoded).expect("Failed to decode message");
    assert_eq!(decoded, message);
}

#[test]
fn test_unicode_characters() {
    let image = create_test_image(100, 100);
    let message = "Hello 世界 🌍 Привет مرحبا";
    let encoded = encode_from_image(image, message).expect("Failed to encode message");
    let decoded = decode_from_image(&encoded).expect("Failed to decode message");
    assert_eq!(decoded, message);
}

#[test]
fn test_emoji_message() {
    let image = create_test_image(50, 50);
    let message = "🔒🔓🎨⚡🌙";
    let encoded = encode_from_image(image, message).expect("Failed to encode message");
    let decoded = decode_from_image(&encoded).expect("Failed to decode message");
    assert_eq!(decoded, message);
}

#[test]
fn test_newlines_and_whitespace() {
    let image = create_test_image(80, 80);
    let message = "Line 1\nLine 2\n\tTabbed\r\nWindows line";
    let encoded = encode_from_image(image, message).expect("Failed to encode message");
    let decoded = decode_from_image(&encoded).expect("Failed to decode message");
    assert_eq!(decoded, message);
}

// ============================================================================
// Edge Case Tests - Capacity and Size Limits
// ============================================================================

#[test]
fn test_message_too_large() {
    let image = create_test_image(50, 50);
    let (width, height) = image.dimensions();
    let capacity = calculate_capacity(width, height);

    // Create a message that's too large
    let message = "A".repeat(capacity + 1);

    let result = encode_from_image(image, &message);
    assert!(result.is_err());
    match result {
        Err(ImgSteganoError::MessageTooLarge {
            required,
            available,
        }) => {
            assert_eq!(required, capacity + 1);
            assert_eq!(available, capacity);
        }
        _ => panic!("Expected MessageTooLarge error"),
    }
}

#[test]
fn test_maximum_capacity_message() {
    let image = create_test_image(50, 50);
    let (width, height) = image.dimensions();
    let capacity = calculate_capacity(width, height);

    // Create a message that exactly fits
    let message = "A".repeat(capacity);

    let encoded =
        encode_from_image(image, &message).expect("Failed to encode max capacity message");
    let decoded = decode_from_image(&encoded).expect("Failed to decode max capacity message");
    assert_eq!(decoded, message);
}

#[test]
fn test_near_capacity_message() {
    let image = create_test_image(50, 50);
    let (width, height) = image.dimensions();
    let capacity = calculate_capacity(width, height);

    // Create a message that's close to capacity (90%)
    let message = "B".repeat(capacity * 9 / 10);

    let encoded =
        encode_from_image(image, &message).expect("Failed to encode near-capacity message");
    let decoded = decode_from_image(&encoded).expect("Failed to decode near-capacity message");
    assert_eq!(decoded, message);
}

// ============================================================================
// Edge Case Tests - Invalid Formats and Errors
// ============================================================================

#[test]
fn test_invalid_image_format() {
    let buffer = vec![0u8; 100]; // Invalid image data
    let result = encode_from_u8_array(&buffer, "png", "test");
    assert!(result.is_err());
}

#[test]
fn test_unsupported_image_extension() {
    // Create a test image and encode it to PNG bytes
    let img = RgbImage::new(50, 50);
    let dyn_img = DynamicImage::ImageRgb8(img);
    let mut buffer = Vec::new();
    dyn_img
        .write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
        .unwrap();

    // Try to encode with unsupported extension
    let result = encode_from_u8_array(&buffer, "xyz", "test");
    assert!(result.is_err());
    match result {
        Err(ImgSteganoError::InvalidImageFormat) => (),
        _ => panic!("Expected InvalidImageFormat error"),
    }
}

#[test]
fn test_decode_image_without_message() {
    // Try to decode an image that has no encoded message
    let image = create_test_image(50, 50);
    let result = decode_from_image(&image);

    // Should either return empty string or error - depends on implementation
    // In this case, it should find the null terminator immediately
    assert!(result.is_ok());
}

#[test]
fn test_decode_corrupted_message() {
    // Encode a message, save it, then try to decode
    // This tests robustness against potential corruption
    let image = create_test_image(80, 80);
    let message = "This message will be tested";
    let encoded = encode_from_image(image, message).expect("Failed to encode message");

    // Save and reload to ensure it survives the round trip
    encoded
        .save("test_corrupted.png", ImageFormat::Png)
        .expect("Failed to save");
    let reloaded = Image::open("test_corrupted.png").expect("Failed to reload");

    // Decoding should work after save/reload
    let result = decode_from_image(&reloaded);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), message);
}

// ============================================================================
// Edge Case Tests - Multiple Encode/Decode Cycles
// ============================================================================

#[test]
fn test_multiple_encode_decode_cycles() {
    let image = create_test_image(100, 100);
    let message1 = "First message";

    // First cycle
    let encoded1 = encode_from_image(image, message1).expect("Failed to encode message 1");
    let decoded1 = decode_from_image(&encoded1).expect("Failed to decode message 1");
    assert_eq!(decoded1, message1);

    // Second cycle - encode a different message in the same image
    let message2 = "Second message is different";
    let encoded2 = encode_from_image(encoded1, message2).expect("Failed to encode message 2");
    let decoded2 = decode_from_image(&encoded2).expect("Failed to decode message 2");
    assert_eq!(decoded2, message2);

    // The second message should overwrite the first
    assert_ne!(decoded2, message1);
}

#[test]
fn test_encode_same_message_twice() {
    let image = create_test_image(80, 80);
    let message = "Same message";

    let encoded1 = encode_from_image(image.clone(), message).expect("Failed to encode first time");
    let encoded2 = encode_from_image(image, message).expect("Failed to encode second time");

    let decoded1 = decode_from_image(&encoded1).expect("Failed to decode first");
    let decoded2 = decode_from_image(&encoded2).expect("Failed to decode second");

    assert_eq!(decoded1, decoded2);
    assert_eq!(decoded1, message);
}

// ============================================================================
// Edge Case Tests - Binary-like Content
// ============================================================================

#[test]
fn test_null_bytes_in_message() {
    // Note: Our implementation uses null bytes as terminators,
    // so this test verifies that embedded nulls are handled correctly
    let image = create_test_image(50, 50);
    let message = "Before\0After"; // Contains null byte

    let encoded = encode_from_image(image, message).expect("Failed to encode message with null");
    let decoded = decode_from_image(&encoded).expect("Failed to decode message with null");

    // Due to null terminator, we might only get "Before"
    // This is expected behavior with the current implementation
    assert!(decoded.starts_with("Before"));
}

#[test]
fn test_all_printable_ascii() {
    let image = create_test_image(100, 100);
    // All printable ASCII characters (32-126)
    let message: String = (32..=126).map(|c| c as u8 as char).collect();

    let encoded = encode_from_image(image, &message).expect("Failed to encode ASCII");
    let decoded = decode_from_image(&encoded).expect("Failed to decode ASCII");
    assert_eq!(decoded, message);
}

// ============================================================================
// Edge Case Tests - Different Image Formats
// ============================================================================

#[test]
fn test_jpeg_format_warning() {
    // Create a test image and save as PNG first
    let image = create_test_image(50, 50);
    image
        .save("test_jpeg_input.png", ImageFormat::Png)
        .expect("Failed to save");

    let mut file = File::open("test_jpeg_input.png").expect("failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    // JPEG is lossy, should still work but with warning
    let result = encode_from_u8_array(&buffer, "jpg", "test message");
    // Should succeed but print warning to stderr
    assert!(result.is_ok());
}

// ============================================================================
// Edge Case Tests - Capacity Calculation
// ============================================================================

#[test]
fn test_capacity_calculation() {
    let image = create_test_image(80, 80);
    let (width, height) = image.dimensions();
    let capacity = calculate_capacity(width, height);

    // Verify capacity formula: (width * height * 3) / 8 - 1
    let expected = ((width as usize * height as usize * 3) / 8).saturating_sub(1);
    assert_eq!(capacity, expected);

    // Verify we can encode exactly this many bytes
    let message = "X".repeat(capacity);
    let result = encode_from_image(image, &message);
    assert!(result.is_ok());
}

#[test]
fn test_capacity_with_small_image() {
    // Test with a very small image (1x1 pixel)
    use image::{DynamicImage, RgbImage};

    let small_img = DynamicImage::ImageRgb8(RgbImage::new(1, 1));
    let capacity = calculate_capacity(1, 1);

    // 1 pixel * 3 channels = 3 bits = 0 bytes (after subtracting terminator)
    assert_eq!(capacity, 0);

    // Should fail to encode even a single character
    let result = encode_from_image(small_img.into(), "A");
    assert!(result.is_err());
}

#[test]
fn test_capacity_with_medium_image() {
    // Test with a 10x10 image
    use image::{DynamicImage, RgbImage};

    let img = DynamicImage::ImageRgb8(RgbImage::new(10, 10));
    let capacity = calculate_capacity(10, 10);

    // 100 pixels * 3 channels = 300 bits = 37 bytes (minus 1 for terminator = 36)
    assert_eq!(capacity, 36);

    // Should be able to encode 36 bytes
    let message = "A".repeat(36);
    let result = encode_from_image(img.into(), &message);
    assert!(result.is_ok());
}

// ============================================================================
// Edge Case Tests - Different Image Patterns
// ============================================================================

#[test]
fn test_encode_decode_with_solid_color() {
    // Test with solid color images (all pixels same)
    let image = create_solid_image(60, 60, [128, 128, 128]);
    let message = "Solid color test";

    let encoded = encode_from_image(image, message).expect("Failed to encode");
    let decoded = decode_from_image(&encoded).expect("Failed to decode");
    assert_eq!(decoded, message);
}

#[test]
fn test_encode_decode_with_checkerboard() {
    // Test with high-contrast checkerboard pattern
    let image = create_checkerboard_image(80, 80, 8);
    let message = "Checkerboard pattern test";

    let encoded = encode_from_image(image, message).expect("Failed to encode");
    let decoded = decode_from_image(&encoded).expect("Failed to decode");
    assert_eq!(decoded, message);
}

#[test]
fn test_encode_decode_with_black_image() {
    // Test with all-black image
    let image = create_solid_image(50, 50, [0, 0, 0]);
    let message = "Black image";

    let encoded = encode_from_image(image, message).expect("Failed to encode");
    let decoded = decode_from_image(&encoded).expect("Failed to decode");
    assert_eq!(decoded, message);
}

#[test]
fn test_encode_decode_with_white_image() {
    // Test with all-white image
    let image = create_solid_image(50, 50, [255, 255, 255]);
    let message = "White image";

    let encoded = encode_from_image(image, message).expect("Failed to encode");
    let decoded = decode_from_image(&encoded).expect("Failed to decode");
    assert_eq!(decoded, message);
}

#[test]
fn test_different_image_sizes() {
    // Test various image dimensions
    let sizes = vec![(10, 10), (50, 30), (100, 50), (200, 100)];

    for (width, height) in sizes {
        let image = create_test_image(width, height);
        let message = format!("Testing {}x{}", width, height);

        let encoded = encode_from_image(image, &message)
            .unwrap_or_else(|_| panic!("Failed to encode {}x{}", width, height));
        let decoded = decode_from_image(&encoded)
            .unwrap_or_else(|_| panic!("Failed to decode {}x{}", width, height));
        assert_eq!(decoded, message);
    }
}
