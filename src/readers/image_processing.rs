use anyhow::Result;
use image::{DynamicImage, GrayImage, Luma};
use std::f32::consts::PI;

#[cfg(feature = "readers")]
use imageproc::{
    contrast::threshold,
    filter::gaussian_blur_f32,
    geometric_transformations::{rotate_about_center, Interpolation, warp, Projection},
    edges::canny,
    hough::{detect_lines, LineDetectionOptions},
    corners::{corners_fast9, Corner},
};

#[derive(Debug, Clone)]
pub(crate) struct Region {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Region {
    #[allow(dead_code)]
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }

    pub fn extract(&self, image: &GrayImage) -> Result<GrayImage> {
        let mut region_image = GrayImage::new(self.width, self.height);
        
        for y in 0..self.height {
            for x in 0..self.width {
                let src_x = self.x + x;
                let src_y = self.y + y;
                
                if src_x < image.width() && src_y < image.height() {
                    let pixel = image.get_pixel(src_x, src_y);
                    region_image.put_pixel(x, y, *pixel);
                }
            }
        }
        
        Ok(region_image)
    }
}

/// Prepara a imagem para detecção de códigos
pub(crate) fn prepare_image(image: &DynamicImage) -> Result<GrayImage> {
    // Converter para escala de cinza
    let gray = image.to_luma8();

    #[cfg(feature = "readers")]
    {
        // Aplicar blur gaussiano para reduzir ruído
        let blurred = gaussian_blur_f32(&gray, 1.0);

        // Binarização usando threshold simples
        let binary = threshold(&blurred, 128);

        Ok(binary)
    }

    #[cfg(not(feature = "readers"))]
    {
        Ok(gray)
    }
}

/// Corrige a orientação da imagem detectando rotação automática
#[cfg(feature = "readers")]
pub(crate) fn correct_orientation(image: &GrayImage) -> Result<GrayImage> {
    // Detectar rotação baseada em linhas dominantes
    let angle = detect_rotation_angle(image)?;
    
    if angle.abs() > 1.0 {
        // Aplicar rotação se o ângulo for significativo (> 1 grau)
        let rotated = rotate_about_center(image, angle, Interpolation::Bilinear, Luma([255]));
        Ok(rotated)
    } else {
        Ok(image.clone())
    }
}

/// Detecta o ângulo de rotação da imagem analisando linhas dominantes
#[cfg(feature = "readers")]
fn detect_rotation_angle(image: &GrayImage) -> Result<f32> {
    // Aplicar filtro de detecção de bordas
    let edges = canny(image, 50.0, 100.0);
    
    // Detectar linhas usando transformada de Hough
    let options = LineDetectionOptions {
        vote_threshold: 40,
        suppression_radius: 8,
    };
    
    let lines = detect_lines(&edges, options);
    
    if lines.is_empty() {
        return Ok(0.0);
    }
    
    // Analisar ângulos das linhas para encontrar orientação dominante
    let mut angles = Vec::new();
    
    for line in &lines {
        // Converter ângulo polar para graus
        let angle_deg = line.angle_in_degrees as f32;
        
        // Normalizar ângulo para [-45, 45] graus
        let normalized_angle = normalize_angle_for_correction(angle_deg);
        angles.push(normalized_angle);
    }
    
    if angles.is_empty() {
        return Ok(0.0);
    }
    
    // Encontrar ângulo mais comum (moda aproximada)
    angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    // Usar mediana como ângulo de correção
    let median_angle = if angles.len() % 2 == 0 {
        (angles[angles.len() / 2 - 1] + angles[angles.len() / 2]) / 2.0
    } else {
        angles[angles.len() / 2]
    };
    
    Ok(-median_angle) // Negativo para corrigir a rotação
}

/// Normaliza ângulo para correção de rotação
fn normalize_angle_for_correction(angle: f32) -> f32 {
    let mut normalized = angle % 180.0;
    
    // Converter para range [-90, 90]
    if normalized > 90.0 {
        normalized -= 180.0;
    } else if normalized < -90.0 {
        normalized += 180.0;
    }
    
    // Limitar a [-45, 45] para correções menores
    if normalized > 45.0 {
        normalized -= 90.0;
    } else if normalized < -45.0 {
        normalized += 90.0;
    }
    
    normalized
}

/// Corrige distorção de perspectiva detectando cantos de códigos retangulares
#[cfg(feature = "readers")]
pub(crate) fn correct_perspective(image: &GrayImage) -> Result<GrayImage> {
    // Detectar cantos candidatos
    let corners = detect_barcode_corners(image)?;
    
    if corners.len() >= 4 {
        // Ordenar cantos para formar retângulo
        let ordered_corners = order_corners_clockwise(&corners);
        
        if let Some(corners) = ordered_corners {
            // Calcular transformação de perspectiva
            return apply_perspective_correction(image, &corners);
        }
    }
    
    // Se não conseguir detectar perspectiva, retornar imagem original
    Ok(image.clone())
}

/// Detecta cantos que podem pertencer a códigos de barras
#[cfg(feature = "readers")]
fn detect_barcode_corners(image: &GrayImage) -> Result<Vec<Corner>> {
    // Aplicar detecção de cantos FAST
    let corners = corners_fast9(image, 12);
    
    // Filtrar cantos baseado em contexto local
    let mut filtered_corners = Vec::new();
    
    for corner in corners {
        if is_corner_in_barcode_context(image, &corner) {
            filtered_corners.push(corner);
        }
    }
    
    Ok(filtered_corners)
}

/// Verifica se um canto está em contexto de código de barras
fn is_corner_in_barcode_context(image: &GrayImage, corner: &Corner) -> bool {
    let x = corner.x;
    let y = corner.y;
    
    // Verificar se está dentro dos limites
    if x < 10 || y < 10 || x >= image.width() - 10 || y >= image.height() - 10 {
        return false;
    }
    
    // Analisar região local ao redor do canto
    let region_size = 20;
    let mut light_pixels = 0;
    let mut dark_pixels = 0;
    
    for dy in 0..region_size {
        for dx in 0..region_size {
            let px = x + dx - region_size / 2;
            let py = y + dy - region_size / 2;
            
            if px < image.width() && py < image.height() {
                let pixel = image.get_pixel(px, py)[0];
                if pixel > 128 {
                    light_pixels += 1;
                } else {
                    dark_pixels += 1;
                }
            }
        }
    }
    
    // Considerar como candidato se há contraste significativo
    let total_pixels = light_pixels + dark_pixels;
    let contrast_ratio = (light_pixels as f32) / (total_pixels as f32);
    
    // Código de barras deve ter contraste entre 20% e 80%
    contrast_ratio > 0.2 && contrast_ratio < 0.8
}

/// Ordena cantos em sentido horário para formar retângulo
fn order_corners_clockwise(corners: &[Corner]) -> Option<[Corner; 4]> {
    if corners.len() < 4 {
        return None;
    }
    
    // Encontrar centro aproximado
    let center_x = corners.iter().map(|c| c.x as f32).sum::<f32>() / corners.len() as f32;
    let center_y = corners.iter().map(|c| c.y as f32).sum::<f32>() / corners.len() as f32;
    
    // Classificar cantos por quadrante
    let mut top_left: Option<Corner> = None;
    let mut top_right: Option<Corner> = None;
    let mut bottom_left: Option<Corner> = None;
    let mut bottom_right: Option<Corner> = None;
    
    for corner in corners {
        let is_left = (corner.x as f32) < center_x;
        let is_top = (corner.y as f32) < center_y;
        
        match (is_left, is_top) {
            (true, true) => {
                if top_left.is_none() || corner.x + corner.y < top_left.unwrap().x + top_left.unwrap().y {
                    top_left = Some(*corner);
                }
            }
            (false, true) => {
                if top_right.is_none() || (corner.x as i32 - corner.y as i32) > (top_right.unwrap().x as i32 - top_right.unwrap().y as i32) {
                    top_right = Some(*corner);
                }
            }
            (true, false) => {
                if bottom_left.is_none() || (corner.y as i32 - corner.x as i32) > (bottom_left.unwrap().y as i32 - bottom_left.unwrap().x as i32) {
                    bottom_left = Some(*corner);
                }
            }
            (false, false) => {
                if bottom_right.is_none() || corner.x + corner.y > bottom_right.unwrap().x + bottom_right.unwrap().y {
                    bottom_right = Some(*corner);
                }
            }
        }
    }
    
    // Verificar se encontramos os 4 cantos
    if let (Some(tl), Some(tr), Some(bl), Some(br)) = (top_left, top_right, bottom_left, bottom_right) {
        Some([tl, tr, br, bl]) // Sentido horário
    } else {
        None
    }
}

/// Aplica correção de perspectiva usando os 4 cantos detectados
#[cfg(feature = "readers")]
fn apply_perspective_correction(image: &GrayImage, corners: &[Corner; 4]) -> Result<GrayImage> {
    // Definir dimensões do retângulo de saída
    let output_width = 400;
    let output_height = 400;
    
    // Pontos de destino (retângulo perfeito)
    let dst_corners = [
        (0.0, 0.0),                                          // top-left
        (output_width as f32, 0.0),                         // top-right  
        (output_width as f32, output_height as f32),        // bottom-right
        (0.0, output_height as f32),                        // bottom-left
    ];
    
    // Pontos de origem (cantos detectados)
    let src_corners = [
        (corners[0].x as f32, corners[0].y as f32),
        (corners[1].x as f32, corners[1].y as f32),
        (corners[2].x as f32, corners[2].y as f32), 
        (corners[3].x as f32, corners[3].y as f32),
    ];
    
    // Calcular transformação de perspectiva
    if let Some(projection) = Projection::from_control_points(src_corners, dst_corners) {
        let corrected = warp(image, &projection, Interpolation::Bilinear, Luma([255]));
        Ok(corrected)
    } else {
        // Se não conseguir calcular projeção, retornar original
        Ok(image.clone())
    }
}

/// Detecta e corrige múltiplas orientações de códigos na mesma imagem
#[cfg(feature = "readers")]
pub(crate) fn detect_and_correct_multiple_orientations(image: &GrayImage) -> Result<Vec<GrayImage>> {
    let mut corrected_images = Vec::new();
    
    // Imagem original
    corrected_images.push(image.clone());
    
    // Rotações comuns: 90°, 180°, 270°
    let rotation_angles = [90.0, 180.0, 270.0];
    
    for &angle in &rotation_angles {
        let radians = angle * PI / 180.0;
        let rotated = rotate_about_center(image, radians, Interpolation::Bilinear, Luma([255]));
        corrected_images.push(rotated);
    }
    
    // Correção de perspectiva na imagem original
    let perspective_corrected = correct_perspective(image)?;
    if !images_are_similar(image, &perspective_corrected) {
        corrected_images.push(perspective_corrected);
    }
    
    Ok(corrected_images)
}

/// Verifica se duas imagens são similares (para evitar duplicatas)
fn images_are_similar(img1: &GrayImage, img2: &GrayImage) -> bool {
    if img1.width() != img2.width() || img1.height() != img2.height() {
        return false;
    }
    
    let total_pixels = (img1.width() * img1.height()) as f32;
    let mut different_pixels = 0;
    
    for y in 0..img1.height() {
        for x in 0..img1.width() {
            let pixel1 = img1.get_pixel(x, y)[0];
            let pixel2 = img2.get_pixel(x, y)[0];
            
            if (pixel1 as i32 - pixel2 as i32).abs() > 10 {
                different_pixels += 1;
            }
        }
    }
    
    let difference_ratio = different_pixels as f32 / total_pixels;
    difference_ratio < 0.05 // Menos de 5% de diferença
}

/// Encontra regiões que podem conter códigos de barras
pub(crate) fn find_regions(image: &GrayImage) -> Result<Vec<Region>> {
    // Por enquanto, retorna a imagem inteira como uma região
    // TODO: Implementar detecção de regiões usando análise de componentes conectados
    let regions = vec![Region {
        x: 0,
        y: 0,
        width: image.width(),
        height: image.height(),
    }];

    Ok(regions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GrayImage, Luma};

    #[test]
    fn test_prepare_image() {
        // Criar imagem de teste
        let mut image = GrayImage::new(100, 100);
        for y in 0..100 {
            for x in 0..100 {
                image.put_pixel(x, y, Luma([128u8]));
            }
        }

        let dynamic = DynamicImage::ImageLuma8(image);
        let result = prepare_image(&dynamic);
        assert!(result.is_ok());
    }

    #[test]
    fn test_find_regions() {
        let image = GrayImage::new(100, 100);
        let regions = find_regions(&image).unwrap();
        assert!(!regions.is_empty());

        let region = &regions[0];
        assert_eq!(region.width, 100);
        assert_eq!(region.height, 100);
    }

    #[test]
    fn test_region_extract() {
        let image = GrayImage::new(100, 100);
        let region = Region {
            x: 10,
            y: 10,
            width: 50,
            height: 50,
        };

        let extracted = region.extract(&image).unwrap();
        assert_eq!(extracted.width(), 50);
        assert_eq!(extracted.height(), 50);
    }
}
