//! Basic usage examples of QuickCodes

use quickcodes::{generate, generate_to_file, BarcodeType, ExportFormat};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 QuickCodes - Basic Usage Examples\n");

    // Example 1: Generate QR Code as SVG
    println!("1. Generating QR Code as SVG...");
    let qr_svg = generate(BarcodeType::QRCode, "Hello, QuickCodes!", ExportFormat::SVG)?;
    fs::write("examples/output/qr_hello.svg", qr_svg)?;
    println!("   ✅ Saved to examples/output/qr_hello.svg");

    // Example 2: Generate EAN-13 as PNG
    println!("2. Generating EAN-13 barcode as PNG...");
    let ean_png = generate(BarcodeType::EAN13, "123456789012", ExportFormat::PNG)?;
    fs::write("examples/output/ean13_example.png", ean_png)?;
    println!("   ✅ Saved to examples/output/ean13_example.png");

    // Example 3: Generate UPC-A as SVG
    println!("3. Generating UPC-A barcode as SVG...");
    let upc_svg = generate(BarcodeType::UPCA, "03600029145", ExportFormat::SVG)?;
    fs::write("examples/output/upc_a_example.svg", upc_svg)?;
    println!("   ✅ Saved to examples/output/upc_a_example.svg");

    // Example 4: Using generate_to_file convenience function
    println!("4. Using generate_to_file for Code128...");
    generate_to_file(
        BarcodeType::Code128,
        "HELLO123",
        "examples/output/code128_example.svg",
    )?;
    println!("   ✅ Saved to examples/output/code128_example.svg");

    // Example 5: Generate QR Code for Pix payment (Brazilian instant payment)
    println!("5. Generating QR Code for Pix payment...");
    let pix_data = "00020126580014BR.GOV.BCB.PIX0114+5551999999995204000053039865405100.005802BR5920Padaria Exemplo6009SAO PAULO62070503***6304ABCD";
    generate_to_file(
        BarcodeType::QRCode,
        pix_data,
        "examples/output/pix_payment.svg",
    )?;
    println!("   ✅ Saved to examples/output/pix_payment.svg");

    // Example 6: Generate QR Code with URL
    println!("6. Generating QR Code with URL...");
    generate_to_file(
        BarcodeType::QRCode,
        "https://github.com/quickcodes/quickcodes",
        "examples/output/github_url.png",
    )?;
    println!("   ✅ Saved to examples/output/github_url.png");

    println!("\n🎉 All examples generated successfully!");
    println!("Check the examples/output/ directory for the generated files.");

    Ok(())
}
