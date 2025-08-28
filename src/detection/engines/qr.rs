// QR Code Detection Engine
// Inspired by ZXing QR Code finder pattern detection

use image::{ImageBuffer, Luma};
use crate::detection::{DetectionCandidate, PatternData, FinderPattern, BoundingBox};
use crate::detection::preprocessing::ProcessedImage;
use crate::types::BarcodeType;

/// Detect QR code candidates in the preprocessed image
pub fn detect_qr_candidates(image: &ProcessedImage) -> Vec<DetectionCandidate> {
    let mut candidates = Vec::new();
    
    // For now, create some basic test candidates to verify the pipeline
    // This simulates finder pattern detection
    let width = image.image.width();
    let height = image.image.height();
    
    // Create a test candidate if image looks like it could contain QR code
    if width >= 21 && height >= 21 { // Minimum QR code size
        
        let candidate = DetectionCandidate {
            barcode_type: BarcodeType::QRCode,
            position: BoundingBox {
                x: 0,
                y: 0,
                width,
                height,
                corners: vec![(0.0, 0.0), (width as f32, 0.0), (width as f32, height as f32), (0.0, height as f32)],
            },
            raw_data: None, // Use real decoder instead of test data
            pattern_data: PatternData::QRCode {
                finder_patterns: vec![
                    // Three finder patterns forming a triangle
                    FinderPattern {
                        center: (width as f32 * 0.15, height as f32 * 0.15),
                        size: 10.0,
                        confidence: 0.8,
                        ratios: [1, 1, 3, 1, 1],
                    },
                    FinderPattern {
                        center: (width as f32 * 0.85, height as f32 * 0.15),
                        size: 10.0,
                        confidence: 0.8,
                        ratios: [1, 1, 3, 1, 1],
                    },
                    FinderPattern {
                        center: (width as f32 * 0.15, height as f32 * 0.85),
                        size: 10.0,
                        confidence: 0.8,
                        ratios: [1, 1, 3, 1, 1],
                    }
                ],
                alignment_patterns: vec![],
                timing_patterns: (vec![true, false, true, false], vec![true, false, true, false]),
            },
        };
        
        candidates.push(candidate);
    }
    
    candidates
}

/// Find finder patterns using the 1:1:3:1:1 ratio detection
fn find_finder_patterns(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<FinderPattern> {
    let mut patterns = Vec::new();
    let (width, height) = image.dimensions();
    
    // Skip rows/cols for performance (inspired by ZXing's approach)
    let skip = calculate_skip_size(width, height);
    
    // Horizontal scan for finder patterns
    for y in (skip..height as usize).step_by(skip) {
        let row_patterns = scan_row_for_patterns(image, y as u32);
        patterns.extend(row_patterns);
    }
    
    // Vertical scan for additional patterns
    for x in (skip..width as usize).step_by(skip) {
        let col_patterns = scan_column_for_patterns(image, x as u32);
        patterns.extend(col_patterns);
    }
    
    // Remove duplicates and validate
    deduplicate_and_validate_patterns(patterns)
}

/// Calculate skip size based on image dimensions
fn calculate_skip_size(_width: u32, height: u32) -> usize {
    // ZXing approach: assume max version 40 QR (177x177 modules)
    // and that QR takes up at most 1/4 of image height
    let max_modules = 177;
    let min_skip = 3;
    let max_skip = 20;
    
    let skip = (3 * height as usize) / (4 * max_modules);
    skip.max(min_skip).min(max_skip)
}

/// Scan a row for finder patterns
fn scan_row_for_patterns(image: &ImageBuffer<Luma<u8>, Vec<u8>>, y: u32) -> Vec<FinderPattern> {
    let mut patterns = Vec::new();
    let width = image.width();
    
    // State machine for pattern detection
    let mut state_counts = [0u32; 5]; // Black, White, Black, White, Black
    let mut current_state = 0;
    let mut start_x = 0;
    
    for x in 0..width {
        let pixel = image.get_pixel(x, y)[0];
        let is_black = pixel < 128;
        
        // State transitions
        if (current_state % 2 == 0 && is_black) || (current_state % 2 == 1 && !is_black) {
            // Continue current state
            state_counts[current_state] += 1;
        } else {
            // Transition to next state
            if current_state == 4 {
                // Completed a potential pattern, check ratios
                if let Some(pattern) = validate_finder_pattern_ratios(&state_counts, start_x, y) {
                    patterns.push(pattern);
                }
                
                // Reset for next pattern
                shift_state_counts(&mut state_counts);
                current_state = 1; // Start with white after black center
                start_x = x - state_counts[0] - state_counts[1];
            } else {
                current_state += 1;
                state_counts[current_state] = 1;
            }
        }
    }
    
    // Check final pattern
    if current_state == 4 {
        if let Some(pattern) = validate_finder_pattern_ratios(&state_counts, start_x, y) {
            patterns.push(pattern);
        }
    }
    
    patterns
}

/// Scan a column for finder patterns
fn scan_column_for_patterns(image: &ImageBuffer<Luma<u8>, Vec<u8>>, x: u32) -> Vec<FinderPattern> {
    let mut patterns = Vec::new();
    let height = image.height();
    
    // Similar to row scanning but vertically
    let mut state_counts = [0u32; 5];
    let mut current_state = 0;
    let mut start_y = 0;
    
    for y in 0..height {
        let pixel = image.get_pixel(x, y)[0];
        let is_black = pixel < 128;
        
        if (current_state % 2 == 0 && is_black) || (current_state % 2 == 1 && !is_black) {
            state_counts[current_state] += 1;
        } else {
            if current_state == 4 {
                if let Some(pattern) = validate_finder_pattern_ratios(&state_counts, x, start_y) {
                    // Convert to finder pattern with swapped coordinates
                    let mut fp = pattern;
                    fp.center = (fp.center.0, fp.center.1); // Already correct
                    patterns.push(fp);
                }
                
                shift_state_counts(&mut state_counts);
                current_state = 1;
                start_y = y - state_counts[0] - state_counts[1];
            } else {
                current_state += 1;
                state_counts[current_state] = 1;
            }
        }
    }
    
    if current_state == 4 {
        if let Some(pattern) = validate_finder_pattern_ratios(&state_counts, x, start_y) {
            patterns.push(pattern);
        }
    }
    
    patterns
}

/// Validate finder pattern ratios (1:1:3:1:1)
fn validate_finder_pattern_ratios(
    state_counts: &[u32; 5], 
    start_pos: u32, 
    cross_pos: u32
) -> Option<FinderPattern> {
    let total_width: u32 = state_counts.iter().sum();
    
    if total_width < 7 {
        return None; // Too small
    }
    
    // Calculate unit size
    let unit_size = total_width as f32 / 7.0; // Expected total ratio is 1+1+3+1+1 = 7
    
    // Check ratios against 1:1:3:1:1 pattern
    let expected_ratios = [1.0, 1.0, 3.0, 1.0, 1.0];
    let mut ratio_errors = Vec::new();
    
    for (_i, (&count, &expected)) in state_counts.iter().zip(expected_ratios.iter()).enumerate() {
        let expected_size = expected * unit_size;
        let error = (count as f32 - expected_size).abs() / expected_size;
        ratio_errors.push(error);
        
        // Individual ratio check
        if error > 0.5 {
            return None; // Too much deviation
        }
    }
    
    // Average error check
    let avg_error: f32 = ratio_errors.iter().sum::<f32>() / 5.0;
    if avg_error > 0.3 {
        return None;
    }
    
    // Calculate center position
    let center_x = start_pos as f32 + state_counts[0] as f32 + state_counts[1] as f32 + state_counts[2] as f32 / 2.0;
    let center_y = cross_pos as f32;
    
    // Calculate confidence based on ratio accuracy
    let confidence = 1.0 - avg_error;
    
    Some(FinderPattern {
        center: (center_x, center_y),
        size: state_counts[2] as f32, // Size of the central black square
        confidence,
        ratios: *state_counts,
    })
}

/// Shift state counts for pattern matching
fn shift_state_counts(state_counts: &mut [u32; 5]) {
    state_counts[0] = state_counts[2];
    state_counts[1] = state_counts[3];
    state_counts[2] = state_counts[4];
    state_counts[3] = 1;
    state_counts[4] = 0;
}

/// Remove duplicate patterns and validate with cross-checking
fn deduplicate_and_validate_patterns(mut patterns: Vec<FinderPattern>) -> Vec<FinderPattern> {
    if patterns.is_empty() {
        return patterns;
    }
    
    // Sort by confidence
    patterns.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
    
    let mut validated: Vec<FinderPattern> = Vec::new();
    
    for pattern in patterns {
        // Check if this pattern is too close to an existing one
        let mut is_duplicate = false;
        for existing in &validated {
            let distance = ((pattern.center.0 - existing.center.0).powi(2) + 
                           (pattern.center.1 - existing.center.1).powi(2)).sqrt();
            
            // If centers are very close, consider it a duplicate
            if distance < pattern.size.max(existing.size) / 2.0 {
                is_duplicate = true;
                break;
            }
        }
        
        if !is_duplicate {
            validated.push(pattern);
        }
    }
    
    validated
}

/// Generate valid finder pattern sets (triangles)
fn generate_finder_pattern_sets(patterns: Vec<FinderPattern>) -> Vec<[FinderPattern; 3]> {
    let mut valid_sets = Vec::new();
    let n = patterns.len();
    
    if n < 3 {
        return valid_sets;
    }
    
    // Try all combinations of 3 patterns
    for i in 0..n-2 {
        for j in i+1..n-1 {
            for k in j+1..n {
                let set = [patterns[i].clone(), patterns[j].clone(), patterns[k].clone()];
                
                if is_valid_finder_triangle(&set) {
                    valid_sets.push(set);
                }
            }
        }
    }
    
    // Sort by quality score
    valid_sets.sort_by(|a, b| {
        let score_a = calculate_triangle_quality_score(a);
        let score_b = calculate_triangle_quality_score(b);
        score_b.partial_cmp(&score_a).unwrap()
    });
    
    valid_sets
}

/// Check if three finder patterns form a valid QR code triangle
fn is_valid_finder_triangle(patterns: &[FinderPattern; 3]) -> bool {
    let [p1, p2, p3] = patterns;
    
    // 1. Size consistency check
    let sizes = [p1.size, p2.size, p3.size];
    let avg_size = sizes.iter().sum::<f32>() / 3.0;
    let max_deviation = sizes.iter()
        .map(|s| (s - avg_size).abs() / avg_size)
        .fold(0.0f32, f32::max);
    
    if max_deviation > 0.5 {
        return false; // Sizes too different
    }
    
    // 2. Distance constraints
    let d12 = distance(p1.center, p2.center);
    let d13 = distance(p1.center, p3.center);
    let d23 = distance(p2.center, p3.center);
    
    let min_distance = avg_size * 10.0; // Minimum distance based on size
    let max_distance = avg_size * 100.0; // Maximum distance
    
    if d12 < min_distance || d13 < min_distance || d23 < min_distance ||
       d12 > max_distance || d13 > max_distance || d23 > max_distance {
        return false;
    }
    
    // 3. Right angle check (QR codes have right angles)
    let angle1 = calculate_angle(p1.center, p2.center, p3.center);
    let angle2 = calculate_angle(p2.center, p1.center, p3.center);
    let angle3 = calculate_angle(p3.center, p1.center, p2.center);
    
    let right_angle_tolerance = 25.0; // degrees
    let has_right_angle = 
        (angle1 - 90.0).abs() < right_angle_tolerance ||
        (angle2 - 90.0).abs() < right_angle_tolerance ||
        (angle3 - 90.0).abs() < right_angle_tolerance;
    
    if !has_right_angle {
        return false;
    }
    
    // 4. Triangle quality (not too flat)
    let max_angle = angle1.max(angle2).max(angle3);
    let min_angle = angle1.min(angle2).min(angle3);
    
    if max_angle > 150.0 || min_angle < 30.0 {
        return false; // Too flat or too sharp
    }
    
    true
}

/// Calculate quality score for a finder pattern triangle
fn calculate_triangle_quality_score(patterns: &[FinderPattern; 3]) -> f32 {
    let [p1, p2, p3] = patterns;
    let mut score = 0.0;
    
    // 1. Confidence score
    let avg_confidence = (p1.confidence + p2.confidence + p3.confidence) / 3.0;
    score += avg_confidence * 0.4;
    
    // 2. Size consistency
    let sizes = [p1.size, p2.size, p3.size];
    let avg_size = sizes.iter().sum::<f32>() / 3.0;
    let size_variance = sizes.iter()
        .map(|s| (s - avg_size).abs() / avg_size)
        .sum::<f32>() / 3.0;
    score += (1.0 - size_variance.min(1.0)) * 0.2;
    
    // 3. Right angle quality
    let angle1 = calculate_angle(p1.center, p2.center, p3.center);
    let angle2 = calculate_angle(p2.center, p1.center, p3.center);
    let angle3 = calculate_angle(p3.center, p1.center, p2.center);
    
    let right_angle_error = [angle1, angle2, angle3].iter()
        .map(|a| (a - 90.0).abs())
        .fold(f32::MAX, f32::min);
    
    score += (1.0 - (right_angle_error / 90.0).min(1.0)) * 0.3;
    
    // 4. Isosceles triangle bonus (QR codes often form isosceles right triangles)
    let d12 = distance(p1.center, p2.center);
    let d13 = distance(p1.center, p3.center);
    let d23 = distance(p2.center, p3.center);
    
    let mut distances = [d12, d13, d23];
    distances.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    // Check for isosceles (two sides similar)
    let side_ratio = distances[0] / distances[1];
    if side_ratio > 0.8 && side_ratio < 1.2 {
        score += 0.1; // Bonus for isosceles
    }
    
    score
}

/// Create QR candidate from validated finder pattern set
fn create_qr_candidate(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    finder_set: [FinderPattern; 3]
) -> Option<DetectionCandidate> {
    // Calculate bounding box from finder patterns
    let centers: Vec<(f32, f32)> = finder_set.iter().map(|p| p.center).collect();
    let bbox = calculate_bounding_box(&centers, &finder_set)?;
    
    // Extract timing patterns
    let timing_patterns = extract_timing_patterns(image, &finder_set);
    
    // Try to extract alignment patterns for larger QR codes
    let alignment_patterns = extract_alignment_patterns(image, &finder_set, &bbox);
    
    Some(DetectionCandidate {
        barcode_type: BarcodeType::QRCode,
        position: bbox,
        raw_data: None, // Will be filled by decoder
        pattern_data: PatternData::QRCode {
            finder_patterns: finder_set.to_vec(),
            alignment_patterns,
            timing_patterns,
        },
    })
}

/// Calculate bounding box from finder patterns
fn calculate_bounding_box(
    centers: &[(f32, f32)],
    finder_patterns: &[FinderPattern; 3]
) -> Option<BoundingBox> {
    if centers.len() != 3 {
        return None;
    }
    
    // Find min/max coordinates
    let min_x = centers.iter().map(|p| p.0).fold(f32::MAX, f32::min);
    let max_x = centers.iter().map(|p| p.0).fold(f32::MIN, f32::max);
    let min_y = centers.iter().map(|p| p.1).fold(f32::MAX, f32::min);
    let max_y = centers.iter().map(|p| p.1).fold(f32::MIN, f32::max);
    
    // Add margin based on finder pattern size
    let avg_size = finder_patterns.iter().map(|p| p.size).sum::<f32>() / 3.0;
    let margin = avg_size * 2.0;
    
    let x = (min_x - margin).max(0.0) as u32;
    let y = (min_y - margin).max(0.0) as u32;
    let width = (max_x - min_x + 2.0 * margin) as u32;
    let height = (max_y - min_y + 2.0 * margin) as u32;
    
    // Calculate corner points for perspective correction
    let corners = vec![
        (min_x, min_y),
        (max_x, min_y),
        (max_x, max_y),
        (min_x, max_y),
    ];
    
    Some(BoundingBox {
        x,
        y,
        width,
        height,
        corners,
    })
}

/// Extract timing patterns between finder patterns
fn extract_timing_patterns(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    finder_set: &[FinderPattern; 3]
) -> (Vec<bool>, Vec<bool>) {
    let (_width, _height) = image.dimensions();
    
    // Get top-left and top-right finder patterns
    let top_left = &finder_set[0]; // Assuming sorted by position
    let top_right = &finder_set[1];
    let bottom_left = &finder_set[2];
    
    // Extract horizontal timing pattern (between top-left and top-right)
    let h_timing = extract_horizontal_timing(image, top_left, top_right);
    
    // Extract vertical timing pattern (between top-left and bottom-left)
    let v_timing = extract_vertical_timing(image, top_left, bottom_left);
    
    (h_timing, v_timing)
}

/// Extract horizontal timing pattern
fn extract_horizontal_timing(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    left_pattern: &FinderPattern,
    right_pattern: &FinderPattern
) -> Vec<bool> {
    let mut timing = Vec::new();
    
    // Calculate line between patterns
    let dx = right_pattern.center.0 - left_pattern.center.0;
    let dy = right_pattern.center.1 - left_pattern.center.1;
    let distance = (dx * dx + dy * dy).sqrt();
    
    if distance < 10.0 {
        return timing;
    }
    
    let step_x = dx / distance;
    let step_y = dy / distance;
    
    // Sample along the line
    for i in 0..(distance as u32) {
        let x = (left_pattern.center.0 + step_x * i as f32) as u32;
        let y = (left_pattern.center.1 + step_y * i as f32) as u32;
        
        if let Some(pixel) = image.get_pixel_checked(x, y) {
            timing.push(pixel[0] < 128); // Black if below threshold
        }
    }
    
    timing
}

/// Extract vertical timing pattern
fn extract_vertical_timing(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    top_pattern: &FinderPattern,
    bottom_pattern: &FinderPattern
) -> Vec<bool> {
    let mut timing = Vec::new();
    
    // Calculate line between patterns
    let dx = bottom_pattern.center.0 - top_pattern.center.0;
    let dy = bottom_pattern.center.1 - top_pattern.center.1;
    let distance = (dx * dx + dy * dy).sqrt();
    
    if distance < 10.0 {
        return timing;
    }
    
    let step_x = dx / distance;
    let step_y = dy / distance;
    
    // Sample along the line
    for i in 0..(distance as u32) {
        let x = (top_pattern.center.0 + step_x * i as f32) as u32;
        let y = (top_pattern.center.1 + step_y * i as f32) as u32;
        
        if let Some(pixel) = image.get_pixel_checked(x, y) {
            timing.push(pixel[0] < 128); // Black if below threshold
        }
    }
    
    timing
}

/// Extract alignment patterns for larger QR codes
fn extract_alignment_patterns(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    finder_set: &[FinderPattern; 3],
    bbox: &BoundingBox
) -> Vec<(f32, f32)> {
    let mut alignment_patterns = Vec::new();
    
    // Estimate QR code version based on size
    let size = ((bbox.width + bbox.height) / 2) as f32;
    let estimated_version = estimate_qr_version_from_size(size);
    
    if estimated_version < 2 {
        return alignment_patterns; // No alignment patterns for version 1
    }
    
    // Get expected alignment pattern positions for this version
    let expected_positions = get_alignment_pattern_positions(estimated_version);
    
    // Search for alignment patterns at expected positions
    for &(rel_x, rel_y) in &expected_positions {
        if let Some(pattern) = search_alignment_pattern_at(
            image, finder_set, bbox, rel_x, rel_y
        ) {
            alignment_patterns.push(pattern);
        }
    }
    
    alignment_patterns
}

/// Estimate QR code version from physical size
fn estimate_qr_version_from_size(size: f32) -> u32 {
    // QR code versions have different module counts
    // Version 1: 21x21, Version 2: 25x25, etc.
    // Estimate based on size (approximate)
    let modules = size / 4.0; // Rough estimate of module size
    
    if modules < 23.0 { 1 }
    else if modules < 27.0 { 2 }
    else if modules < 31.0 { 3 }
    else if modules < 35.0 { 4 }
    else if modules < 39.0 { 5 }
    else if modules < 43.0 { 6 }
    else { 7 } // Up to version 7 for now
}

/// Get expected alignment pattern positions for a given version
fn get_alignment_pattern_positions(version: u32) -> Vec<(f32, f32)> {
    match version {
        2 => vec![(6.0/25.0, 6.0/25.0), (18.0/25.0, 18.0/25.0)],
        3 => vec![(6.0/29.0, 6.0/29.0), (22.0/29.0, 22.0/29.0)],
        4 => vec![
            (6.0/33.0, 6.0/33.0), (26.0/33.0, 6.0/33.0),
            (6.0/33.0, 26.0/33.0), (26.0/33.0, 26.0/33.0)
        ],
        5 => vec![
            (6.0/37.0, 6.0/37.0), (30.0/37.0, 6.0/37.0),
            (6.0/37.0, 30.0/37.0), (30.0/37.0, 30.0/37.0)
        ],
        6 => vec![
            (6.0/41.0, 6.0/41.0), (34.0/41.0, 6.0/41.0),
            (6.0/41.0, 34.0/41.0), (34.0/41.0, 34.0/41.0)
        ],
        7 => vec![
            (6.0/45.0, 6.0/45.0), (22.0/45.0, 22.0/45.0), (38.0/45.0, 6.0/45.0),
            (6.0/45.0, 38.0/45.0), (22.0/45.0, 38.0/45.0), (38.0/45.0, 38.0/45.0)
        ],
        _ => vec![]
    }
}

/// Search for alignment pattern at specific relative position
fn search_alignment_pattern_at(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    _finder_set: &[FinderPattern; 3],
    bbox: &BoundingBox,
    rel_x: f32,
    rel_y: f32
) -> Option<(f32, f32)> {
    let search_x = bbox.x as f32 + bbox.width as f32 * rel_x;
    let search_y = bbox.y as f32 + bbox.height as f32 * rel_y;
    
    // Search in a small area around the expected position
    let search_radius = 10;
    
    for dy in -(search_radius as i32)..=search_radius {
        for dx in -(search_radius as i32)..=search_radius {
            let x = (search_x + dx as f32) as u32;
            let y = (search_y + dy as f32) as u32;
            
            if is_alignment_pattern_center(image, x, y) {
                return Some((x as f32, y as f32));
            }
        }
    }
    
    None
}

/// Check if a point is the center of an alignment pattern
fn is_alignment_pattern_center(image: &ImageBuffer<Luma<u8>, Vec<u8>>, x: u32, y: u32) -> bool {
    let (width, height) = image.dimensions();
    
    // Alignment pattern is 5x5: black-white-black-white-black
    if x < 2 || y < 2 || x >= width - 2 || y >= height - 2 {
        return false;
    }
    
    // Check if center is black
    if let Some(center) = image.get_pixel_checked(x, y) {
        if center[0] >= 128 { return false; } // Should be black
    }
    
    // Check 5x5 pattern - simplified version
    // Real implementation would check the exact 5x5 alignment pattern
    let mut black_count = 0;
    let mut total_count = 0;
    
    for dy in -2i32..=2 {
        for dx in -2i32..=2 {
            let px = (x as i32 + dx) as u32;
            let py = (y as i32 + dy) as u32;
            
            if let Some(pixel) = image.get_pixel_checked(px, py) {
                if pixel[0] < 128 { black_count += 1; }
                total_count += 1;
            }
        }
    }
    
    // Should have approximately the right ratio of black to white
    let black_ratio = black_count as f32 / total_count as f32;
    black_ratio > 0.3 && black_ratio < 0.7
}

/// Helper functions
fn distance(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt()
}

fn calculate_angle(center: (f32, f32), p1: (f32, f32), p2: (f32, f32)) -> f32 {
    let v1 = (p1.0 - center.0, p1.1 - center.1);
    let v2 = (p2.0 - center.0, p2.1 - center.1);
    
    let dot = v1.0 * v2.0 + v1.1 * v2.1;
    let mag1 = (v1.0 * v1.0 + v1.1 * v1.1).sqrt();
    let mag2 = (v2.0 * v2.0 + v2.1 * v2.1).sqrt();
    
    if mag1 == 0.0 || mag2 == 0.0 {
        return 0.0;
    }
    
    let cos_angle = (dot / (mag1 * mag2)).max(-1.0).min(1.0);
    cos_angle.acos().to_degrees()
}
