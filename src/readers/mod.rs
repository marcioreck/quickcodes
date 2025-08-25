use anyhow::Result;
use image::{DynamicImage, ImageFormat};
use std::path::Path;

mod image_processing;
mod detector;
mod decoder;

use image_processing::prepare_image;
use decoder::decode_image;
use crate::types::ReadResult;

/// Lê códigos de barras de um arquivo de imagem
pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<ReadResult>> {
    // Carregar imagem
    let image = image::open(path)?;
    
    // Processar e decodificar
    read_from_image(&image)
}

/// Lê códigos de barras de dados de imagem em memória
pub fn read_from_bytes(data: &[u8], format: Option<&str>) -> Result<Vec<ReadResult>> {
    // Determinar formato
    let format = match format {
        Some(f) => match f.to_lowercase().as_str() {
            "png" => ImageFormat::Png,
            "jpg" | "jpeg" => ImageFormat::Jpeg,
            _ => return Err(anyhow::anyhow!("Unsupported image format")),
        },
        None => image::guess_format(data)?,
    };

    // Carregar imagem
    let image = image::load_from_memory_with_format(data, format)?;
    
    // Processar e decodificar
    read_from_image(&image)
}

/// Lê códigos de barras de uma imagem carregada
fn read_from_image(image: &DynamicImage) -> Result<Vec<ReadResult>> {
    // Preparar imagem
    let processed = prepare_image(image)?;
    
    // Decodificar
    decode_image(&processed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GrayImage, Luma};
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_read_from_file_not_found() {
        let result = read_from_file("nonexistent.png");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_from_file_invalid_format() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Not an image").unwrap();

        let result = read_from_file(&file_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_from_bytes_invalid_format() {
        let data = vec![0u8; 100];
        let result = read_from_bytes(&data, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_from_bytes_empty() {
        let data = Vec::new();
        let result = read_from_bytes(&data, Some("png"));
        assert!(result.is_err());
    }

    #[test]
    fn test_read_from_image_empty() {
        let image = DynamicImage::ImageLuma8(GrayImage::new(100, 100));
        let result = read_from_image(&image).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_read_from_image_with_pattern() {
        let mut gray = GrayImage::new(100, 100);
        
        // Criar um padrão de barras simples
        for x in 0..100 {
            let value = if x % 2 == 0 { 0 } else { 255 };
            for y in 40..60 {
                gray.put_pixel(x, y, Luma([value]));
            }
        }

        let image = DynamicImage::ImageLuma8(gray);
        let result = read_from_image(&image).unwrap();
        // Por enquanto, esperamos que não decodifique nada
        // pois as implementações específicas ainda não estão prontas
        assert!(result.is_empty());
    }
}