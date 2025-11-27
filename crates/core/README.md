# img_stegano_core

Core library for LSB (Least Significant Bit) steganography in images.

## Technical Background

### What is Steganography?

Steganography is the practice of concealing information within other non-secret data. Unlike cryptography, which makes data unreadable, steganography hides the very existence of the message. The word derives from Greek: "steganos" (covered) and "graphein" (writing).

### LSB Steganography

This library implements LSB (Least Significant Bit) steganography, a technique that exploits the way digital images store color information.

#### How Digital Images Store Color

Digital images represent colors using pixels, where each pixel contains color channel values:

- **RGB Images**: Each pixel has 3 channels (Red, Green, Blue)
- **Channel Values**: Each channel is typically an 8-bit value (0-255)
- **Binary Representation**: Each channel value is stored as 8 bits

Example pixel:
```
Red:   214 = 11010110
Green: 87  = 01010111
Blue:  142 = 10001110
```

#### The LSB Technique

The least significant bit (rightmost bit) of a number has the smallest impact on its value. Changing it only alters the value by 1:

```
Original: 11010110 (214)
Modified: 11010111 (215)  <- Only LSB changed
Difference: 1 (imperceptible to human eye)
```

**Encoding Process:**

1. Convert message text to binary
2. For each bit in the message:
   - Take the next color channel value
   - Replace its LSB with the message bit
   - Store the modified value back

3. Add a null terminator (8 zero bits) to mark message end

**Decoding Process:**

1. Extract the LSB from each color channel
2. Group bits into bytes (8 bits = 1 character)
3. Stop when null terminator is encountered
4. Convert bytes back to text

#### Storage Capacity

For an image with dimensions W x H:
```
Capacity (bytes) = (W * H * 3) / 8 - 1
```

- W * H = total pixels
- * 3 = three RGB channels per pixel
- / 8 = convert bits to bytes
- - 1 = reserve space for null terminator

Example: A 1920x1080 image can store approximately 777,599 bytes (759 KB) of hidden data.

### Image Formats and Lossiness

#### What is Lossy Compression?

Image compression reduces file size by removing information. There are two types:

**Lossless Compression:**
- Reduces file size without losing any data
- Original image can be perfectly reconstructed
- Like ZIP compression for images
- Examples: PNG, BMP (uncompressed), TIFF (uncompressed)

**Lossy Compression:**
- Reduces file size by discarding "unnecessary" information
- Original image cannot be perfectly reconstructed
- Optimized for human perception (removes details humans can't easily see)
- Examples: JPEG, WebP (default), HEIC

#### Why Lossy Formats Fail for Steganography

Lossy compression algorithms modify pixel values in ways that destroy LSB data:

**JPEG Example:**

1. **Encoding**: You modify LSBs to hide message
   ```
   Original: R=214, G=87, B=142
   Modified: R=215, G=86, B=143  (LSBs changed)
   ```

2. **JPEG Compression**: Applies DCT (Discrete Cosine Transform) + Quantization
   ```
   Saved values: R=213, G=89, B=140  (approximated values)
   ```

3. **Decoding**: LSBs are now completely different
   ```
   Expected LSBs: 1, 0, 1
   Actual LSBs:   1, 1, 0  (corrupted!)
   ```

The hidden message becomes corrupted or unreadable.

#### Format-Specific Issues

**PNG (Recommended):**
- Uses DEFLATE compression (lossless)
- Preserves exact pixel values
- Standardized format with consistent implementations
- Universal support across platforms and libraries

**JPEG (Not Supported):**
- Uses DCT + quantization (lossy)
- Pixel values change during save/load cycles
- LSB modifications are destroyed
- Results in corrupted messages or invalid UTF-8 errors

**WebP:**
- Can be lossless OR lossy
- Default mode is lossy
- Even lossless mode may use different internal representations
- Unreliable for LSB steganography

**BMP:**
- Uncompressed (lossless)
- Row padding requirements (rows must be multiples of 4 bytes)
- BGR byte ordering instead of RGB
- Padding and byte order transformations can corrupt LSB data

**TIFF:**
- Supports multiple compression schemes (uncompressed, LZW, PackBits, etc.)
- Different color space representations
- Byte ordering issues (little-endian vs big-endian)
- Inconsistent behavior across different TIFF variants

### Why This Library Uses PNG Only

After testing various formats, PNG emerged as the only reliably consistent format for LSB steganography:

1. **True Lossless**: DEFLATE compression preserves exact pixel values
2. **Standardized**: Consistent implementation across all platforms
3. **RGB Preservation**: No byte ordering or padding issues
4. **Universal Support**: Works reliably in browsers, image libraries, and native applications
5. **Proven**: Extensive testing confirms LSB data survives encode/decode cycles

Other formats, even when technically lossless, have format-specific quirks (padding, byte ordering, color space conversions) that can corrupt LSB data during the image library's encoding/decoding pipeline.

## Usage

See the main [README](../README.md) for usage examples.

## Security Considerations

1. **Not Encryption**: LSB steganography hides data but does not encrypt it. Anyone who knows to look for hidden data can extract it.

2. **Statistical Detection**: LSB modifications create statistical patterns that can be detected by steganalysis tools.

3. **Combine with Encryption**: For sensitive data, encrypt the message before hiding it in an image.

4. **Format Matters**: Always use PNG. Other formats will corrupt your hidden data.

## References

- [Steganography on Wikipedia](https://en.wikipedia.org/wiki/Steganography)
- [LSB Steganography Technique](https://en.wikipedia.org/wiki/Bit_numbering#Least_significant_bit)
- [PNG Specification](https://www.w3.org/TR/PNG/)
- [JPEG Compression](https://en.wikipedia.org/wiki/JPEG#JPEG_compression)

