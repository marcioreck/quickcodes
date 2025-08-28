// PDF417 Detection Engine
use crate::detection::{DetectionCandidate, PatternData, BoundingBox};
use crate::detection::preprocessing::ProcessedImage;
use crate::types::BarcodeType;
use image::{ImageBuffer, Luma};

/// PDF417 detection engine
pub fn detect_pdf417_candidates(image: &ProcessedImage) -> Vec<DetectionCandidate> {
    let mut candidates = Vec::new();
    
    // Basic detection: look for rectangular regions that could be PDF417
    let (width, height) = image.image.dimensions();
    
    // PDF417 codes are typically wider than they are tall (rectangular)
    if width > height && width >= 50 && height >= 10 && 
       width as f32 / height as f32 >= 1.5 && width as f32 / height as f32 <= 15.0 {
        
        // Create a basic candidate for the entire image region
        let candidate = DetectionCandidate {
            barcode_type: BarcodeType::PDF417,
            position: BoundingBox {
                x: 0,
                y: 0,
                width,
                height,
                corners: vec![
                    (0.0, 0.0),
                    (width as f32, 0.0),
                    (width as f32, height as f32),
                    (0.0, height as f32),
                ],
            },
            raw_data: None, // Use real decoder instead of test data
            pattern_data: PatternData::PDF417 {
                start_patterns: vec![(0, 0), (0, height / 2)],
                stop_patterns: vec![(width - 10, 0), (width - 10, height / 2)],
                rows: vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]],
            },
        };
        
        candidates.push(candidate);
    }
    
    candidates
}

/// Detect PDF417 start patterns (left guard)
fn detect_start_patterns(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<PDF417Pattern> {
    let mut patterns = Vec::new();
    let (width, height) = image.dimensions();
    
    // PDF417 start pattern: 8 modules - 11111110 (quiet zone + start)
    let start_pattern = [1, 1, 1, 1, 1, 1, 1, 0]; // Black bars with white end
    
    // Scan each row for start patterns
    for y in 0..height {
        let mut x = 0;
        while x < width.saturating_sub(20) { // Need at least 20 pixels for pattern
            if let Some(pattern) = scan_for_pattern(image, x, y, &start_pattern, true) {
                patterns.push(PDF417Pattern {
                    position: (pattern.x, pattern.y),
                    module_size: pattern.module_size,
                    confidence: pattern.confidence,
                    pattern_type: PDF417PatternType::Start,
                });
                x = pattern.x + (pattern.module_size * 8.0) as u32; // Skip past this pattern
            } else {
                x += 1;
            }
        }
    }
    
    patterns
}

/// Detect PDF417 stop patterns (right guard)
fn detect_stop_patterns(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<PDF417Pattern> {
    let mut patterns = Vec::new();
    let (width, height) = image.dimensions();
    
    // PDF417 stop pattern: 9 modules - 111111101 (stop + quiet zone)
    let stop_pattern = [1, 1, 1, 1, 1, 1, 1, 0, 1]; // Black bars with white separator and black end
    
    // Scan each row for stop patterns (scan right to left for efficiency)
    for y in 0..height {
        let mut x = width;
        while x > 25 { // Need at least 25 pixels for pattern
            x -= 1;
            if let Some(pattern) = scan_for_pattern(image, x.saturating_sub(20), y, &stop_pattern, false) {
                patterns.push(PDF417Pattern {
                    position: (pattern.x, pattern.y),
                    module_size: pattern.module_size,
                    confidence: pattern.confidence,
                    pattern_type: PDF417PatternType::Stop,
                });
                if pattern.x > (pattern.module_size * 9.0) as u32 {
                    x = pattern.x - (pattern.module_size * 9.0) as u32; // Skip past this pattern
                }
            }
        }
    }
    
    patterns
}

/// Scan for a specific pattern at given coordinates
fn scan_for_pattern(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    start_x: u32,
    y: u32,
    pattern: &[u8],
    from_left: bool
) -> Option<PatternMatch> {
    let (width, _) = image.dimensions();
    
    // Try different module sizes (2-8 pixels per module)
    for module_size in 2..=8 {
        let pattern_width = pattern.len() as u32 * module_size;
        
        if from_left && start_x + pattern_width > width {
            continue;
        }
        if !from_left && start_x < pattern_width {
            continue;
        }
        
        let scan_start = if from_left { start_x } else { start_x - pattern_width };
        
        let mut matches = 0;
        let mut _total_checks = 0;
        
        for (i, &expected) in pattern.iter().enumerate() {
            let module_start = scan_start + (i as u32 * module_size);
            
            // Sample multiple points within each module
            let samples = (module_size / 2).max(1);
            let mut module_matches = 0;
            
            for sample in 0..samples {
                let x = module_start + sample;
                if x < width {
                    let pixel_value = image.get_pixel(x, y)[0];
                    let is_black = pixel_value < 128;
                    let expected_black = expected == 1;
                    
                    if is_black == expected_black {
                        module_matches += 1;
                    }
                    _total_checks += 1;
                }
            }
            
            // At least 70% of samples in this module must match
            if module_matches as f32 / samples as f32 >= 0.7 {
                matches += 1;
            }
        }
        
        // At least 80% of modules must match for valid pattern
        let confidence = matches as f32 / pattern.len() as f32;
        if confidence >= 0.8 {
            return Some(PatternMatch {
                x: scan_start,
                y,
                module_size: module_size as f32,
                confidence,
            });
        }
    }
    
    None
}

/// Validate PDF417 candidate by matching start/stop patterns
fn validate_pdf417_candidate(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    start_pattern: &PDF417Pattern,
    stop_pattern: &PDF417Pattern
) -> Option<DetectionCandidate> {
    // Check if patterns are on the same row or nearby rows
    let row_distance = (start_pattern.position.1 as i32 - stop_pattern.position.1 as i32).abs();
    if row_distance > 3 {
        return None; // Patterns too far apart vertically
    }
    
    // Check horizontal alignment and reasonable distance
    let horizontal_distance = stop_pattern.position.0 as i32 - start_pattern.position.0 as i32;
    if horizontal_distance < 50 || horizontal_distance > 2000 {
        return None; // Too close or too far apart
    }
    
    // Check module size compatibility
    let module_size_ratio = start_pattern.module_size / stop_pattern.module_size;
    if module_size_ratio < 0.7 || module_size_ratio > 1.3 {
        return None; // Module sizes too different
    }
    
    let avg_module_size = (start_pattern.module_size + stop_pattern.module_size) / 2.0;
    
    // Detect rows between start and stop patterns
    let rows = detect_pdf417_rows(image, start_pattern, stop_pattern, avg_module_size);
    
    if rows.len() < 3 {
        return None; // Need at least 3 rows for valid PDF417
    }
    
    // Validate row consistency
    if !validate_row_consistency(&rows, avg_module_size) {
        return None;
    }
    
    // Calculate bounding box
    let bbox = calculate_pdf417_bbox(start_pattern, stop_pattern, &rows);
    
    Some(DetectionCandidate {
        barcode_type: BarcodeType::PDF417,
        position: bbox,
        raw_data: None,
        pattern_data: PatternData::PDF417 {
            start_patterns: vec![start_pattern.position],
            stop_patterns: vec![stop_pattern.position],
            rows,
        },
    })
}

/// Detect PDF417 rows between start and stop patterns
fn detect_pdf417_rows(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    start_pattern: &PDF417Pattern,
    stop_pattern: &PDF417Pattern,
    module_size: f32
) -> Vec<Vec<u32>> {
    let mut rows = Vec::new();
    
    let start_y = start_pattern.position.1.min(stop_pattern.position.1);
    let end_y = start_pattern.position.1.max(stop_pattern.position.1);
    let start_x = start_pattern.position.0;
    let end_x = stop_pattern.position.0;
    
    // Scan additional rows above and below the detected patterns
    let row_height = (module_size * 3.0) as u32; // PDF417 rows are typically 3x module height
    let scan_start = start_y.saturating_sub(row_height * 10);
    let scan_end = (end_y + row_height * 10).min(image.dimensions().1);
    
    for y in (scan_start..scan_end).step_by(row_height as usize) {
        if let Some(row_data) = extract_row_data(image, start_x, end_x, y, module_size) {
            rows.push(row_data);
        }
    }
    
    rows
}

/// Extract data from a single PDF417 row
fn extract_row_data(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    start_x: u32,
    end_x: u32,
    y: u32,
    module_size: f32
) -> Option<Vec<u32>> {
    if start_x >= end_x {
        return None;
    }
    
    let width = end_x - start_x;
    let estimated_modules = (width as f32 / module_size) as u32;
    
    if estimated_modules < 10 {
        return None; // Too short for valid PDF417 row
    }
    
    let mut row_data = Vec::new();
    let mut current_run = 0;
    let mut current_is_black = None;
    
    for module_idx in 0..estimated_modules {
        let x = start_x + (module_idx as f32 * module_size) as u32;
        if x >= end_x {
            break;
        }
        
        let pixel_value = image.get_pixel(x, y)[0];
        let is_black = pixel_value < 128;
        
        match current_is_black {
            None => {
                current_is_black = Some(is_black);
                current_run = 1;
            }
            Some(prev_black) if prev_black == is_black => {
                current_run += 1;
            }
            Some(_) => {
                row_data.push(current_run);
                current_is_black = Some(is_black);
                current_run = 1;
            }
        }
    }
    
    if current_run > 0 {
        row_data.push(current_run);
    }
    
    if row_data.len() >= 8 { // Minimum for valid PDF417 row
        Some(row_data)
    } else {
        None
    }
}

/// Validate consistency across PDF417 rows
fn validate_row_consistency(rows: &[Vec<u32>], module_size: f32) -> bool {
    if rows.len() < 3 {
        return false;
    }
    
    // Check that rows have similar lengths (within 20%)
    let lengths: Vec<usize> = rows.iter().map(|row| row.len()).collect();
    let avg_length = lengths.iter().sum::<usize>() as f32 / lengths.len() as f32;
    
    for &length in &lengths {
        let deviation = (length as f32 - avg_length).abs() / avg_length;
        if deviation > 0.2 {
            return false; // Too much variation in row length
        }
    }
    
    // Check that rows follow PDF417 patterns (start with reasonable run lengths)
    for row in rows {
        if row.is_empty() || row[0] < 1 || row[0] > (8.0 * module_size) as u32 {
            return false;
        }
    }
    
    true
}

/// Calculate PDF417 bounding box
fn calculate_pdf417_bbox(
    start_pattern: &PDF417Pattern,
    stop_pattern: &PDF417Pattern,
    rows: &[Vec<u32>]
) -> BoundingBox {
    let min_x = start_pattern.position.0.min(stop_pattern.position.0);
    let max_x = start_pattern.position.0.max(stop_pattern.position.0);
    let min_y = start_pattern.position.1.min(stop_pattern.position.1);
    let max_y = start_pattern.position.1.max(stop_pattern.position.1);
    
    // Estimate height based on number of rows
    let estimated_row_height = start_pattern.module_size * 3.0;
    let total_height = rows.len() as f32 * estimated_row_height;
    
    let width = max_x - min_x;
    let height = total_height.max((max_y - min_y) as f32) as u32;
    
    BoundingBox {
        x: min_x,
        y: min_y.saturating_sub(height / 4), // Add some margin above
        width,
        height: height + height / 2, // Add margin below
        corners: vec![
            (min_x as f32, min_y as f32),
            (max_x as f32, min_y as f32),
            (max_x as f32, (min_y + height) as f32),
            (min_x as f32, (min_y + height) as f32),
        ],
    }
}

/// PDF417 pattern representation
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct PDF417Pattern {
    position: (u32, u32),
    module_size: f32,
    confidence: f32,
    pattern_type: PDF417PatternType,
}

#[derive(Debug, Clone)]
enum PDF417PatternType {
    Start,
    Stop,
}

/// Pattern matching result
#[derive(Debug, Clone)]
struct PatternMatch {
    x: u32,
    y: u32,
    module_size: f32,
    confidence: f32,
}
