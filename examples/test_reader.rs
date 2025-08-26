/// Test the barcode reader functionality
/// 
/// This example tests reading barcodes from generated images
use quickcodes::{generate_to_file, read_all_from_file, read_from_file, BarcodeType, ExportFormat};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Testing QuickCodes Reader Functionality\n");

    // Test QR Code reading
    test_qr_code_reading()?;
    
    // Test with non-existent file
    test_error_handling()?;

    println!("\n✅ Reader tests completed!");
    Ok(())
}

fn test_qr_code_reading() -> Result<(), Box<dyn std::error::Error>> {
    println!("📱 Testing QR Code Reading:");
    
    // Use existing QR Code image
    let qr_image_path = "examples/output/test_sheet_v1_barcode_QRCode_https___github.com_marcioreck_quickcodes.png";
    
    if Path::new(qr_image_path).exists() {
        println!("  📁 Reading from: {}", qr_image_path);
        
        match read_from_file(qr_image_path) {
            Ok(result) => {
                println!("  ✅ Successfully read QR Code!");
                println!("     Type: {:?}", result.barcode_type);
                println!("     Data: {}", result.data);
                println!("     Confidence: {:.2}", result.confidence);
            }
            Err(e) => {
                println!("  ❌ Failed to read QR Code: {}", e);
            }
        }
        
        // Test reading all codes
        println!("\n  🔍 Testing read_all_from_file:");
        match read_all_from_file(qr_image_path) {
            Ok(results) => {
                println!("  ✅ Found {} barcode(s)", results.len());
                for (i, result) in results.iter().enumerate() {
                    println!("     Code {}: {:?} = {}", i + 1, result.barcode_type, result.data);
                }
            }
            Err(e) => {
                println!("  ❌ Failed to read all codes: {}", e);
            }
        }
    } else {
        println!("  ⚠️  QR Code test image not found. Generating one...");
        
        // Generate a test QR code
        let test_data = "https://github.com/marcioreck/quickcodes";
        generate_to_file(BarcodeType::QRCode, test_data, "examples/output/test_reader_qr.png")?;
        
        println!("  📁 Generated test QR code. Testing reading...");
        match read_from_file("examples/output/test_reader_qr.png") {
            Ok(result) => {
                println!("  ✅ Successfully read generated QR Code!");
                println!("     Type: {:?}", result.barcode_type);
                println!("     Data: {}", result.data);
                println!("     Confidence: {:.2}", result.confidence);
                
                // Verify data matches
                if result.data == test_data {
                    println!("  ✅ Data matches expected content!");
                } else {
                    println!("  ⚠️  Data doesn't match: expected '{}', got '{}'", test_data, result.data);
                }
            }
            Err(e) => {
                println!("  ❌ Failed to read generated QR Code: {}", e);
            }
        }
    }
    
    Ok(())
}

fn test_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🚨 Testing Error Handling:");
    
    // Test with non-existent file
    match read_from_file("non_existent_file.png") {
        Ok(_) => {
            println!("  ❌ Should have failed for non-existent file");
        }
        Err(e) => {
            println!("  ✅ Correctly failed for non-existent file: {}", e);
        }
    }
    
    // Test with invalid image file
    std::fs::write("examples/output/test_invalid.txt", "This is not an image")?;
    match read_from_file("examples/output/test_invalid.txt") {
        Ok(_) => {
            println!("  ❌ Should have failed for invalid image");
        }
        Err(e) => {
            println!("  ✅ Correctly failed for invalid image: {}", e);
        }
    }
    
    // Clean up test file
    let _ = std::fs::remove_file("examples/output/test_invalid.txt");
    
    Ok(())
}
