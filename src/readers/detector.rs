//! Automatic barcode detection and decoding
//!
//! This module provides unified barcode detection that automatically
//! tries different barcode formats and returns all found codes.

use super::{linear_reader, qr_reader, ReadResult};
use crate::types::{BarcodeType, Result};

/// Information about a detected barcode before decoding
#[derive(Debug, Clone)]
pub struct BarcodeDetection {
    /// Likely barcode type
    pub likely_type: BarcodeType,
    /// Confidence that this is actually a barcode (0.0 to 1.0)
    pub detection_confidence: f64,
    /// Bounding box (x, y, width, height)
    pub bounds: (u32, u32, u32, u32),
}

/// Detect and read all barcodes in an image
///
/// This function automatically detects different barcode formats
/// and attempts to decode them all.
///
/// # Arguments
/// * `image` - Grayscale image to analyze
///
/// # Returns
/// Returns all successfully decoded barcodes
#[cfg(feature = "readers")]
pub fn detect_and_read_barcodes(image: &image::GrayImage) -> Result<Vec<ReadResult>> {
    let mut all_results = Vec::new();

    // Try QR Code detection first (usually most reliable)
    match qr_reader::read_qr_from_image(image) {
        Ok(qr_results) => {
            all_results.extend(qr_results);
        }
        Err(_) => {
            // QR reading failed, continue with other formats
        }
    }

    // Try linear barcode detection
    match linear_reader::read_linear_from_image(image) {
        Ok(linear_results) => {
            all_results.extend(linear_results);
        }
        Err(_) => {
            // Linear reading failed, continue
        }
    }

    // Remove duplicates and sort by confidence
    all_results.sort_by(|a, b| {
        b.confidence
            .partial_cmp(&a.confidence)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    all_results.dedup_by(|a, b| a.data == b.data && a.barcode_type == b.barcode_type);

    Ok(all_results)
}

/// Placeholder implementation when readers feature is not enabled
#[cfg(not(feature = "readers"))]
pub fn detect_and_read_barcodes(_image: &image::GrayImage) -> Result<Vec<ReadResult>> {
    use crate::types::QuickCodesError;
    Err(QuickCodesError::UnsupportedFormat(
        "Barcode detection not available - enable the 'readers' feature".to_string(),
    ))
}

/// Detect potential barcode regions without decoding
///
/// This function identifies areas in the image that likely contain barcodes
/// but doesn't attempt to decode them. Useful for UI highlighting.
///
/// # Arguments
/// * `image` - Grayscale image to analyze
///
/// # Returns
/// Returns detected barcode regions with type hints
#[cfg(feature = "readers")]
pub fn detect_barcode_regions(image: &image::GrayImage) -> Result<Vec<BarcodeDetection>> {
    use imageproc::edges::canny;

    let mut detections = Vec::new();
    let _width = image.width();
    let _height = image.height();

    // Apply edge detection
    let edges = canny(image, 50.0, 100.0);

    // Look for QR Code-like patterns (square finder patterns)
    let qr_detections = detect_qr_patterns(&edges);
    detections.extend(qr_detections);

    // Look for linear barcode patterns (horizontal lines)
    let linear_detections = detect_linear_patterns(&edges);
    detections.extend(linear_detections);

    // Look for DataMatrix patterns (L-shaped borders)
    let datamatrix_detections = detect_datamatrix_patterns(&edges);
    detections.extend(datamatrix_detections);

    Ok(detections)
}

/// Detect QR Code-like patterns
#[cfg(feature = "readers")]
fn detect_qr_patterns(edges: &image::GrayImage) -> Vec<BarcodeDetection> {
    let mut detections = Vec::new();
    let width = edges.width();
    let height = edges.height();

    // Look for square patterns that could be QR finder patterns
    for y in (0..height.saturating_sub(20)).step_by(10) {
        for x in (0..width.saturating_sub(20)).step_by(10) {
            let score = analyze_qr_finder_pattern(edges, x, y);
            if score > 0.7 {
                detections.push(BarcodeDetection {
                    likely_type: BarcodeType::QRCode,
                    detection_confidence: score,
                    bounds: (x, y, 100, 100), // Rough estimate
                });
            }
        }
    }

    detections
}

/// Analyze if a region looks like a QR finder pattern
#[cfg(feature = "readers")]
fn analyze_qr_finder_pattern(edges: &image::GrayImage, x: u32, y: u32) -> f64 {
    // This is a simplified analysis
    // Real QR detection would look for the 1:1:3:1:1 ratio pattern

    let mut edge_count = 0;
    let mut total_pixels = 0;

    for dy in 0..20 {
        for dx in 0..20 {
            if x + dx < edges.width() && y + dy < edges.height() {
                total_pixels += 1;
                if edges.get_pixel(x + dx, y + dy)[0] > 128 {
                    edge_count += 1;
                }
            }
        }
    }

    if total_pixels == 0 {
        return 0.0;
    }

    let edge_ratio = edge_count as f64 / total_pixels as f64;

    // QR finder patterns should have a moderate edge density
    if edge_ratio > 0.3 && edge_ratio < 0.7 {
        edge_ratio
    } else {
        0.0
    }
}

/// Detect linear barcode patterns
#[cfg(feature = "readers")]
fn detect_linear_patterns(edges: &image::GrayImage) -> Vec<BarcodeDetection> {
    let mut detections = Vec::new();
    let width = edges.width();
    let height = edges.height();

    // Look for horizontal line patterns
    for y in (0..height).step_by(5) {
        let mut line_start = None;
        let mut edge_count = 0;

        for x in 0..width {
            let pixel = edges.get_pixel(x, y)[0];

            if pixel > 128 {
                if line_start.is_none() {
                    line_start = Some(x);
                }
                edge_count += 1;
            } else if let Some(start_x) = line_start {
                if edge_count > 30 && (x - start_x) > 80 {
                    let confidence = (edge_count as f64 / (x - start_x) as f64).min(1.0);

                    // Determine likely barcode type based on length and pattern
                    let likely_type = if x - start_x > 200 {
                        BarcodeType::Code128
                    } else if x - start_x > 120 {
                        BarcodeType::EAN13
                    } else {
                        BarcodeType::UPCA
                    };

                    detections.push(BarcodeDetection {
                        likely_type,
                        detection_confidence: confidence,
                        bounds: (start_x, y.saturating_sub(10), x - start_x, 20),
                    });
                }
                line_start = None;
                edge_count = 0;
            }
        }
    }

    detections
}

/// Detect DataMatrix-like patterns
#[cfg(feature = "readers")]
fn detect_datamatrix_patterns(edges: &image::GrayImage) -> Vec<BarcodeDetection> {
    let mut detections = Vec::new();
    let width = edges.width();
    let height = edges.height();

    // Look for L-shaped patterns characteristic of DataMatrix
    for y in (0..height.saturating_sub(30)).step_by(10) {
        for x in (0..width.saturating_sub(30)).step_by(10) {
            let score = analyze_datamatrix_pattern(edges, x, y);
            if score > 0.6 {
                detections.push(BarcodeDetection {
                    likely_type: BarcodeType::DataMatrix,
                    detection_confidence: score,
                    bounds: (x, y, 50, 50), // Estimate based on typical DataMatrix size
                });
            }
        }
    }

    detections
}

/// Analyze if a region looks like a DataMatrix finder pattern
#[cfg(feature = "readers")]
fn analyze_datamatrix_pattern(edges: &image::GrayImage, x: u32, y: u32) -> f64 {
    // DataMatrix has L-shaped solid borders on left and bottom
    let size = 30;
    let mut left_edge_count = 0;
    let mut bottom_edge_count = 0;

    // Check left border
    for dy in 0..size {
        if y + dy < edges.height() && edges.get_pixel(x, y + dy)[0] > 128 {
            left_edge_count += 1;
        }
    }

    // Check bottom border
    for dx in 0..size {
        if x + dx < edges.width() && edges.get_pixel(x + dx, y + size - 1)[0] > 128 {
            bottom_edge_count += 1;
        }
    }

    let left_ratio = left_edge_count as f64 / size as f64;
    let bottom_ratio = bottom_edge_count as f64 / size as f64;

    // DataMatrix should have strong edges on left and bottom
    if left_ratio > 0.7 && bottom_ratio > 0.7 {
        (left_ratio + bottom_ratio) / 2.0
    } else {
        0.0
    }
}

/// Placeholder implementation when readers feature is not enabled
#[cfg(not(feature = "readers"))]
pub fn detect_barcode_regions(_image: &image::GrayImage) -> Result<Vec<BarcodeDetection>> {
    use crate::types::QuickCodesError;
    Err(QuickCodesError::UnsupportedFormat(
        "Barcode detection not available - enable the 'readers' feature".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "readers")]
    fn test_detect_barcodes_empty_image() {
        use image::GrayImage;

        let img = GrayImage::new(100, 100);
        let result = detect_and_read_barcodes(&img);

        assert!(result.is_ok());
        let barcodes = result.unwrap();
        assert!(barcodes.is_empty());
    }

    #[test]
    #[cfg(feature = "readers")]
    fn test_detect_regions_empty_image() {
        use image::GrayImage;

        let img = GrayImage::new(100, 100);
        let result = detect_barcode_regions(&img);

        assert!(result.is_ok());
        let _detections = result.unwrap();
        // May or may not be empty depending on noise in the image
    }

    #[test]
    #[cfg(not(feature = "readers"))]
    fn test_detect_barcodes_feature_disabled() {
        use image::GrayImage;

        let img = GrayImage::new(100, 100);
        let result = detect_and_read_barcodes(&img);

        assert!(result.is_err());
    }

    #[test]
    fn test_barcode_detection_struct() {
        let detection = BarcodeDetection {
            likely_type: BarcodeType::QRCode,
            detection_confidence: 0.85,
            bounds: (10, 20, 100, 100),
        };

        assert_eq!(detection.likely_type, BarcodeType::QRCode);
        assert_eq!(detection.detection_confidence, 0.85);
        assert_eq!(detection.bounds, (10, 20, 100, 100));
    }
}
