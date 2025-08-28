use wasm_bindgen::prelude::*;
use js_sys::Array;
use web_sys::{File, ImageData};
use serde::{Deserialize, Serialize};

// Import QuickCodes main functionality
use quickcodes::{
    generate as core_generate, 
    BarcodeType, 
    ExportFormat,
};

// Enable console.log! macro for debugging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

/// WebAssembly Result type for JavaScript
#[derive(Serialize, Deserialize)]
pub struct WasmResult {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>,
}

/// WebAssembly ReadResult type for JavaScript
#[derive(Serialize, Deserialize)]
pub struct WasmReadResult {
    pub success: bool,
    pub barcode_type: Option<String>,
    pub data: Option<String>,
    pub error: Option<String>,
}

/// Initialize the WASM module and set up panic hook
#[wasm_bindgen(start)]
pub fn init() {
    // Set up panic hook for better error messages
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log!("QuickCodes WASM module initialized successfully!");
}

/// Generate barcode and return as base64 string
#[wasm_bindgen]
pub fn generate(barcode_type: &str, data: &str, format: &str) -> JsValue {
    let result = generate_internal(barcode_type, data, format);
    serde_wasm_bindgen::to_value(&result).unwrap()
}

fn generate_internal(barcode_type: &str, data: &str, format: &str) -> WasmResult {
    // Parse barcode type
    let bc_type = match barcode_type.to_uppercase().as_str() {
        "QRCODE" | "QR" => BarcodeType::QRCode,
        "EAN13" => BarcodeType::EAN13,
        "UPCA" | "UPC" => BarcodeType::UPCA,
        "CODE128" => BarcodeType::Code128,
        "DATAMATRIX" => BarcodeType::DataMatrix,
        "PDF417" => BarcodeType::PDF417,
        "AZTEC" => BarcodeType::Aztec,
        "CODE39" => BarcodeType::Code39,
        "ITF14" => BarcodeType::ITF14,
        "CODABAR" => BarcodeType::Codabar,
        _ => return WasmResult {
            success: false,
            data: None,
            error: Some(format!("Unsupported barcode type: {}", barcode_type)),
        }
    };

    // Parse export format
    let export_format = match format.to_uppercase().as_str() {
        "SVG" => ExportFormat::SVG,
        "PNG" => ExportFormat::PNG,
        _ => return WasmResult {
            success: false,
            data: None,
            error: Some(format!("Unsupported format: {}. WASM supports SVG and PNG only.", format)),
        }
    };

    // Generate barcode
    match core_generate(bc_type, data, export_format) {
        Ok(bytes) => {
            // Convert to base64
            let base64 = base64_encode(&bytes);
            WasmResult {
                success: true,
                data: Some(base64),
                error: None,
            }
        }
        Err(e) => WasmResult {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }
    }
}

/// Generate SVG barcode and return as string
#[wasm_bindgen]
pub fn generate_svg(barcode_type: &str, data: &str) -> JsValue {
    let result = generate_svg_internal(barcode_type, data);
    serde_wasm_bindgen::to_value(&result).unwrap()
}

fn generate_svg_internal(barcode_type: &str, data: &str) -> WasmResult {
    let result = generate_internal(barcode_type, data, "SVG");
    if result.success {
        // For SVG, decode base64 back to string since it's text-based
        if let Some(base64_data) = result.data {
            match base64_decode(&base64_data) {
                Ok(svg_bytes) => {
                    match String::from_utf8(svg_bytes) {
                        Ok(svg_string) => WasmResult {
                            success: true,
                            data: Some(svg_string),
                            error: None,
                        },
                        Err(e) => WasmResult {
                            success: false,
                            data: None,
                            error: Some(format!("Failed to convert SVG to string: {}", e)),
                        }
                    }
                }
                Err(e) => WasmResult {
                    success: false,
                    data: None,
                    error: Some(format!("Failed to decode base64: {}", e)),
                }
            }
        } else {
            WasmResult {
                success: false,
                data: None,
                error: Some("No data returned from generation".to_string()),
            }
        }
    } else {
        result
    }
}

/// Read barcode from image data (ImageData from Canvas)
#[wasm_bindgen]
pub fn read_from_image_data(image_data: &ImageData) -> JsValue {
    #[cfg(feature = "readers")]
    {
        match read_from_image_data_internal(image_data) {
            Ok(result) => result,
            Err(e) => e
        }
    }
    #[cfg(not(feature = "readers"))]
    {
        let result = WasmReadResult {
            success: false,
            barcode_type: None,
            data: None,
            error: Some("Reader feature not enabled".to_string()),
        };
        serde_wasm_bindgen::to_value(&result).unwrap()
    }
}

#[cfg(feature = "readers")]
pub fn read_from_image_data_internal(image_data: &ImageData) -> Result<JsValue, JsValue> {
    console_log!("📊 ImageData dimensions: {}x{}", image_data.width(), image_data.height());
    console_log!("📊 ImageData raw length: {}", image_data.data().len());
    
    // Converter ImageData para RGB
    let width = image_data.width();
    let height = image_data.height();
    let rgba_data = image_data.data().to_vec();
    
    console_log!("📊 RGB data length: {}", rgba_data.len());
    console_log!("📊 Expected RGB length: {}", (width * height * 4) as usize);
    
    // Converter RGBA para RGB
    let mut rgb_data = Vec::with_capacity((width * height * 3) as usize);
    for chunk in rgba_data.chunks_exact(4) {
        rgb_data.push(chunk[0]); // R
        rgb_data.push(chunk[1]); // G 
        rgb_data.push(chunk[2]); // B
        // Ignorar o canal Alpha (chunk[3])
    }
    
    // Criar RgbImage
    let rgb_image = match image::RgbImage::from_raw(width, height, rgb_data) {
        Some(img) => {
            console_log!("✅ Successfully created RgbImage");
            img
        },
        None => {
            console_log!("❌ Failed to create RgbImage");
            return Err(JsValue::from_str("Failed to create RgbImage from raw data"));
        }
    };
    
    // Converter para PNG bytes
    let mut png_data = Vec::new();
    match rgb_image.write_to(&mut std::io::Cursor::new(&mut png_data), image::ImageFormat::Png) {
        Ok(_) => {
            console_log!("✅ Successfully converted to PNG, size: {} bytes", png_data.len());
        },
        Err(e) => {
            console_log!("❌ Failed to convert to PNG: {:?}", e);
            return Err(JsValue::from_str(&format!("Failed to convert to PNG: {}", e)));
        }
    }
    
    // Por enquanto, sempre retornar que não encontrou nada até implementarmos detecção real
    console_log!("🔍 Barcode detection temporarily disabled to prevent false positives");
    Ok(serde_wasm_bindgen::to_value(&WasmReadResult {
        success: false,
        barcode_type: None,
        data: None,
        error: Some("Barcode detection temporarily disabled to prevent false positives".to_string()),
    })?)
}

/// Read barcode from file (File object from browser)
#[wasm_bindgen]
pub async fn read_from_file(_file: &File) -> JsValue {
    // Simplified implementation - return not implemented for now
    let result = WasmReadResult {
        success: false,
        barcode_type: None,
        data: None,
        error: Some("File reading not yet implemented in WASM. Use read_from_image_data instead.".to_string()),
    };
    serde_wasm_bindgen::to_value(&result).unwrap()
}



/// Get list of supported barcode types
#[wasm_bindgen]
pub fn get_supported_types() -> JsValue {
    let types = vec![
        "QRCode", "EAN13", "UPCA", "Code128", 
        "DataMatrix", "PDF417", "Aztec", 
        "Code39", "ITF14", "Codabar"
    ];
    
    let js_array = Array::new();
    for type_name in types {
        js_array.push(&JsValue::from_str(type_name));
    }
    
    js_array.into()
}

/// Get list of supported export formats
#[wasm_bindgen]
pub fn get_supported_formats() -> JsValue {
    let formats = vec!["SVG", "PNG"]; // PDF not supported in WASM yet
    
    let js_array = Array::new();
    for format in formats {
        js_array.push(&JsValue::from_str(format));
    }
    
    js_array.into()
}

/// Utility function to encode bytes as base64
fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    
    for chunk in data.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &b) in chunk.iter().enumerate() {
            buf[i] = b;
        }
        
        let b = u32::from_be_bytes([0, buf[0], buf[1], buf[2]]);
        
        result.push(CHARS[((b >> 18) & 63) as usize] as char);
        result.push(CHARS[((b >> 12) & 63) as usize] as char);
        result.push(if chunk.len() > 1 { CHARS[((b >> 6) & 63) as usize] as char } else { '=' });
        result.push(if chunk.len() > 2 { CHARS[(b & 63) as usize] as char } else { '=' });
    }
    
    result
}

/// Utility function to decode base64
fn base64_decode(data: &str) -> Result<Vec<u8>, String> {
    let mut result = Vec::new();
    let chars: Vec<char> = data.chars().collect();
    
    // Simple base64 decode implementation
    for chunk in chars.chunks(4) {
        if chunk.len() != 4 {
            return Err("Invalid base64 length".to_string());
        }
        
        let mut values = [0u8; 4];
        for (i, &c) in chunk.iter().enumerate() {
            values[i] = match c {
                'A'..='Z' => (c as u8) - b'A',
                'a'..='z' => (c as u8) - b'a' + 26,
                '0'..='9' => (c as u8) - b'0' + 52,
                '+' => 62,
                '/' => 63,
                '=' => 0,
                _ => return Err(format!("Invalid base64 character: {}", c)),
            };
        }
        
        let combined = ((values[0] as u32) << 18) 
                     | ((values[1] as u32) << 12) 
                     | ((values[2] as u32) << 6) 
                     | (values[3] as u32);
        
        result.push((combined >> 16) as u8);
        if chunk[2] != '=' {
            result.push((combined >> 8) as u8);
        }
        if chunk[3] != '=' {
            result.push(combined as u8);
        }
    }
    
    Ok(result)
}
