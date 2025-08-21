//! PNG export functionality

#[cfg(feature = "png")]
use crate::types::{Barcode, BarcodeModules, QuickCodesError, Result};
#[cfg(feature = "png")]
use image::{ImageBuffer, Rgb, RgbImage};
#[cfg(feature = "png")]
use std::io::Cursor;

/// Export a barcode to PNG format
#[cfg(feature = "png")]
pub fn export_png(barcode: &Barcode) -> Result<Vec<u8>> {
    match &barcode.modules {
        BarcodeModules::Linear(pattern) => export_linear_png(barcode, pattern),
        BarcodeModules::Matrix(matrix) => export_matrix_png(barcode, matrix),
    }
}

/// Export a linear (1D) barcode to PNG
#[cfg(feature = "png")]
fn export_linear_png(barcode: &Barcode, pattern: &[bool]) -> Result<Vec<u8>> {
    let module_width = 2u32; // Width of each module in pixels
    let height = 60u32; // Height of the barcode
    let margin = barcode.config.margin;

    let total_width = (pattern.len() as u32 * module_width) + (2 * margin);
    let total_height = height + (2 * margin);

    // Create image buffer
    let mut img: RgbImage = ImageBuffer::new(total_width, total_height);

    // Fill with white background
    for pixel in img.pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }

    // Draw barcode bars
    let mut x = margin;
    for &is_black in pattern {
        if is_black {
            // Draw vertical bar
            for y in margin..(margin + height) {
                for bar_x in x..(x + module_width) {
                    if bar_x < total_width && y < total_height {
                        img.put_pixel(bar_x, y, Rgb([0, 0, 0]));
                    }
                }
            }
        }
        x += module_width;
    }

    // Convert to PNG bytes
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    img.write_to(&mut cursor, image::ImageFormat::Png)
        .map_err(|e| QuickCodesError::ExportError(format!("PNG export failed: {}", e)))?;

    Ok(buffer)
}

/// Export a matrix (2D) barcode to PNG
#[cfg(feature = "png")]
fn export_matrix_png(barcode: &Barcode, matrix: &[Vec<bool>]) -> Result<Vec<u8>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Err(QuickCodesError::ExportError("Matrix is empty".to_string()));
    }

    let module_size = 4u32; // Size of each module in pixels
    let margin = barcode.config.margin;

    let matrix_width = matrix[0].len() as u32 * module_size;
    let matrix_height = matrix.len() as u32 * module_size;
    let total_width = matrix_width + (2 * margin);
    let total_height = matrix_height + (2 * margin);

    // Create image buffer
    let mut img: RgbImage = ImageBuffer::new(total_width, total_height);

    // Fill with white background
    for pixel in img.pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }

    // Draw matrix modules
    for (row, row_data) in matrix.iter().enumerate() {
        for (col, &is_black) in row_data.iter().enumerate() {
            if is_black {
                let start_x = margin + (col as u32 * module_size);
                let start_y = margin + (row as u32 * module_size);

                // Fill the module rectangle
                for y in start_y..(start_y + module_size) {
                    for x in start_x..(start_x + module_size) {
                        if x < total_width && y < total_height {
                            img.put_pixel(x, y, Rgb([0, 0, 0]));
                        }
                    }
                }
            }
        }
    }

    // Convert to PNG bytes
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    img.write_to(&mut cursor, image::ImageFormat::Png)
        .map_err(|e| QuickCodesError::ExportError(format!("PNG export failed: {}", e)))?;

    Ok(buffer)
}

#[cfg(all(test, feature = "png"))]
mod tests {
    use super::*;
    use crate::generators::ean13::generate_ean13;
    use crate::generators::qr::generate_qr;

    #[test]
    fn test_png_export_qr() {
        let barcode = generate_qr("Test").unwrap();
        let png_data = export_png(&barcode);
        assert!(png_data.is_ok());

        let data = png_data.unwrap();
        assert!(!data.is_empty());
        // PNG files start with specific magic bytes
        assert_eq!(&data[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }

    #[test]
    fn test_png_export_ean13() {
        let barcode = generate_ean13("123456789012").unwrap();
        let png_data = export_png(&barcode);
        assert!(png_data.is_ok());

        let data = png_data.unwrap();
        assert!(!data.is_empty());
        // PNG files start with specific magic bytes
        assert_eq!(&data[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }
}
