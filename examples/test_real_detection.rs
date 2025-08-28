// Test the detection system with a real barcode image
use quickcodes::detection::{AdvancedDetector, DetectionConfig};
use quickcodes::types::BarcodeType;
use image::open;
use std::path::Path;

fn main() {
    println!("🔍 Testing QuickCodes Detection System");
    println!("======================================");
    
    let test_images = vec![
        "examples/output/demo_qr.png",
        "examples/output/demo_datamatrix.png", 
        "examples/output/ean13_example.png",
        "examples/output/barcode.png"
    ];
    
    // Configuration for general detection
    let config = DetectionConfig {
        target_formats: vec![
            BarcodeType::QRCode,
            BarcodeType::DataMatrix,
            BarcodeType::EAN13,
            BarcodeType::UPCA,
            BarcodeType::Code128,
            BarcodeType::Code39,
            BarcodeType::ITF14,
            BarcodeType::Codabar,
            BarcodeType::PDF417,
            BarcodeType::Aztec,
        ],
        min_confidence: 0.7,
        max_codes_per_image: 5,
        enable_rotation_correction: true,
        enable_perspective_correction: true,
        enable_multi_scale: true,
        enable_contextual_analysis: true,
    };

    let detector = AdvancedDetector::new(config);

    for image_path in test_images {
        println!("\n📷 Testing: {}", image_path);
        
        if !Path::new(image_path).exists() {
            println!("   ⚠️  File not found, skipping...");
            continue;
        }

        match open(image_path) {
            Ok(img) => {
                let gray_img = img.to_luma8();
                println!("   📊 Image: {}x{}", gray_img.width(), gray_img.height());
                
                let results = detector.detect_all(&gray_img);
                
                if results.is_empty() {
                    println!("   ❌ No codes detected");
                } else {
                    println!("   ✅ Found {} code(s):", results.len());
                    for (i, result) in results.iter().enumerate() {
                        println!("      {}. {:?} (confidence: {:.2})", 
                                 i + 1, result.barcode_type, result.confidence);
                        println!("         Data: \"{}\"", result.data);
                        println!("         Position: ({}, {}) {}x{}", 
                                 result.position.x, result.position.y,
                                 result.position.width, result.position.height);
                        println!("         Scores: geometric={:.2}, pattern={:.2}, content={:.2}",
                                 result.geometric_score, result.pattern_score, result.content_score);
                    }
                }
            },
            Err(e) => {
                println!("   ❌ Error loading image: {}", e);
            }
        }
    }
    
    println!("\n🏁 Detection test completed!");
}
