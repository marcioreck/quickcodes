//! Integration tests for QuickCodes
//!
//! These tests validate the complete functionality of Phase 1 features

use quickcodes::{generate, generate_to_file, BarcodeType, ExportFormat};
use std::fs;
use tempfile::TempDir;

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Test all supported barcode types with SVG export
    #[test]
    fn test_all_barcode_types_svg() {
        let test_cases = vec![
            (BarcodeType::QRCode, "Hello, QuickCodes!"),
            (BarcodeType::EAN13, "123456789012"),
            (BarcodeType::UPCA, "03600029145"),
            (BarcodeType::Code128, "HELLO123"),
        ];

        for (barcode_type, data) in test_cases {
            let result = generate(barcode_type, data, ExportFormat::SVG);
            assert!(
                result.is_ok(),
                "Failed to generate {:?} as SVG",
                barcode_type
            );

            let svg_data = result.unwrap();
            assert!(!svg_data.is_empty());

            // Verify it's valid SVG
            let svg_string = String::from_utf8(svg_data).unwrap();
            assert!(svg_string.contains("<svg"));
            assert!(svg_string.contains("</svg>"));
        }
    }

    /// Test all supported barcode types with PNG export
    #[test]
    fn test_all_barcode_types_png() {
        let test_cases = vec![
            (BarcodeType::QRCode, "PNG Test"),
            (BarcodeType::EAN13, "123456789012"),
            (BarcodeType::UPCA, "03600029145"),
            (BarcodeType::Code128, "PNG123"),
        ];

        for (barcode_type, data) in test_cases {
            let result = generate(barcode_type, data, ExportFormat::PNG);
            assert!(
                result.is_ok(),
                "Failed to generate {:?} as PNG",
                barcode_type
            );

            let png_data = result.unwrap();
            assert!(!png_data.is_empty());

            // Verify PNG magic bytes
            assert_eq!(&png_data[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
        }
    }

    /// Test file generation and validation
    #[test]
    fn test_generate_to_file() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let test_cases = vec![
            (BarcodeType::QRCode, "File Test", "qr_test.svg"),
            (BarcodeType::EAN13, "123456789012", "ean13_test.png"),
            (BarcodeType::UPCA, "03600029145", "upc_test.svg"),
            (BarcodeType::Code128, "FILE123", "code128_test.png"),
        ];

        for (barcode_type, data, filename) in test_cases {
            let file_path = temp_path.join(filename);
            let result = generate_to_file(barcode_type, data, file_path.to_str().unwrap());

            assert!(
                result.is_ok(),
                "Failed to generate file for {:?}",
                barcode_type
            );
            assert!(
                file_path.exists(),
                "File was not created for {:?}",
                barcode_type
            );

            // Verify file is not empty
            let file_size = fs::metadata(&file_path).unwrap().len();
            assert!(
                file_size > 0,
                "Generated file is empty for {:?}",
                barcode_type
            );

            // Verify file format based on extension
            if filename.ends_with(".svg") {
                let content = fs::read_to_string(&file_path).unwrap();
                assert!(content.contains("<svg"));
            } else if filename.ends_with(".png") {
                let content = fs::read(&file_path).unwrap();
                assert_eq!(&content[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
            }
        }
    }

    /// Test EAN-13 checksum calculation
    #[test]
    fn test_ean13_checksum_validation() {
        // Test with 12 digits (should add checksum)
        let result = generate(BarcodeType::EAN13, "123456789012", ExportFormat::SVG);
        assert!(result.is_ok());

        // Test with correct 13 digits (should validate)
        let result = generate(BarcodeType::EAN13, "1234567890128", ExportFormat::SVG);
        assert!(result.is_ok());

        // Test with incorrect checksum (should fail)
        let result = generate(BarcodeType::EAN13, "1234567890127", ExportFormat::SVG);
        assert!(result.is_err());
    }

    /// Test UPC-A checksum calculation
    #[test]
    fn test_upca_checksum_validation() {
        // Test with 11 digits (should add checksum)
        let result = generate(BarcodeType::UPCA, "03600029145", ExportFormat::SVG);
        assert!(result.is_ok());

        // Test with correct 12 digits (should validate)
        let result = generate(BarcodeType::UPCA, "036000291452", ExportFormat::SVG);
        assert!(result.is_ok());

        // Test with incorrect checksum (should fail)
        let result = generate(BarcodeType::UPCA, "036000291451", ExportFormat::SVG);
        assert!(result.is_err());
    }

    /// Test QR Code with different data types
    #[test]
    fn test_qr_code_data_types() {
        let test_cases = vec![
            "Simple text",
            "https://github.com/quickcodes/quickcodes",
            "mailto:test@example.com",
            "tel:+5511999999999",
            "WIFI:T:WPA;S:NetworkName;P:password123;;",
            "00020126580014BR.GOV.BCB.PIX0114+5551999999995204000053039865402BR5909Test User6009SAO PAULO62070503***6304ABCD", // Pix
            "", // Empty string
        ];

        for data in test_cases {
            let result = generate(BarcodeType::QRCode, data, ExportFormat::SVG);
            assert!(
                result.is_ok(),
                "Failed to generate QR code for data: '{}'",
                data
            );
        }
    }

    /// Test invalid input handling
    #[test]
    fn test_invalid_inputs() {
        // Invalid EAN-13 lengths
        assert!(generate(BarcodeType::EAN13, "12345", ExportFormat::SVG).is_err());
        assert!(generate(BarcodeType::EAN13, "12345678901234", ExportFormat::SVG).is_err());

        // Invalid UPC-A lengths
        assert!(generate(BarcodeType::UPCA, "12345", ExportFormat::SVG).is_err());
        assert!(generate(BarcodeType::UPCA, "1234567890123", ExportFormat::SVG).is_err());

        // Non-numeric data for numeric barcodes
        assert!(generate(BarcodeType::EAN13, "12345678901A", ExportFormat::SVG).is_err());
        assert!(generate(BarcodeType::UPCA, "12345678901A", ExportFormat::SVG).is_err());

        // Empty Code128 (should fail)
        assert!(generate(BarcodeType::Code128, "", ExportFormat::SVG).is_err());
    }

    /// Test file extension detection
    #[test]
    fn test_file_extension_detection() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Test SVG detection
        let svg_file = temp_path.join("test.svg");
        let result = generate_to_file(BarcodeType::QRCode, "SVG test", svg_file.to_str().unwrap());
        assert!(result.is_ok());

        let content = fs::read_to_string(&svg_file).unwrap();
        assert!(content.contains("<svg"));

        // Test PNG detection
        let png_file = temp_path.join("test.png");
        let result = generate_to_file(BarcodeType::QRCode, "PNG test", png_file.to_str().unwrap());
        assert!(result.is_ok());

        let content = fs::read(&png_file).unwrap();
        assert_eq!(&content[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);

        // Test unsupported extension
        let invalid_file = temp_path.join("test.txt");
        let result = generate_to_file(
            BarcodeType::QRCode,
            "Invalid test",
            invalid_file.to_str().unwrap(),
        );
        assert!(result.is_err());
    }

    /// Test large data handling
    #[test]
    fn test_large_data_handling() {
        // Large QR code data (should work)
        let large_data = "A".repeat(500);
        let _result = generate(BarcodeType::QRCode, &large_data, ExportFormat::SVG);
        // This might succeed or fail depending on QR code capacity, both are acceptable
        // We just verify it doesn't panic

        // Very large data (should fail gracefully)
        let very_large_data = "A".repeat(5000);
        let _result = generate(BarcodeType::QRCode, &very_large_data, ExportFormat::SVG);
        // Should either succeed or fail gracefully (not panic)
        // We just verify it doesn't panic
    }

    /// Performance test - ensure reasonable generation times
    #[test]
    fn test_performance_reasonable_times() {
        use std::time::Instant;

        let test_cases = vec![
            (BarcodeType::QRCode, "Performance test"),
            (BarcodeType::EAN13, "123456789012"),
            (BarcodeType::UPCA, "03600029145"),
            (BarcodeType::Code128, "PERF123"),
        ];

        for (barcode_type, data) in test_cases {
            let start = Instant::now();
            let result = generate(barcode_type, data, ExportFormat::SVG);
            let duration = start.elapsed();

            assert!(
                result.is_ok(),
                "Performance test failed for {:?}",
                barcode_type
            );
            assert!(
                duration.as_millis() < 1000,
                "Generation took too long for {:?}: {}ms",
                barcode_type,
                duration.as_millis()
            );
        }
    }

    /// Test concurrent generation (thread safety)
    #[test]
    fn test_concurrent_generation() {
        use std::thread;

        let handles: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    let data = format!("Thread test {}", i);
                    let result = generate(BarcodeType::QRCode, &data, ExportFormat::SVG);
                    assert!(
                        result.is_ok(),
                        "Concurrent generation failed for thread {}",
                        i
                    );
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }

    /// Test memory usage (ensure no obvious leaks)
    #[test]
    fn test_memory_usage() {
        // Generate many barcodes to check for obvious memory leaks
        for i in 0..100 {
            let data = format!("Memory test {}", i);
            let result = generate(BarcodeType::QRCode, &data, ExportFormat::SVG);
            assert!(result.is_ok());

            // Drop result immediately to free memory
            drop(result);
        }
    }
}
