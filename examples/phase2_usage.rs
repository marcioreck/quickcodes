//! Phase 2 Usage Examples
//!
//! This example demonstrates the new advanced 2D barcode formats
//! introduced in Phase 2: DataMatrix, PDF417, and Aztec Code.

use quickcodes::{generate_to_file, BarcodeType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 QuickCodes Phase 2 - Advanced 2D Codes Examples");
    println!("================================================");

    // Create output directory
    std::fs::create_dir_all("examples/output")?;

    // 1. DataMatrix - Pharmaceutical/ANVISA use case
    println!("📦 Generating DataMatrix (Pharmaceutical)...");
    let pharma_data = "010123456789012815240101"; // GS1 format for pharmaceuticals
    generate_to_file(
        BarcodeType::DataMatrix,
        pharma_data,
        "examples/output/datamatrix_pharma.svg",
    )?;
    println!("   ✅ Saved: examples/output/datamatrix_pharma.svg");

    // 2. DataMatrix - Industrial tracking
    println!("🏭 Generating DataMatrix (Industrial)...");
    let industrial_data = "PART:ABC123|LOT:20240115|SN:001234567";
    generate_to_file(
        BarcodeType::DataMatrix,
        industrial_data,
        "examples/output/datamatrix_industrial.png",
    )?;
    println!("   ✅ Saved: examples/output/datamatrix_industrial.png");

    // 3. PDF417 - Official documents
    println!("📄 Generating PDF417 (Document)...");
    let document_data = "DRIVER LICENSE|DOE,JOHN|DOB:1990-01-01|EXP:2025-12-31|CLASS:A";
    generate_to_file(
        BarcodeType::PDF417,
        document_data,
        "examples/output/pdf417_document.svg",
    )?;
    println!("   ✅ Saved: examples/output/pdf417_document.svg");

    // 4. PDF417 - Large data capacity
    println!("📊 Generating PDF417 (Large Data)...");
    let large_data = format!(
        "INVOICE:INV-{}\nDATE:{}\nCUSTOMER:{}\nAMOUNT:{}\nTAX:{}\nTOTAL:{}\nITEMS:{}",
        "2024-001234",
        "2024-01-15T10:30:00Z",
        "Acme Corporation Ltd, 123 Business St, Business City, BC 12345",
        "1,250.00",
        "187.50",
        "1,437.50",
        "Widget A (x5), Widget B (x3), Service Package Premium"
    );
    generate_to_file(
        BarcodeType::PDF417,
        &large_data,
        "examples/output/pdf417_invoice.png",
    )?;
    println!("   ✅ Saved: examples/output/pdf417_invoice.png");

    // 5. Aztec - Transport ticket
    println!("🚌 Generating Aztec (Transport)...");
    let ticket_data = "TKT:A12345|FROM:NYC Penn Station|TO:Boston South|DATE:2024-01-15|TIME:14:30|SEAT:12A|PRICE:89.50";
    generate_to_file(
        BarcodeType::Aztec,
        ticket_data,
        "examples/output/aztec_transport.svg",
    )?;
    println!("   ✅ Saved: examples/output/aztec_transport.svg");

    // 6. Aztec - Event ticket
    println!("🎫 Generating Aztec (Event)...");
    let event_data = "EVENT:Rock Concert 2024|VENUE:Madison Square Garden|DATE:2024-03-15|TIME:20:00|SECTION:A|ROW:12|SEAT:15|PRICE:125.00|GATE:B";
    generate_to_file(
        BarcodeType::Aztec,
        event_data,
        "examples/output/aztec_event.png",
    )?;
    println!("   ✅ Saved: examples/output/aztec_event.png");

    // 7. Unicode support test
    println!("🌍 Generating with Unicode data...");
    let unicode_data = "Olá! 你好! مرحبا! Здравствуйте! 🇧🇷🇨🇳🇸🇦🇷🇺";
    generate_to_file(
        BarcodeType::DataMatrix,
        unicode_data,
        "examples/output/datamatrix_unicode.svg",
    )?;
    println!("   ✅ Saved: examples/output/datamatrix_unicode.svg");

    println!();
    println!("🎉 Phase 2 examples completed successfully!");
    println!("📁 Check the examples/output/ directory for generated barcodes");
    println!();
    println!("📊 Summary:");
    println!("   • DataMatrix: Perfect for pharmaceutical tracking (ANVISA compliance)");
    println!("   • PDF417: Ideal for official documents and large data capacity");
    println!("   • Aztec: Great for transport and event tickets");
    println!("   • All formats support Unicode text");

    Ok(())
}
