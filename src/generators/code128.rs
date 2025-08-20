//! Code128 barcode generator

use crate::types::{Barcode, BarcodeConfig, BarcodeModules, BarcodeType, Result, QuickCodesError};

/// Generate a Code128 barcode with default configuration
pub fn generate_code128(data: &str) -> Result<Barcode> {
    generate_code128_with_config(data, &BarcodeConfig::default())
}

/// Generate a Code128 barcode with custom configuration
pub fn generate_code128_with_config(data: &str, config: &BarcodeConfig) -> Result<Barcode> {
    // For now, implement a basic Code128 Auto mode
    // This is a simplified implementation - full Code128 is quite complex
    
    if data.is_empty() {
        return Err(QuickCodesError::InvalidData(
            "Code128 data cannot be empty".to_string()
        ));
    }
    
    // Generate a basic pattern (placeholder implementation)
    // In a full implementation, this would:
    // 1. Analyze the data to choose optimal encoding (A/B/C)
    // 2. Generate proper Code128 patterns
    // 3. Calculate check digit
    // 4. Add start/stop codes
    
    let pattern = generate_basic_code128_pattern(data)?;
    
    Ok(Barcode {
        barcode_type: BarcodeType::Code128,
        data: data.to_string(),
        modules: BarcodeModules::Linear(pattern),
        config: config.clone(),
    })
}

/// Generate a basic Code128 pattern (simplified implementation)
fn generate_basic_code128_pattern(data: &str) -> Result<Vec<bool>> {
    // This is a placeholder implementation
    // Real Code128 would need proper encoding tables and algorithms
    
    let mut pattern = Vec::new();
    
    // Start pattern (simplified)
    pattern.extend([true, true, false, true, false, false, true, true, false, true, false]);
    
    // Data encoding (very simplified - just alternating pattern based on characters)
    for (i, ch) in data.chars().enumerate() {
        let char_code = ch as u8;
        let base_pattern = match i % 3 {
            0 => [true, true, false, false, true, true, false],
            1 => [true, false, true, true, false, false, true],
            _ => [false, true, true, false, true, true, false],
        };
        
        // Modify pattern based on character
        for (j, &bit) in base_pattern.iter().enumerate() {
            pattern.push(bit ^ ((char_code + j as u8) % 2 == 0));
        }
    }
    
    // Stop pattern (simplified)
    pattern.extend([true, true, false, false, true, false, true, true, false, false, true]);
    
    Ok(pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code128_generation() {
        let result = generate_code128("Hello123");
        assert!(result.is_ok());
        
        let barcode = result.unwrap();
        assert_eq!(barcode.barcode_type, BarcodeType::Code128);
        assert_eq!(barcode.data, "Hello123");
        
        match barcode.modules {
            BarcodeModules::Linear(pattern) => {
                assert!(!pattern.is_empty());
            }
            _ => panic!("Code128 should generate a linear pattern"),
        }
    }

    #[test]
    fn test_code128_empty_data() {
        let result = generate_code128("");
        assert!(result.is_err());
    }
}
