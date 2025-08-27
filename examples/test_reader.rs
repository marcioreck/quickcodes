use quickcodes::{read_from_file, read_all_from_file};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Testing QuickCodes Reader with QR Code images");
    println!("================================================");

    // Testar com imagem QR Code PNG
    let qr_png_path = "examples/output/test_sheet_v1_barcode_QRCode_https___github.com_marcioreck_quickcodes.png";
    if Path::new(qr_png_path).exists() {
        println!("\n📸 Testing QR Code PNG: {}", qr_png_path);
        match read_from_file(qr_png_path) {
            Ok(result) => {
                println!("✅ Success!");
                println!("   Type: {:?}", result.barcode_type);
                println!("   Data: {}", result.data);
                println!("   Confidence: {:.2}", result.confidence);
            }
            Err(e) => {
                println!("❌ Error: {}", e);
            }
        }
    } else {
        println!("⚠️  QR Code PNG not found: {}", qr_png_path);
    }

    // Testar com imagem QR Code SVG
    let qr_svg_path = "examples/output/test_sheet_v1_barcode_QRCode_https___github.com_marcioreck_quickcodes.svg";
    if Path::new(qr_svg_path).exists() {
        println!("\n📸 Testing QR Code SVG: {}", qr_svg_path);
        match read_from_file(qr_svg_path) {
            Ok(result) => {
                println!("✅ Success!");
                println!("   Type: {:?}", result.barcode_type);
                println!("   Data: {}", result.data);
                println!("   Confidence: {:.2}", result.confidence);
            }
            Err(e) => {
                println!("❌ Error: {}", e);
            }
        }
    } else {
        println!("⚠️  QR Code SVG not found: {}", qr_svg_path);
    }

    // Testar com read_all_from_file
    if Path::new(qr_png_path).exists() {
        println!("\n📸 Testing read_all_from_file with QR Code PNG");
        match read_all_from_file(qr_png_path) {
            Ok(results) => {
                println!("✅ Found {} barcode(s):", results.len());
                for (i, result) in results.iter().enumerate() {
                    println!("   {}. Type: {:?}, Data: {}, Confidence: {:.2}", 
                             i + 1, result.barcode_type, result.data, result.confidence);
                }
            }
            Err(e) => {
                println!("❌ Error: {}", e);
            }
        }
    }

    println!("\n🔍 Reader test completed!");

    Ok(())
}
