//! QR Code reader implementation
//!
//! This module provides QR Code detection and decoding capabilities
//! using the rqrr crate for robust QR code recognition.

use super::ReadResult;
use crate::types::BarcodeType;

/// Read QR Code from grayscale image
///
/// # Arguments
/// * `image` - Grayscale image data
///
/// # Returns
/// Returns all QR codes found in the image
#[cfg(feature = "readers")]
pub fn read_qr_from_image(image: &image::GrayImage) -> crate::types::Result<Vec<ReadResult>> {
    use rqrr::PreparedImage;

    let width = image.width();
    let height = image.height();

    // Prepare image for QR detection
    let mut img = PreparedImage::prepare_from_greyscale(width as usize, height as usize, |x, y| {
        image.get_pixel(x as u32, y as u32)[0]
    });

    let grids = img.detect_grids();
    let mut results = Vec::new();

    for grid in grids {
        match grid.decode() {
            Ok((meta, content)) => {
                // Calculate bounding box from grid bounds (simplified)
                // grid.bounds is an array of 4 points, we'll use a rough bounding box
                let position = Some((0u32, 0u32, 100u32, 100u32)); // Placeholder

                // Calculate confidence based on error correction level and version
                let confidence = match meta.ecc_level {
                    0 => 0.7,  // Low
                    1 => 0.8,  // Medium
                    2 => 0.9,  // Quartile
                    3 => 0.95, // High
                    _ => 0.8,
                };

                results.push(ReadResult {
                    barcode_type: BarcodeType::QRCode,
                    data: content,
                    confidence,
                    position,
                });
            }
            Err(_) => {
                // Failed to decode this QR code, continue with others
                continue;
            }
        }
    }

    Ok(results)
}

/// Placeholder implementation when readers feature is not enabled
#[cfg(not(feature = "readers"))]
pub fn read_qr_from_image(_image: &image::GrayImage) -> crate::types::Result<Vec<ReadResult>> {
    Err(crate::types::QuickCodesError::UnsupportedFormat(
        "QR reader not available - enable the 'readers' feature".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "readers")]
    fn test_qr_reader_empty_image() {
        use image::GrayImage;

        let img = GrayImage::new(100, 100);
        let result = read_qr_from_image(&img);

        assert!(result.is_ok());
        let qr_codes = result.unwrap();
        assert!(qr_codes.is_empty());
    }

    #[test]
    #[cfg(not(feature = "readers"))]
    fn test_qr_reader_feature_disabled() {
        use image::GrayImage;

        let img = GrayImage::new(100, 100);
        let result = read_qr_from_image(&img);

        assert!(result.is_err());
    }
}
