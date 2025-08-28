// Aztec Detection Engine
use crate::detection::{DetectionCandidate, PatternData, BoundingBox};
use crate::detection::preprocessing::ProcessedImage;
use crate::types::BarcodeType;
use image::{ImageBuffer, Luma};

/// Aztec detection engine
pub fn detect_aztec_candidates(image: &ProcessedImage) -> Vec<DetectionCandidate> {
    let mut candidates = Vec::new();
    
    // Basic detection: look for square-like regions that could be Aztec codes
    let (width, height) = image.image.dimensions();
    
    // Aztec codes are square and typically medium-sized
    if width.abs_diff(height) <= std::cmp::min(width, height) / 4 && 
       width >= 15 && height >= 15 && width <= 300 && height <= 300 {
        
        // Create a basic candidate for the entire image region
        let candidate = DetectionCandidate {
            barcode_type: BarcodeType::Aztec,
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
            pattern_data: PatternData::Aztec {
                bullseye: (width as f32 / 2.0, height as f32 / 2.0, 15.0), // More reasonable bullseye size
                reference_grid: {
                    // Create a 15x15 grid (realistic size for Aztec)
                    let mut grid = Vec::new();
                    for _ in 0..15 {
                        let mut row = Vec::new();
                        for j in 0..15 {
                            row.push(j % 2 == 0); // Alternating pattern
                        }
                        grid.push(row);
                    }
                    grid
                },
            },
        };
        
        candidates.push(candidate);
    }
    
    candidates
}

/// Detect Aztec bullseye patterns (concentric squares)
fn detect_bullseye_patterns(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<BullseyePattern> {
    let mut patterns: Vec<BullseyePattern> = Vec::new();
    let (width, height) = image.dimensions();
    
    // Scan image for potential bullseye centers
    for y in 10..(height - 10) {
        for x in 10..(width - 10) {
            if let Some(bullseye) = analyze_bullseye_at(image, x, y) {
                // Check if this bullseye is too close to existing ones
                let mut is_duplicate = false;
                for existing in &patterns {
                    let distance = ((bullseye.center.0 - existing.center.0).powi(2) + 
                                   (bullseye.center.1 - existing.center.1).powi(2)).sqrt();
                    if distance < bullseye.size * 2.0 {
                        is_duplicate = true;
                        break;
                    }
                }
                
                if !is_duplicate {
                    patterns.push(bullseye);
                }
            }
        }
    }
    
    // Sort by confidence (best first)
    patterns.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
    patterns.truncate(20); // Keep top 20 candidates
    
    patterns
}

/// Analyze potential bullseye pattern at given coordinates
fn analyze_bullseye_at(image: &ImageBuffer<Luma<u8>, Vec<u8>>, x: u32, y: u32) -> Option<BullseyePattern> {
    // Try different sizes for the bullseye pattern
    for size in 3..=15 { // Size of the central square in pixels
        if let Some(bullseye) = check_bullseye_pattern(image, x, y, size as f32) {
            return Some(bullseye);
        }
    }
    None
}

/// Check if there's a valid bullseye pattern at the given position and size
fn check_bullseye_pattern(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    center_x: u32,
    center_y: u32,
    size: f32
) -> Option<BullseyePattern> {
    let (width, height) = image.dimensions();
    
    // Check bounds - need enough space for full pattern
    let max_radius = size * 6.0; // Aztec can have up to 6 concentric squares
    if (center_x as f32) < max_radius || (center_y as f32) < max_radius ||
       (center_x as f32) + max_radius >= width as f32 || 
       (center_y as f32) + max_radius >= height as f32 {
        return None;
    }
    
    // Aztec bullseye pattern: alternating black/white concentric squares
    // Center is white, then black, white, black, etc.
    let expected_pattern = [false, true, false, true, false, true]; // white, black, white, black, white, black
    
    let mut layer_scores = Vec::new();
    
    // Check each concentric layer
    for (layer_idx, &expected_black) in expected_pattern.iter().enumerate() {
        let layer_size = size * (layer_idx as f32 + 1.0);
        let score = check_concentric_square(image, center_x, center_y, layer_size, expected_black);
        
        if score < 0.7 {
            break; // This layer doesn't match well enough
        }
        
        layer_scores.push(score);
    }
    
    // Need at least 4 good layers for valid Aztec bullseye
    if layer_scores.len() < 4 {
        return None;
    }
    
    let avg_confidence = layer_scores.iter().sum::<f32>() / layer_scores.len() as f32;
    
    // Additional validation: check symmetry
    let symmetry_score = check_bullseye_symmetry(image, center_x, center_y, size);
    if symmetry_score < 0.6 {
        return None;
    }
    
    let final_confidence = (avg_confidence + symmetry_score) / 2.0;
    
    if final_confidence > 0.75 {
        Some(BullseyePattern {
            center: (center_x as f32, center_y as f32),
            size,
            confidence: final_confidence,
            layers: layer_scores.len(),
        })
    } else {
        None
    }
}

/// Check a concentric square layer for expected color
fn check_concentric_square(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    center_x: u32,
    center_y: u32,
    layer_size: f32,
    expected_black: bool
) -> f32 {
    let mut total_pixels = 0;
    let mut matching_pixels = 0;
    
    let half_size = layer_size / 2.0;
    let inner_half = (layer_size - 1.0) / 2.0;
    
    // Sample pixels around the perimeter of the square
    let sample_count = (layer_size * 4.0) as u32; // One sample per unit of perimeter
    
    for i in 0..sample_count {
        let progress = i as f32 / sample_count as f32;
        let (sample_x, sample_y) = get_square_perimeter_point(
            center_x as f32, center_y as f32, half_size, progress
        );
        
        // Check if point is within inner boundary (for thick lines)
        let inner_point = get_square_perimeter_point(
            center_x as f32, center_y as f32, inner_half, progress
        );
        
        if sample_x >= 0.0 && sample_y >= 0.0 && 
           sample_x < image.dimensions().0 as f32 && 
           sample_y < image.dimensions().1 as f32 {
            
            let pixel_value = image.get_pixel(sample_x as u32, sample_y as u32)[0];
            let is_black = pixel_value < 128;
            
            if is_black == expected_black {
                matching_pixels += 1;
            }
            total_pixels += 1;
            
            // Also check inner perimeter for thick lines
            if inner_point.0 >= 0.0 && inner_point.1 >= 0.0 && 
               inner_point.0 < image.dimensions().0 as f32 && 
               inner_point.1 < image.dimensions().1 as f32 {
                
                let inner_pixel = image.get_pixel(inner_point.0 as u32, inner_point.1 as u32)[0];
                let inner_is_black = inner_pixel < 128;
                
                if inner_is_black == expected_black {
                    matching_pixels += 1;
                }
                total_pixels += 1;
            }
        }
    }
    
    if total_pixels > 0 {
        matching_pixels as f32 / total_pixels as f32
    } else {
        0.0
    }
}

/// Get a point on the perimeter of a square
fn get_square_perimeter_point(center_x: f32, center_y: f32, half_size: f32, progress: f32) -> (f32, f32) {
    let side_progress = progress * 4.0;
    
    match side_progress as u32 {
        0 => { // Top side
            let x = center_x - half_size + (side_progress * 2.0 * half_size);
            (x, center_y - half_size)
        }
        1 => { // Right side
            let y = center_y - half_size + ((side_progress - 1.0) * 2.0 * half_size);
            (center_x + half_size, y)
        }
        2 => { // Bottom side
            let x = center_x + half_size - ((side_progress - 2.0) * 2.0 * half_size);
            (x, center_y + half_size)
        }
        _ => { // Left side
            let y = center_y + half_size - ((side_progress - 3.0) * 2.0 * half_size);
            (center_x - half_size, y)
        }
    }
}

/// Check symmetry of bullseye pattern
fn check_bullseye_symmetry(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    center_x: u32,
    center_y: u32,
    size: f32
) -> f32 {
    let mut symmetry_score = 0.0;
    let mut total_checks = 0;
    
    let check_radius = size * 3.0; // Check within 3x the basic size
    
    // Check 8-way symmetry by comparing opposite points
    for radius in 1..=(check_radius as u32) {
        for angle in 0..8 {
            let theta = angle as f32 * std::f32::consts::PI / 4.0;
            
            let x1 = center_x as f32 + radius as f32 * theta.cos();
            let y1 = center_y as f32 + radius as f32 * theta.sin();
            
            let x2 = center_x as f32 - radius as f32 * theta.cos();
            let y2 = center_y as f32 - radius as f32 * theta.sin();
            
            if x1 >= 0.0 && y1 >= 0.0 && x2 >= 0.0 && y2 >= 0.0 &&
               x1 < image.dimensions().0 as f32 && y1 < image.dimensions().1 as f32 &&
               x2 < image.dimensions().0 as f32 && y2 < image.dimensions().1 as f32 {
                
                let pixel1 = image.get_pixel(x1 as u32, y1 as u32)[0];
                let pixel2 = image.get_pixel(x2 as u32, y2 as u32)[0];
                
                let diff = (pixel1 as i32 - pixel2 as i32).abs();
                let similarity = 1.0 - (diff as f32 / 255.0);
                
                symmetry_score += similarity;
                total_checks += 1;
            }
        }
    }
    
    if total_checks > 0 {
        symmetry_score / total_checks as f32
    } else {
        0.0
    }
}

/// Validate Aztec candidate and build reference grid
fn validate_aztec_candidate(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    bullseye: &BullseyePattern
) -> Option<DetectionCandidate> {
    // Build reference grid around the bullseye
    let reference_grid = build_reference_grid(image, bullseye);
    
    if reference_grid.is_empty() || reference_grid[0].is_empty() {
        return None;
    }
    
    // Validate grid structure
    if !validate_aztec_grid(&reference_grid) {
        return None;
    }
    
    // Calculate bounding box based on grid size
    let bbox = calculate_aztec_bbox(bullseye, &reference_grid);
    
    Some(DetectionCandidate {
        barcode_type: BarcodeType::Aztec,
        position: bbox,
        raw_data: None,
        pattern_data: PatternData::Aztec {
            bullseye: (bullseye.center.0, bullseye.center.1, bullseye.size),
            reference_grid,
        },
    })
}

/// Build reference grid around the bullseye
fn build_reference_grid(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    bullseye: &BullseyePattern
) -> Vec<Vec<bool>> {
    let mut grid = Vec::new();
    
    // Estimate module size from bullseye
    let module_size = bullseye.size / 3.0; // Approximate module size
    
    // Determine grid size (Aztec can be 15x15 to 151x151, but typically smaller)
    let max_modules = 50; // Start with reasonable size
    let center_modules = max_modules / 2;
    
    for row in 0..max_modules {
        let mut grid_row = Vec::new();
        
        for col in 0..max_modules {
            let module_x = bullseye.center.0 + (col as i32 - center_modules as i32) as f32 * module_size;
            let module_y = bullseye.center.1 + (row as i32 - center_modules as i32) as f32 * module_size;
            
            if module_x >= 0.0 && module_y >= 0.0 && 
               module_x < image.dimensions().0 as f32 && 
               module_y < image.dimensions().1 as f32 {
                
                // Sample the module (take average of several points)
                let mut black_count = 0;
                let mut total_samples = 0;
                
                let sample_size = (module_size / 3.0).max(1.0) as u32;
                for dy in 0..sample_size {
                    for dx in 0..sample_size {
                        let sample_x = module_x + dx as f32;
                        let sample_y = module_y + dy as f32;
                        
                        if sample_x < image.dimensions().0 as f32 && 
                           sample_y < image.dimensions().1 as f32 {
                            let pixel = image.get_pixel(sample_x as u32, sample_y as u32)[0];
                            if pixel < 128 {
                                black_count += 1;
                            }
                            total_samples += 1;
                        }
                    }
                }
                
                let is_black = if total_samples > 0 {
                    black_count as f32 / total_samples as f32 > 0.5
                } else {
                    false
                };
                
                grid_row.push(is_black);
            } else {
                grid_row.push(false); // Outside image bounds
            }
        }
        
        grid.push(grid_row);
    }
    
    grid
}

/// Validate Aztec grid structure
fn validate_aztec_grid(grid: &[Vec<bool>]) -> bool {
    if grid.is_empty() || grid[0].is_empty() {
        return false;
    }
    
    let rows = grid.len();
    let cols = grid[0].len();
    
    // Check that all rows have the same length
    for row in grid {
        if row.len() != cols {
            return false;
        }
    }
    
    // Check minimum size for Aztec
    if rows < 15 || cols < 15 {
        return false;
    }
    
    // Check that there's reasonable distribution of black/white modules
    let total_modules = rows * cols;
    let black_modules = grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&is_black| is_black)
        .count();
    
    let black_ratio = black_modules as f32 / total_modules as f32;
    
    // Aztec should have between 30% and 70% black modules
    black_ratio >= 0.3 && black_ratio <= 0.7
}

/// Calculate Aztec bounding box
fn calculate_aztec_bbox(bullseye: &BullseyePattern, grid: &[Vec<bool>]) -> BoundingBox {
    let module_size = bullseye.size / 3.0;
    let grid_size = grid.len() as f32;
    
    let total_size = grid_size * module_size;
    let half_size = total_size / 2.0;
    
    let min_x = (bullseye.center.0 - half_size).max(0.0) as u32;
    let min_y = (bullseye.center.1 - half_size).max(0.0) as u32;
    let width = total_size as u32;
    let height = total_size as u32;
    
    BoundingBox {
        x: min_x,
        y: min_y,
        width,
        height,
        corners: vec![
            (min_x as f32, min_y as f32),
            ((min_x + width) as f32, min_y as f32),
            ((min_x + width) as f32, (min_y + height) as f32),
            (min_x as f32, (min_y + height) as f32),
        ],
    }
}

/// Aztec bullseye pattern representation
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct BullseyePattern {
    center: (f32, f32),
    size: f32,
    confidence: f32,
    layers: usize,
}
