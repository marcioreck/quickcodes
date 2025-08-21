//! Barcode readers and decoders
//!
//! This module provides functionality to read and decode various barcode formats
//! from images. It supports automatic format detection and multiple barcode
//! detection in a single image.

pub mod detector;
pub mod linear_reader;
pub mod qr_reader;

// Re-export reader functions
pub use detector::{detect_and_read_barcodes, BarcodeDetection};
pub use linear_reader::read_linear_from_image;
pub use qr_reader::read_qr_from_image;

use crate::types::{BarcodeType, QuickCodesError, Result};
use std::path::Path;

/// Result of barcode reading operation
#[derive(Debug, Clone, PartialEq)]
pub struct ReadResult {
    /// The type of barcode that was detected
    pub barcode_type: BarcodeType,
    /// The decoded data
    pub data: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Position in the image (x, y, width, height)
    pub position: Option<(u32, u32, u32, u32)>,
}

/// Read barcode from image file
///
/// Automatically detects the barcode format and decodes it.
///
/// # Arguments
/// * `image_path` - Path to the image file
///
/// # Returns
/// Returns the first barcode found, or an error if none found
///
/// # Examples
/// ```rust
/// use quickcodes::read_from_file;
///
/// // This example would work if you had an actual barcode image
/// // let result = read_from_file("barcode.png")?;
/// // println!("Found {:?} with data: {}", result.barcode_type, result.data);
/// ```
pub fn read_from_file<P: AsRef<Path>>(image_path: P) -> Result<ReadResult> {
    let results = read_all_from_file(image_path)?;
    results
        .into_iter()
        .next()
        .ok_or_else(|| QuickCodesError::GenerationError("No barcodes found in image".to_string()))
}

/// Read all barcodes from image file
///
/// Detects and decodes all barcodes found in the image.
///
/// # Arguments
/// * `image_path` - Path to the image file
///
/// # Returns
/// Returns a vector of all barcodes found
pub fn read_all_from_file<P: AsRef<Path>>(image_path: P) -> Result<Vec<ReadResult>> {
    #[cfg(not(feature = "readers"))]
    {
        return Err(QuickCodesError::UnsupportedFormat(
            "Reader functionality not available - enable the 'readers' feature".to_string(),
        ));
    }

    #[cfg(feature = "readers")]
    {
        use image::open;

        // Load image
        let img = open(image_path)
            .map_err(|e| QuickCodesError::ImageError(format!("Failed to load image: {}", e)))?;

        // Convert to grayscale for processing
        let gray_img = img.to_luma8();

        // Detect and read barcodes
        detect_and_read_barcodes(&gray_img)
    }
}

/// Read barcode from image bytes
///
/// # Arguments
/// * `image_data` - Image data as bytes
/// * `format` - Image format hint (e.g., "png", "jpg")
///
/// # Returns
/// Returns the first barcode found, or an error if none found
pub fn read_from_bytes(image_data: &[u8], format: Option<&str>) -> Result<ReadResult> {
    let results = read_all_from_bytes(image_data, format)?;
    results
        .into_iter()
        .next()
        .ok_or_else(|| QuickCodesError::GenerationError("No barcodes found in image".to_string()))
}

/// Read all barcodes from image bytes
pub fn read_all_from_bytes(image_data: &[u8], _format: Option<&str>) -> Result<Vec<ReadResult>> {
    #[cfg(not(feature = "readers"))]
    {
        return Err(QuickCodesError::UnsupportedFormat(
            "Reader functionality not available - enable the 'readers' feature".to_string(),
        ));
    }

    #[cfg(feature = "readers")]
    {
        use image::load_from_memory;

        // Load image from bytes
        let img = load_from_memory(image_data).map_err(|e| {
            QuickCodesError::ImageError(format!("Failed to load image from bytes: {}", e))
        })?;

        // Convert to grayscale for processing
        let gray_img = img.to_luma8();

        // Detect and read barcodes
        detect_and_read_barcodes(&gray_img)
    }
}
