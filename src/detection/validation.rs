// Multi-Stage Validation and Anti-False Positive System
// Comprehensive filtering pipeline to achieve 90%+ accuracy

use crate::detection::{DetectionCandidate, AdvancedDetectionResult, DetectionConfig, BoundingBox};
use crate::detection::confidence::{ConfidenceScorer, ConfidenceScore};
use crate::detection::preprocessing::ImageQualityMetrics;

/// Multi-stage validation pipeline
pub struct ValidationPipeline;

impl ValidationPipeline {
    /// Complete validation and scoring pipeline
    pub fn validate_and_score(
        candidate: DetectionCandidate,
        config: &DetectionConfig,
    ) -> Option<AdvancedDetectionResult> {
        // Stage 1: Quick rejection filters
        if !Self::pre_filter_candidate(&candidate) {
            return None;
        }
        
        // Stage 2: Comprehensive confidence scoring
        let confidence_score = ConfidenceScorer::calculate_confidence(&candidate);
        
        // Stage 3: Threshold filtering
        if confidence_score.overall < config.min_confidence {
            return None;
        }
        
        // Stage 4: Content validation (try to decode)
        let validated_data = Self::validate_content(&candidate, &confidence_score)?;
        
        // Stage 5: Final contextual validation
        if !Self::final_validation(&candidate, &confidence_score, config) {
            return None;
        }
        
        Some(AdvancedDetectionResult {
            data: validated_data,
            barcode_type: candidate.barcode_type,
            confidence: confidence_score.overall,
            geometric_score: confidence_score.geometric,
            pattern_score: confidence_score.pattern,
            content_score: confidence_score.content,
            position: candidate.position,
        })
    }
    
    /// Stage 1: Quick pre-filtering to reject obvious false positives
    fn pre_filter_candidate(candidate: &DetectionCandidate) -> bool {
        // Basic sanity checks
        if candidate.position.width == 0 || candidate.position.height == 0 {
            return false;
        }
        
        // Size constraints
        if candidate.position.width < 10 || candidate.position.height < 10 {
            return false;
        }
        
        if candidate.position.width > 5000 || candidate.position.height > 5000 {
            return false;
        }
        
        // Aspect ratio constraints based on barcode type
        let aspect_ratio = candidate.position.width as f32 / candidate.position.height as f32;
        
        match candidate.barcode_type {
            // 2D codes should be roughly square
            crate::types::BarcodeType::QRCode | 
            crate::types::BarcodeType::DataMatrix | 
            crate::types::BarcodeType::Aztec => {
                if aspect_ratio < 0.3 || aspect_ratio > 3.0 {
                    return false;
                }
            },
            // PDF417 can be more rectangular
            crate::types::BarcodeType::PDF417 => {
                if aspect_ratio < 0.1 || aspect_ratio > 10.0 {
                    return false;
                }
            },
            // 1D codes should be wider than tall
            _ => {
                if aspect_ratio < 1.5 || aspect_ratio > 20.0 {
                    return false;
                }
            }
        }
        
        true
    }
    
    /// Stage 4: Content validation through decoding attempt
    fn validate_content(
        candidate: &DetectionCandidate, 
        confidence_score: &ConfidenceScore
    ) -> Option<String> {
        // If we already have extracted data with good content score, use it
        if let Some(ref data) = candidate.raw_data {
            if confidence_score.content > 0.7 {
                return Some(data.clone());
            }
        }
        
        // Otherwise, attempt decoding based on pattern data
        match &candidate.pattern_data {
            crate::detection::PatternData::QRCode { .. } => {
                // Use existing QR decoder
                Self::decode_qr_from_pattern(candidate)
            },
            crate::detection::PatternData::DataMatrix { .. } => {
                // Use existing DataMatrix decoder
                Self::decode_datamatrix_from_pattern(candidate)
            },
            crate::detection::PatternData::Linear { .. } => {
                // Use existing linear decoders
                Self::decode_linear_from_pattern(candidate)
            },
            crate::detection::PatternData::PDF417 { .. } => {
                // Use existing PDF417 decoder
                Self::decode_pdf417_from_pattern(candidate)
            },
            crate::detection::PatternData::Aztec { .. } => {
                // Use existing Aztec decoder
                Self::decode_aztec_from_pattern(candidate)
            },
        }
    }
    
    /// Stage 5: Final contextual validation
    fn final_validation(
        candidate: &DetectionCandidate,
        confidence_score: &ConfidenceScore,
        _config: &DetectionConfig,
    ) -> bool {
        // Ensure minimum scores in critical areas
        if confidence_score.geometric < 0.5 {
            return false;
        }
        
        if confidence_score.pattern < 0.4 {
            return false;
        }
        
        // Format-specific validation
        match candidate.barcode_type {
            crate::types::BarcodeType::QRCode => {
                // QR codes need high pattern integrity
                confidence_score.pattern >= 0.6
            },
            crate::types::BarcodeType::DataMatrix => {
                // DataMatrix needs good geometric score
                confidence_score.geometric >= 0.6
            },
            crate::types::BarcodeType::EAN13 | 
            crate::types::BarcodeType::UPCA |
            crate::types::BarcodeType::ITF14 => {
                // These need valid checksums
                confidence_score.content >= 0.9
            },
            _ => true,
        }
    }
    
    /// Non-maximum suppression to eliminate duplicate detections
    pub fn non_maximum_suppression(
        mut results: Vec<AdvancedDetectionResult>,
        config: &DetectionConfig,
    ) -> Vec<AdvancedDetectionResult> {
        if results.is_empty() {
            return results;
        }
        
        // Sort by confidence score (highest first)
        results.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        
        let mut final_results = Vec::new();
        let mut suppressed = vec![false; results.len()];
        
        for i in 0..results.len() {
            if suppressed[i] {
                continue;
            }
            
            final_results.push(results[i].clone());
            
            // Suppress overlapping detections with lower confidence
            for j in (i+1)..results.len() {
                if suppressed[j] {
                    continue;
                }
                
                let overlap = Self::calculate_overlap(&results[i].position, &results[j].position);
                
                // Suppress if significant overlap (>50%) and same type
                if overlap > 0.5 && results[i].barcode_type == results[j].barcode_type {
                    suppressed[j] = true;
                }
                
                // Also suppress if very high overlap regardless of type
                if overlap > 0.8 {
                    suppressed[j] = true;
                }
            }
            
            // Respect max codes limit
            if final_results.len() >= config.max_codes_per_image {
                break;
            }
        }
        
        final_results
    }
    
    /// Calculate overlap ratio between two bounding boxes
    fn calculate_overlap(box1: &BoundingBox, box2: &BoundingBox) -> f32 {
        let x1 = box1.x.max(box2.x);
        let y1 = box1.y.max(box2.y);
        let x2 = (box1.x + box1.width).min(box2.x + box2.width);
        let y2 = (box1.y + box1.height).min(box2.y + box2.height);
        
        if x2 <= x1 || y2 <= y1 {
            return 0.0; // No overlap
        }
        
        let intersection = (x2 - x1) * (y2 - y1);
        let area1 = box1.width * box1.height;
        let area2 = box2.width * box2.height;
        let union = area1 + area2 - intersection;
        
        if union == 0 {
            return 0.0;
        }
        
        intersection as f32 / union as f32
    }
    
    /// Contextual analysis for adaptive thresholding
    pub fn analyze_detection_context(
        results: &[AdvancedDetectionResult],
        image_quality: &ImageQualityMetrics,
    ) -> DetectionContext {
        let mut context = DetectionContext::default();
        
        // Analyze detection density
        context.detection_density = results.len() as f32 / 100.0; // Normalized per 100 codes
        
        // Analyze confidence distribution
        if !results.is_empty() {
            let total_confidence: f32 = results.iter().map(|r| r.confidence).sum();
            context.avg_confidence = total_confidence / results.len() as f32;
            
            let max_confidence = results.iter()
                .map(|r| r.confidence)
                .fold(0.0f32, f32::max);
            context.max_confidence = max_confidence;
        }
        
        // Use image quality metrics
        context.image_noise_level = image_quality.noise_level;
        context.image_contrast = image_quality.overall_contrast;
        context.image_sharpness = image_quality.sharpness_score;
        
        // Suggest adaptive threshold
        context.suggested_threshold = Self::calculate_adaptive_threshold(&context);
        
        context
    }
    
    /// Calculate adaptive threshold based on context
    fn calculate_adaptive_threshold(context: &DetectionContext) -> f32 {
        let mut threshold: f32 = 0.85; // Base threshold
        
        // Adjust for image quality
        if context.image_noise_level > 0.5 {
            threshold += 0.05; // Be more strict with noisy images
        }
        
        if context.image_contrast < 0.3 {
            threshold += 0.03; // Be more strict with low contrast
        }
        
        if context.image_sharpness < 0.3 {
            threshold += 0.02; // Be more strict with blurry images
        }
        
        // Adjust for detection patterns
        if context.detection_density > 0.1 {
            threshold += 0.02; // Be more strict when many detections
        }
        
        if context.avg_confidence < 0.7 {
            threshold -= 0.02; // Be less strict if overall confidence is low
        }
        
        threshold.min(0.95).max(0.75) // Clamp to reasonable range
    }
    
    // Decoder integration methods (placeholders for now)
    fn decode_qr_from_pattern(candidate: &DetectionCandidate) -> Option<String> {
        // Integrate with existing QR decoder
        candidate.raw_data.clone()
    }
    
    fn decode_datamatrix_from_pattern(candidate: &DetectionCandidate) -> Option<String> {
        // Integrate with existing DataMatrix decoder
        candidate.raw_data.clone()
    }
    
    fn decode_linear_from_pattern(candidate: &DetectionCandidate) -> Option<String> {
        // Integrate with existing linear decoders
        candidate.raw_data.clone()
    }
    
    fn decode_pdf417_from_pattern(candidate: &DetectionCandidate) -> Option<String> {
        // Integrate with existing PDF417 decoder
        candidate.raw_data.clone()
    }
    
    fn decode_aztec_from_pattern(candidate: &DetectionCandidate) -> Option<String> {
        // Integrate with existing Aztec decoder
        candidate.raw_data.clone()
    }
}

/// Context information for adaptive detection
#[derive(Debug, Clone)]
pub struct DetectionContext {
    pub detection_density: f32,
    pub avg_confidence: f32,
    pub max_confidence: f32,
    pub image_noise_level: f32,
    pub image_contrast: f32,
    pub image_sharpness: f32,
    pub suggested_threshold: f32,
}

impl Default for DetectionContext {
    fn default() -> Self {
        Self {
            detection_density: 0.0,
            avg_confidence: 0.0,
            max_confidence: 0.0,
            image_noise_level: 0.0,
            image_contrast: 1.0,
            image_sharpness: 1.0,
            suggested_threshold: 0.85,
        }
    }
}

/// Public validation interface
pub fn validate_and_score(
    candidate: DetectionCandidate,
    config: &DetectionConfig,
) -> Option<AdvancedDetectionResult> {
    ValidationPipeline::validate_and_score(candidate, config)
}

/// Public non-maximum suppression interface  
pub fn non_maximum_suppression(
    results: Vec<AdvancedDetectionResult>,
    config: &DetectionConfig,
) -> Vec<AdvancedDetectionResult> {
    ValidationPipeline::non_maximum_suppression(results, config)
}
