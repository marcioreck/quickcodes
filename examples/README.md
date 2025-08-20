# QuickCodes Examples

This directory contains usage examples for the QuickCodes library.

## Running Examples

```bash
# Run the basic usage example
cargo run --example basic_usage

# This will generate various barcodes in the examples/output/ directory
```

## Generated Files

After running `basic_usage`, you'll find these files in `examples/output/`:

- **qr_hello.svg** - QR Code with "Hello, QuickCodes!" text
- **ean13_example.png** - EAN-13 barcode (123456789012 + check digit)
- **upc_a_example.svg** - UPC-A barcode (036000291452)
- **code128_example.svg** - Code128 barcode with "HELLO123"
- **pix_payment.svg** - QR Code for Brazilian Pix payment
- **github_url.png** - QR Code with GitHub URL

## Example Code

```rust
use quickcodes::{generate, BarcodeType, ExportFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate QR Code as SVG
    let qr_svg = generate(BarcodeType::QRCode, "Hello!", ExportFormat::SVG)?;
    std::fs::write("qr_code.svg", qr_svg)?;
    
    // Generate EAN-13 as PNG  
    let ean_png = generate(BarcodeType::EAN13, "123456789012", ExportFormat::PNG)?;
    std::fs::write("barcode.png", ean_png)?;
    
    Ok(())
}
```

## Viewing the Results

- **SVG files** can be opened in any web browser or vector graphics editor
- **PNG files** can be opened in any image viewer
- All generated barcodes are production-ready and follow industry standards
