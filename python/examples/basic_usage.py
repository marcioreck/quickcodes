#!/usr/bin/env python3
"""
QuickCodes Python Examples
Basic usage demonstration of the QuickCodes library
"""

import quickcodes
import os

def main():
    print("🚀 QuickCodes Python - Basic Usage Examples\n")
    
    # Create output directory
    os.makedirs("python_output", exist_ok=True)
    
    # Example 1: Generate QR Code as SVG
    print("1. Generating QR Code as SVG...")
    qr_svg = quickcodes.generate("QRCode", "Hello from Python!", "SVG")
    with open("python_output/qr_python.svg", "wb") as f:
        f.write(qr_svg)
    print("   ✅ Saved to python_output/qr_python.svg")
    
    # Example 2: Generate EAN-13 as PNG
    print("2. Generating EAN-13 barcode as PNG...")
    ean_png = quickcodes.generate("EAN13", "123456789012", "PNG")
    with open("python_output/ean13_python.png", "wb") as f:
        f.write(ean_png)
    print("   ✅ Saved to python_output/ean13_python.png")
    
    # Example 3: Using generate_to_file
    print("3. Using generate_to_file for UPC-A...")
    quickcodes.generate_to_file("UPCA", "03600029145", "python_output/upc_python.svg")
    print("   ✅ Saved to python_output/upc_python.svg")
    
    # Example 4: QR Code with custom settings
    print("4. Generating QR Code with high error correction...")
    qr_high_ec = quickcodes.generate(
        "QRCode", 
        "High error correction QR", 
        "SVG",
        margin=20,
        error_correction="High"
    )
    with open("python_output/qr_high_ec.svg", "wb") as f:
        f.write(qr_high_ec)
    print("   ✅ Saved to python_output/qr_high_ec.svg")
    
    # Example 5: Generate Code128
    print("5. Generating Code128 barcode...")
    quickcodes.generate_to_file("Code128", "PYTHON123", "python_output/code128_python.svg")
    print("   ✅ Saved to python_output/code128_python.svg")
    
    # Example 6: Pix payment QR Code
    print("6. Generating QR Code for Pix payment...")
    pix_data = "00020126580014BR.GOV.BCB.PIX0114+5551999999995204000053039865405100.005802BR5920Python Example6009SAO PAULO62070503***6304ABCD"
    quickcodes.generate_to_file("QRCode", pix_data, "python_output/pix_python.png")
    print("   ✅ Saved to python_output/pix_python.png")
    
    print("\n🎉 All Python examples generated successfully!")
    print("Check the python_output/ directory for the generated files.")

if __name__ == "__main__":
    main()
