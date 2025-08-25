use quickcodes::{generate_to_file, BarcodeType};

fn main() {
    println!("🚀 QuickCodes - Legacy Formats Examples (Phase 3)\n");

    // Criar diretório de saída se não existir
    std::fs::create_dir_all("examples/output").unwrap();

    // 1. Code39 - Alfanumérico + Símbolos
    println!("1. Generating Code39 barcode for serial number...");
    generate_to_file(BarcodeType::Code39, "SERIAL123ABC", "examples/output/code39_serial.svg").unwrap();
    println!("   ✅ Saved to examples/output/code39_serial.svg");

    // 2. ITF-14 - Embalagens
    println!("2. Generating ITF-14 barcode for packaging...");
    generate_to_file(BarcodeType::ITF14, "1234567890123", "examples/output/itf14_box.png").unwrap();
    println!("   ✅ Saved to examples/output/itf14_box.png");

    // 3. Codabar - Bibliotecas
    println!("3. Generating Codabar for library system...");
    generate_to_file(BarcodeType::Codabar, "A1234567890B", "examples/output/codabar_library.svg").unwrap();
    println!("   ✅ Saved to examples/output/codabar_library.svg");

    println!("\n🎉 All legacy format examples generated successfully!");
    println!("Check the examples/output/ directory for the generated files.");
}
