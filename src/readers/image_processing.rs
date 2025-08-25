use image::{DynamicImage, GrayImage, ImageBuffer, Luma};
use imageproc::{
    contrast::adaptive_threshold,
    edges::canny,
    filter::gaussian_blur_f32,
    geometric_transformations::{rotate_about_center, Interpolation},
    morphology::{dilate, erode},
};
use anyhow::Result;

/// Prepara a imagem para detecção de códigos
pub(crate) fn prepare_image(image: &DynamicImage) -> Result<GrayImage> {
    // Converter para escala de cinza
    let gray = image.to_luma8();

    // Aplicar blur gaussiano para reduzir ruído
    let blurred = gaussian_blur_f32(&gray, 1.0);

    // Binarização adaptativa
    let binary = adaptive_threshold(&blurred, 11);

    // Detecção de bordas
    let edges = canny(&binary, 50.0, 100.0);

    // Dilatação para conectar componentes próximos
    let dilated = dilate(&edges, imageproc::morphology::Kernel::new_diamond(1));

    // Erosão para remover ruído
    let processed = erode(&dilated, imageproc::morphology::Kernel::new_diamond(1));

    Ok(processed)
}

/// Detecta a orientação da imagem e corrige se necessário
pub(crate) fn correct_orientation(image: &GrayImage) -> Result<GrayImage> {
    // TODO: Implementar detecção de orientação usando transformada de Hough
    // Por enquanto, retorna a imagem original
    Ok(image.clone())
}

/// Encontra regiões que podem conter códigos de barras
pub(crate) fn find_regions(image: &GrayImage) -> Result<Vec<Region>> {
    let mut regions = Vec::new();
    
    // TODO: Implementar detecção de regiões usando análise de componentes conectados
    // Por enquanto, retorna a imagem inteira como uma região
    regions.push(Region {
        x: 0,
        y: 0,
        width: image.width(),
        height: image.height(),
        angle: 0.0,
        confidence: 1.0,
    });

    Ok(regions)
}

/// Representa uma região da imagem que pode conter um código
#[derive(Debug, Clone)]
pub(crate) struct Region {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub angle: f32,
    pub confidence: f32,
}

impl Region {
    /// Extrai a região da imagem original
    pub fn extract(&self, image: &GrayImage) -> Result<GrayImage> {
        let mut region = ImageBuffer::new(self.width, self.height);
        
        // Copiar pixels da região
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(pixel) = image.get_pixel_checked(self.x + x, self.y + y) {
                    region.put_pixel(x, y, *pixel);
                }
            }
        }

        // Rotacionar se necessário
        if self.angle != 0.0 {
            region = rotate_about_center(
                &region,
                self.angle.to_radians(),
                Interpolation::Bilinear,
                Luma([0u8]),
            );
        }

        Ok(region)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::GrayImage;

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
            angle: 0.0,
            confidence: 1.0,
        };

        let extracted = region.extract(&image).unwrap();
        assert_eq!(extracted.width(), 50);
        assert_eq!(extracted.height(), 50);
    }

    #[test]
    fn test_region_extract_with_rotation() {
        let image = GrayImage::new(100, 100);
        let region = Region {
            x: 10,
            y: 10,
            width: 50,
            height: 50,
            angle: 45.0,
            confidence: 1.0,
        };

        let extracted = region.extract(&image).unwrap();
        assert!(extracted.width() > 0);
        assert!(extracted.height() > 0);
    }
}
