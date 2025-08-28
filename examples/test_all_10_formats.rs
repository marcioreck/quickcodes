// Comprehensive Detection and Decoding Test for ALL 10 Barcode Formats
// Tests both detection AND decoding validation with 100% success target

use quickcodes::detection::{AdvancedDetector, DetectionConfig};
use quickcodes::types::BarcodeType;
use image::open;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    println!("🎯 COMPREHENSIVE BARCODE DETECTION & DECODING TEST - ALL 10 FORMATS");
    println!("Target: 100% detection + 100% decoding success for all formats\n");

    // Create detector with higher confidence for better accuracy
    let mut config = DetectionConfig::default();
    config.min_confidence = 0.6; // Higher threshold for more accurate detection
    config.max_codes_per_image = 3; // Limit to reduce false positives
    let detector = AdvancedDetector::new(config);

    // Test cases using our own generated images (PNG only for reliability)
    let test_cases = vec![
        // QR Code tests (3 variations)
        ("examples/output/demo_qr_hello.png", BarcodeType::QRCode, "Hello, QuickCodes!"),
        ("examples/output/demo_qr.png", BarcodeType::QRCode, "Hello, QuickCodes!"),
        ("examples/output/test_sheet_v1_barcode_QRCode_https___github.com_marcioreck_quickcodes.png", BarcodeType::QRCode, "https://github.com/marcioreck/quickcodes"),
        
        // DataMatrix tests (3 variations)
        ("examples/output/test_datamatrix_original.png", BarcodeType::DataMatrix, "Hello, DataMatrix!"),
        ("examples/output/test_datamatrix_90.png", BarcodeType::DataMatrix, "Hello, DataMatrix!"), 
        ("examples/output/test_sheet_v1_barcode_DataMatrix_010123456789012815240101.png", BarcodeType::DataMatrix, "010123456789012815240101"),
        
        // PDF417 tests (1 available)
        ("examples/output/pdf417_invoice.png", BarcodeType::PDF417, "DRIVER LICENSE|DOE,JOHN|DOB:1990-01-01"),
        
        // Aztec tests (1 available)
        ("examples/output/aztec_event.png", BarcodeType::Aztec, "TKT:A12345|FROM:NYC|TO:BOS|DATE:2025-08-21"),
        
        // EAN-13 tests (multiple variations)
        ("examples/output/test_cpp_ean13.png", BarcodeType::EAN13, "1234567890123"),
        ("examples/output/test_dotnet_ean13.png", BarcodeType::EAN13, "1234567890123"),
        ("examples/output/test_go_ean13.png", BarcodeType::EAN13, "1234567890123"),
        
        // UPC-A tests
        ("examples/output/test_sheet_v1_barcode_UPCA_03600029145.png", BarcodeType::UPCA, "036000291452"),
        
        // Code128 tests  
        ("examples/output/test_sheet_v1_barcode_Code128_HELLO123.png", BarcodeType::Code128, "HELLO123"),
        
        // Code39 tests - look for generated ones
        ("examples/output/test_sheet_v1_barcode_Code39_SERIAL-123ABC.png", BarcodeType::Code39, "SERIAL-123ABC"),
        
        // ITF-14 tests
        ("examples/output/itf14_box.png", BarcodeType::ITF14, "1234567890123"),
        ("examples/output/test_sheet_v1_barcode_ITF14_1234567890123.png", BarcodeType::ITF14, "1234567890123"),
        
        // Codabar tests - look for generated ones
        ("examples/output/test_sheet_v1_barcode_Codabar_A1234567890B.png", BarcodeType::Codabar, "A1234567890B"),
        
        // Generic barcode test
        ("examples/output/barcode.png", BarcodeType::QRCode, "Hello, QuickCodes!"),
    ];

    let mut results_by_format = HashMap::new();
    let mut total_tests = 0;
    let mut detection_successes = 0;
    let mut decoding_successes = 0;

    // Initialize counters for all 10 formats
    for format in [
        BarcodeType::QRCode, BarcodeType::DataMatrix, BarcodeType::PDF417,
        BarcodeType::Aztec, BarcodeType::EAN13, BarcodeType::UPCA,
        BarcodeType::Code128, BarcodeType::Code39, BarcodeType::ITF14,
        BarcodeType::Codabar
    ] {
        results_by_format.insert(format, (0, 0, 0)); // (detected, decoded, failed)
    }

    for (image_path, expected_format, expected_data) in test_cases {
        total_tests += 1;
        println!("📋 Test {}: {}", total_tests, image_path.split('/').last().unwrap_or(image_path));
        println!("   Expected: {:?} - '{}'", expected_format, expected_data);

        // Try to load the image
        let image = match open(image_path) {
            Ok(img) => img.to_luma8(),
            Err(e) => {
                println!("   ❌ FAILED TO LOAD: {}", e);
                let (detected, decoded, failed) = results_by_format.get_mut(&expected_format).unwrap();
                *failed += 1;
                continue;
            }
        };

        // Detect barcodes
        let detections = detector.detect_all(&image);
        
        if detections.is_empty() {
            println!("   ❌ DETECTION FAILED: No barcodes detected");
            let (detected, decoded, failed) = results_by_format.get_mut(&expected_format).unwrap();
            *failed += 1;
        } else {
            detection_successes += 1;
            println!("   ✅ DETECTION: {} barcode(s) found", detections.len());
            
            // Look for exact format and data match
            let mut found_perfect_match = false;
            let mut found_format_match = false;
            let mut found_data_match = false;
            
            for (i, detection) in detections.iter().enumerate() {
                println!("     🔍 #{}: {:?} (conf: {:.2}) - '{}'", 
                         i+1, detection.barcode_type, detection.confidence, detection.data);
                
                let format_match = detection.barcode_type == expected_format;
                let data_match = detection.data == expected_data;
                
                if format_match && data_match {
                    found_perfect_match = true;
                    break;
                } else if format_match {
                    found_format_match = true;
                } else if data_match {
                    found_data_match = true;
                }
            }
            
            // Update results
            let (detected, decoded, failed) = results_by_format.get_mut(&expected_format).unwrap();
            *detected += 1;
            
            if found_perfect_match {
                *decoded += 1;
                decoding_successes += 1;
                println!("   🎉 PERFECT MATCH: Format AND data correct!");
            } else if found_format_match {
                println!("   ⚠️  PARTIAL: Format correct, data mismatch");
                println!("      Expected: '{}'", expected_data);
            } else if found_data_match {
                println!("   ⚠️  PARTIAL: Data correct, format mismatch");
                println!("      Expected: {:?}", expected_format);
            } else {
                println!("   ❌ NO MATCH: Neither format nor data match");
            }
        }
        
        println!();
    }

    // Print comprehensive summary
    println!("📊 FINAL COMPREHENSIVE SUMMARY");
    println!("═══════════════════════════════");
    println!("Total Tests:           {}", total_tests);
    println!("Detection Successes:   {} ({:.1}%)", detection_successes, 
             100.0 * detection_successes as f32 / total_tests as f32);
    println!("Perfect Decodings:     {} ({:.1}%)", decoding_successes,
             100.0 * decoding_successes as f32 / total_tests as f32);
    
    if detection_successes > 0 {
        println!("Decoding Efficiency:   {:.1}%", 
                 100.0 * decoding_successes as f32 / detection_successes as f32);
    }
    println!();

    println!("📋 DETAILED RESULTS BY FORMAT:");
    println!("═══════════════════════════════");
    let mut total_formats_tested = 0;
    let mut perfect_formats = 0;
    
    for format in [
        BarcodeType::QRCode, BarcodeType::DataMatrix, BarcodeType::PDF417,
        BarcodeType::Aztec, BarcodeType::EAN13, BarcodeType::UPCA,
        BarcodeType::Code128, BarcodeType::Code39, BarcodeType::ITF14,
        BarcodeType::Codabar
    ] {
        let (detected, decoded, failed) = results_by_format.get(&format).unwrap();
        let total_for_format = detected + failed;
        
        if total_for_format > 0 {
            total_formats_tested += 1;
            let detection_rate = 100.0 * *detected as f32 / total_for_format as f32;
            let decoding_rate = if *detected > 0 { 
                100.0 * *decoded as f32 / *detected as f32 
            } else { 0.0 };
            
            let status = if *decoded == *detected && *detected > 0 {
                perfect_formats += 1;
                "🎯 PERFECT"
            } else if *decoded > 0 {
                "✅ PARTIAL"
            } else if *detected > 0 {
                "⚠️  DETECT ONLY"
            } else {
                "❌ FAILED"
            };
            
            println!("  {:12} {} - Tests: {}, Detected: {}/{} ({:.0}%), Decoded: {}/{} ({:.0}%)",
                     format!("{:?}:", format), status, total_for_format,
                     detected, total_for_format, detection_rate,
                     decoded, detected, decoding_rate);
        } else {
            println!("  {:12} ⭕ NO TESTS - No test images found", format!("{:?}:", format));
        }
    }

    println!();
    println!("🎯 OVERALL SUCCESS METRICS:");
    println!("═══════════════════════════");
    println!("Formats Tested:        {}/10", total_formats_tested);
    println!("Perfect Formats:       {}/{} ({:.0}%)", 
             perfect_formats, total_formats_tested,
             if total_formats_tested > 0 { 100.0 * perfect_formats as f32 / total_formats_tested as f32 } else { 0.0 });
    
    if perfect_formats == total_formats_tested && total_formats_tested >= 8 {
        println!("\n🏆 MISSION ACCOMPLISHED!");
        println!("   All tested formats have perfect detection + decoding!");
    } else if decoding_successes == detection_successes && detection_successes > 0 {
        println!("\n🎉 EXCELLENT PROGRESS!");
        println!("   All detected barcodes were perfectly decoded!");
    } else {
        println!("\n🔧 NEEDS IMPROVEMENT:");
        println!("   Some formats need better decoding accuracy");
    }

    Ok(())
}
