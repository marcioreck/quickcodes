use image::DynamicImage;

use crate::types::{BarcodeType, QuickCodesError, ReadResult, Result};

/// Detect and decode barcodes in an image
///
/// # Arguments
///
/// * `img` - The image to scan for barcodes
///
/// # Returns
///
/// Returns a vector of all detected barcodes
pub fn detect_barcodes(img: &DynamicImage) -> Result<Vec<ReadResult>> {
    #[cfg(feature = "readers")]
    {
        let mut results = Vec::new();

        // Try QR code detection first
        if let Ok(qr_results) = detect_qr_codes(img) {
            results.extend(qr_results);
        }

        // Try linear barcode detection
        if let Ok(linear_results) = detect_linear_barcodes(img) {
            results.extend(linear_results);
        }

        if results.is_empty() {
            Err(QuickCodesError::ReaderError("No barcodes detected".to_string()))
        } else {
            Ok(results)
        }
    }

    #[cfg(not(feature = "readers"))]
    {
        Err(QuickCodesError::UnsupportedFormat(
            "Reader functionality not available - enable the 'readers' feature".to_string(),
        ))
    }
}

#[cfg(feature = "readers")]
fn detect_qr_codes(img: &DynamicImage) -> Result<Vec<ReadResult>> {
    use rqrr::PreparedImage;

    let gray = img.to_luma8();
    let mut prepared = PreparedImage::prepare(gray);
    let grids = prepared.detect_grids();

    let mut results = Vec::new();
    for grid in grids {
        if let Ok(content) = grid.decode() {
            results.push(ReadResult {
                barcode_type: BarcodeType::QRCode,
                data: content.0,
                confidence: 1.0,
            });
        }
    }

    Ok(results)
}

#[cfg(feature = "readers")]
fn detect_linear_barcodes(img: &DynamicImage) -> Result<Vec<ReadResult>> {
    // TODO: Implement linear barcode detection
    Ok(Vec::new())
}