use anyhow::Result;
use image::{DynamicImage, GrayImage, ImageBuffer, Luma};

#[cfg(feature = "readers")]
use imageproc::{
    contrast::threshold,
    filter::gaussian_blur_f32,
};

#[derive(Debug, Clone)]
pub(crate) struct Region {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Region {
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

/// Detecta a orientação da imagem e corrige se necessário
pub(crate) fn correct_orientation(image: &GrayImage) -> Result<GrayImage> {
    // TODO: Implementar detecção de orientação usando transformada de Hough
    // Por enquanto, retorna a imagem original
    Ok(image.clone())
}

/// Encontra regiões que podem conter códigos de barras
pub(crate) fn find_regions(image: &GrayImage) -> Result<Vec<Region>> {
    let mut regions = Vec::new();

    // Por enquanto, retorna a imagem inteira como uma região
    // TODO: Implementar detecção de regiões usando análise de componentes conectados
    regions.push(Region {
        x: 0,
        y: 0,
        width: image.width(),
        height: image.height(),
    });

    Ok(regions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, GrayImage, Luma, RgbImage};

    #[test]
    fn test_region_extract() {
        let mut image = GrayImage::new(100, 100);
        // Preencher uma região específica
        for x in 10..20 {
            for y in 10..20 {
                image.put_pixel(x, y, Luma([255]));
            }
        }

        let region = Region::new(10, 10, 10, 10);
        let extracted = region.extract(&image).unwrap();
        
        assert_eq!(extracted.width(), 10);
        assert_eq!(extracted.height(), 10);
        assert_eq!(extracted.get_pixel(0, 0)[0], 255);
    }

    #[test]
    fn test_prepare_image() {
        let rgb_image = RgbImage::new(100, 100);
        let dynamic_image = DynamicImage::ImageRgb8(rgb_image);
        
        let result = prepare_image(&dynamic_image);
        assert!(result.is_ok());
        
        let gray = result.unwrap();
        assert_eq!(gray.width(), 100);
        assert_eq!(gray.height(), 100);
    }

    #[test]
    fn test_find_regions() {
        let image = GrayImage::new(100, 100);
        let regions = find_regions(&image).unwrap();
        
        assert!(!regions.is_empty());
        assert_eq!(regions[0].width, 100);
        assert_eq!(regions[0].height, 100);
    }

    #[test]
    fn test_correct_orientation() {
        let image = GrayImage::new(100, 100);
        let result = correct_orientation(&image).unwrap();
        
        assert_eq!(result.width(), 100);
        assert_eq!(result.height(), 100);
    }
}
