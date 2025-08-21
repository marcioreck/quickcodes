//! UPC-A barcode generator

use crate::types::{Barcode, BarcodeConfig, BarcodeModules, BarcodeType, QuickCodesError, Result};

// UPC-A uses the same patterns as EAN-13 for the digits
// Left side patterns (odd positions)
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

// Right side patterns
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

// Guard patterns
const START_GUARD: [u8; 3] = [1, 0, 1];
const CENTER_GUARD: [u8; 5] = [0, 1, 0, 1, 0];
const END_GUARD: [u8; 3] = [1, 0, 1];

/// Generate a UPC-A barcode with default configuration
pub fn generate_upc_a(data: &str) -> Result<Barcode> {
    generate_upc_a_with_config(data, &BarcodeConfig::default())
}

/// Generate a UPC-A barcode with custom configuration
pub fn generate_upc_a_with_config(data: &str, config: &BarcodeConfig) -> Result<Barcode> {
    // Validate and process input data
    let digits = process_upc_a_data(data)?;

    // Generate the barcode pattern
    let pattern = generate_upc_a_pattern(&digits)?;

    Ok(Barcode {
        barcode_type: BarcodeType::UPCA,
        data: format_upc_a_data(&digits),
        modules: BarcodeModules::Linear(pattern),
        config: config.clone(),
    })
}

/// Process and validate UPC-A input data
fn process_upc_a_data(data: &str) -> Result<Vec<u8>> {
    let cleaned = data.replace([' ', '-'], "");

    // Check if all characters are digits
    if !cleaned.chars().all(|c| c.is_ascii_digit()) {
        return Err(QuickCodesError::InvalidData(
            "UPC-A data must contain only digits".to_string(),
        ));
    }

    let digits: Vec<u8> = cleaned
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    match digits.len() {
        11 => {
            // Calculate and append check digit
            let mut with_check = digits;
            let check_digit = calculate_upc_a_check_digit(&with_check);
            with_check.push(check_digit);
            Ok(with_check)
        }
        12 => {
            // Verify check digit
            let check_digit = calculate_upc_a_check_digit(&digits[..11]);
            if check_digit != digits[11] {
                return Err(QuickCodesError::InvalidData(format!(
                    "Invalid UPC-A check digit. Expected {}, got {}",
                    check_digit, digits[11]
                )));
            }
            Ok(digits)
        }
        _ => Err(QuickCodesError::InvalidData(
            "UPC-A data must be 11 or 12 digits long".to_string(),
        )),
    }
}

/// Calculate UPC-A check digit
fn calculate_upc_a_check_digit(digits: &[u8]) -> u8 {
    let sum: u32 = digits
        .iter()
        .enumerate()
        .map(|(i, &digit)| {
            if i % 2 == 0 {
                (digit as u32) * 3
            } else {
                digit as u32
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

/// Generate the binary pattern for UPC-A
fn generate_upc_a_pattern(digits: &[u8]) -> Result<Vec<bool>> {
    if digits.len() != 12 {
        return Err(QuickCodesError::GenerationError(
            "UPC-A requires exactly 12 digits".to_string(),
        ));
    }

    let mut pattern = Vec::new();

    // Start guard
    pattern.extend(START_GUARD.iter().map(|&b| b == 1));

    // Left side (first 6 digits)
    for &digit in &digits[0..6] {
        let digit_pattern = LEFT_PATTERNS[digit as usize];
        pattern.extend(digit_pattern.iter().map(|&b| b == 1));
    }

    // Center guard
    pattern.extend(CENTER_GUARD.iter().map(|&b| b == 1));

    // Right side (last 6 digits)
    for &digit in &digits[6..12] {
        let digit_pattern = RIGHT_PATTERNS[digit as usize];
        pattern.extend(digit_pattern.iter().map(|&b| b == 1));
    }

    // End guard
    pattern.extend(END_GUARD.iter().map(|&b| b == 1));

    Ok(pattern)
}

/// Format UPC-A data for display
fn format_upc_a_data(digits: &[u8]) -> String {
    digits.iter().map(|d| d.to_string()).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upc_a_check_digit_calculation() {
        // Test case: 03600029145 should have check digit 2
        let digits = [0, 3, 6, 0, 0, 0, 2, 9, 1, 4, 5];
        let check_digit = calculate_upc_a_check_digit(&digits);
        assert_eq!(check_digit, 2);
    }

    #[test]
    fn test_upc_a_generation_with_11_digits() {
        let result = generate_upc_a("03600029145");
        assert!(result.is_ok());

        let barcode = result.unwrap();
        assert_eq!(barcode.barcode_type, BarcodeType::UPCA);
        assert_eq!(barcode.data, "036000291452"); // With check digit

        match barcode.modules {
            BarcodeModules::Linear(pattern) => {
                // UPC-A should have 95 modules total (same as EAN-13)
                assert_eq!(pattern.len(), 95);
            }
            _ => panic!("UPC-A should generate a linear pattern"),
        }
    }

    #[test]
    fn test_upc_a_generation_with_12_digits() {
        let result = generate_upc_a("036000291452");
        assert!(result.is_ok());

        let barcode = result.unwrap();
        assert_eq!(barcode.data, "036000291452");
    }

    #[test]
    fn test_upc_a_invalid_check_digit() {
        let result = generate_upc_a("036000291451"); // Wrong check digit
        assert!(result.is_err());
    }

    #[test]
    fn test_upc_a_invalid_length() {
        let result = generate_upc_a("12345");
        assert!(result.is_err());
    }

    #[test]
    fn test_upc_a_non_numeric() {
        let result = generate_upc_a("03600029145A");
        assert!(result.is_err());
    }
}
