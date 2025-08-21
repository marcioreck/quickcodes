//! SVG export functionality

#[cfg(feature = "svg")]
use crate::types::{Barcode, BarcodeModules, BarcodeType, QuickCodesError, Result};
#[cfg(feature = "svg")]
use svg::node::element::{Rectangle, Text};
#[cfg(feature = "svg")]
use svg::Document;

/// Export a barcode to SVG format
#[cfg(feature = "svg")]
pub fn export_svg(barcode: &Barcode) -> Result<Vec<u8>> {
    match &barcode.modules {
        BarcodeModules::Linear(pattern) => export_linear_svg(barcode, pattern),
        BarcodeModules::Matrix(matrix) => export_matrix_svg(barcode, matrix),
    }
}

/// Export a linear (1D) barcode to SVG
#[cfg(feature = "svg")]
fn export_linear_svg(barcode: &Barcode, pattern: &[bool]) -> Result<Vec<u8>> {
    let module_width = 2.0; // Width of each module in SVG units
    let height = 60.0; // Height of the barcode
    let margin = barcode.config.margin as f64;
    let text_height = if barcode.config.include_text {
        20.0
    } else {
        0.0
    };

    let total_width = (pattern.len() as f64 * module_width) + (2.0 * margin);
    let total_height = height + text_height + (2.0 * margin);

    let mut document = Document::new()
        .set("width", total_width)
        .set("height", total_height)
        .set("viewBox", (0, 0, total_width as i32, total_height as i32))
        .set("xmlns", "http://www.w3.org/2000/svg");

    // White background
    let background = Rectangle::new()
        .set("width", "100%")
        .set("height", "100%")
        .set("fill", "white");
    document = document.add(background);

    // Draw barcode bars
    let mut x = margin;
    for &is_black in pattern {
        if is_black {
            let bar = Rectangle::new()
                .set("x", x)
                .set("y", margin)
                .set("width", module_width)
                .set("height", height)
                .set("fill", "black");
            document = document.add(bar);
        }
        x += module_width;
    }

    // Add human-readable text if enabled
    if barcode.config.include_text {
        let text_y = margin + height + 15.0;
        let text_x = total_width / 2.0;

        let text = Text::new(&barcode.data)
            .set("x", text_x)
            .set("y", text_y)
            .set("text-anchor", "middle")
            .set("font-family", "monospace")
            .set("font-size", "12")
            .set("fill", "black");
        document = document.add(text);
    }

    // Convert to bytes
    let svg_string = document.to_string();
    Ok(svg_string.into_bytes())
}

/// Export a matrix (2D) barcode to SVG
#[cfg(feature = "svg")]
fn export_matrix_svg(barcode: &Barcode, matrix: &[Vec<bool>]) -> Result<Vec<u8>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Err(QuickCodesError::ExportError("Matrix is empty".to_string()));
    }

    let module_size = 4.0; // Size of each module in SVG units
    let margin = barcode.config.margin as f64;
    let text_height = if barcode.config.include_text {
        20.0
    } else {
        0.0
    };

    let matrix_width = matrix[0].len() as f64 * module_size;
    let matrix_height = matrix.len() as f64 * module_size;
    let total_width = matrix_width + (2.0 * margin);
    let total_height = matrix_height + text_height + (2.0 * margin);

    let mut document = Document::new()
        .set("width", total_width)
        .set("height", total_height)
        .set("viewBox", (0, 0, total_width as i32, total_height as i32))
        .set("xmlns", "http://www.w3.org/2000/svg");

    // White background
    let background = Rectangle::new()
        .set("width", "100%")
        .set("height", "100%")
        .set("fill", "white");
    document = document.add(background);

    // Draw matrix modules
    for (row, row_data) in matrix.iter().enumerate() {
        for (col, &is_black) in row_data.iter().enumerate() {
            if is_black {
                let x = margin + (col as f64 * module_size);
                let y = margin + (row as f64 * module_size);

                let module = Rectangle::new()
                    .set("x", x)
                    .set("y", y)
                    .set("width", module_size)
                    .set("height", module_size)
                    .set("fill", "black");
                document = document.add(module);
            }
        }
    }

    // Add human-readable text if enabled
    if barcode.config.include_text {
        let text_y = margin + matrix_height + 15.0;
        let text_x = total_width / 2.0;

        let display_text = match barcode.barcode_type {
            BarcodeType::QRCode => {
                // For QR codes, truncate long data for display
                if barcode.data.len() > 30 {
                    format!("{}...", &barcode.data[..27])
                } else {
                    barcode.data.clone()
                }
            }
            _ => barcode.data.clone(),
        };

        let text = Text::new(&display_text)
            .set("x", text_x)
            .set("y", text_y)
            .set("text-anchor", "middle")
            .set("font-family", "monospace")
            .set("font-size", "10")
            .set("fill", "black");
        document = document.add(text);
    }

    // Convert to bytes
    let svg_string = document.to_string();
    Ok(svg_string.into_bytes())
}

#[cfg(all(test, feature = "svg"))]
mod tests {
    use super::*;
    use crate::generators::ean13::generate_ean13;
    use crate::generators::qr::generate_qr;

    #[test]
    fn test_svg_export_qr() {
        let barcode = generate_qr("Test").unwrap();
        let svg_data = export_svg(&barcode);
        assert!(svg_data.is_ok());

        let svg_string = String::from_utf8(svg_data.unwrap()).unwrap();
        assert!(svg_string.contains("<svg"));
        assert!(svg_string.contains("</svg>"));
        assert!(svg_string.contains("rect"));
    }

    #[test]
    fn test_svg_export_ean13() {
        let barcode = generate_ean13("123456789012").unwrap();
        let svg_data = export_svg(&barcode);
        assert!(svg_data.is_ok());

        let svg_string = String::from_utf8(svg_data.unwrap()).unwrap();
        assert!(svg_string.contains("<svg"));
        assert!(svg_string.contains("</svg>"));
        assert!(svg_string.contains("rect"));
        assert!(svg_string.contains("1234567890128")); // Should include the text
    }
}
