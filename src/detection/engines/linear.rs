// Linear Barcode Detection Engine
use crate::detection::{DetectionCandidate, PatternData, BoundingBox};
use crate::detection::preprocessing::ProcessedImage;
use crate::types::BarcodeType;
use image::{ImageBuffer, Luma};

/// Linear barcode detection engine with multi-angle scanning
pub fn detect_linear_candidates(
    image: &ProcessedImage, 
    barcode_type: &BarcodeType
) -> Vec<DetectionCandidate> {
    let mut candidates = Vec::new();
    
    // Multi-angle scanning: 0°, 45°, 90°, 135°
    let scan_angles = [0.0, 45.0, 90.0, 135.0];
    
    for &angle in &scan_angles {
        let angle_candidates = scan_at_angle(&image.image, barcode_type, angle);
        candidates.extend(angle_candidates);
    }
    
    // Remove duplicates and low-confidence candidates
    filter_and_deduplicate_linear_candidates(candidates)
}

/// Scan for linear barcodes at a specific angle
fn scan_at_angle(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    barcode_type: &BarcodeType,
    angle_degrees: f32
) -> Vec<DetectionCandidate> {
    let mut candidates = Vec::new();
    let (width, height) = image.dimensions();
    
    let angle_rad = angle_degrees.to_radians();
    let _cos_angle = angle_rad.cos();
    let _sin_angle = angle_rad.sin();
    
    // Determine scan lines based on angle
    let scan_lines = generate_scan_lines(width, height, angle_rad);
    
    for scan_line in scan_lines {
        if let Some(candidate) = scan_line_for_barcode(image, barcode_type, &scan_line, angle_degrees) {
            candidates.push(candidate);
        }
    }
    
    candidates
}

/// Generate scan lines for the given angle
fn generate_scan_lines(width: u32, height: u32, angle_rad: f32) -> Vec<ScanLine> {
    let mut scan_lines = Vec::new();
    
    let _cos_angle = angle_rad.cos();
    let _sin_angle = angle_rad.sin();
    
    // For horizontal-ish angles (0° and 180°)
    if angle_rad.abs() < std::f32::consts::PI / 4.0 || 
       (angle_rad - std::f32::consts::PI).abs() < std::f32::consts::PI / 4.0 {
        
        // Scan horizontal lines
        for y in (0..height).step_by(5) { // Every 5 pixels
            scan_lines.push(ScanLine {
                start: (0.0, y as f32),
                end: (width as f32, y as f32),
                angle: angle_rad,
            });
        }
    }
    // For vertical-ish angles (90° and 270°)
    else if (angle_rad - std::f32::consts::PI / 2.0).abs() < std::f32::consts::PI / 4.0 ||
            (angle_rad - 3.0 * std::f32::consts::PI / 2.0).abs() < std::f32::consts::PI / 4.0 {
        
        // Scan vertical lines
        for x in (0..width).step_by(5) { // Every 5 pixels
            scan_lines.push(ScanLine {
                start: (x as f32, 0.0),
                end: (x as f32, height as f32),
                angle: angle_rad,
            });
        }
    }
    // For diagonal angles (45° and 135°)
    else {
        // Scan diagonal lines
        let diagonal_step = 10; // Fewer diagonal scans as they're more expensive
        
        // Diagonal from top-left to bottom-right (45°)
        if (angle_rad - std::f32::consts::PI / 4.0).abs() < std::f32::consts::PI / 8.0 {
            for offset in (0..(width + height)).step_by(diagonal_step) {
                let start_x = if offset < height { 0.0 } else { (offset - height) as f32 };
                let start_y = if offset < height { (height - offset) as f32 } else { 0.0 };
                let end_x = if offset < width { offset as f32 } else { width as f32 };
                let end_y = if offset < width { height as f32 } else { (height - (offset - width)) as f32 };
                
                scan_lines.push(ScanLine {
                    start: (start_x, start_y),
                    end: (end_x, end_y),
                    angle: angle_rad,
                });
            }
        }
        // Diagonal from top-right to bottom-left (135°)
        else {
            for offset in (0..(width + height)).step_by(diagonal_step) {
                let start_x = if offset < height { width as f32 } else { (width - (offset - height)) as f32 };
                let start_y = if offset < height { (height - offset) as f32 } else { 0.0 };
                let end_x = if offset < width { (width - offset) as f32 } else { 0.0 };
                let end_y = if offset < width { height as f32 } else { (height - (offset - width)) as f32 };
                
                scan_lines.push(ScanLine {
                    start: (start_x, start_y),
                    end: (end_x, end_y),
                    angle: angle_rad,
                });
            }
        }
    }
    
    scan_lines
}

/// Scan a single line for barcode patterns
fn scan_line_for_barcode(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    barcode_type: &BarcodeType,
    scan_line: &ScanLine,
    angle_degrees: f32
) -> Option<DetectionCandidate> {
    // Extract intensity profile along the scan line
    let intensity_profile = extract_intensity_profile(image, scan_line);
    
    if intensity_profile.len() < 20 {
        return None; // Too short for valid barcode
    }
    
    // Convert to bars and spaces
    let bars_and_spaces = convert_to_bars_and_spaces(&intensity_profile);
    
    if bars_and_spaces.len() < 10 {
        return None; // Need minimum number of bars/spaces
    }
    
    // Validate pattern based on barcode type
    if let Some(validation_result) = validate_barcode_pattern(barcode_type, &bars_and_spaces) {
        // Calculate bounding box
        let bbox = calculate_linear_bbox(scan_line, &bars_and_spaces, angle_degrees);
        
        return Some(DetectionCandidate {
            barcode_type: barcode_type.clone(),
            position: bbox,
            raw_data: None,
            pattern_data: PatternData::Linear {
                bars_and_spaces: bars_and_spaces.clone(),
                start_end_patterns: validation_result.start_end_patterns,
                scan_line: (
                    scan_line.start.0 as u32,
                    scan_line.start.1 as u32,
                    scan_line.end.0 as u32,
                    scan_line.end.1 as u32
                ),
            },
        });
    }
    
    None
}

/// Extract intensity profile along a scan line
fn extract_intensity_profile(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    scan_line: &ScanLine
) -> Vec<u8> {
    let mut profile = Vec::new();
    
    let dx = scan_line.end.0 - scan_line.start.0;
    let dy = scan_line.end.1 - scan_line.start.1;
    let length = (dx * dx + dy * dy).sqrt();
    
    if length < 1.0 {
        return profile;
    }
    
    let step_count = length as u32;
    let step_x = dx / length;
    let step_y = dy / length;
    
    for i in 0..step_count {
        let x = scan_line.start.0 + step_x * i as f32;
        let y = scan_line.start.1 + step_y * i as f32;
        
        if x >= 0.0 && y >= 0.0 && 
           x < image.dimensions().0 as f32 && 
           y < image.dimensions().1 as f32 {
            let pixel_value = image.get_pixel(x as u32, y as u32)[0];
            profile.push(pixel_value);
        }
    }
    
    profile
}

/// Convert intensity profile to bars and spaces (run-length encoding)
fn convert_to_bars_and_spaces(intensity_profile: &[u8]) -> Vec<u32> {
    if intensity_profile.is_empty() {
        return Vec::new();
    }
    
    let mut bars_and_spaces = Vec::new();
    let mut current_run = 1;
    let threshold = 128; // Black/white threshold
    let mut current_is_black = intensity_profile[0] < threshold;
    
    for &pixel in intensity_profile.iter().skip(1) {
        let is_black = pixel < threshold;
        
        if is_black == current_is_black {
            current_run += 1;
        } else {
            bars_and_spaces.push(current_run);
            current_run = 1;
            current_is_black = is_black;
        }
    }
    
    bars_and_spaces.push(current_run);
    bars_and_spaces
}

/// Validate barcode pattern based on type
fn validate_barcode_pattern(
    barcode_type: &BarcodeType,
    bars_and_spaces: &[u32]
) -> Option<LinearValidationResult> {
    match barcode_type {
        BarcodeType::EAN13 => validate_ean13_pattern(bars_and_spaces),
        BarcodeType::Code128 => validate_code128_pattern(bars_and_spaces),
        BarcodeType::Code39 => validate_code39_pattern(bars_and_spaces),
        BarcodeType::ITF14 => validate_itf14_pattern(bars_and_spaces),
        BarcodeType::Codabar => validate_codabar_pattern(bars_and_spaces),
        BarcodeType::UPCA => validate_upca_pattern(bars_and_spaces),
        _ => None, // Not a linear barcode
    }
}

/// Validate EAN-13 pattern
fn validate_ean13_pattern(bars_and_spaces: &[u32]) -> Option<LinearValidationResult> {
    // EAN-13 has specific structure: start guard + 6 digits + center guard + 6 digits + end guard
    // But real images may have different run-length counts, so be more flexible
    
    if bars_and_spaces.len() < 20 || bars_and_spaces.len() > 100 {
        return None; // Too short or too long
    }
    
    // Look for alternating pattern (bar-space-bar-space...)
    let mut alternating_count = 0;
    for i in 0..bars_and_spaces.len() - 1 {
        // In EAN-13, bars and spaces should alternate reasonably
        if (i % 2 == 0 && bars_and_spaces[i] > 0) || (i % 2 == 1 && bars_and_spaces[i] > 0) {
            alternating_count += 1;
        }
    }
    
    // Should have reasonable alternating pattern
    if alternating_count < bars_and_spaces.len() / 2 {
        return None;
    }
    
    // Look for guard-like patterns (short runs)
    let mut short_runs = 0;
    let mut long_runs = 0;
    let avg_run = bars_and_spaces.iter().sum::<u32>() as f32 / bars_and_spaces.len() as f32;
    
    for &run in bars_and_spaces {
        if (run as f32) < avg_run * 2.0 {
            short_runs += 1;
        } else {
            long_runs += 1;
        }
    }
    
    // EAN-13 should have more short runs than long runs (guards + data)
    if short_runs < long_runs {
        return None;
    }
    
    // Check for reasonable run-length distribution
    let min_run = *bars_and_spaces.iter().min().unwrap_or(&0);
    let max_run = *bars_and_spaces.iter().max().unwrap_or(&0);
    
    if min_run == 0 || max_run > min_run * 10 {
        return None; // Too much variation
    }
    
    Some(LinearValidationResult {
        confidence: 0.7, // Lower confidence since we're being more permissive
        start_end_patterns: (vec![1, 1, 1], vec![1, 1, 1]),
    })
}

/// Validate Code128 pattern
fn validate_code128_pattern(bars_and_spaces: &[u32]) -> Option<LinearValidationResult> {
    // Code128 has more complex patterns, for now use basic validation
    if bars_and_spaces.len() < 15 || bars_and_spaces.len() > 100 {
        return None;
    }
    
    // Basic run-length validation for Code128-like patterns
    let avg_run = bars_and_spaces.iter().sum::<u32>() as f32 / bars_and_spaces.len() as f32;
    let mut valid_runs = 0;
    
    for &run in bars_and_spaces {
        if (run as f32) < avg_run * 4.0 && run > 0 {
            valid_runs += 1;
        }
    }
    
    if valid_runs < bars_and_spaces.len() * 3 / 4 {
        return None;
    }
    
    Some(LinearValidationResult {
        confidence: 0.7,
        start_end_patterns: (vec![2, 1, 2, 3, 2, 2], vec![2, 3, 3, 1, 1, 1, 2]),
    })
}

/// Validate Code39 pattern
fn validate_code39_pattern(bars_and_spaces: &[u32]) -> Option<LinearValidationResult> {
    // Code39 has wider bars and spaces, more tolerance needed
    if bars_and_spaces.len() < 15 || bars_and_spaces.len() > 150 {
        return None;
    }
    
    // Code39 typically has alternating narrow/wide elements
    let avg_run = bars_and_spaces.iter().sum::<u32>() as f32 / bars_and_spaces.len() as f32;
    let mut narrow_count = 0;
    let mut wide_count = 0;
    
    for &run in bars_and_spaces {
        if (run as f32) < avg_run * 1.5 {
            narrow_count += 1;
        } else if (run as f32) < avg_run * 4.0 {
            wide_count += 1;
        }
    }
    
    // Code39 should have both narrow and wide elements
    if narrow_count < 5 || wide_count < 3 {
        return None;
    }
    
    Some(LinearValidationResult {
        confidence: 0.7,
        start_end_patterns: (vec![1, 2, 1, 1, 2, 1, 1, 2, 1], vec![1, 2, 1, 1, 2, 1, 1, 2, 1]),
    })
}

/// Validate ITF-14 pattern (Interleaved 2 of 5)
fn validate_itf14_pattern(bars_and_spaces: &[u32]) -> Option<LinearValidationResult> {
    // ITF has specific patterns but can be flexible for detection
    if bars_and_spaces.len() < 15 || bars_and_spaces.len() > 120 {
        return None;
    }
    
    // ITF has a distinctive pattern with pairs of bars/spaces
    let avg_run = bars_and_spaces.iter().sum::<u32>() as f32 / bars_and_spaces.len() as f32;
    let mut valid_runs = 0;
    
    // ITF allows more variation than other formats
    for &run in bars_and_spaces {
        if run > 0 && (run as f32) < avg_run * 5.0 {
            valid_runs += 1;
        }
    }
    
    // More lenient validation for ITF14
    if valid_runs < bars_and_spaces.len() * 2 / 3 {
        return None;
    }
    
    Some(LinearValidationResult {
        confidence: 0.65, // Lower confidence due to more lenient validation
        start_end_patterns: (vec![1, 1, 1, 1], vec![2, 1, 1]),
    })
}

/// Validate Codabar pattern
fn validate_codabar_pattern(bars_and_spaces: &[u32]) -> Option<LinearValidationResult> {
    // Codabar has wide ratio typically 2:1 or 3:1
    if bars_and_spaces.len() < 15 || bars_and_spaces.len() > 100 {
        return None;
    }
    
    let avg_run = bars_and_spaces.iter().sum::<u32>() as f32 / bars_and_spaces.len() as f32;
    let mut narrow_count = 0;
    let mut wide_count = 0;
    
    for &run in bars_and_spaces {
        if (run as f32) < avg_run * 1.8 {
            narrow_count += 1;
        } else if (run as f32) < avg_run * 4.0 {
            wide_count += 1;
        }
    }
    
    // Codabar should have both narrow and wide elements
    if narrow_count < 8 || wide_count < 4 {
        return None;
    }
    
    Some(LinearValidationResult {
        confidence: 0.7,
        start_end_patterns: (vec![1, 1, 1, 1, 2, 2, 1], vec![1, 1, 1, 1, 2, 2, 1]),
    })
}

/// Validate UPC-A pattern (similar to EAN-13)
fn validate_upca_pattern(bars_and_spaces: &[u32]) -> Option<LinearValidationResult> {
    // UPC-A has similar structure to EAN-13 but slightly different
    if bars_and_spaces.len() < 45 || bars_and_spaces.len() > 65 {
        return None;
    }
    
    // UPC-A guard patterns
    let start_guard = find_guard_pattern(bars_and_spaces, &[1, 1, 1], 0);
    let center_guard = find_guard_pattern(bars_and_spaces, &[1, 1, 1, 1, 1], bars_and_spaces.len() / 3);
    let end_guard = find_guard_pattern(bars_and_spaces, &[1, 1, 1], bars_and_spaces.len() * 2 / 3);
    
    if start_guard.is_none() || center_guard.is_none() || end_guard.is_none() {
        return None;
    }
    
    Some(LinearValidationResult {
        confidence: 0.8,
        start_end_patterns: (vec![1, 1, 1], vec![1, 1, 1]),
    })
}

/// Find guard pattern in bars_and_spaces array
fn find_guard_pattern(bars_and_spaces: &[u32], pattern: &[u32], start_search: usize) -> Option<usize> {
    let search_end = (start_search + 20).min(bars_and_spaces.len());
    
    for i in start_search..search_end {
        if pattern_matches_at_position(bars_and_spaces, pattern, i, 0.3) {
            return Some(i);
        }
    }
    None
}

/// Check if pattern matches at specific position with tolerance
fn pattern_matches_at_position(
    bars_and_spaces: &[u32],
    pattern: &[u32],
    position: usize,
    tolerance: f32
) -> bool {
    if position + pattern.len() > bars_and_spaces.len() {
        return false;
    }
    
    let actual_slice = &bars_and_spaces[position..position + pattern.len()];
    
    // For guard patterns, check if they're roughly equal (ratio-based)
    if pattern.iter().all(|&x| x == pattern[0]) {
        // All elements should be roughly equal
        let avg = actual_slice.iter().sum::<u32>() as f32 / actual_slice.len() as f32;
        for &actual in actual_slice {
            let ratio = actual as f32 / avg;
            if ratio < (1.0 - tolerance) || ratio > (1.0 + tolerance) {
                return false;
            }
        }
        return true;
    }
    
    // Calculate ratio-based comparison for other patterns
    for (actual, expected) in actual_slice.iter().zip(pattern.iter()) {
        if *expected == 0 {
            continue; // Skip zero elements
        }
        
        let ratio = *actual as f32 / *expected as f32;
        if ratio < (1.0 - tolerance) || ratio > (1.0 + tolerance) {
            return false;
        }
    }
    
    true
}

/// Calculate bounding box for linear barcode
fn calculate_linear_bbox(
    scan_line: &ScanLine,
    bars_and_spaces: &[u32],
    angle_degrees: f32
) -> BoundingBox {
    let total_modules: u32 = bars_and_spaces.iter().sum();
    
    // Estimate module width
    let line_length = ((scan_line.end.0 - scan_line.start.0).powi(2) + 
                      (scan_line.end.1 - scan_line.start.1).powi(2)).sqrt();
    let module_width = line_length / total_modules as f32;
    
    // Calculate barcode dimensions
    let barcode_width = line_length;
    let barcode_height = module_width * 20.0; // Linear barcodes are typically 20 modules high
    
    // Calculate center point
    let center_x = (scan_line.start.0 + scan_line.end.0) / 2.0;
    let center_y = (scan_line.start.1 + scan_line.end.1) / 2.0;
    
    // Create bounding box considering rotation
    let half_width = barcode_width / 2.0;
    let half_height = barcode_height / 2.0;
    
    let angle_rad = angle_degrees.to_radians();
    let cos_angle = angle_rad.cos();
    let sin_angle = angle_rad.sin();
    
    // Calculate rotated corners
    let corners = vec![
        (center_x - half_width * cos_angle + half_height * sin_angle,
         center_y - half_width * sin_angle - half_height * cos_angle),
        (center_x + half_width * cos_angle + half_height * sin_angle,
         center_y + half_width * sin_angle - half_height * cos_angle),
        (center_x + half_width * cos_angle - half_height * sin_angle,
         center_y + half_width * sin_angle + half_height * cos_angle),
        (center_x - half_width * cos_angle - half_height * sin_angle,
         center_y - half_width * sin_angle + half_height * cos_angle),
    ];
    
    // Find axis-aligned bounding box
    let min_x = corners.iter().map(|c| c.0).fold(f32::INFINITY, f32::min).max(0.0) as u32;
    let max_x = corners.iter().map(|c| c.0).fold(f32::NEG_INFINITY, f32::max) as u32;
    let min_y = corners.iter().map(|c| c.1).fold(f32::INFINITY, f32::min).max(0.0) as u32;
    let max_y = corners.iter().map(|c| c.1).fold(f32::NEG_INFINITY, f32::max) as u32;
    
    BoundingBox {
        x: min_x,
        y: min_y,
        width: max_x - min_x,
        height: max_y - min_y,
        corners,
    }
}

/// Filter and remove duplicate linear candidates
fn filter_and_deduplicate_linear_candidates(
    mut candidates: Vec<DetectionCandidate>
) -> Vec<DetectionCandidate> {
    // Sort by confidence (derived from pattern validation)
    candidates.sort_by(|a, b| {
        let conf_a = get_candidate_confidence(a);
        let conf_b = get_candidate_confidence(b);
        conf_b.partial_cmp(&conf_a).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    let mut filtered = Vec::new();
    
    for candidate in candidates {
        let mut is_duplicate = false;
        
        // Check if this candidate overlaps significantly with existing ones
        for existing in &filtered {
            if candidates_overlap(&candidate, existing, 0.5) {
                is_duplicate = true;
                break;
            }
        }
        
        if !is_duplicate && get_candidate_confidence(&candidate) > 0.6 {
            filtered.push(candidate);
        }
    }
    
    filtered.truncate(10); // Keep top 10 candidates
    filtered
}

/// Get confidence from candidate pattern data
fn get_candidate_confidence(candidate: &DetectionCandidate) -> f32 {
    match &candidate.pattern_data {
        PatternData::Linear { .. } => 0.7, // Base confidence for linear codes
        _ => 0.0,
    }
}

/// Check if two candidates overlap significantly
fn candidates_overlap(a: &DetectionCandidate, b: &DetectionCandidate, threshold: f32) -> bool {
    let bbox_a = &a.position;
    let bbox_b = &b.position;
    
    let overlap_x = (bbox_a.x.max(bbox_b.x), (bbox_a.x + bbox_a.width).min(bbox_b.x + bbox_b.width));
    let overlap_y = (bbox_a.y.max(bbox_b.y), (bbox_a.y + bbox_a.height).min(bbox_b.y + bbox_b.height));
    
    if overlap_x.0 >= overlap_x.1 || overlap_y.0 >= overlap_y.1 {
        return false; // No overlap
    }
    
    let overlap_area = (overlap_x.1 - overlap_x.0) * (overlap_y.1 - overlap_y.0);
    let area_a = bbox_a.width * bbox_a.height;
    let area_b = bbox_b.width * bbox_b.height;
    let min_area = area_a.min(area_b);
    
    overlap_area as f32 / min_area as f32 > threshold
}

/// Scan line representation
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ScanLine {
    start: (f32, f32),
    end: (f32, f32),
    angle: f32,
}

/// Linear barcode validation result
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct LinearValidationResult {
    confidence: f32,
    start_end_patterns: (Vec<u32>, Vec<u32>),
}
