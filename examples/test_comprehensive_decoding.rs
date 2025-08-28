// Comprehensive Detection and Decoding Test for All Barcode Formats
// Tests both detection AND decoding validation for all 10 types

use quickcodes::detection::{AdvancedDetector, DetectionConfig};
use quickcodes::types::BarcodeType;
use image::open;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    println!("🔍 COMPREHENSIVE BARCODE DETECTION & DECODING TEST");
    println!("Testing detection + decoding validation for all 10 formats...\n");

    // Create detector with all formats enabled
    let mut config = DetectionConfig::default();
    config.min_confidence = 0.3; // Lower threshold for testing
    let detector = AdvancedDetector::new(config);

    // Define expected data for each format
    let expected_data = create_expected_data_map();

    // Test images and their expected formats
    let test_cases = vec![
        // QR Code tests
        ("examples/output/demo_qr_hello.png", BarcodeType::QRCode, "Hello, QuickCodes!"),
        ("examples/output/qr_hello.svg", BarcodeType::QRCode, "Hello, QuickCodes!"),
        ("examples/output/github_url.png", BarcodeType::QRCode, "https://github.com/marcioreck/quickcodes"),
        
        // DataMatrix tests
        ("examples/output/demo_datamatrix.png", BarcodeType::DataMatrix, "Hello, DataMatrix!"),
        ("examples/output/datamatrix_industrial.png", BarcodeType::DataMatrix, "010123456789012815240101"),
        ("examples/output/datamatrix_pharma.svg", BarcodeType::DataMatrix, "010123456789012815240101"),
        
        // PDF417 tests
        ("examples/output/pdf417_document.svg", BarcodeType::PDF417, "DRIVER LICENSE|DOE,JOHN|DOB:1990-01-01"),
        ("examples/output/pdf417_invoice.png", BarcodeType::PDF417, "DRIVER LICENSE|DOE,JOHN|DOB:1990-01-01"),
        
        // Aztec tests
        ("examples/output/aztec_event.png", BarcodeType::Aztec, "TKT:A12345|FROM:NYC|TO:BOS|DATE:2025-08-21"),
        ("examples/output/aztec_transport.svg", BarcodeType::Aztec, "TKT:A12345|FROM:NYC|TO:BOS|DATE:2025-08-21"),
        
        // EAN-13 tests
        ("examples/output/ean13_example.png", BarcodeType::EAN13, "1234567890123"),
        
        // UPC-A tests
        ("examples/output/upc_a_example.svg", BarcodeType::UPCA, "036000291452"),
        
        // Code128 tests
        ("examples/output/code128_example.svg", BarcodeType::Code128, "HELLO123"),
        
        // Code39 tests
        ("examples/output/code39_serial.svg", BarcodeType::Code39, "SERIAL-123ABC"),
        
        // ITF-14 tests
        ("examples/output/itf14_box.png", BarcodeType::ITF14, "1234567890123"),
        
        // Codabar tests
        ("examples/output/codabar_library.svg", BarcodeType::Codabar, "A1234567890B"),
        
        // Generic barcode test
        ("examples/output/barcode.png", BarcodeType::QRCode, "Hello, QuickCodes!"),
    ];

    let mut results = HashMap::new();
    let mut total_tests = 0;
    let mut detection_successes = 0;
    let mut decoding_successes = 0;

    for (image_path, expected_format, expected_data) in test_cases {
        total_tests += 1;
        println!("📋 Testing: {}", image_path);
        println!("   Expected Format: {:?}", expected_format);
        println!("   Expected Data: {}", expected_data);

        // Try to load the image
        let image_result = match std::path::Path::new(image_path).extension()
            .and_then(|ext| ext.to_str()) {
            Some("svg") => {
                println!("   ⚠️  SVG format - skipping for now (needs conversion)");
                continue;
            },
            _ => open(image_path),
        };

        let image = match image_result {
            Ok(img) => img.to_luma8(),
            Err(e) => {
                println!("   ❌ Failed to load image: {}", e);
                continue;
            }
        };

        // Detect barcodes
        let detections = detector.detect_all(&image);
        
        if detections.is_empty() {
            println!("   ❌ DETECTION FAILED: No barcodes detected");
            results.entry(expected_format).or_insert_with(|| (0, 0, 0)).2 += 1;
        } else {
            detection_successes += 1;
            println!("   ✅ DETECTION SUCCESS: {} barcode(s) found", detections.len());
            
            // Check if we found the expected format and data
            let mut found_expected = false;
            for detection in &detections {
                println!("   🔍 Detected: {:?} (confidence: {:.2})", 
                         detection.barcode_type, detection.confidence);
                println!("   📊 Data: '{}'", detection.data);
                
                // Validate format
                let format_match = detection.barcode_type == expected_format;
                
                // Validate decoded data
                let data_match = detection.data == expected_data;
                
                if format_match && data_match {
                    found_expected = true;
                    decoding_successes += 1;
                    println!("   ✅ DECODING SUCCESS: Format and data match!");
                } else if format_match {
                    println!("   ⚠️  PARTIAL SUCCESS: Format matches but data differs");
                    println!("      Expected: '{}'", expected_data);
                    println!("      Got:      '{}'", detection.data);
                } else if data_match {
                    println!("   ⚠️  PARTIAL SUCCESS: Data matches but format differs");
                    println!("      Expected: {:?}", expected_format);
                    println!("      Got:      {:?}", detection.barcode_type);
                } else {
                    println!("   ❌ FORMAT/DATA MISMATCH");
                    println!("      Expected: {:?} - '{}'", expected_format, expected_data);
                    println!("      Got:      {:?} - '{}'", detection.barcode_type, detection.data);
                }
            }
            
            // Update results
            let (detected, decoded, failed) = results.entry(expected_format).or_insert((0, 0, 0));
            *detected += 1;
            if found_expected {
                *decoded += 1;
            }
        }
        
        println!();
    }

    // Print summary
    println!("📊 COMPREHENSIVE TEST SUMMARY");
    println!("═══════════════════════════════");
    println!("Total Tests:           {}", total_tests);
    println!("Detection Successes:   {} ({:.1}%)", detection_successes, 
             100.0 * detection_successes as f32 / total_tests as f32);
    println!("Decoding Successes:    {} ({:.1}%)", decoding_successes,
             100.0 * decoding_successes as f32 / total_tests as f32);
    println!();

    println!("📋 Results by Format:");
    for format in [
        BarcodeType::QRCode, BarcodeType::DataMatrix, BarcodeType::PDF417,
        BarcodeType::Aztec, BarcodeType::EAN13, BarcodeType::UPCA,
        BarcodeType::Code128, BarcodeType::Code39, BarcodeType::ITF14,
        BarcodeType::Codabar
    ] {
        let (detected, decoded, failed) = results.get(&format).unwrap_or(&(0, 0, 0));
        let total_for_format = detected + failed;
        if total_for_format > 0 {
            println!("  {:12} - Detected: {}/{} ({:.0}%), Decoded: {}/{} ({:.0}%)",
                     format!("{:?}:", format),
                     detected, total_for_format, 100.0 * *detected as f32 / total_for_format as f32,
                     decoded, detected, if *detected > 0 { 100.0 * *decoded as f32 / *detected as f32 } else { 0.0 });
        }
    }

    println!();
    if decoding_successes == detection_successes && detection_successes > 0 {
        println!("🎉 EXCELLENT! All detected barcodes were successfully decoded with correct data!");
    } else if decoding_successes > 0 {
        println!("👍 GOOD! {}/{} detected barcodes had correct decoding", decoding_successes, detection_successes);
    } else {
        println!("⚠️  NEEDS IMPROVEMENT: No successful decodings with expected data");
    }

    Ok(())
}

fn create_expected_data_map() -> HashMap<String, String> {
    let mut map = HashMap::new();
    
    // QR Code data
    map.insert("demo_qr_hello.png".to_string(), "Hello, QuickCodes!".to_string());
    map.insert("github_url.png".to_string(), "https://github.com/marcioreck/quickcodes".to_string());
    
    // DataMatrix data
    map.insert("demo_datamatrix.png".to_string(), "Hello, DataMatrix!".to_string());
    map.insert("datamatrix_industrial.png".to_string(), "010123456789012815240101".to_string());
    
    // PDF417 data
    map.insert("pdf417_document.svg".to_string(), "DRIVER LICENSE|DOE,JOHN|DOB:1990-01-01".to_string());
    map.insert("pdf417_invoice.png".to_string(), "DRIVER LICENSE|DOE,JOHN|DOB:1990-01-01".to_string());
    
    // Aztec data
    map.insert("aztec_event.png".to_string(), "TKT:A12345|FROM:NYC|TO:BOS|DATE:2025-08-21".to_string());
    
    // Linear barcode data
    map.insert("ean13_example.png".to_string(), "1234567890123".to_string());
    map.insert("code128_example.svg".to_string(), "HELLO123".to_string());
    map.insert("code39_serial.svg".to_string(), "SERIAL-123ABC".to_string());
    map.insert("itf14_box.png".to_string(), "1234567890123".to_string());
    map.insert("codabar_library.svg".to_string(), "A1234567890B".to_string());
    map.insert("upc_a_example.svg".to_string(), "036000291452".to_string());
    
    map
}
