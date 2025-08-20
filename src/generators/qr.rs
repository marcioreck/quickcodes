//! QR Code generator

use crate::types::{Barcode, BarcodeConfig, BarcodeModules, BarcodeType, QRErrorCorrection, Result, QuickCodesError};
use qrcode::{QrCode, EcLevel};

/// Generate a QR Code with default configuration
pub fn generate_qr(data: &str) -> Result<Barcode> {
    generate_qr_with_config(data, &BarcodeConfig::default())
}

/// Generate a QR Code with custom configuration
pub fn generate_qr_with_config(data: &str, config: &BarcodeConfig) -> Result<Barcode> {
    // Convert our error correction level to qrcode crate's level
    let ec_level = match config.qr_config.error_correction {
        QRErrorCorrection::Low => EcLevel::L,
        QRErrorCorrection::Medium => EcLevel::M,
        QRErrorCorrection::Quartile => EcLevel::Q,
        QRErrorCorrection::High => EcLevel::H,
    };

    // Generate QR code
    let qr_code = QrCode::with_error_correction_level(data, ec_level)
        .map_err(|e| QuickCodesError::GenerationError(format!("QR generation failed: {}", e)))?;

    // Convert to our internal representation
    let width = qr_code.width();
    let mut matrix = Vec::with_capacity(width);
    
    for y in 0..width {
        let mut row = Vec::with_capacity(width);
        for x in 0..width {
            // Convert qrcode::Color to bool (Dark = true, Light = false)
            row.push(qr_code[(x, y)] == qrcode::Color::Dark);
        }
        matrix.push(row);
    }

    Ok(Barcode {
        barcode_type: BarcodeType::QRCode,
        data: data.to_string(),
        modules: BarcodeModules::Matrix(matrix),
        config: config.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qr_generation() {
        let result = generate_qr("Hello, World!");
        assert!(result.is_ok());
        
        let barcode = result.unwrap();
        assert_eq!(barcode.barcode_type, BarcodeType::QRCode);
        assert_eq!(barcode.data, "Hello, World!");
        
        match barcode.modules {
            BarcodeModules::Matrix(matrix) => {
                assert!(!matrix.is_empty());
                assert!(!matrix[0].is_empty());
                // QR codes should be square
                assert_eq!(matrix.len(), matrix[0].len());
            }
            _ => panic!("QR code should generate a matrix"),
        }
    }

    #[test]
    fn test_qr_different_error_corrections() {
        let mut config = BarcodeConfig::default();
        
        // Test all error correction levels
        for ec_level in [
            QRErrorCorrection::Low,
            QRErrorCorrection::Medium,
            QRErrorCorrection::Quartile,
            QRErrorCorrection::High,
        ] {
            config.qr_config.error_correction = ec_level;
            let result = generate_qr_with_config("Test data", &config);
            assert!(result.is_ok(), "Failed with error correction: {:?}", ec_level);
        }
    }

    #[test]
    fn test_qr_empty_data() {
        let result = generate_qr("");
        assert!(result.is_ok()); // Empty string is valid for QR codes
    }

    #[test]
    fn test_qr_large_data() {
        let large_data = "A".repeat(1000);
        let result = generate_qr(&large_data);
        // This might fail if data is too large for QR code capacity
        // but we test that it handles the error gracefully
        match result {
            Ok(_) => (), // Success is fine
            Err(QuickCodesError::GenerationError(_)) => (), // Expected error is fine
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }
}
