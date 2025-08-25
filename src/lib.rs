use anyhow::Result as AnyhowResult;

pub mod exporters;
pub mod generators;
pub mod readers;
pub mod types;

#[cfg(feature = "python")]
pub mod python;

// C API for language bindings
mod c_api;

// Re-export public API
pub use exporters::*;
pub use generators::*;
pub use readers::{read_from_file as readers_read_from_file, read_from_bytes as readers_read_from_bytes};
pub use types::{BarcodeType, ExportFormat, QuickCodesError, ReadResult, Result};

/// Main generation function - unified API for all barcode types
///
/// # Arguments
///
/// * `barcode_type` - The type of barcode to generate
/// * `data` - The data to encode
/// * `format` - The output format (PNG, SVG)
///
/// # Returns
///
/// Returns the generated barcode as bytes in the specified format
///
/// # Examples
///
/// ```rust
/// use quickcodes::{generate, BarcodeType, ExportFormat};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Generate QR Code as SVG
///     let svg_data = generate(BarcodeType::QRCode, "https://example.com", ExportFormat::SVG)?;
///
///     // Generate EAN-13 as PNG
///     let png_data = generate(BarcodeType::EAN13, "123456789012", ExportFormat::PNG)?;
///     
///     Ok(())
/// }
/// ```
pub fn generate(
    barcode_type: BarcodeType,
    data: &str,
    format: ExportFormat,
) -> AnyhowResult<Vec<u8>> {
    if data.is_empty() {
        return Err(anyhow::anyhow!("Data cannot be empty"));
    }

    let barcode = match barcode_type {
        // Phase 1: Core formats
        BarcodeType::QRCode => generators::qr::generate_qr(data)?,
        BarcodeType::EAN13 => generators::ean13::generate_ean13(data)?,
        BarcodeType::UPCA => generators::upc::generate_upc_a(data)?,
        BarcodeType::Code128 => generators::code128::generate_code128(data)?,

        // Phase 2: Advanced 2D codes
        BarcodeType::DataMatrix => generators::datamatrix::generate_datamatrix(data)?,
        BarcodeType::PDF417 => generators::pdf417::generate_pdf417(data)?,
        BarcodeType::Aztec => generators::aztec::generate_aztec(data)?,

        // Phase 3: Legacy formats (not yet implemented)
        BarcodeType::Code39 | BarcodeType::ITF14 | BarcodeType::Codabar => {
            return Err(anyhow::anyhow!(
                "Barcode type {} not yet implemented - coming in Phase 3",
                match barcode_type {
                    BarcodeType::Code39 => "Code39",
                    BarcodeType::ITF14 => "ITF-14",
                    BarcodeType::Codabar => "Codabar",
                    _ => unreachable!(),
                }
            ))
        }
    };

    match format {
        #[cfg(feature = "svg")]
        ExportFormat::SVG => Ok(exporters::svg::export_svg(&barcode)?),
        #[cfg(not(feature = "svg"))]
        ExportFormat::SVG => Err(anyhow::anyhow!(
            "SVG export not available - enable the 'svg' feature"
        )),
        #[cfg(feature = "png")]
        ExportFormat::PNG => Ok(exporters::png::export_png(&barcode)?),
        #[cfg(not(feature = "png"))]
        ExportFormat::PNG => Err(anyhow::anyhow!(
            "PNG export not available - enable the 'png' feature"
        )),
        #[cfg(feature = "pdf")]
        ExportFormat::PDF => Ok(exporters::pdf::export_pdf(&barcode)?),
        #[cfg(not(feature = "pdf"))]
        ExportFormat::PDF => Err(anyhow::anyhow!(
            "PDF export not available - enable the 'pdf' feature"
        )),
    }
}

/// Generate and save barcode to file
///
/// # Arguments
///
/// * `barcode_type` - The type of barcode to generate
/// * `data` - The data to encode
/// * `output_path` - Path where to save the file
///
/// # Examples
///
/// ```rust
/// use quickcodes::{generate_to_file, BarcodeType};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Generate QR Code and save as SVG
///     generate_to_file(BarcodeType::QRCode, "Hello", "output.svg")?;
///
///     // Generate EAN-13 and save as PNG
///     generate_to_file(BarcodeType::EAN13, "123456789012", "barcode.png")?;
///     
///     Ok(())
/// }
/// ```
pub fn generate_to_file(
    barcode_type: BarcodeType,
    data: &str,
    output_path: &str,
) -> AnyhowResult<()> {
    let format = ExportFormat::from_extension(output_path)?;
    let barcode_data = generate(barcode_type, data, format)?;

    std::fs::write(output_path, barcode_data)?;
    Ok(())
}

/// Read barcode from image file
///
/// Automatically detects and decodes the first barcode found in the image.
///
/// # Arguments
///
/// * `image_path` - Path to the image file
///
/// # Returns
///
/// Returns the decoded barcode data and type information
///
/// # Examples
///
/// ```rust
/// use quickcodes::read_from_file;
///
/// // This example would work if you had an actual barcode image
/// // let result = read_from_file("barcode.png")?;
/// // println!("Found {}: {}", result.barcode_type, result.data);
/// ```
pub fn read_from_file<P: AsRef<std::path::Path>>(_image_path: P) -> AnyhowResult<ReadResult> {
    #[cfg(feature = "readers")]
    {
        Ok(readers_read_from_file(_image_path)?)
    }

    #[cfg(not(feature = "readers"))]
    {
        Err(anyhow::anyhow!(
            "Reader functionality not available - enable the 'readers' feature"
        ))
    }
}

/// Read all barcodes from image file
///
/// Detects and decodes all barcodes found in the image.
///
/// # Arguments
///
/// * `image_path` - Path to the image file
///
/// # Returns
///
/// Returns a vector of all decoded barcodes
pub fn read_all_from_file<P: AsRef<std::path::Path>>(
    _image_path: P,
) -> AnyhowResult<Vec<ReadResult>> {
    #[cfg(feature = "readers")]
    {
        Ok(readers::read_all_from_file(_image_path)?)
    }

    #[cfg(not(feature = "readers"))]
    {
        Err(anyhow::anyhow!(
            "Reader functionality not available - enable the 'readers' feature"
        ))
    }
}

/// Read barcode from image bytes
///
/// # Arguments
///
/// * `image_data` - Image data as bytes
/// * `format` - Optional format hint (e.g., "png", "jpg")
///
/// # Returns
///
/// Returns the first decoded barcode
pub fn read_from_bytes(_image_data: &[u8], _format: Option<&str>) -> AnyhowResult<ReadResult> {
    #[cfg(feature = "readers")]
    {
        Ok(readers_read_from_bytes(_image_data, _format)?)
    }

    #[cfg(not(feature = "readers"))]
    {
        Err(anyhow::anyhow!(
            "Reader functionality not available - enable the 'readers' feature"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "svg")]
    fn test_qr_generation() {
        let result = generate(BarcodeType::QRCode, "test", ExportFormat::SVG);
        assert!(result.is_ok());
    }

    #[test]
    #[cfg(feature = "svg")]
    fn test_datamatrix_generation() {
        let result = generate(BarcodeType::DataMatrix, "test", ExportFormat::SVG);
        assert!(result.is_ok());
    }

    #[test]
    #[cfg(feature = "svg")]
    fn test_pdf417_generation() {
        let result = generate(BarcodeType::PDF417, "test", ExportFormat::SVG);
        assert!(result.is_ok());
    }

    #[test]
    #[cfg(feature = "svg")]
    fn test_aztec_generation() {
        let result = generate(BarcodeType::Aztec, "test", ExportFormat::SVG);
        assert!(result.is_ok());
    }

    #[test]
    #[cfg(feature = "svg")]
    fn test_unimplemented_barcode_type() {
        let result = generate(BarcodeType::Code39, "test", ExportFormat::SVG);
        assert!(result.is_err());
    }

    #[test]
    #[cfg(feature = "pdf")]
    fn test_pdf_export() {
        let result = generate(BarcodeType::QRCode, "test", ExportFormat::PDF);
        assert!(result.is_ok());

        let pdf_data = result.unwrap();
        assert!(!pdf_data.is_empty());
        assert!(pdf_data.starts_with(b"%PDF"));
    }

    #[test]
    #[cfg(not(feature = "readers"))]
    fn test_read_feature_disabled() {
        let result = read_from_bytes(&[0u8; 100], Some("png"));
        assert!(result.is_err());
    }
}