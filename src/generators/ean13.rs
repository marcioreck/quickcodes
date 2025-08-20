//! EAN-13 barcode generator

use crate::types::{Barcode, BarcodeConfig, BarcodeModules, BarcodeType, Result, QuickCodesError};

// EAN-13 encoding patterns
const LEFT_PATTERNS: [[u8; 7]; 10] = [
    [0, 0, 0, 1, 1, 0, 1], // 0
    [0, 0, 1, 1, 0, 0, 1], // 1
    [0, 0, 1, 0, 0, 1, 1], // 2
    [0, 1, 1, 1, 1, 0, 1], // 3
    [0, 1, 0, 0, 0, 1, 1], // 4
    [0, 1, 1, 0, 0, 0, 1], // 5
    [0, 1, 0, 1, 1, 1, 1], // 6
    [0, 1, 1, 1, 0, 1, 1], // 7
    [0, 1, 1, 0, 1, 1, 1], // 8
    [0, 0, 0, 1, 0, 1, 1], // 9
];

const LEFT_PATTERNS_G: [[u8; 7]; 10] = [
    [0, 1, 0, 0, 1, 1, 1], // 0
    [0, 1, 1, 0, 0, 1, 1], // 1
    [0, 0, 1, 1, 0, 1, 1], // 2
    [0, 1, 0, 0, 0, 0, 1], // 3
    [0, 0, 1, 1, 1, 0, 1], // 4
    [0, 1, 1, 1, 0, 0, 1], // 5
    [0, 0, 0, 0, 1, 0, 1], // 6
    [0, 0, 1, 0, 0, 0, 1], // 7
    [0, 0, 0, 1, 0, 0, 1], // 8
    [0, 0, 1, 0, 1, 1, 1], // 9
];

const RIGHT_PATTERNS: [[u8; 7]; 10] = [
    [1, 1, 1, 0, 0, 1, 0], // 0
    [1, 1, 0, 0, 1, 1, 0], // 1
    [1, 1, 0, 1, 1, 0, 0], // 2
    [1, 0, 0, 0, 0, 1, 0], // 3
    [1, 0, 1, 1, 1, 0, 0], // 4
    [1, 0, 0, 1, 1, 1, 0], // 5
    [1, 0, 1, 0, 0, 0, 0], // 6
    [1, 0, 0, 0, 1, 0, 0], // 7
    [1, 0, 0, 1, 0, 0, 0], // 8
    [1, 1, 1, 0, 1, 0, 0], // 9
];

// First digit patterns (determines which pattern set to use for left side)
const FIRST_DIGIT_PATTERNS: [&str; 10] = [
    "LLLLLL", // 0
    "LLGLGG", // 1
    "LLGGLG", // 2
    "LLGGGL", // 3
    "LGLLGG", // 4
    "LGGLLG", // 5
    "LGGGLL", // 6
    "LGLGLG", // 7
    "LGLGGL", // 8
    "LGGLGL", // 9
];

// Guard patterns
const START_GUARD: [u8; 3] = [1, 0, 1];
const CENTER_GUARD: [u8; 5] = [0, 1, 0, 1, 0];
const END_GUARD: [u8; 3] = [1, 0, 1];

/// Generate an EAN-13 barcode with default configuration
pub fn generate_ean13(data: &str) -> Result<Barcode> {
    generate_ean13_with_config(data, &BarcodeConfig::default())
}

/// Generate an EAN-13 barcode with custom configuration
pub fn generate_ean13_with_config(data: &str, config: &BarcodeConfig) -> Result<Barcode> {
    // Validate and process input data
    let digits = process_ean13_data(data)?;
    
    // Generate the barcode pattern
    let pattern = generate_ean13_pattern(&digits)?;
    
    Ok(Barcode {
        barcode_type: BarcodeType::EAN13,
        data: format_ean13_data(&digits),
        modules: BarcodeModules::Linear(pattern),
        config: config.clone(),
    })
}

/// Process and validate EAN-13 input data
fn process_ean13_data(data: &str) -> Result<Vec<u8>> {
    let cleaned = data.replace(' ', "").replace('-', "");
    
    // Check if all characters are digits
    if !cleaned.chars().all(|c| c.is_ascii_digit()) {
        return Err(QuickCodesError::InvalidData(
            "EAN-13 data must contain only digits".to_string()
        ));
    }
    
    let digits: Vec<u8> = cleaned
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    
    match digits.len() {
        12 => {
            // Calculate and append check digit
            let mut with_check = digits;
            let check_digit = calculate_ean13_check_digit(&with_check);
            with_check.push(check_digit);
            Ok(with_check)
        }
        13 => {
            // Verify check digit
            let check_digit = calculate_ean13_check_digit(&digits[..12]);
            if check_digit != digits[12] {
                return Err(QuickCodesError::InvalidData(
                    format!("Invalid EAN-13 check digit. Expected {}, got {}", 
                            check_digit, digits[12])
                ));
            }
            Ok(digits)
        }
        _ => Err(QuickCodesError::InvalidData(
            "EAN-13 data must be 12 or 13 digits long".to_string()
        )),
    }
}

/// Calculate EAN-13 check digit
fn calculate_ean13_check_digit(digits: &[u8]) -> u8 {
    let sum: u32 = digits
        .iter()
        .enumerate()
        .map(|(i, &digit)| {
            if i % 2 == 0 {
                digit as u32
            } else {
                (digit as u32) * 3
            }
        })
        .sum();
    
    let remainder = sum % 10;
    if remainder == 0 {
        0
    } else {
        10 - remainder as u8
    }
}

/// Generate the binary pattern for EAN-13
fn generate_ean13_pattern(digits: &[u8]) -> Result<Vec<bool>> {
    if digits.len() != 13 {
        return Err(QuickCodesError::GenerationError(
            "EAN-13 requires exactly 13 digits".to_string()
        ));
    }
    
    let mut pattern = Vec::new();
    
    // Start guard
    pattern.extend(START_GUARD.iter().map(|&b| b == 1));
    
    // First digit determines the pattern for left side
    let first_digit = digits[0] as usize;
    let left_pattern = FIRST_DIGIT_PATTERNS[first_digit];
    
    // Left side (digits 1-6)
    for (i, &digit) in digits[1..7].iter().enumerate() {
        let digit_pattern = match left_pattern.chars().nth(i).unwrap() {
            'L' => LEFT_PATTERNS[digit as usize],
            'G' => LEFT_PATTERNS_G[digit as usize],
            _ => return Err(QuickCodesError::GenerationError(
                "Invalid left pattern character".to_string()
            )),
        };
        pattern.extend(digit_pattern.iter().map(|&b| b == 1));
    }
    
    // Center guard
    pattern.extend(CENTER_GUARD.iter().map(|&b| b == 1));
    
    // Right side (digits 7-12)
    for &digit in &digits[7..13] {
        let digit_pattern = RIGHT_PATTERNS[digit as usize];
        pattern.extend(digit_pattern.iter().map(|&b| b == 1));
    }
    
    // End guard
    pattern.extend(END_GUARD.iter().map(|&b| b == 1));
    
    Ok(pattern)
}

/// Format EAN-13 data for display
fn format_ean13_data(digits: &[u8]) -> String {
    digits.iter().map(|d| d.to_string()).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ean13_check_digit_calculation() {
        // Test case: 123456789012 should have check digit 8
        let digits = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
        let check_digit = calculate_ean13_check_digit(&digits);
        assert_eq!(check_digit, 8);
    }

    #[test]
    fn test_ean13_generation_with_12_digits() {
        let result = generate_ean13("123456789012");
        assert!(result.is_ok());
        
        let barcode = result.unwrap();
        assert_eq!(barcode.barcode_type, BarcodeType::EAN13);
        assert_eq!(barcode.data, "1234567890128"); // With check digit
        
        match barcode.modules {
            BarcodeModules::Linear(pattern) => {
                // EAN-13 should have 95 modules total
                assert_eq!(pattern.len(), 95);
            }
            _ => panic!("EAN-13 should generate a linear pattern"),
        }
    }

    #[test]
    fn test_ean13_generation_with_13_digits() {
        let result = generate_ean13("1234567890128");
        assert!(result.is_ok());
        
        let barcode = result.unwrap();
        assert_eq!(barcode.data, "1234567890128");
    }

    #[test]
    fn test_ean13_invalid_check_digit() {
        let result = generate_ean13("1234567890127"); // Wrong check digit
        assert!(result.is_err());
    }

    #[test]
    fn test_ean13_invalid_length() {
        let result = generate_ean13("12345");
        assert!(result.is_err());
    }

    #[test]
    fn test_ean13_non_numeric() {
        let result = generate_ean13("12345678901A");
        assert!(result.is_err());
    }

    #[test]
    fn test_ean13_with_spaces_and_hyphens() {
        let result = generate_ean13("123 456 789-012");
        assert!(result.is_ok());
        
        let barcode = result.unwrap();
        assert_eq!(barcode.data, "1234567890128");
    }
}
