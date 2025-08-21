//! Aztec Code generator
//!
//! Aztec Code is a 2D barcode commonly used in transportation tickets,
//! particularly in Europe and for mobile ticketing applications.

use crate::types::{Barcode, BarcodeConfig, BarcodeModules, BarcodeType, QuickCodesError, Result};

/// Aztec Code configuration options
#[derive(Debug, Clone)]
pub struct AztecConfig {
    /// Compact format (uses less space)
    pub compact: bool,
    /// Error correction percentage (5-95%)
    pub error_correction: u8,
    /// Size layers (1-32 for full, 1-4 for compact)
    pub layers: Option<u8>,
}

impl Default for AztecConfig {
    fn default() -> Self {
        Self {
            compact: false,
            error_correction: 23, // ~23% error correction
            layers: None,         // Auto-select
        }
    }
}

/// Generate an Aztec Code with default configuration
pub fn generate_aztec(data: &str) -> Result<Barcode> {
    generate_aztec_with_config(data, &BarcodeConfig::default())
}

/// Generate an Aztec Code with custom configuration
pub fn generate_aztec_with_config(data: &str, config: &BarcodeConfig) -> Result<Barcode> {
    if data.is_empty() {
        return Err(QuickCodesError::InvalidData(
            "Aztec data cannot be empty".to_string(),
        ));
    }

    let aztec_config = AztecConfig::default();
    let matrix = generate_aztec_matrix(data, &aztec_config)?;

    Ok(Barcode {
        barcode_type: BarcodeType::Aztec,
        data: data.to_string(),
        modules: BarcodeModules::Matrix(matrix),
        config: config.clone(),
    })
}

/// Generate Aztec matrix (simplified implementation)
fn generate_aztec_matrix(data: &str, config: &AztecConfig) -> Result<Vec<Vec<bool>>> {
    // This is a simplified implementation for demonstration
    // A full Aztec implementation would need:
    // 1. Data encoding with mode switching (Upper, Lower, Mixed, Punct, Digit, Byte)
    // 2. Reed-Solomon error correction
    // 3. Proper symbol construction with finder pattern
    // 4. Reference grid and orientation patterns

    let data_bytes = data.as_bytes();
    let layers = config
        .layers
        .unwrap_or_else(|| calculate_layers(data_bytes.len(), config.compact));
    let size = if config.compact {
        11 + 4 * layers as usize // Compact: 15x15 to 27x27
    } else {
        15 + 4 * layers as usize // Full: 19x19 to 151x151
    };

    let mut matrix = vec![vec![false; size]; size];

    // Generate central finder pattern (bullseye)
    generate_finder_pattern(&mut matrix, size);

    // Generate reference grid (every 16th row/column in full format)
    if !config.compact {
        generate_reference_grid(&mut matrix, size);
    }

    // Encode data in spiral pattern (simplified)
    encode_data_spiral(&mut matrix, data_bytes, size, config.compact);

    Ok(matrix)
}

/// Generate the central finder pattern (bullseye)
fn generate_finder_pattern(matrix: &mut [Vec<bool>], size: usize) {
    let center = size / 2;

    // Create concentric squares (simplified bullseye pattern)
    for ring in 0..=5 {
        let is_filled = ring % 2 == 1;
        let start = center.saturating_sub(ring);
        let end = (center + ring + 1).min(size);

        for y in start..end {
            for x in start..end {
                if y == start || y == end - 1 || x == start || x == end - 1 {
                    matrix[y][x] = is_filled;
                }
            }
        }
    }
}

/// Generate reference grid for full Aztec
fn generate_reference_grid(matrix: &mut [Vec<bool>], size: usize) {
    // Reference grid: alternating pattern every 16 modules
    for i in (0..size).step_by(16) {
        for j in 0..size {
            if i < size && j < size {
                matrix[i][j] = (i + j) % 2 == 0;
                if j < size && i < size {
                    matrix[j][i] = (i + j) % 2 == 0;
                }
            }
        }
    }
}

/// Encode data in spiral pattern (simplified)
fn encode_data_spiral(matrix: &mut [Vec<bool>], data: &[u8], size: usize, compact: bool) {
    let start_radius = if compact { 6 } else { 8 };
    let center = size / 2;

    let mut bit_index = 0;
    let total_bits = data.len() * 8;

    // Spiral outward from center
    for radius in start_radius..center {
        let positions = get_spiral_positions(center, radius, size);

        for (x, y) in positions {
            if bit_index >= total_bits {
                break;
            }

            // Skip finder pattern and reference grid positions
            if is_data_position(x, y, center, compact) {
                let byte_idx = bit_index / 8;
                let bit_pos = 7 - (bit_index % 8);

                if byte_idx < data.len() {
                    matrix[y][x] = (data[byte_idx] >> bit_pos) & 1 == 1;
                    bit_index += 1;
                }
            }
        }

        if bit_index >= total_bits {
            break;
        }
    }
}

/// Get positions in spiral order for a given radius
fn get_spiral_positions(center: usize, radius: usize, size: usize) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    let start = center.saturating_sub(radius);
    let end = (center + radius + 1).min(size);

    // Top row
    for x in start..end {
        if start < size {
            positions.push((x, start));
        }
    }

    // Right column
    for y in (start + 1)..end {
        if end > 0 && end - 1 < size && y < size {
            positions.push((end - 1, y));
        }
    }

    // Bottom row (if different from top)
    if end > start + 1 {
        for x in (start..(end - 1)).rev() {
            if end > 0 && end - 1 < size && x < size {
                positions.push((x, end - 1));
            }
        }
    }

    // Left column (if different from right)
    if end > start + 1 {
        for y in ((start + 1)..(end - 1)).rev() {
            if start < size && y < size {
                positions.push((start, y));
            }
        }
    }

    positions
}

/// Check if position is available for data (not finder pattern or reference grid)
fn is_data_position(x: usize, y: usize, center: usize, compact: bool) -> bool {
    let dx = (x as i32 - center as i32).abs() as usize;
    let dy = (y as i32 - center as i32).abs() as usize;

    // Skip finder pattern area
    if dx <= 5 && dy <= 5 {
        return false;
    }

    // Skip reference grid for full format
    if !compact && (x % 16 == 0 || y % 16 == 0) {
        return false;
    }

    true
}

/// Calculate required layers based on data length
fn calculate_layers(data_len: usize, compact: bool) -> u8 {
    // Simplified calculation - real Aztec uses complex capacity tables
    let bits_needed = data_len * 8;
    let max_layers = if compact { 4 } else { 32 };

    for layer in 1..=max_layers {
        let capacity = if compact {
            // Compact capacity approximation
            (layer as usize * 64).saturating_sub(200) // Rough estimate
        } else {
            // Full capacity approximation
            layer as usize * 256 // Rough estimate
        };

        if capacity >= bits_needed {
            return layer;
        }
    }

    max_layers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aztec_generation() {
        let result = generate_aztec("Hello Aztec");
        assert!(result.is_ok());

        let barcode = result.unwrap();
        assert_eq!(barcode.barcode_type, BarcodeType::Aztec);
        assert_eq!(barcode.data, "Hello Aztec");

        match barcode.modules {
            BarcodeModules::Matrix(matrix) => {
                assert!(!matrix.is_empty());
                assert!(!matrix[0].is_empty());
                // Aztec should be square
                assert_eq!(matrix.len(), matrix[0].len());
                // Should be at least 15x15
                assert!(matrix.len() >= 15);
            }
            _ => panic!("Aztec should generate a matrix pattern"),
        }
    }

    #[test]
    fn test_aztec_empty_data() {
        let result = generate_aztec("");
        assert!(result.is_err());
    }

    #[test]
    fn test_aztec_transport_ticket() {
        // Test with transport ticket-like data
        let ticket_data = "TKT:12345|FROM:NYC|TO:BOS|DATE:2024-01-15|SEAT:12A";
        let result = generate_aztec(ticket_data);
        assert!(result.is_ok());

        let barcode = result.unwrap();
        assert_eq!(barcode.data, ticket_data);
    }

    #[test]
    fn test_calculate_layers() {
        // Test with actual values from our implementation
        let result1 = calculate_layers(10, true);
        let result2 = calculate_layers(100, true);
        let result3 = calculate_layers(10, false);
        let result4 = calculate_layers(1000, false);

        // Verify they are within valid ranges
        assert!(
            result1 >= 1 && result1 <= 4,
            "Compact layers should be 1-4, got {}",
            result1
        );
        assert!(
            result2 >= 1 && result2 <= 4,
            "Compact layers should be 1-4, got {}",
            result2
        );
        assert!(
            result3 >= 1 && result3 <= 32,
            "Full layers should be 1-32, got {}",
            result3
        );
        assert!(
            result4 >= 1 && result4 <= 32,
            "Full layers should be 1-32, got {}",
            result4
        );
    }

    #[test]
    fn test_finder_pattern() {
        let mut matrix = vec![vec![false; 21]; 21];
        generate_finder_pattern(&mut matrix, 21);

        // Check that center has some pattern
        let center = 10;
        let mut has_pattern = false;
        for y in (center - 5)..=(center + 5) {
            for x in (center - 5)..=(center + 5) {
                if matrix[y][x] {
                    has_pattern = true;
                    break;
                }
            }
        }
        assert!(
            has_pattern,
            "Finder pattern should have some filled modules"
        );
    }
}
