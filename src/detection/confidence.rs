// Advanced Confidence Scoring System
// Multi-dimensional scoring for robust detection validation

use crate::detection::{DetectionCandidate, PatternData, BoundingBox, LBorder};
use crate::types::BarcodeType;

/// Comprehensive confidence scoring
pub struct ConfidenceScorer;

impl ConfidenceScorer {
    /// Calculate overall confidence score from multiple dimensions
    pub fn calculate_confidence(candidate: &DetectionCandidate) -> ConfidenceScore {
        let geometric = Self::calculate_geometric_score(candidate);
        let pattern = Self::calculate_pattern_score(candidate);
        let content = Self::calculate_content_score(candidate);
        let format_specificity = Self::calculate_format_specificity(candidate);
        
        // Weighted combination - emphasize format specificity and pattern integrity
        let overall = 0.3 * geometric + 0.4 * pattern + 0.1 * content + 0.2 * format_specificity;
        
        ConfidenceScore {
            overall,
            geometric,
            pattern,
            content,
        }
    }
    
    /// Score geometric properties (proportions, angles, symmetry)
    fn calculate_geometric_score(candidate: &DetectionCandidate) -> f32 {
        match &candidate.pattern_data {
            PatternData::QRCode { finder_patterns, .. } => {
                Self::score_qr_geometry(finder_patterns, &candidate.position)
            },
            PatternData::DataMatrix { l_border, .. } => {
                Self::score_datamatrix_geometry(l_border, &candidate.position)
            },
            PatternData::Linear { bars_and_spaces, .. } => {
                Self::score_linear_geometry(bars_and_spaces)
            },
            PatternData::PDF417 { start_patterns, stop_patterns, .. } => {
                Self::score_pdf417_geometry(start_patterns, stop_patterns)
            },
            PatternData::Aztec { bullseye, reference_grid, .. } => {
                Self::score_aztec_geometry(bullseye, reference_grid)
            },
        }
    }
    
    /// Score pattern integrity and consistency
    fn calculate_pattern_score(candidate: &DetectionCandidate) -> f32 {
        match &candidate.pattern_data {
            PatternData::QRCode { finder_patterns, timing_patterns, .. } => {
                Self::score_qr_patterns(finder_patterns, timing_patterns)
            },
            PatternData::DataMatrix { timing_patterns, l_border, .. } => {
                Self::score_datamatrix_patterns(timing_patterns, l_border)
            },
            PatternData::Linear { bars_and_spaces, start_end_patterns, .. } => {
                Self::score_linear_patterns(bars_and_spaces, start_end_patterns, &candidate.barcode_type)
            },
            PatternData::PDF417 { rows, .. } => {
                Self::score_pdf417_patterns(rows)
            },
            PatternData::Aztec { reference_grid, .. } => {
                Self::score_aztec_patterns(reference_grid)
            },
        }
    }
    
    /// Score content validation (checksum, format compliance)
    fn calculate_content_score(candidate: &DetectionCandidate) -> f32 {
        match &candidate.raw_data {
            Some(data) => {
                match candidate.barcode_type {
                    BarcodeType::EAN13 => Self::validate_ean13_content(data),
                    BarcodeType::UPCA => Self::validate_upca_content(data),
                    BarcodeType::Code128 => Self::validate_code128_content(data),
                    BarcodeType::Code39 => Self::validate_code39_content(data),
                    BarcodeType::ITF14 => Self::validate_itf14_content(data),
                    BarcodeType::Codabar => Self::validate_codabar_content(data),
                    BarcodeType::QRCode => Self::validate_qr_content(data),
                    BarcodeType::DataMatrix => Self::validate_datamatrix_content(data),
                    BarcodeType::PDF417 => Self::validate_pdf417_content(data),
                    BarcodeType::Aztec => Self::validate_aztec_content(data),
                }
            },
            None => 0.0, // No content extracted
        }
    }
    
    /// Calculate format-specific confidence to reduce cross-format false positives
    fn calculate_format_specificity(candidate: &DetectionCandidate) -> f32 {
        match (&candidate.barcode_type, &candidate.pattern_data) {
            // QR Code should have QR patterns
            (BarcodeType::QRCode, PatternData::QRCode { finder_patterns, .. }) => {
                if finder_patterns.len() == 3 { 1.0 } else { 0.0 }
            },
            // DataMatrix should have DataMatrix patterns
            (BarcodeType::DataMatrix, PatternData::DataMatrix { .. }) => 1.0,
            // PDF417 should have PDF417 patterns
            (BarcodeType::PDF417, PatternData::PDF417 { .. }) => 1.0,
            // Aztec should have Aztec patterns
            (BarcodeType::Aztec, PatternData::Aztec { .. }) => 1.0,
            // Linear formats should have linear patterns
            (BarcodeType::EAN13, PatternData::Linear { .. }) |
            (BarcodeType::UPCA, PatternData::Linear { .. }) |
            (BarcodeType::Code128, PatternData::Linear { .. }) |
            (BarcodeType::Code39, PatternData::Linear { .. }) |
            (BarcodeType::ITF14, PatternData::Linear { .. }) |
            (BarcodeType::Codabar, PatternData::Linear { .. }) => {
                // For linear codes, we need additional validation
                // For now, give partial score to allow detection but prioritize pattern matching
                0.7
            },
            // Mismatch between expected format and pattern type
            _ => 0.1, // Heavy penalty for format/pattern mismatch
        }
    }
    
    /// Score QR Code geometric properties
    fn score_qr_geometry(finder_patterns: &[crate::detection::FinderPattern], position: &BoundingBox) -> f32 {
        if finder_patterns.len() != 3 {
            return 0.0;
        }
        
        let mut score = 0.0;
        let total_checks = 6.0;
        
        // Check triangle properties
        let fp1 = &finder_patterns[0];
        let fp2 = &finder_patterns[1];
        let fp3 = &finder_patterns[2];
        
        // 1. Size consistency (finder patterns should be similar size)
        let sizes = [fp1.size, fp2.size, fp3.size];
        let avg_size = sizes.iter().sum::<f32>() / 3.0;
        let size_variance = sizes.iter()
            .map(|s| (s - avg_size).abs() / avg_size)
            .sum::<f32>() / 3.0;
        
        if size_variance < 0.2 {
            score += 1.0; // Good size consistency
        } else if size_variance < 0.4 {
            score += 0.5; // Moderate consistency
        }
        
        // 2. Right angle check
        let angle_score = Self::check_right_angles(
            (fp1.center.0, fp1.center.1),
            (fp2.center.0, fp2.center.1),
            (fp3.center.0, fp3.center.1)
        );
        score += angle_score;
        
        // 3. Distance ratios (should form isosceles right triangle)
        let dist12 = Self::distance(fp1.center, fp2.center);
        let dist13 = Self::distance(fp1.center, fp3.center);
        let dist23 = Self::distance(fp2.center, fp3.center);
        
        let mut distances = [dist12, dist13, dist23];
        distances.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        // Check if it's approximately a right triangle (a² + b² ≈ c²)
        let a_sq = distances[0] * distances[0];
        let b_sq = distances[1] * distances[1];
        let c_sq = distances[2] * distances[2];
        let pythagorean_error = ((a_sq + b_sq) - c_sq).abs() / c_sq;
        
        if pythagorean_error < 0.1 {
            score += 2.0; // Good right triangle
        } else if pythagorean_error < 0.2 {
            score += 1.0; // Moderate triangle
        }
        
        // 4. Position within bounding box
        if Self::all_points_in_bounds(&[fp1.center, fp2.center, fp3.center], position) {
            score += 1.0;
        }
        
        // 5. Pattern ratio consistency (each should be close to 1:1:3:1:1)
        let ratio_score = finder_patterns.iter()
            .map(|fp| Self::validate_finder_pattern_ratios(&fp.ratios))
            .sum::<f32>() / 3.0;
        score += ratio_score;
        
        score / total_checks
    }
    
    /// Score DataMatrix geometric properties
    fn score_datamatrix_geometry(l_border: &crate::detection::LBorder, position: &BoundingBox) -> f32 {
        let mut score = 0.0;
        let total_checks = 4.0;
        
        // 1. Right angle between L-border lines
        let angle_score = Self::check_line_perpendicularity(
            l_border.vertical_line,
            l_border.horizontal_line
        );
        score += angle_score;
        
        // 2. Lines should intersect near corner
        let intersection = Self::line_intersection(
            l_border.vertical_line,
            l_border.horizontal_line
        );
        if let Some(intersect) = intersection {
            let corner_distance = Self::distance(intersect, l_border.corner);
            if corner_distance < 5.0 {
                score += 1.0;
            } else if corner_distance < 10.0 {
                score += 0.5;
            }
        }
        
        // 3. L-border should be within bounding box
        let points = [
            l_border.vertical_line.0,
            l_border.vertical_line.1,
            l_border.horizontal_line.0,
            l_border.horizontal_line.1,
            l_border.corner
        ];
        if Self::all_points_in_bounds(&points, position) {
            score += 1.0;
        }
        
        // 4. L-border confidence from detection
        score += l_border.confidence;
        
        score / total_checks
    }
    
    /// Score linear barcode geometry
    fn score_linear_geometry(bars_and_spaces: &[u32]) -> f32 {
        if bars_and_spaces.is_empty() {
            return 0.0;
        }
        
        let mut score = 0.0;
        let total_checks = 3.0;
        
        // 1. Reasonable number of bars/spaces
        let element_count = bars_and_spaces.len();
        if element_count >= 20 && element_count <= 200 {
            score += 1.0;
        } else if element_count >= 10 && element_count <= 300 {
            score += 0.5;
        }
        
        // 2. Bar/space width consistency
        let avg_width = bars_and_spaces.iter().sum::<u32>() as f32 / element_count as f32;
        let variance = bars_and_spaces.iter()
            .map(|w| (*w as f32 - avg_width).abs() / avg_width)
            .sum::<f32>() / element_count as f32;
        
        if variance < 0.5 {
            score += 1.0;
        } else if variance < 1.0 {
            score += 0.5;
        }
        
        // 3. Alternating pattern (bars and spaces should alternate)
        score += 1.0; // Assume good alternation for now
        
        score / total_checks
    }
    
    /// Score PDF417 geometry
    fn score_pdf417_geometry(start_patterns: &[(u32, u32)], stop_patterns: &[(u32, u32)]) -> f32 {
        if start_patterns.is_empty() || stop_patterns.is_empty() {
            return 0.0;
        }
        
        let mut score = 0.0;
        let total_checks = 3.0;
        
        // 1. Matching number of start/stop patterns
        if start_patterns.len() == stop_patterns.len() {
            score += 1.0;
        }
        
        // 2. Reasonable number of rows
        let row_count = start_patterns.len();
        if row_count >= 3 && row_count <= 90 {
            score += 1.0;
        } else if row_count >= 1 && row_count <= 200 {
            score += 0.5;
        }
        
        // 3. Pattern alignment (start/stop should be vertically aligned)
        if row_count > 1 {
            let start_alignment = Self::check_vertical_alignment(start_patterns);
            let stop_alignment = Self::check_vertical_alignment(stop_patterns);
            score += (start_alignment + stop_alignment) / 2.0;
        } else {
            score += 1.0;
        }
        
        score / total_checks
    }
    
    /// Score Aztec geometry
    fn score_aztec_geometry(bullseye: &(f32, f32, f32), reference_grid: &[Vec<bool>]) -> f32 {
        let mut score = 0.0;
        let total_checks = 3.0;
        
        // 1. Bullseye size should be reasonable
        let size = bullseye.2;
        if size >= 5.0 && size <= 100.0 {
            score += 1.0;
        } else if size >= 2.0 && size <= 200.0 {
            score += 0.5;
        }
        
        // 2. Reference grid should be square-ish
        if !reference_grid.is_empty() {
            let height = reference_grid.len();
            let width = reference_grid[0].len();
            let aspect_ratio = width as f32 / height as f32;
            
            if aspect_ratio >= 0.8 && aspect_ratio <= 1.2 {
                score += 1.0;
            } else if aspect_ratio >= 0.6 && aspect_ratio <= 1.4 {
                score += 0.5;
            }
        }
        
        // 3. Reasonable grid size
        if !reference_grid.is_empty() {
            let size = reference_grid.len();
            if size >= 11 && size <= 151 {
                score += 1.0;
            } else if size >= 5 && size <= 200 {
                score += 0.5;
            }
        }
        
        score / total_checks
    }
    
    // Pattern scoring methods
    fn score_qr_patterns(
        finder_patterns: &[crate::detection::FinderPattern], 
        timing_patterns: &(Vec<bool>, Vec<bool>)
    ) -> f32 {
        let mut score = 0.0;
        let total_checks = 4.0;
        
        // 1. Finder pattern ratio validation
        let avg_ratio_score = finder_patterns.iter()
            .map(|fp| Self::validate_finder_pattern_ratios(&fp.ratios))
            .sum::<f32>() / finder_patterns.len() as f32;
        score += avg_ratio_score;
        
        // 2. Timing pattern validation (should alternate)
        let horizontal_timing_score = Self::validate_timing_pattern(&timing_patterns.0);
        let vertical_timing_score = Self::validate_timing_pattern(&timing_patterns.1);
        score += (horizontal_timing_score + vertical_timing_score) / 2.0;
        
        // 3. Pattern consistency across finder patterns
        let avg_confidence = finder_patterns.iter()
            .map(|fp| fp.confidence)
            .sum::<f32>() / finder_patterns.len() as f32;
        score += avg_confidence;
        
        // 4. Cross-pattern validation
        score += 1.0; // Placeholder for more complex validation
        
        score / total_checks
    }
    
    fn score_datamatrix_patterns(timing_patterns: &(Vec<bool>, Vec<bool>), l_border: &LBorder) -> f32 {
        let mut score = 0.0;
        let total_checks = 3.0;
        
        // 1. Vertical timing pattern alternation
        score += Self::validate_timing_pattern(&timing_patterns.0);
        
        // 2. Horizontal timing pattern alternation
        score += Self::validate_timing_pattern(&timing_patterns.1);
        
        // 3. L-border confidence
        score += (l_border.confidence / 100.0).min(1.0); // Normalize confidence
        
        score / total_checks
    }
    
    fn score_linear_patterns(
        bars_and_spaces: &[u32], 
        start_end_patterns: &(Vec<u32>, Vec<u32>),
        barcode_type: &BarcodeType
    ) -> f32 {
        let mut score = 0.0;
        let total_checks = 3.0;
        
        // 1. Start pattern validation
        if Self::validate_linear_start_pattern(&start_end_patterns.0, barcode_type) {
            score += 1.0;
        }
        
        // 2. End pattern validation
        if Self::validate_linear_end_pattern(&start_end_patterns.1, barcode_type) {
            score += 1.0;
        }
        
        // 3. Overall pattern structure
        score += Self::validate_linear_structure(bars_and_spaces, barcode_type);
        
        score / total_checks
    }
    
    fn score_pdf417_patterns(rows: &[Vec<u32>]) -> f32 {
        if rows.is_empty() {
            return 0.0;
        }
        
        let mut score = 0.0;
        let total_checks = 2.0;
        
        // 1. Row consistency (all rows should have similar structure)
        let first_row_len = rows[0].len();
        let length_consistency = rows.iter()
            .filter(|row| row.len() == first_row_len)
            .count() as f32 / rows.len() as f32;
        score += length_consistency;
        
        // 2. Pattern density (reasonable number of elements per row)
        let avg_elements = rows.iter()
            .map(|row| row.len())
            .sum::<usize>() as f32 / rows.len() as f32;
        
        if avg_elements >= 10.0 && avg_elements <= 100.0 {
            score += 1.0;
        } else if avg_elements >= 5.0 && avg_elements <= 200.0 {
            score += 0.5;
        }
        
        score / total_checks
    }
    
    fn score_aztec_patterns(reference_grid: &[Vec<bool>]) -> f32 {
        if reference_grid.is_empty() {
            return 0.0;
        }
        
        // For now, just check if we have a reasonable grid
        let height = reference_grid.len();
        let width = reference_grid[0].len();
        
        if height >= 11 && height <= 151 && width >= 11 && width <= 151 {
            1.0
        } else {
            0.5
        }
    }
    
    // Content validation methods
    fn validate_ean13_content(data: &str) -> f32 {
        if data.len() != 13 {
            return 0.0;
        }
        
        if !data.chars().all(|c| c.is_ascii_digit()) {
            return 0.0;
        }
        
        // Validate checksum
        if Self::validate_ean13_checksum(data) {
            1.0
        } else {
            0.0
        }
    }
    
    fn validate_upca_content(data: &str) -> f32 {
        if data.len() != 12 {
            return 0.0;
        }
        
        if !data.chars().all(|c| c.is_ascii_digit()) {
            return 0.0;
        }
        
        // Validate checksum
        if Self::validate_upca_checksum(data) {
            1.0
        } else {
            0.0
        }
    }
    
    fn validate_code128_content(data: &str) -> f32 {
        // Code128 can contain any ASCII character
        if data.is_empty() || data.len() > 1000 {
            return 0.0;
        }
        
        if data.chars().all(|c| c.is_ascii()) {
            1.0
        } else {
            0.5
        }
    }
    
    fn validate_code39_content(data: &str) -> f32 {
        if data.is_empty() {
            return 0.0;
        }
        
        // Code39 character set
        const CODE39_CHARS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-. $/+%";
        
        if data.chars().all(|c| CODE39_CHARS.contains(c)) {
            1.0
        } else {
            0.0
        }
    }
    
    fn validate_itf14_content(data: &str) -> f32 {
        if data.len() != 14 {
            return 0.0;
        }
        
        if !data.chars().all(|c| c.is_ascii_digit()) {
            return 0.0;
        }
        
        // Validate checksum
        if Self::validate_itf14_checksum(data) {
            1.0
        } else {
            0.0
        }
    }
    
    fn validate_codabar_content(data: &str) -> f32 {
        if data.len() < 3 {
            return 0.0;
        }
        
        // Codabar character set
        const CODABAR_CHARS: &str = "0123456789-$:/.+ABCD";
        
        if data.chars().all(|c| CODABAR_CHARS.contains(c)) {
            // Should start and end with A, B, C, or D
            let first = data.chars().next().unwrap();
            let last = data.chars().last().unwrap();
            
            if "ABCD".contains(first) && "ABCD".contains(last) {
                1.0
            } else {
                0.5
            }
        } else {
            0.0
        }
    }
    
    fn validate_qr_content(_data: &str) -> f32 {
        // QR codes can contain almost any data
        1.0
    }
    
    fn validate_datamatrix_content(_data: &str) -> f32 {
        // DataMatrix can contain various data types
        1.0
    }
    
    fn validate_pdf417_content(_data: &str) -> f32 {
        // PDF417 can contain various data types
        1.0
    }
    
    fn validate_aztec_content(_data: &str) -> f32 {
        // Aztec can contain various data types
        1.0
    }
    
    // Helper methods
    fn distance(p1: (f32, f32), p2: (f32, f32)) -> f32 {
        let dx = p1.0 - p2.0;
        let dy = p1.1 - p2.1;
        (dx * dx + dy * dy).sqrt()
    }
    
    fn check_right_angles(p1: (f32, f32), p2: (f32, f32), p3: (f32, f32)) -> f32 {
        // Check angles in triangle formed by three points
        let angles = [
            Self::angle_between_points(p1, p2, p3),
            Self::angle_between_points(p2, p1, p3),
            Self::angle_between_points(p3, p1, p2),
        ];
        
        // Look for angle close to 90 degrees
        let right_angle_score = angles.iter()
            .map(|&angle| {
                let diff = (angle - 90.0).abs();
                if diff < 5.0 {
                    1.0
                } else if diff < 15.0 {
                    0.5
                } else {
                    0.0
                }
            })
            .fold(0.0f32, f32::max);
            
        right_angle_score
    }
    
    fn angle_between_points(center: (f32, f32), p1: (f32, f32), p2: (f32, f32)) -> f32 {
        let v1 = (p1.0 - center.0, p1.1 - center.1);
        let v2 = (p2.0 - center.0, p2.1 - center.1);
        
        let dot = v1.0 * v2.0 + v1.1 * v2.1;
        let mag1 = (v1.0 * v1.0 + v1.1 * v1.1).sqrt();
        let mag2 = (v2.0 * v2.0 + v2.1 * v2.1).sqrt();
        
        if mag1 == 0.0 || mag2 == 0.0 {
            return 0.0;
        }
        
        let cos_angle = dot / (mag1 * mag2);
        let cos_angle = cos_angle.max(-1.0).min(1.0); // Clamp to valid range
        cos_angle.acos().to_degrees()
    }
    
    fn all_points_in_bounds(points: &[(f32, f32)], bounds: &BoundingBox) -> bool {
        points.iter().all(|&(x, y)| {
            x >= bounds.x as f32 && 
            x <= (bounds.x + bounds.width) as f32 &&
            y >= bounds.y as f32 && 
            y <= (bounds.y + bounds.height) as f32
        })
    }
    
    fn check_line_perpendicularity(
        line1: ((f32, f32), (f32, f32)),
        line2: ((f32, f32), (f32, f32))
    ) -> f32 {
        let v1 = (line1.1.0 - line1.0.0, line1.1.1 - line1.0.1);
        let v2 = (line2.1.0 - line2.0.0, line2.1.1 - line2.0.1);
        
        let dot = v1.0 * v2.0 + v1.1 * v2.1;
        let mag1 = (v1.0 * v1.0 + v1.1 * v1.1).sqrt();
        let mag2 = (v2.0 * v2.0 + v2.1 * v2.1).sqrt();
        
        if mag1 == 0.0 || mag2 == 0.0 {
            return 0.0;
        }
        
        let cos_angle = (dot / (mag1 * mag2)).abs();
        
        // Lines are perpendicular when cos(angle) ≈ 0
        if cos_angle < 0.1 {
            1.0
        } else if cos_angle < 0.3 {
            0.5
        } else {
            0.0
        }
    }
    
    fn line_intersection(
        line1: ((f32, f32), (f32, f32)),
        line2: ((f32, f32), (f32, f32))
    ) -> Option<(f32, f32)> {
        let (x1, y1) = line1.0;
        let (x2, y2) = line1.1;
        let (x3, y3) = line2.0;
        let (x4, y4) = line2.1;
        
        let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        
        if denom.abs() < 1e-6 {
            return None; // Lines are parallel
        }
        
        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom;
        
        Some((x1 + t * (x2 - x1), y1 + t * (y2 - y1)))
    }
    
    fn check_vertical_alignment(patterns: &[(u32, u32)]) -> f32 {
        if patterns.len() < 2 {
            return 1.0;
        }
        
        let avg_x = patterns.iter().map(|p| p.0 as f32).sum::<f32>() / patterns.len() as f32;
        let variance = patterns.iter()
            .map(|p| (p.0 as f32 - avg_x).abs())
            .sum::<f32>() / patterns.len() as f32;
        
        if variance < 5.0 {
            1.0
        } else if variance < 15.0 {
            0.5
        } else {
            0.0
        }
    }
    
    fn validate_finder_pattern_ratios(ratios: &[u32; 5]) -> f32 {
        // Expected ratio is 1:1:3:1:1
        if ratios.iter().any(|&r| r == 0) {
            return 0.0;
        }
        
        let unit = ratios[0] as f32;
        let expected = [unit, unit, 3.0 * unit, unit, unit];
        
        let errors: Vec<f32> = ratios.iter().zip(expected.iter())
            .map(|(&actual, &expected)| {
                let diff = (actual as f32 - expected).abs();
                diff / expected
            })
            .collect();
        
        let avg_error = errors.iter().sum::<f32>() / 5.0;
        
        if avg_error < 0.2 {
            1.0
        } else if avg_error < 0.5 {
            0.5
        } else {
            0.0
        }
    }
    
    fn validate_timing_pattern(pattern: &[bool]) -> f32 {
        if pattern.len() < 2 {
            return 0.0;
        }
        
        let mut alternating_count = 0;
        for i in 1..pattern.len() {
            if pattern[i] != pattern[i-1] {
                alternating_count += 1;
            }
        }
        
        let alternating_ratio = alternating_count as f32 / (pattern.len() - 1) as f32;
        
        if alternating_ratio > 0.8 {
            1.0
        } else if alternating_ratio > 0.6 {
            0.5
        } else {
            0.0
        }
    }
    
    fn validate_linear_start_pattern(pattern: &[u32], _barcode_type: &BarcodeType) -> bool {
        // Placeholder - implement specific start pattern validation per barcode type
        !pattern.is_empty()
    }
    
    fn validate_linear_end_pattern(pattern: &[u32], _barcode_type: &BarcodeType) -> bool {
        // Placeholder - implement specific end pattern validation per barcode type
        !pattern.is_empty()
    }
    
    fn validate_linear_structure(bars_and_spaces: &[u32], _barcode_type: &BarcodeType) -> f32 {
        // Placeholder - implement specific structure validation per barcode type
        if bars_and_spaces.is_empty() {
            0.0
        } else {
            0.8
        }
    }
    
    // Checksum validation methods
    fn validate_ean13_checksum(data: &str) -> bool {
        let digits: Vec<u32> = data.chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        
        if digits.len() != 13 {
            return false;
        }
        
        let check_digit = digits[12];
        let calculated = Self::calculate_ean13_checksum(&digits[..12]);
        
        check_digit == calculated
    }
    
    fn calculate_ean13_checksum(digits: &[u32]) -> u32 {
        let sum: u32 = digits.iter().enumerate()
            .map(|(i, &digit)| {
                if i % 2 == 0 { digit } else { digit * 3 }
            })
            .sum();
        
        let remainder = sum % 10;
        if remainder == 0 { 0 } else { 10 - remainder }
    }
    
    fn validate_upca_checksum(data: &str) -> bool {
        let digits: Vec<u32> = data.chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        
        if digits.len() != 12 {
            return false;
        }
        
        let check_digit = digits[11];
        let calculated = Self::calculate_upca_checksum(&digits[..11]);
        
        check_digit == calculated
    }
    
    fn calculate_upca_checksum(digits: &[u32]) -> u32 {
        let sum: u32 = digits.iter().enumerate()
            .map(|(i, &digit)| {
                if i % 2 == 0 { digit * 3 } else { digit }
            })
            .sum();
        
        let remainder = sum % 10;
        if remainder == 0 { 0 } else { 10 - remainder }
    }
    
    fn validate_itf14_checksum(data: &str) -> bool {
        let digits: Vec<u32> = data.chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        
        if digits.len() != 14 {
            return false;
        }
        
        let check_digit = digits[13];
        let calculated = Self::calculate_itf14_checksum(&digits[..13]);
        
        check_digit == calculated
    }
    
    fn calculate_itf14_checksum(digits: &[u32]) -> u32 {
        let sum: u32 = digits.iter().enumerate()
            .map(|(i, &digit)| {
                if i % 2 == 0 { digit * 3 } else { digit }
            })
            .sum();
        
        let remainder = sum % 10;
        if remainder == 0 { 0 } else { 10 - remainder }
    }
}

/// Confidence score breakdown
#[derive(Debug, Clone)]
pub struct ConfidenceScore {
    pub overall: f32,
    pub geometric: f32,
    pub pattern: f32,
    pub content: f32,
}
