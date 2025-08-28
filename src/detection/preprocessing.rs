// Advanced Image Preprocessing Module
// Inspired by ZXing preprocessing pipeline with additional robustness

use image::{ImageBuffer, Luma};
use crate::detection::DetectionConfig;

/// Preprocessed image variant for multi-approach detection
#[derive(Debug, Clone)]
pub struct ProcessedImage {
    pub image: ImageBuffer<Luma<u8>, Vec<u8>>,
    pub preprocessing_type: PreprocessingType,
    pub quality_metrics: ImageQualityMetrics,
}

#[derive(Debug, Clone)]
pub enum PreprocessingType {
    Original,
    OtsuThreshold,
    AdaptiveThreshold,
    GradientBased,
    NoiseFiltered,
    ContrastEnhanced,
    EdgeEnhanced,
}

/// Image quality assessment for adaptive processing
#[derive(Debug, Clone)]
pub struct ImageQualityMetrics {
    pub overall_contrast: f32,
    pub noise_level: f32,
    pub edge_density: f32,
    pub brightness_distribution: BrightnessHistogram,
    pub sharpness_score: f32,
}

#[derive(Debug, Clone)]
pub struct BrightnessHistogram {
    pub bins: [u32; 256],
    pub mean: f32,
    pub std_dev: f32,
    pub dynamic_range: f32,
}

/// Main preprocessing pipeline
pub fn preprocess_image(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    _config: &DetectionConfig,
) -> Vec<ProcessedImage> {
    let mut processed_images = Vec::new();
    
    // Analyze image quality first
    let quality_metrics = analyze_image_quality(image);
    
    // Always include original
    processed_images.push(ProcessedImage {
        image: image.clone(),
        preprocessing_type: PreprocessingType::Original,
        quality_metrics: quality_metrics.clone(),
    });
    
    // Apply different preprocessing based on image characteristics
    if quality_metrics.overall_contrast < 0.3 {
        // Low contrast - apply contrast enhancement
        if let Some(enhanced) = enhance_contrast(image) {
            processed_images.push(ProcessedImage {
                image: enhanced,
                preprocessing_type: PreprocessingType::ContrastEnhanced,
                quality_metrics: quality_metrics.clone(),
            });
        }
    }
    
    if quality_metrics.noise_level > 0.4 {
        // High noise - apply noise filtering
        if let Some(filtered) = apply_noise_filter(image) {
            processed_images.push(ProcessedImage {
                image: filtered,
                preprocessing_type: PreprocessingType::NoiseFiltered,
                quality_metrics: quality_metrics.clone(),
            });
        }
    }
    
    // Always try different binarization methods
    if let Some(otsu) = apply_otsu_threshold(image) {
        processed_images.push(ProcessedImage {
            image: otsu,
            preprocessing_type: PreprocessingType::OtsuThreshold,
            quality_metrics: quality_metrics.clone(),
        });
    }
    
    if let Some(adaptive) = apply_adaptive_threshold(image) {
        processed_images.push(ProcessedImage {
            image: adaptive,
            preprocessing_type: PreprocessingType::AdaptiveThreshold,
            quality_metrics: quality_metrics.clone(),
        });
    }
    
    // For edge-heavy images, try edge enhancement
    if quality_metrics.edge_density > 0.6 {
        if let Some(edge_enhanced) = enhance_edges(image) {
            processed_images.push(ProcessedImage {
                image: edge_enhanced,
                preprocessing_type: PreprocessingType::EdgeEnhanced,
                quality_metrics: quality_metrics.clone(),
            });
        }
    }
    
    processed_images
}

/// Comprehensive image quality analysis
pub fn analyze_image_quality(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageQualityMetrics {
    let histogram = calculate_brightness_histogram(image);
    let contrast = calculate_global_contrast(image);
    let noise = estimate_noise_level(image);
    let edge_density = calculate_edge_density(image);
    let sharpness = calculate_sharpness(image);
    
    ImageQualityMetrics {
        overall_contrast: contrast,
        noise_level: noise,
        edge_density,
        brightness_distribution: histogram,
        sharpness_score: sharpness,
    }
}

/// Calculate brightness histogram with statistics
fn calculate_brightness_histogram(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> BrightnessHistogram {
    let mut bins = [0u32; 256];
    let mut sum = 0f32;
    let total_pixels = (image.width() * image.height()) as f32;
    
    for pixel in image.pixels() {
        let value = pixel[0] as usize;
        bins[value] += 1;
        sum += pixel[0] as f32;
    }
    
    let mean = sum / total_pixels;
    
    // Calculate standard deviation
    let mut variance_sum = 0f32;
    for pixel in image.pixels() {
        let diff = pixel[0] as f32 - mean;
        variance_sum += diff * diff;
    }
    let std_dev = (variance_sum / total_pixels).sqrt();
    
    // Calculate dynamic range
    let mut min_val = 255u8;
    let mut max_val = 0u8;
    for pixel in image.pixels() {
        min_val = min_val.min(pixel[0]);
        max_val = max_val.max(pixel[0]);
    }
    let dynamic_range = (max_val - min_val) as f32 / 255.0;
    
    BrightnessHistogram {
        bins,
        mean,
        std_dev,
        dynamic_range,
    }
}

/// Calculate global contrast using RMS contrast
fn calculate_global_contrast(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> f32 {
    let histogram = calculate_brightness_histogram(image);
    let mean = histogram.mean;
    
    let mut contrast_sum = 0f32;
    let total_pixels = (image.width() * image.height()) as f32;
    
    for pixel in image.pixels() {
        let diff = pixel[0] as f32 - mean;
        contrast_sum += diff * diff;
    }
    
    (contrast_sum / total_pixels).sqrt() / 255.0
}

/// Estimate noise level using local variance
fn estimate_noise_level(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> f32 {
    let (width, height) = image.dimensions();
    let mut noise_sum = 0f32;
    let mut count = 0;
    
    // Sample 3x3 neighborhoods throughout the image
    for y in (1..height-1).step_by(10) {
        for x in (1..width-1).step_by(10) {
            let center = image.get_pixel(x, y)[0] as f32;
            let mut local_sum = 0f32;
            
            // Calculate local variance in 3x3 neighborhood
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let px = (x as i32 + dx) as u32;
                    let py = (y as i32 + dy) as u32;
                    let value = image.get_pixel(px, py)[0] as f32;
                    let diff = value - center;
                    local_sum += diff * diff;
                }
            }
            
            noise_sum += local_sum / 9.0;
            count += 1;
        }
    }
    
    if count > 0 {
        (noise_sum / count as f32).sqrt() / 255.0
    } else {
        0.0
    }
}

/// Calculate edge density using Sobel operator
fn calculate_edge_density(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> f32 {
    let (width, height) = image.dimensions();
    let mut edge_count = 0;
    let total_pixels = (width - 2) * (height - 2);
    
    for y in 1..height-1 {
        for x in 1..width-1 {
            let gx = sobel_x(image, x, y);
            let gy = sobel_y(image, x, y);
            let magnitude = (gx * gx + gy * gy).sqrt();
            
            if magnitude > 30.0 { // Edge threshold
                edge_count += 1;
            }
        }
    }
    
    edge_count as f32 / total_pixels as f32
}

/// Calculate image sharpness using Laplacian variance
fn calculate_sharpness(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> f32 {
    let (width, height) = image.dimensions();
    let mut laplacian_sum = 0f32;
    let mut count = 0;
    
    for y in 1..height-1 {
        for x in 1..width-1 {
            let laplacian = laplacian_operator(image, x, y);
            laplacian_sum += laplacian * laplacian;
            count += 1;
        }
    }
    
    if count > 0 {
        laplacian_sum / count as f32
    } else {
        0.0
    }
}

/// Sobel X operator
fn sobel_x(image: &ImageBuffer<Luma<u8>, Vec<u8>>, x: u32, y: u32) -> f32 {
    let kernel = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
    apply_kernel(image, x, y, &kernel)
}

/// Sobel Y operator  
fn sobel_y(image: &ImageBuffer<Luma<u8>, Vec<u8>>, x: u32, y: u32) -> f32 {
    let kernel = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];
    apply_kernel(image, x, y, &kernel)
}

/// Laplacian operator for sharpness
fn laplacian_operator(image: &ImageBuffer<Luma<u8>, Vec<u8>>, x: u32, y: u32) -> f32 {
    let kernel = [[0, -1, 0], [-1, 4, -1], [0, -1, 0]];
    apply_kernel(image, x, y, &kernel)
}

/// Apply 3x3 kernel to image at position
fn apply_kernel(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>, 
    x: u32, 
    y: u32, 
    kernel: &[[i32; 3]; 3]
) -> f32 {
    let mut sum = 0f32;
    
    for dy in -1..=1 {
        for dx in -1..=1 {
            let px = (x as i32 + dx) as u32;
            let py = (y as i32 + dy) as u32;
            let pixel_value = image.get_pixel(px, py)[0] as f32;
            let kernel_value = kernel[(dy + 1) as usize][(dx + 1) as usize] as f32;
            sum += pixel_value * kernel_value;
        }
    }
    
    sum
}

/// Apply Otsu's automatic threshold
pub fn apply_otsu_threshold(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Option<ImageBuffer<Luma<u8>, Vec<u8>>> {
    let histogram = calculate_brightness_histogram(image);
    let threshold = calculate_otsu_threshold(&histogram.bins);
    
    let mut result: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(image.width(), image.height());
    for (src, dst) in image.pixels().zip(result.pixels_mut()) {
        dst[0] = if src[0] > threshold { 255 } else { 0 };
    }
    
    Some(result)
}

/// Calculate optimal threshold using Otsu's method
fn calculate_otsu_threshold(histogram: &[u32; 256]) -> u8 {
    let total: u32 = histogram.iter().sum();
    let mut sum_total = 0f32;
    
    for (i, &count) in histogram.iter().enumerate() {
        sum_total += i as f32 * count as f32;
    }
    
    let mut max_variance = 0f32;
    let mut threshold = 0u8;
    let mut weight_bg = 0f32;
    let mut sum_bg = 0f32;
    
    for t in 0..256 {
        weight_bg += histogram[t] as f32;
        if weight_bg == 0.0 { continue; }
        
        let weight_fg = total as f32 - weight_bg;
        if weight_fg == 0.0 { break; }
        
        sum_bg += t as f32 * histogram[t] as f32;
        
        let mean_bg = sum_bg / weight_bg;
        let mean_fg = (sum_total - sum_bg) / weight_fg;
        
        let variance_between = weight_bg * weight_fg * (mean_bg - mean_fg) * (mean_bg - mean_fg);
        
        if variance_between > max_variance {
            max_variance = variance_between;
            threshold = t as u8;
        }
    }
    
    threshold
}

/// Apply adaptive threshold with local mean
pub fn apply_adaptive_threshold(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Option<ImageBuffer<Luma<u8>, Vec<u8>>> {
    let (width, height) = image.dimensions();
    let mut result: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    let window_size = 15; // Adaptive window size
    let c = 10; // Constant subtracted from mean
    
    for y in 0..height {
        for x in 0..width {
            let local_mean = calculate_local_mean(image, x, y, window_size);
            let pixel_value = image.get_pixel(x, y)[0];
            
            result.get_pixel_mut(x, y)[0] = if pixel_value as f32 > local_mean - c as f32 {
                255
            } else {
                0
            };
        }
    }
    
    Some(result)
}

/// Calculate local mean in window around pixel
fn calculate_local_mean(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>, 
    cx: u32, 
    cy: u32, 
    window_size: u32
) -> f32 {
    let (width, height) = image.dimensions();
    let half_window = window_size / 2;
    let mut sum = 0f32;
    let mut count = 0;
    
    let start_x = cx.saturating_sub(half_window);
    let end_x = (cx + half_window).min(width - 1);
    let start_y = cy.saturating_sub(half_window);
    let end_y = (cy + half_window).min(height - 1);
    
    for y in start_y..=end_y {
        for x in start_x..=end_x {
            sum += image.get_pixel(x, y)[0] as f32;
            count += 1;
        }
    }
    
    if count > 0 { sum / count as f32 } else { 128.0 }
}

/// Enhance contrast using CLAHE (simplified version)
pub fn enhance_contrast(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Option<ImageBuffer<Luma<u8>, Vec<u8>>> {
    // Simplified contrast enhancement - in production would use full CLAHE
    let histogram = calculate_brightness_histogram(image);
    let mut result: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(image.width(), image.height());
    
    // Calculate lookup table for histogram equalization
    let mut cumulative = [0u32; 256];
    cumulative[0] = histogram.bins[0];
    for i in 1..256 {
        cumulative[i] = cumulative[i-1] + histogram.bins[i];
    }
    
    let total_pixels = image.width() * image.height();
    let mut lookup = [0u8; 256];
    for i in 0..256 {
        lookup[i] = ((cumulative[i] as f32 / total_pixels as f32) * 255.0) as u8;
    }
    
    for (src, dst) in image.pixels().zip(result.pixels_mut()) {
        dst[0] = lookup[src[0] as usize];
    }
    
    Some(result)
}

/// Apply noise filtering using median filter
pub fn apply_noise_filter(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Option<ImageBuffer<Luma<u8>, Vec<u8>>> {
    let (width, height) = image.dimensions();
    let mut result: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            let filtered_value = median_filter(image, x, y, 3);
            result.get_pixel_mut(x, y)[0] = filtered_value;
        }
    }
    
    Some(result)
}

/// Apply median filter at pixel
fn median_filter(image: &ImageBuffer<Luma<u8>, Vec<u8>>, cx: u32, cy: u32, size: u32) -> u8 {
    let (width, height) = image.dimensions();
    let half_size = size / 2;
    let mut values = Vec::new();
    
    let start_x = cx.saturating_sub(half_size);
    let end_x = (cx + half_size).min(width - 1);
    let start_y = cy.saturating_sub(half_size);
    let end_y = (cy + half_size).min(height - 1);
    
    for y in start_y..=end_y {
        for x in start_x..=end_x {
            values.push(image.get_pixel(x, y)[0]);
        }
    }
    
    values.sort_unstable();
    values[values.len() / 2]
}

/// Enhance edges using unsharp masking
pub fn enhance_edges(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Option<ImageBuffer<Luma<u8>, Vec<u8>>> {
    let (width, height) = image.dimensions();
    let mut result: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    
    // Apply unsharp masking
    for y in 1..height-1 {
        for x in 1..width-1 {
            let original = image.get_pixel(x, y)[0] as f32;
            let laplacian = laplacian_operator(image, x, y);
            
            // Unsharp mask: original + k * laplacian
            let enhanced = original + 0.5 * laplacian;
            let clamped = enhanced.max(0.0).min(255.0) as u8;
            
            result.get_pixel_mut(x, y)[0] = clamped;
        }
    }
    
    // Copy borders
    for y in 0..height {
        result.get_pixel_mut(0, y)[0] = image.get_pixel(0, y)[0];
        result.get_pixel_mut(width-1, y)[0] = image.get_pixel(width-1, y)[0];
    }
    for x in 0..width {
        result.get_pixel_mut(x, 0)[0] = image.get_pixel(x, 0)[0];
        result.get_pixel_mut(x, height-1)[0] = image.get_pixel(x, height-1)[0];
    }
    
    Some(result)
}
