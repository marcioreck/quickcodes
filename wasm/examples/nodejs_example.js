// QuickCodes WASM Node.js Example
// Demonstrates how to use QuickCodes WebAssembly module in Node.js

const fs = require('fs');
const path = require('path');

// Import the WebAssembly module
// Note: After building, this will be available at ../pkg/nodejs/quickcodes_wasm.js
async function main() {
    try {
        // Import the WASM module (adjust path after build)
        const wasm = await import('../pkg/nodejs/quickcodes_wasm.js');
        
        console.log('🚀 QuickCodes WASM Node.js Example\n');
        
        // Test supported types
        const supportedTypes = wasm.get_supported_types();
        const supportedFormats = wasm.get_supported_formats();
        
        console.log('📋 Supported barcode types:', supportedTypes);
        console.log('📋 Supported export formats:', supportedFormats);
        console.log('');
        
        // Example 1: Generate QR Code as SVG
        console.log('📝 Example 1: Generating QR Code as SVG...');
        const qrResult = wasm.generate_svg('QRCode', 'https://github.com/marcioreck/quickcodes');
        
        if (qrResult.success) {
            fs.writeFileSync('output_qr.svg', qrResult.data);
            console.log('✅ QR Code saved as output_qr.svg');
        } else {
            console.error('❌ Failed to generate QR Code:', qrResult.error);
        }
        
        // Example 2: Generate EAN-13 as PNG (base64)
        console.log('\n📝 Example 2: Generating EAN-13 as PNG...');
        const eanResult = wasm.generate('EAN13', '123456789012', 'PNG');
        
        if (eanResult.success) {
            const pngBuffer = Buffer.from(eanResult.data, 'base64');
            fs.writeFileSync('output_ean13.png', pngBuffer);
            console.log('✅ EAN-13 saved as output_ean13.png');
        } else {
            console.error('❌ Failed to generate EAN-13:', eanResult.error);
        }
        
        // Example 3: Generate DataMatrix for pharmaceutical tracking
        console.log('\n📝 Example 3: Generating DataMatrix for pharmaceutical...');
        const dmResult = wasm.generate_svg('DataMatrix', '010123456789012815240101');
        
        if (dmResult.success) {
            fs.writeFileSync('output_datamatrix.svg', dmResult.data);
            console.log('✅ DataMatrix saved as output_datamatrix.svg');
        } else {
            console.error('❌ Failed to generate DataMatrix:', dmResult.error);
        }
        
        // Example 4: Generate PDF417 for documents
        console.log('\n📝 Example 4: Generating PDF417 for document...');
        const pdfResult = wasm.generate_svg('PDF417', 'DRIVER LICENSE|DOE,JOHN|DOB:1990-01-01|EXP:2028-01-01');
        
        if (pdfResult.success) {
            fs.writeFileSync('output_pdf417.svg', pdfResult.data);
            console.log('✅ PDF417 saved as output_pdf417.svg');
        } else {
            console.error('❌ Failed to generate PDF417:', pdfResult.error);
        }
        
        // Example 5: Generate Aztec for transport tickets
        console.log('\n📝 Example 5: Generating Aztec for transport ticket...');
        const aztecResult = wasm.generate_svg('Aztec', 'TKT:12345|FROM:NYC|TO:BOS|DATE:2025-08-21|SEAT:12A');
        
        if (aztecResult.success) {
            fs.writeFileSync('output_aztec.svg', aztecResult.data);
            console.log('✅ Aztec saved as output_aztec.svg');
        } else {
            console.error('❌ Failed to generate Aztec:', aztecResult.error);
        }
        
        // Example 6: Read barcode from file (if readers feature is enabled)
        console.log('\n📝 Example 6: Reading barcode from image file...');
        
        // Create a simple test image file path
        const testImagePath = 'test_barcode.png';
        
        if (fs.existsSync(testImagePath)) {
            try {
                const imageBuffer = fs.readFileSync(testImagePath);
                const file = new File([imageBuffer], 'test.png', { type: 'image/png' });
                
                const readResult = await wasm.read_from_file(file);
                
                if (readResult.success) {
                    console.log('✅ Barcode read successfully!');
                    console.log('   Type:', readResult.barcode_type);
                    console.log('   Data:', readResult.data);
                } else {
                    console.log('🔍 No barcode found in image:', readResult.error);
                }
            } catch (error) {
                console.log('⚠️ Reader test skipped (feature may not be enabled):', error.message);
            }
        } else {
            console.log('⚠️ Reader test skipped (no test image file found)');
            console.log('   To test reading, place a barcode image as "test_barcode.png"');
        }
        
        console.log('\n🎉 All examples completed successfully!');
        console.log('\n📁 Generated files:');
        console.log('   - output_qr.svg (QR Code)');
        console.log('   - output_ean13.png (EAN-13)');
        console.log('   - output_datamatrix.svg (DataMatrix)');
        console.log('   - output_pdf417.svg (PDF417)');
        console.log('   - output_aztec.svg (Aztec)');
        
    } catch (error) {
        console.error('❌ Error running examples:', error);
        console.log('\n💡 Make sure to build the WASM module first:');
        console.log('   cd ../');
        console.log('   ./build.sh');
    }
}

// Helper function to create File object in Node.js environment
// (This is a polyfill since File is a browser API)
class File {
    constructor(buffer, name, options = {}) {
        this.buffer = buffer;
        this.name = name;
        this.type = options.type || '';
        this.size = buffer.length;
    }
    
    arrayBuffer() {
        return Promise.resolve(this.buffer);
    }
    
    text() {
        return Promise.resolve(this.buffer.toString());
    }
}

// Make File available globally for the WASM module
global.File = File;

// Run the examples
main().catch(console.error);
