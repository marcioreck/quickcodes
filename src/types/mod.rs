//! Core types and data structures for QuickCodes

use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

/// Supported barcode types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BarcodeType {
    // 1D Barcodes
    EAN13,
    UPCA,
    Code128,
    Code39,
    ITF14,
    Codabar,

    // 2D Codes
    QRCode,
    DataMatrix,
    PDF417,
    Aztec,
}

/// Export formats supported by QuickCodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportFormat {
    PNG,
    SVG,
    PDF,
}

impl ExportFormat {
    /// Determine export format from file extension
    pub fn from_extension(path: &str) -> Result<Self> {
        let path = Path::new(path);
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("png") => Ok(ExportFormat::PNG),
            Some("svg") => Ok(ExportFormat::SVG),
            Some("pdf") => Ok(ExportFormat::PDF),
            _ => Err(QuickCodesError::UnsupportedFormat(
                "Unsupported file extension. Use .png, .svg, or .pdf".to_string(),
            )),
        }
    }
}

/// QR Code error correction levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum QRErrorCorrection {
    Low, // ~7% recovery
    #[default]
    Medium, // ~15% recovery
    Quartile, // ~25% recovery
    High, // ~30% recovery
}

/// Configuration options for barcode generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarcodeConfig {
    /// Width of the barcode in pixels (for raster formats)
    pub width: Option<u32>,
    /// Height of the barcode in pixels (for raster formats)
    pub height: Option<u32>,
    /// DPI for high-quality output
    pub dpi: Option<u32>,
    /// QR Code specific configuration
    pub qr_config: QRConfig,
    /// Whether to include human-readable text
    pub include_text: bool,
    /// Margin/quiet zone size
    pub margin: u32,
}

impl Default for BarcodeConfig {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            dpi: Some(300),
            qr_config: QRConfig::default(),
            include_text: true,
            margin: 10,
        }
    }
}

/// QR Code specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QRConfig {
    pub error_correction: QRErrorCorrection,
    pub version: Option<u8>, // 1-40, None for auto
}

impl Default for QRConfig {
    fn default() -> Self {
        Self {
            error_correction: QRErrorCorrection::Medium,
            version: None,
        }
    }
}

/// Internal representation of a barcode
#[derive(Debug, Clone)]
pub struct Barcode {
    /// The type of barcode
    pub barcode_type: BarcodeType,
    /// Original data that was encoded
    pub data: String,
    /// 2D matrix representation (for 2D codes) or 1D pattern (for 1D codes)
    pub modules: BarcodeModules,
    /// Configuration used to generate this barcode
    pub config: BarcodeConfig,
}

/// Module representation for different barcode types
#[derive(Debug, Clone)]
pub enum BarcodeModules {
    /// 1D barcode as a series of bars (true = black bar, false = white space)
    Linear(Vec<bool>),
    /// 2D barcode as a matrix (true = black module, false = white module)
    Matrix(Vec<Vec<bool>>),
}

/// Errors that can occur during barcode generation or processing
#[derive(Error, Debug)]
pub enum QuickCodesError {
    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Unsupported barcode type: {0:?}")]
    UnsupportedBarcodeType(BarcodeType),

    #[error("Unsupported export format: {0}")]
    UnsupportedFormat(String),

    #[error("Generation error: {0}")]
    GenerationError(String),

    #[error("Export error: {0}")]
    ExportError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Image processing error: {0}")]
    ImageError(String),
}

/// Result type for QuickCodes operations
pub type Result<T> = std::result::Result<T, QuickCodesError>;
