//! DataMatrix barcode generator
//!
//! DataMatrix is a 2D barcode commonly used in pharmaceutical and industrial applications.
//! It's particularly important for ANVISA compliance in Brazil.
//!
//! This is a simplified implementation for demonstration purposes.
//! A production implementation would use proper DataMatrix encoding algorithms.

use crate::types::{Barcode, BarcodeConfig, BarcodeModules, BarcodeType, QuickCodesError, Result};

/// Generate a DataMatrix with default configuration
pub fn generate_datamatrix(data: &str) -> Result<Barcode> {
    generate_datamatrix_with_config(data, &BarcodeConfig::default())
}

/// Generate a DataMatrix with custom configuration
pub fn generate_datamatrix_with_config(data: &str, config: &BarcodeConfig) -> Result<Barcode> {
    if data.is_empty() {
        return Err(QuickCodesError::InvalidData(
            "DataMatrix data cannot be empty".to_string(),
        ));
    }

    // Generate a simplified DataMatrix pattern
    // This is a placeholder implementation for demonstration
    // A full implementation would need proper DataMatrix encoding algorithms
    let matrix = generate_datamatrix_pattern(data)?;

    Ok(Barcode {
        barcode_type: BarcodeType::DataMatrix,
        data: data.to_string(),
        modules: BarcodeModules::Matrix(matrix),
        config: config.clone(),
    })
}

/// Generate a simplified DataMatrix pattern
fn generate_datamatrix_pattern(data: &str) -> Result<Vec<Vec<bool>>> {
    // This is a simplified implementation for demonstration
    // Real DataMatrix encoding is quite complex and involves:
    // 1. Data encoding (ASCII, C40, Text, X12, EDIFACT, Base256)
    // 2. Error correction using Reed-Solomon
    // 3. Symbol construction with finder patterns
    // 4. Module placement algorithm

    let data_bytes = data.as_bytes();

    // Calculate size based on data length (simplified)
    let size = calculate_datamatrix_size(data_bytes.len());
    let mut matrix = vec![vec![false; size]; size];

    // Generate finder patterns (L-shaped borders)
    generate_finder_patterns(&mut matrix, size);

    // Fill data area with encoded data (simplified)
    fill_data_area(&mut matrix, data_bytes, size);

    Ok(matrix)
}

/// Calculate DataMatrix size based on data length (simplified)
fn calculate_datamatrix_size(data_len: usize) -> usize {
    // Simplified size calculation
    // Real DataMatrix has specific size standards: 10x10, 12x12, 14x14, etc.
    match data_len {
        0..=6 => 10,
        7..=10 => 12,
        11..=16 => 14,
        17..=24 => 16,
        25..=36 => 18,
        37..=44 => 20,
        45..=60 => 22,
        61..=72 => 24,
        73..=88 => 26,
        _ => ((data_len as f64).sqrt() as usize + 8).max(28),
    }
}

/// Generate finder patterns (L-shaped solid borders)
fn generate_finder_patterns(matrix: &mut [Vec<bool>], size: usize) {
    // Left border (solid line)
    for y in 0..size {
        matrix[y][0] = true;
    }

    // Bottom border (solid line)
    for x in 0..size {
        matrix[size - 1][x] = true;
    }

    // Top border (alternating pattern)
    for x in (0..size).step_by(2) {
        matrix[0][x] = true;
    }

    // Right border (alternating pattern)
    for y in (1..size).step_by(2) {
        matrix[y][size - 1] = true;
    }
}

/// Fill data area with encoded data (simplified)
fn fill_data_area(matrix: &mut [Vec<bool>], data: &[u8], size: usize) {
    let mut bit_index = 0;
    let total_bits = data.len() * 8;

    // Fill in a diagonal pattern (simplified placement algorithm)
    for diagonal in 0..(size * 2) {
        for i in 0..=diagonal {
            let x = i;
            let y = diagonal - i;

            if x < size && y < size && x > 0 && y > 0 && x < size - 1 && y < size - 1 {
                if bit_index < total_bits {
                    let byte_idx = bit_index / 8;
                    let bit_pos = 7 - (bit_index % 8);

                    if byte_idx < data.len() {
                        matrix[y][x] = (data[byte_idx] >> bit_pos) & 1 == 1;
                        bit_index += 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datamatrix_generation() {
        let result = generate_datamatrix("Hello DataMatrix");
        assert!(result.is_ok());

        let barcode = result.unwrap();
        assert_eq!(barcode.barcode_type, BarcodeType::DataMatrix);
        assert_eq!(barcode.data, "Hello DataMatrix");

        match barcode.modules {
            BarcodeModules::Matrix(matrix) => {
                assert!(!matrix.is_empty());
                assert!(!matrix[0].is_empty());
                // DataMatrix should be square
                assert_eq!(matrix.len(), matrix[0].len());
            }
            _ => panic!("DataMatrix should generate a matrix pattern"),
        }
    }

    #[test]
    fn test_datamatrix_empty_data() {
        let result = generate_datamatrix("");
        assert!(result.is_err());
    }

    #[test]
    fn test_datamatrix_pharmaceutical_data() {
        // Test with pharmaceutical-like data (GS1 format)
        let gs1_data = "010123456789012815240101";
        let result = generate_datamatrix(gs1_data);
        assert!(result.is_ok());

        let barcode = result.unwrap();
        assert_eq!(barcode.data, gs1_data);
    }

    #[test]
    fn test_datamatrix_unicode_data() {
        let result = generate_datamatrix("Olá DataMatrix! 🇧🇷");
        assert!(result.is_ok());
    }
}
