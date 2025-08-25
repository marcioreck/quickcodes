use std::path::Path;
use std::str::FromStr;
use thiserror::Error;
use serde::{Serialize, Deserialize};

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

    // 2D Barcodes
    QRCode,
    DataMatrix,
    PDF417,
    Aztec,
}

#[derive(Debug, Error)]
pub enum BarcodeTypeParseError {
    #[error("Invalid barcode type: {0}")]
    InvalidType(String),
}

impl FromStr for BarcodeType {
    type Err = BarcodeTypeParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "QRCode" => Ok(BarcodeType::QRCode),
            "EAN13" => Ok(BarcodeType::EAN13),
            "UPCA" => Ok(BarcodeType::UPCA),
            "Code128" => Ok(BarcodeType::Code128),
            "Code39" => Ok(BarcodeType::Code39),
            "DataMatrix" => Ok(BarcodeType::DataMatrix),
            "PDF417" => Ok(BarcodeType::PDF417),
            "Aztec" => Ok(BarcodeType::Aztec),
            "ITF14" => Ok(BarcodeType::ITF14),
            "Codabar" => Ok(BarcodeType::Codabar),
            _ => Err(BarcodeTypeParseError::InvalidType(s.to_string())),
        }
    }
}

/// Export formats supported by the library
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    PNG,
    SVG,
    PDF,
}

impl ExportFormat {
    pub fn from_extension(path: &str) -> anyhow::Result<Self> {
        let ext = Path::new(path)
            .extension()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("File has no extension"))?
            .to_lowercase();

        match ext.as_str() {
            "png" => Ok(ExportFormat::PNG),
            "svg" => Ok(ExportFormat::SVG),
            "pdf" => Ok(ExportFormat::PDF),
            _ => Err(anyhow::anyhow!("Unsupported file format: {}", ext)),
        }
    }
}

/// Configuration for barcode generation
#[derive(Debug, Clone)]
pub struct BarcodeConfig {
    pub width: u32,
    pub height: u32,
    pub margin: u32,
    pub foreground: [u8; 4],
    pub background: [u8; 4],
    pub include_text: bool,
    pub qr_config: QRConfig,
}

impl Default for BarcodeConfig {
    fn default() -> Self {
        Self {
            width: 300,
            height: 150,
            margin: 10,
            foreground: [0, 0, 0, 255],
            background: [255, 255, 255, 255],
            include_text: true,
            qr_config: QRConfig::default(),
        }
    }
}

/// QR Code specific configuration
#[derive(Debug, Clone)]
pub struct QRConfig {
    pub error_correction: QRErrorCorrection,
}

impl Default for QRConfig {
    fn default() -> Self {
        Self {
            error_correction: QRErrorCorrection::Medium,
        }
    }
}

/// QR Code error correction levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QRErrorCorrection {
    Low,
    Medium,
    Quartile,
    High,
}

/// Internal representation of a barcode
#[derive(Debug, Clone)]
pub struct Barcode {
    pub barcode_type: BarcodeType,
    pub data: String,
    pub modules: BarcodeModules,
    pub config: BarcodeConfig,
}

/// Matrix of modules (pixels) that make up a barcode
#[derive(Debug, Clone)]
pub enum BarcodeModules {
    Linear(Vec<bool>),
    Matrix(Vec<Vec<bool>>),
}

impl BarcodeModules {
    pub fn new_linear(width: usize) -> Self {
        BarcodeModules::Linear(vec![false; width])
    }

    pub fn new_matrix(width: usize, height: usize) -> Self {
        BarcodeModules::Matrix(vec![vec![false; width]; height])
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        match self {
            BarcodeModules::Linear(data) => {
                if x < data.len() && y == 0 {
                    data[x]
                } else {
                    false
                }
            }
            BarcodeModules::Matrix(data) => {
                if y < data.len() && x < data[0].len() {
                    data[y][x]
                } else {
                    false
                }
            }
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        match self {
            BarcodeModules::Linear(data) => {
                if x < data.len() && y == 0 {
                    data[x] = value;
                }
            }
            BarcodeModules::Matrix(data) => {
                if y < data.len() && x < data[0].len() {
                    data[y][x] = value;
                }
            }
        }
    }

    /// Retorna os módulos como um vetor linear (1D)
    pub fn as_linear(&self) -> Option<&Vec<bool>> {
        match self {
            BarcodeModules::Linear(data) => Some(data),
            BarcodeModules::Matrix(_) => None,
        }
    }

    /// Retorna os módulos como uma matriz (2D)
    pub fn as_matrix(&self) -> Option<&Vec<Vec<bool>>> {
        match self {
            BarcodeModules::Linear(_) => None,
            BarcodeModules::Matrix(data) => Some(data),
        }
    }
}

/// Result of reading a barcode from an image
#[derive(Debug, Clone, PartialEq)]
pub struct ReadResult {
    pub barcode_type: BarcodeType,
    pub data: String,
    pub confidence: f32,
}

/// Error types for the library
#[derive(Debug, Error)]
pub enum QuickCodesError {
    #[error("Invalid barcode data: {0}")]
    InvalidData(String),

    #[error("Invalid barcode type: {0}")]
    InvalidType(String),

    #[error("Generation error: {0}")]
    GenerationError(String),

    #[error("Export error: {0}")]
    ExportError(String),

    #[error("Reader error: {0}")]
    ReaderError(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Image error: {0}")]
    ImageError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, QuickCodesError>;