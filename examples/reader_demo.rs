use quickcodes::{generate_to_file, read_from_file, read_all_from_file, BarcodeType};
use std::path::Path;

type BoxError = Box<dyn std::error::Error>;

fn main() -> Result<(), BoxError> {
    println!("🔍 QuickCodes Reader Demo - Reading Generated Barcodes");
    println!("=====================================================");

    // Criar alguns códigos para testar
    let test_data = vec![
        (BarcodeType::QRCode, "https://github.com/marcioreck/quickcodes", "demo_qr.png"),
        (BarcodeType::QRCode, "Hello, World!", "demo_qr_hello.png"),
        (BarcodeType::DataMatrix, "010123456789012815240101", "demo_datamatrix.png"),
    ];

    println!("\n📦 Generating test barcodes...");
    for (barcode_type, data, filename) in &test_data {
        let path = format!("examples/output/{}", filename);
        match generate_to_file(*barcode_type, data, &path) {
            Ok(_) => println!("   ✅ Generated {}: {}", filename, data),
            Err(e) => println!("   ❌ Failed to generate {}: {}", filename, e),
        }
    }

    println!("\n🔍 Reading barcodes back...");
    for (expected_type, expected_data, filename) in &test_data {
        let path = format!("examples/output/{}", filename);
        if Path::new(&path).exists() {
            println!("\n📸 Reading {}:", filename);
            match read_from_file(&path) {
                Ok(result) => {
                    println!("   ✅ Success!");
                    println!("      Expected: {:?} = \"{}\"", expected_type, expected_data);
                    println!("      Found:    {:?} = \"{}\"", result.barcode_type, result.data);
                    println!("      Confidence: {:.2}", result.confidence);
                    
                    // Verificar se o tipo está correto
                    if result.barcode_type == *expected_type {
                        println!("      ✅ Type detection: CORRECT");
                    } else {
                        println!("      ⚠️  Type detection: Expected {:?}, got {:?}", expected_type, result.barcode_type);
                    }
                }
                Err(e) => {
                    println!("   ❌ Error: {}", e);
                }
            }
        } else {
            println!("⚠️  File not found: {}", path);
        }
    }

    println!("\n🔍 Testing read_all_from_file...");
    // Testar com uma imagem que sabemos que existe
    let test_file = "examples/output/demo_qr.png";
    if Path::new(test_file).exists() {
        match read_all_from_file(test_file) {
            Ok(results) => {
                println!("   ✅ Found {} barcode(s) in {}:", results.len(), test_file);
                for (i, result) in results.iter().enumerate() {
                    println!("      {}. {:?}: \"{}\" (confidence: {:.2})", 
                             i + 1, result.barcode_type, result.data, result.confidence);
                }
            }
            Err(e) => {
                println!("   ❌ Error reading all codes: {}", e);
            }
        }
    }

    println!("\n📊 Reader Demo Summary:");
    println!("   • ✅ QR Code detection: WORKING PERFECTLY! (usando rqrr real)");
    println!("   • 🚧 DataMatrix detection: In development");
    println!("   • 🚧 1D Barcode detection: In development");
    println!("   • ✅ PNG format support: Working");
    println!("   • ❌ SVG format support: Not available for reading");
    println!("   • 🔧 Reading accuracy: Depends on image quality and barcode type");

    println!("\n🎯 Next Steps:");
    println!("   1. ✅ Implementar decodificação real de QR Code: CONCLUÍDO!");
    println!("   2. Add DataMatrix decoding support");
    println!("   3. Add 1D barcode decoding (EAN-13, Code128, etc.)");
    println!("   4. Improve detection algorithms for better accuracy");
    println!("   5. Add support for rotated and skewed codes");

    Ok(())
}
