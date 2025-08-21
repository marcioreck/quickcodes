//! PDF export functionality for barcodes
//!
//! This module provides native PDF export capabilities using the printpdf crate.
//! Supports both 1D and 2D barcodes with high-quality vector output.
//!
//! Note: This is a simplified implementation for demonstration purposes.
//! A production implementation would use more advanced PDF rendering techniques.

use crate::types::{Barcode, BarcodeModules, QuickCodesError, Result};

/// Export a barcode to PDF format
///
/// This is a simplified implementation that generates a basic PDF
/// with the barcode data as text. A full implementation would render
/// the actual barcode graphics.
///
/// # Arguments
/// * `barcode` - The barcode to export
///
/// # Returns
/// Returns the PDF data as bytes
pub fn export_pdf(barcode: &Barcode) -> Result<Vec<u8>> {
    // For now, create a simple PDF with the barcode information
    // This is a placeholder implementation
    generate_simple_pdf(barcode)
}

/// Generate a simple PDF with barcode information
fn generate_simple_pdf(barcode: &Barcode) -> Result<Vec<u8>> {
    use printpdf::{BuiltinFont, Mm, PdfDocument};

    // Create a new PDF document
    let (doc, page1, layer1) =
        PdfDocument::new("QuickCodes Barcode", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Add built-in font
    let font = doc
        .add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| QuickCodesError::ExportError(format!("Font error: {}", e)))?;

    // Add barcode information as text
    current_layer.begin_text_section();
    current_layer.set_font(&font, 12.0);
    current_layer.set_text_cursor(Mm(20.0), Mm(250.0));
    current_layer.write_text(format!("Barcode Type: {:?}", barcode.barcode_type), &font);
    current_layer.set_text_cursor(Mm(20.0), Mm(230.0));
    current_layer.write_text(format!("Data: {}", barcode.data), &font);

    // Add dimensions info
    let dimensions = match &barcode.modules {
        BarcodeModules::Linear(pattern) => format!("Pattern Length: {}", pattern.len()),
        BarcodeModules::Matrix(matrix) => format!(
            "Matrix Size: {}x{}",
            matrix.len(),
            matrix.first().map(|row| row.len()).unwrap_or(0)
        ),
    };
    current_layer.set_text_cursor(Mm(20.0), Mm(210.0));
    current_layer.write_text(dimensions, &font);

    current_layer.end_text_section();

    // Save to bytes
    let mut buffer = Vec::new();
    {
        let mut writer = std::io::BufWriter::new(&mut buffer);
        doc.save(&mut writer)
            .map_err(|e| QuickCodesError::ExportError(format!("PDF save failed: {}", e)))?;
    } // writer is dropped here, releasing the borrow

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{BarcodeConfig, BarcodeType};

    #[test]
    #[cfg(feature = "pdf")]
    fn test_pdf_export_linear() {
        let barcode = Barcode {
            barcode_type: BarcodeType::Code128,
            data: "TEST123".to_string(),
            modules: BarcodeModules::Linear(vec![
                true, false, true, true, false, true, false, false,
            ]),
            config: BarcodeConfig::default(),
        };

        let result = export_pdf(&barcode);
        assert!(result.is_ok());

        let pdf_data = result.unwrap();
        assert!(!pdf_data.is_empty());

        // Check PDF header
        assert!(pdf_data.starts_with(b"%PDF"));
    }

    #[test]
    #[cfg(feature = "pdf")]
    fn test_pdf_export_matrix() {
        let matrix = vec![
            vec![true, false, true],
            vec![false, true, false],
            vec![true, false, true],
        ];

        let barcode = Barcode {
            barcode_type: BarcodeType::QRCode,
            data: "TEST".to_string(),
            modules: BarcodeModules::Matrix(matrix),
            config: BarcodeConfig::default(),
        };

        let result = export_pdf(&barcode);
        assert!(result.is_ok());

        let pdf_data = result.unwrap();
        assert!(!pdf_data.is_empty());

        // Check PDF header
        assert!(pdf_data.starts_with(b"%PDF"));
    }

    #[test]
    #[cfg(feature = "pdf")]
    fn test_pdf_export_empty_matrix() {
        let barcode = Barcode {
            barcode_type: BarcodeType::QRCode,
            data: "TEST".to_string(),
            modules: BarcodeModules::Matrix(vec![]),
            config: BarcodeConfig::default(),
        };

        let result = export_pdf(&barcode);
        // Our simplified PDF implementation handles empty matrices gracefully
        assert!(result.is_ok());
    }
}
