use napi::bindgen_prelude::*;
use napi_derive::napi;
use quickcodes::{generate, generate_to_file, read_from_file, read_all_from_file, BarcodeType, ExportFormat};

#[napi(object)]
#[derive(Debug)]
pub struct ReadResult {
    pub barcode_type: String,
    pub data: String,
    pub confidence: f32,
}

impl From<quickcodes::ReadResult> for ReadResult {
    fn from(result: quickcodes::ReadResult) -> Self {
        Self {
            barcode_type: format!("{:?}", result.barcode_type),
            data: result.data,
            confidence: result.confidence,
        }
    }
}

/// Generate a barcode and return it as bytes
#[napi]
pub fn generate_barcode(barcode_type: String, data: String, format: String) -> Result<Vec<u8>> {
    let bc_type = match barcode_type.as_str() {
        "QRCode" => BarcodeType::QRCode,
        "EAN13" => BarcodeType::EAN13,
        "UPCA" => BarcodeType::UPCA,
        "Code128" => BarcodeType::Code128,
        "DataMatrix" => BarcodeType::DataMatrix,
        "PDF417" => BarcodeType::PDF417,
        "Aztec" => BarcodeType::Aztec,
        "Code39" => BarcodeType::Code39,
        "ITF14" => BarcodeType::ITF14,
        "Codabar" => BarcodeType::Codabar,
        _ => return Err(Error::new(Status::InvalidArg, "Invalid barcode type")),
    };

    let export_format = match format.as_str() {
        "SVG" => ExportFormat::SVG,
        "PNG" => ExportFormat::PNG,
        "PDF" => ExportFormat::PDF,
        _ => return Err(Error::new(Status::InvalidArg, "Invalid export format")),
    };

    match generate(bc_type, &data, export_format) {
        Ok(bytes) => Ok(bytes),
        Err(e) => Err(Error::new(Status::GenericFailure, format!("Generation failed: {}", e))),
    }
}

/// Generate a barcode and save it to a file
#[napi]
pub fn generate_barcode_to_file(barcode_type: String, data: String, filename: String) -> Result<()> {
    let bc_type = match barcode_type.as_str() {
        "QRCode" => BarcodeType::QRCode,
        "EAN13" => BarcodeType::EAN13,
        "UPCA" => BarcodeType::UPCA,
        "Code128" => BarcodeType::Code128,
        "DataMatrix" => BarcodeType::DataMatrix,
        "PDF417" => BarcodeType::PDF417,
        "Aztec" => BarcodeType::Aztec,
        "Code39" => BarcodeType::Code39,
        "ITF14" => BarcodeType::ITF14,
        "Codabar" => BarcodeType::Codabar,
        _ => return Err(Error::new(Status::InvalidArg, "Invalid barcode type")),
    };

    match generate_to_file(bc_type, &data, &filename) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::new(Status::GenericFailure, format!("Generation failed: {}", e))),
    }
}

/// Read the first barcode found in an image file
#[napi]
pub fn read_barcode_from_file(filename: String) -> Result<ReadResult> {
    match read_from_file(&filename) {
        Ok(result) => Ok(result.into()),
        Err(e) => Err(Error::new(Status::GenericFailure, format!("Reading failed: {}", e))),
    }
}

/// Read all barcodes found in an image file
#[napi]
pub fn read_all_barcodes_from_file(filename: String) -> Result<Vec<ReadResult>> {
    match read_all_from_file(&filename) {
        Ok(results) => Ok(results.into_iter().map(|r| r.into()).collect()),
        Err(e) => Err(Error::new(Status::GenericFailure, format!("Reading failed: {}", e))),
    }
}

/// Read barcode from image bytes
#[napi]
pub fn read_barcode_from_bytes(bytes: Vec<u8>) -> Result<ReadResult> {
    match quickcodes::read_from_bytes(&bytes) {
        Ok(result) => Ok(result.into()),
        Err(e) => Err(Error::new(Status::GenericFailure, format!("Reading failed: {}", e))),
    }
}

/// Get version info
#[napi]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Get supported barcode types
#[napi]
pub fn get_supported_types() -> Vec<String> {
    vec![
        "QRCode".to_string(),
        "EAN13".to_string(),
        "UPCA".to_string(),
        "Code128".to_string(),
        "DataMatrix".to_string(),
        "PDF417".to_string(),
        "Aztec".to_string(),
        "Code39".to_string(),
        "ITF14".to_string(),
        "Codabar".to_string(),
    ]
}

/// Get supported export formats
#[napi]
pub fn get_supported_formats() -> Vec<String> {
    vec![
        "SVG".to_string(),
        "PNG".to_string(),
        "PDF".to_string(),
    ]
}
