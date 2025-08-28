use quickcodes::detection::{AdvancedDetector, DetectionConfig};
use quickcodes::types::BarcodeType;
use image::open;
use std::collections::HashMap;

fn main() {
    println!("🧪 Comprehensive Barcode Detection Test - All 10 Types");
    println!("{}", "=".repeat(60));
    
    // Test cases: (format, image_path, expected_data)
    let test_cases = vec![
        // 1D Linear Barcodes
        (BarcodeType::EAN13, "examples/output/ean13_example.png", "Expected: 1234567890123"),
        (BarcodeType::UPCA, "examples/output/test_sheet_v1_barcode_UPCA_03600029145.png", "Expected: 036000291452"),
        (BarcodeType::Code128, "examples/output/test_sheet_v1_barcode_Code128_HELLO123.png", "Expected: HELLO123"),
        (BarcodeType::Code39, "examples/output/test_sheet_v1_barcode_Code39_SERIAL-123ABC.png", "Expected: SERIAL-123ABC"),
        (BarcodeType::ITF14, "examples/output/test_sheet_v1_barcode_ITF14_1234567890123.png", "Expected: 1234567890123"),
        (BarcodeType::Codabar, "examples/output/test_sheet_v1_barcode_Codabar_A1234567890B.png", "Expected: A1234567890B"),
        
        // 2D Matrix Barcodes
        (BarcodeType::QRCode, "examples/output/demo_qr.png", "Expected: Hello, QuickCodes!"),
        (BarcodeType::QRCode, "examples/output/test_sheet_v1_barcode_QRCode_https___github.com_marcioreck_quickcodes.png", "Expected: GitHub URL"),
        (BarcodeType::DataMatrix, "examples/output/demo_datamatrix.png", "Expected: Hello, DataMatrix!"),
        (BarcodeType::DataMatrix, "examples/output/test_sheet_v1_barcode_DataMatrix_010123456789012815240101.png", "Expected: 010123456789012815240101"),
        (BarcodeType::PDF417, "examples/output/test_sheet_v1_barcode_PDF417_DRIVER LICENSE_DOE,JOHN_DOB_1990-01-01.png", "Expected: DRIVER LICENSE|DOE,JOHN|DOB:1990-01-01"),
        (BarcodeType::Aztec, "examples/output/test_sheet_v1_barcode_Aztec_TKT_A12345_FROM_NYC_TO_BOS_DATE_2025-08-21.png", "Expected: TKT:A12345|FROM:NYC|TO:BOS|DATE:2025-08-21"),
    ];
    
    // Detection statistics
    let mut stats = DetectionStats::new();
    
    for (i, (barcode_type, image_path, expected)) in test_cases.iter().enumerate() {
        println!("\n📷 Test {}: {:?}", i + 1, barcode_type);
        println!("   🔍 Image: {}", image_path);
        println!("   🎯 {}", expected);
        
        match open(image_path) {
            Ok(img) => {
                let gray_img = img.to_luma8();
                println!("   📊 Image: {}x{}", gray_img.width(), gray_img.height());
                
                // Configure detector for this specific format
                let config = DetectionConfig {
                    target_formats: vec![*barcode_type],
                    min_confidence: 0.3, // Lower threshold for testing
                    max_codes_per_image: 5,
                    enable_rotation_correction: false,
                    enable_perspective_correction: false,
                    enable_multi_scale: false,
                    enable_contextual_analysis: false,
                };
                
                let detector = AdvancedDetector::new(config);
                let results = detector.detect_all(&gray_img);
                
                if results.is_empty() {
                    println!("   ❌ No codes detected");
                    stats.add_result(*barcode_type, false, None);
                } else {
                    println!("   ✅ Found {} candidate(s)", results.len());
                    for (j, result) in results.iter().enumerate() {
                        println!("      🎯 #{}: {:?} (confidence: {:.2})", 
                                 j + 1, result.barcode_type, result.confidence);
                        println!("         📝 Data: \"{}\"", result.data);
                        println!("         📍 Position: ({}, {}) {}x{}", 
                                 result.position.x, result.position.y,
                                 result.position.width, result.position.height);
                        println!("         📊 Scores: geometric={:.2}, pattern={:.2}, content={:.2}",
                                 result.geometric_score, result.pattern_score, result.content_score);
                    }
                    stats.add_result(*barcode_type, true, Some(results.len()));
                }
            },
            Err(e) => {
                println!("   ❌ Could not load image: {}", e);
                stats.add_result(*barcode_type, false, None);
            }
        }
    }
    
    // Print final statistics
    println!("\n{}", "=".repeat(60));
    println!("📊 DETECTION SUMMARY");
    println!("{}", "=".repeat(60));
    stats.print_summary();
    
    println!("\n🔬 ANALYSIS:");
    stats.print_analysis();
    
    println!("\n🎯 NEXT STEPS:");
    println!("   1. Implement format-specific decoders for failed detections");
    println!("   2. Tune confidence thresholds for each format");
    println!("   3. Add rotation and perspective correction");
    println!("   4. Test with webcam for real-time detection");
}

#[derive(Debug)]
struct DetectionStats {
    results: Vec<(BarcodeType, usize, usize, Vec<usize>)>, // (type, total, detected, candidate_counts)
}

impl DetectionStats {
    fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }
    
    fn add_result(&mut self, barcode_type: BarcodeType, detected: bool, candidate_count: Option<usize>) {
        // Find existing entry or create new one
        if let Some(entry) = self.results.iter_mut().find(|(bt, _, _, _)| *bt == barcode_type) {
            entry.1 += 1; // total
            if detected {
                entry.2 += 1; // detected
                if let Some(count) = candidate_count {
                    entry.3.push(count); // candidate counts
                }
            }
        } else {
            let detected_count = if detected { 1 } else { 0 };
            let candidate_counts = if let Some(count) = candidate_count { vec![count] } else { Vec::new() };
            self.results.push((barcode_type, 1, detected_count, candidate_counts));
        }
    }
    
    fn print_summary(&self) {
        for (barcode_type, total, detected, candidate_counts) in &self.results {
            let success_rate = if *total > 0 { (*detected as f32 / *total as f32) * 100.0 } else { 0.0 };
            let avg_candidates = if !candidate_counts.is_empty() {
                candidate_counts.iter().sum::<usize>() as f32 / candidate_counts.len() as f32
            } else {
                0.0
            };
            
            println!("   {:>12}: {}/{} ({:>5.1}%) - Avg candidates: {:.1}", 
                     format!("{:?}", barcode_type), detected, total, success_rate, avg_candidates);
        }
    }
    
    fn print_analysis(&self) {
        let total_tests = self.results.iter().map(|(_, total, _, _)| total).sum::<usize>();
        let total_detections = self.results.iter().map(|(_, _, detected, _)| detected).sum::<usize>();
        let overall_success = if total_tests > 0 { (total_detections as f32 / total_tests as f32) * 100.0 } else { 0.0 };
        
        println!("   📈 Overall Detection Rate: {}/{} ({:.1}%)", total_detections, total_tests, overall_success);
        
        // Best performing formats
        let mut sorted_results = self.results.clone();
        sorted_results.sort_by(|a, b| {
            let rate_a = if a.1 > 0 { a.2 as f32 / a.1 as f32 } else { 0.0 };
            let rate_b = if b.1 > 0 { b.2 as f32 / b.1 as f32 } else { 0.0 };
            rate_b.partial_cmp(&rate_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        println!("   🏆 Best Performers:");
        for (barcode_type, total, detected, _) in sorted_results.iter().take(3) {
            if *total > 0 {
                let rate = (*detected as f32 / *total as f32) * 100.0;
                println!("      {:?}: {:.1}%", barcode_type, rate);
            }
        }
        
        println!("   🔧 Needs Improvement:");
        for (barcode_type, total, detected, _) in sorted_results.iter().rev().take(3) {
            if *total > 0 {
                let rate = (*detected as f32 / *total as f32) * 100.0;
                if rate < 100.0 {
                    println!("      {:?}: {:.1}%", barcode_type, rate);
                }
            }
        }
    }
}
