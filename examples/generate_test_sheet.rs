use ::image::{self as image_crate, imageops, GenericImageView};
use chrono::Local;
use printpdf::*;
use quickcodes::{generate, BarcodeType, ExportFormat};
use std::fs;
use std::fs::File;
use std::io::BufWriter;
// Helper trait to estimate text width for IndirectFontRef
trait FontTextWidth {
    fn get_text_width(&self, text: &str, font_size: f32) -> Option<f32>;
}

impl FontTextWidth for IndirectFontRef {
    fn get_text_width(&self, text: &str, font_size: f32) -> Option<f32> {
        // printpdf does not expose glyph metrics or font family, so we use a fixed factor
        // If you want to distinguish mono fonts, pass a factor from add_text
        // For now, use 0.5 for all fonts
        let avg_factor = 0.5;
        Some(font_size * text.len() as f32 * avg_factor)
    }
}

const PAGE_WIDTH_MM: f32 = 210.0; // A4 width in mm
const PAGE_HEIGHT_MM: f32 = 297.0; // A4 height in mm
const MARGIN_MM: f32 = 15.0; // Margem externa em mm
const INNER_MARGIN_MM: f32 = 0.0; // Margem interna em mm
const MM_TO_POINTS: f32 = 72.0 / 25.4; // Conversão de mm para pontos (1 inch = 72 points = 25.4 mm)

// Fator de escala uniforme para todos os códigos de barras
const SCALE_FACTOR: f32 = 3.0;

fn mm_to_points(mm: f32) -> f32 {
    mm * MM_TO_POINTS
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Criar novo documento PDF
    let (doc, page1, layer1) = PdfDocument::new(
        "QuickCodes Test Sheet",
        Mm(PAGE_WIDTH_MM),
        Mm(PAGE_HEIGHT_MM),
        "Layer 1",
    );
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Configurar fontes
    let font = doc.add_external_font(File::open(
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    )?)?;
    let font_bold = doc.add_external_font(File::open(
        "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
    )?)?;
    let font_mono = doc.add_external_font(File::open(
        "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
    )?)?;

    // Posição inicial
    let mut y_pos = PAGE_HEIGHT_MM - MARGIN_MM;

    // Adicionar cabeçalho
    y_pos -= 8.0;
    add_text(
        &current_layer,
        &font_bold,
        7.0,
        "QuickCodes - Testes v1.0",
        PAGE_WIDTH_MM / 2.0,
        y_pos,
        true,
    );
    y_pos -= 4.5;
    add_text(
        &current_layer,
        &font,
        4.0,
        &format!(
            "Data de Geração: {}",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        ),
        PAGE_WIDTH_MM / 2.0,
        y_pos,
        true,
    );
    y_pos -= 10.0;

    // 1. Códigos 1D
    add_text(
        &current_layer,
        &font_bold,
        5.0,
        "1. Códigos 1D (Lineares)",
        MARGIN_MM,
        y_pos,
        false,
    );
    y_pos -= 8.0;

    // Definir prefixo para arquivos temporários baseado no nome do PDF
    let test_sheet_prefix = "test_sheet_v1";

    // Limpar arquivos antigos antes de gerar novo PDF
    cleanup_old_files(test_sheet_prefix)?;

    // EAN-13
    add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        MARGIN_MM,
        y_pos,
        "EAN-13",
        BarcodeType::EAN13,
        "123456789012", // 12 dígitos, o 13º será calculado
        test_sheet_prefix,
    )?;

    y_pos -= 100.0;

    // UPC-A
    add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        MARGIN_MM,
        y_pos,
        "UPC-A",
        BarcodeType::UPCA,
        "03600029145", // 11 dígitos, o 12º será calculado
        test_sheet_prefix,
    )?;

    // Criar nova página para continuar
    let (page3, layer3) = doc.add_page(Mm(PAGE_WIDTH_MM), Mm(PAGE_HEIGHT_MM), "Layer 3");
    let current_layer = doc.get_page(page3).get_layer(layer3);

    // Resetar posição para nova página
    y_pos = PAGE_HEIGHT_MM - MARGIN_MM - 8.0;

    // Code128
    add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        MARGIN_MM,
        y_pos,
        "Code128",
        BarcodeType::Code128,
        "HELLO123",
        test_sheet_prefix,
    )?;

    y_pos -= 100.0;

    // Code39
    add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        MARGIN_MM,
        y_pos,
        "Code39",
        BarcodeType::Code39,
        "SERIAL-123ABC",
        test_sheet_prefix,
    )?;

    // Criar nova página para continuar
    let (page5, layer5) = doc.add_page(Mm(PAGE_WIDTH_MM), Mm(PAGE_HEIGHT_MM), "Layer 5");
    let current_layer = doc.get_page(page5).get_layer(layer5);

    // Resetar posição para nova página
    y_pos = PAGE_HEIGHT_MM - MARGIN_MM - 8.0;

    // ITF-14
    add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        MARGIN_MM,
        y_pos,
        "ITF-14",
        BarcodeType::ITF14,
        "1234567890123", // 13 dígitos, o 14º será calculado
        test_sheet_prefix,
    )?;

    y_pos -= 100.0;

    // Codabar
    add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        MARGIN_MM,
        y_pos,
        "Codabar",
        BarcodeType::Codabar,
        "A1234567890B",
        test_sheet_prefix,
    )?;

    // Criar nova página para continuar
    let (page7, layer7) = doc.add_page(Mm(PAGE_WIDTH_MM), Mm(PAGE_HEIGHT_MM), "Layer 7");
    let current_layer = doc.get_page(page7).get_layer(layer7);

    // Resetar posição para nova página
    y_pos = PAGE_HEIGHT_MM - MARGIN_MM - 8.0;

    // 2. Códigos 2D
    add_text(
        &current_layer,
        &font_bold,
        5.0,
        "2. Códigos 2D (Matriciais)",
        MARGIN_MM,
        y_pos,
        false,
    );
    y_pos -= 8.0;

    // QR Code
    add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        MARGIN_MM,
        y_pos,
        "QR Code",
        BarcodeType::QRCode,
        "https://github.com/marcioreck/quickcodes",
        test_sheet_prefix,
    )?;

    y_pos -= 100.0;

    // DataMatrix
    add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        MARGIN_MM,
        y_pos,
        "DataMatrix",
        BarcodeType::DataMatrix,
        "010123456789012815240101",
        test_sheet_prefix,
    )?;

    // Criar nova página para continuar
    let (page9, layer9) = doc.add_page(Mm(PAGE_WIDTH_MM), Mm(PAGE_HEIGHT_MM), "Layer 9");
    let current_layer = doc.get_page(page9).get_layer(layer9);

    // Resetar posição para nova página
    y_pos = PAGE_HEIGHT_MM - MARGIN_MM - 8.0;

    // PDF417
    add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        MARGIN_MM,
        y_pos,
        "PDF417",
        BarcodeType::PDF417,
        "DRIVER LICENSE|DOE,JOHN|DOB:1990-01-01",
        test_sheet_prefix,
    )?;

    y_pos -= 100.0;

    // Aztec
    add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        MARGIN_MM,
        y_pos,
        "Aztec",
        BarcodeType::Aztec,
        "TKT:A12345|FROM:NYC|TO:BOS|DATE:2025-08-21",
        test_sheet_prefix,
    )?;

    y_pos -= 12.0 * 10.0;

    // Adicionar rodapé
    add_text(
        &current_layer,
        &font,
        4.0,
        "Dispositivo de Leitura: _________________",
        MARGIN_MM,
        y_pos,
        false,
    );
    y_pos -= 4.0;
    add_text(
        &current_layer,
        &font,
        4.0,
        "App Utilizado: _________________",
        MARGIN_MM,
        y_pos,
        false,
    );
    y_pos -= 4.0;
    add_text(
        &current_layer,
        &font,
        4.0,
        "Data do Teste: _________________",
        MARGIN_MM,
        y_pos,
        false,
    );
    y_pos -= 4.0;
    add_text(
        &current_layer,
        &font,
        4.0,
        "Condições de Luz: □ Natural  □ Artificial  □ Baixa",
        MARGIN_MM,
        y_pos,
        false,
    );

    // Salvar o PDF
    let output_path = "examples/output/test_sheet.pdf";
    std::fs::create_dir_all("examples/output")?;
    doc.save(&mut BufWriter::new(File::create(output_path)?))?;

    println!("Folha de teste gerada em: {}", output_path);
    Ok(())
}

fn add_text(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    size: f32,
    text: &str,
    x: f32,
    y: f32,
    center: bool,
) {
    let points_size = mm_to_points(size);

    // Improved center calculation using font metrics
    let text_width_pts = FontTextWidth::get_text_width(font, text, points_size)
        .unwrap_or(points_size * text.len() as f32 * 0.5); // fallback estimate

    let text_width_mm = text_width_pts / MM_TO_POINTS;
    let x_pos = if center { x - text_width_mm / 2.0 } else { x };

    layer.use_text(text, points_size, Mm(x_pos), Mm(y), font);
}

fn add_barcode_section(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    font_mono: &IndirectFontRef,
    x_pos: f32,
    y_pos: f32,
    title: &str,
    barcode_type: BarcodeType,
    data: &str,
    prefix: &str,
) -> Result<f32, Box<dyn std::error::Error>> {
    let mut y = y_pos;

    // Título e dados
    add_text(layer, font, 4.0, &format!("{}:", title), x_pos, y, false);
    y -= 5.0; // Maior espaçamento entre título e dados
    add_text(
        layer,
        font_mono,
        3.5,
        &format!("Dados: {}", data),
        x_pos + 2.0,
        y,
        false,
    );
    y -= 8.0; // Maior espaçamento entre dados e código de barras

    // Gerar códigos de barras PNG e SVG usando o prefixo
    let barcode_svg = String::from_utf8(generate(barcode_type, data, ExportFormat::SVG)?)?;
    let barcode_png = generate(barcode_type, data, ExportFormat::PNG)?;

    // Salvar arquivos com prefixo
    let output_dir = "examples/output";
    std::fs::create_dir_all(output_dir)?;

    let safe_data = data.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
    let svg_filename = format!(
        "{}/{}_barcode_{:?}_{}.svg",
        output_dir, prefix, barcode_type, safe_data
    );
    let png_filename = format!(
        "{}/{}_barcode_{:?}_{}.png",
        output_dir, prefix, barcode_type, safe_data
    );

    std::fs::write(&svg_filename, &barcode_svg)?;
    std::fs::write(&png_filename, &barcode_png)?;

    // Carregar arquivo PNG existente diretamente
    println!("Carregando PNG existente: {}", png_filename);
    let png_data = std::fs::read(&png_filename)?;

    // Usar a biblioteca image para obter as dimensões do PNG
    let img = image_crate::load_from_memory(&png_data)?;
    let (png_width_px, png_height_px) = img.dimensions();

    println!(
        "Dimensões PNG original: {}x{} pixels",
        png_width_px, png_height_px
    );

    // Converter para RGB se necessário
    let rgb_img = img.to_rgb8();

    // Aplicar escala uniforme mantendo proporções
    let scaled_width = (png_width_px as f32 * SCALE_FACTOR) as u32;
    let scaled_height = (png_height_px as f32 * SCALE_FACTOR) as u32;

    println!(
        "Aplicando escala {}x: {}x{} → {}x{} pixels",
        SCALE_FACTOR, png_width_px, png_height_px, scaled_width, scaled_height
    );

    // Redimensionar a imagem PNG mantendo proporções
    let resized_img = imageops::resize(
        &rgb_img,
        scaled_width,
        scaled_height,
        imageops::FilterType::Lanczos3,
    );
    let final_image_data = resized_img.as_raw().clone();

    // Usar as dimensões escaladas
    let pdf_width_pt = scaled_width;
    let pdf_height_pt = scaled_height;

    // Calcular dimensões finais em mm para posicionamento
    let final_width_mm = pdf_width_pt as f32 / MM_TO_POINTS;
    let final_height_mm = pdf_height_pt as f32 / MM_TO_POINTS;

    println!(
        "Dimensões finais no PDF: {}x{} pt = {:.1}x{:.1} mm",
        pdf_width_pt, pdf_height_pt, final_width_mm, final_height_mm
    );

    if pdf_width_pt == 0 || pdf_height_pt == 0 {
        return Err("PDF image dimensions are zero; cannot create image".into());
    }

    // Converter para XObject usando os dados PNG escalados
    let image = Image::from(ImageXObject {
        width: Px(pdf_width_pt as usize),
        height: Px(pdf_height_pt as usize),
        color_space: ColorSpace::Rgb,
        bits_per_component: ColorBits::Bit8,
        interpolate: true,
        image_data: final_image_data,
        image_filter: None,
        clipping_bbox: None,
        smask: None,
    });

    // Adicionar ao PDF com dimensões escaladas
    image.add_to_layer(
        layer.clone(),
        ImageTransform {
            translate_x: Some(Mm(x_pos + INNER_MARGIN_MM)),
            translate_y: Some(Mm(y - final_height_mm / 4.0)),
            scale_x: None,
            scale_y: None,
            ..Default::default()
        },
    );

    // Campos de resultado
    y -= final_height_mm / 4.0 + 8.0;
    add_text(
        layer,
        font,
        3.5,
        "Resultado: □ OK  □ Falha",
        x_pos + 2.0,
        y,
        false,
    );
    y -= 4.0; // Maior espaçamento entre linhas
    add_text(
        layer,
        font,
        3.5,
        "Observações: _________________________",
        x_pos + 2.0,
        y,
        false,
    );

    Ok(y)
}

fn cleanup_old_files(prefix: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = "examples/output";

    if let Ok(entries) = fs::read_dir(output_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                if let Some(name_str) = file_name.to_str() {
                    // Verificar se o arquivo tem o prefixo correto e é PNG ou SVG
                    if name_str.starts_with(&format!("{}_barcode_", prefix))
                        && (name_str.ends_with(".png") || name_str.ends_with(".svg"))
                    {
                        let file_path = entry.path();
                        if let Err(e) = fs::remove_file(&file_path) {
                            println!("Aviso: Não foi possível remover {:?}: {}", file_path, e);
                        } else {
                            println!("Removido: {:?}", file_path);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
