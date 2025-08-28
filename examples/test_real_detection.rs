// Test the detection system with a real barcode image
use quickcodes::detection::{AdvancedDetector, DetectionConfig};
use quickcodes::types::BarcodeType;
use image::open;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Real Barcode Detection Test");
    
    // Check if we have any barcode images in the examples output
    let test_images = [
        "examples/output/demo_qr.png",
        "examples/output/demo_datamatrix.png", 
        "examples/output/ean13_example.png",
        "barcode.png"
    ];
    
    // Configure detection for high accuracy
    let config = DetectionConfig {
        min_confidence: 0.7, // Lower threshold for testing
        enable_rotation_correction: true,
        enable_perspective_correction: true,
        target_formats: vec![
            BarcodeType::QRCode,
            BarcodeType::DataMatrix,
            BarcodeType::EAN13,
            BarcodeType::Code128,
        ],
        enable_multi_scale: true,
        enable_contextual_analysis: true,
        max_codes_per_image: 10,
    };
    
    let detector = AdvancedDetector::new(config);
    
    for image_path in &test_images {
        if Path::new(image_path).exists() {
            println!("\n📷 Testing image: {}", image_path);
            
            match open(image_path) {
                Ok(img) => {
                    // Convert to grayscale ImageBuffer as expected by detector
                    let gray_img = img.to_luma8();
                    let results = detector.detect_all(&gray_img);
                    println!("   📊 Found {} potential codes", results.len());
                    
                    for (i, result) in results.iter().enumerate() {
                        println!("   🔍 Code {}: {:?} (confidence: {:.2})", 
                               i + 1, result.barcode_type, result.confidence);
                        println!("      📍 Location: ({}, {}) to ({}, {})",
                               result.position.x, result.position.y,
                               result.position.x + result.position.width,
                               result.position.y + result.position.height);
                    }
                    
                    if results.is_empty() {
                        println!("   ℹ️  No codes detected - this is expected as detection engines need full implementation");
                    }
                }
                Err(e) => {
                    println!("   ❌ Could not load image: {}", e);
                }
            }
        }
    }
    
    println!("\n✅ Real detection test completed!");
    println!("📋 Next steps:");
    println!("   1. Implement full QR finder pattern scanning");
    println!("   2. Add DataMatrix L-border detection");
    println!("   3. Implement linear barcode scanning");
    println!("   4. Add format-specific validation");
    println!("   5. Optimize performance and accuracy");
    
    Ok(())
}
