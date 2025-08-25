use printpdf::*;
use quickcodes::{generate, BarcodeType, ExportFormat};
use std::fs::File;
use std::io::BufWriter;
use chrono::Local;
use usvg::{Options, Tree};
use resvg::render;
use tiny_skia::{Pixmap, Transform};

const PAGE_WIDTH_MM: f32 = 210.0;  // A4 width in mm
const PAGE_HEIGHT_MM: f32 = 297.0; // A4 height in mm
const MARGIN_MM: f32 = 15.0;       // Margem externa em mm
const INNER_MARGIN_MM: f32 = 2.0;  // Margem interna em mm
const MAX_WIDTH_MM: f32 = PAGE_WIDTH_MM - (2.0 * MARGIN_MM) - (2.0 * INNER_MARGIN_MM);  // Largura máxima útil

// Tamanhos base dos códigos (já em tamanho final desejado)
const BARCODE_1D_WIDTH: f32 = 160.0;      // Largura para códigos 1D
const BARCODE_1D_HEIGHT: f32 = 80.0;      // Altura aumentada para melhor leitura
const BARCODE_2D_LARGE: f32 = 120.0;      // QR e Aztec quadrados
const BARCODE_2D_MEDIUM: f32 = 100.0;     // DataMatrix um pouco menor
const BARCODE_PDF417_WIDTH: f32 = 160.0;  // Mesmo que 1D
const BARCODE_PDF417_HEIGHT: f32 = 80.0;  // Mesmo que 1D

// Conversão mm para pontos (pts)
const MM_TO_POINTS: f32 = 2.834645669291339; // 1mm = 2.83... pts

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
    let font = doc.add_external_font(File::open("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf")?)?;
    let font_bold = doc.add_external_font(File::open("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf")?)?;
    let font_mono = doc.add_external_font(File::open("/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf")?)?;

    // Posição inicial
    let mut y_pos = PAGE_HEIGHT_MM - MARGIN_MM;

    // Adicionar cabeçalho
    y_pos -= 8.0;
    add_text(&current_layer, &font_bold, 7.0, "QuickCodes - Testes v1.0", PAGE_WIDTH_MM/2.0, y_pos, true);
    y_pos -= 4.0;
    add_text(&current_layer, &font, 4.0, &format!("Data de Geração: {}", Local::now().format("%Y-%m-%d %H:%M:%S")), PAGE_WIDTH_MM/2.0, y_pos, true);
    y_pos -= 10.0;

    // 1. Códigos 1D
    add_text(&current_layer, &font_bold, 5.0, "1. Códigos 1D (Lineares)", MARGIN_MM, y_pos, false);
    y_pos -= 8.0;

    // EAN-13
    y_pos = add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        y_pos,
        "EAN-13",
        BarcodeType::EAN13,
        "123456789012",  // 12 dígitos, o 13º será calculado
        BARCODE_1D_WIDTH,
        BARCODE_1D_HEIGHT,
    )?;

    // Criar nova página para continuar
    let (page2, layer2) = doc.add_page(Mm(PAGE_WIDTH_MM), Mm(PAGE_HEIGHT_MM), "Layer 2");
    let current_layer = doc.get_page(page2).get_layer(layer2);
    
    // Resetar posição para nova página
    y_pos = PAGE_HEIGHT_MM - MARGIN_MM - 8.0;

    // UPC-A
    y_pos = add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        y_pos,
        "UPC-A",
        BarcodeType::UPCA,
        "03600029145",  // 11 dígitos, o 12º será calculado
        BARCODE_1D_WIDTH,
        BARCODE_1D_HEIGHT,
    )?;

    // Criar nova página para continuar
    let (page3, layer3) = doc.add_page(Mm(PAGE_WIDTH_MM), Mm(PAGE_HEIGHT_MM), "Layer 3");
    let current_layer = doc.get_page(page3).get_layer(layer3);
    
    // Resetar posição para nova página
    y_pos = PAGE_HEIGHT_MM - MARGIN_MM - 8.0;

    // Code128
    y_pos = add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        y_pos,
        "Code128",
        BarcodeType::Code128,
        "HELLO123",
        BARCODE_1D_WIDTH,
        BARCODE_1D_HEIGHT,
    )?;

    // Criar nova página para continuar
    let (page4, layer4) = doc.add_page(Mm(PAGE_WIDTH_MM), Mm(PAGE_HEIGHT_MM), "Layer 4");
    let current_layer = doc.get_page(page4).get_layer(layer4);
    
    // Resetar posição para nova página
    y_pos = PAGE_HEIGHT_MM - MARGIN_MM - 8.0;

    // Code39
    y_pos = add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        y_pos,
        "Code39",
        BarcodeType::Code39,
        "SERIAL-123ABC",
        BARCODE_1D_WIDTH,
        BARCODE_1D_HEIGHT,
    )?;

    // Criar nova página para continuar
    let (page5, layer5) = doc.add_page(Mm(PAGE_WIDTH_MM), Mm(PAGE_HEIGHT_MM), "Layer 5");
    let current_layer = doc.get_page(page5).get_layer(layer5);
    
    // Resetar posição para nova página
    y_pos = PAGE_HEIGHT_MM - MARGIN_MM - 8.0;

    // ITF-14
    y_pos = add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        y_pos,
        "ITF-14",
        BarcodeType::ITF14,
        "1234567890123",  // 13 dígitos, o 14º será calculado
        BARCODE_1D_WIDTH,
        BARCODE_1D_HEIGHT,
    )?;

    // Criar nova página para continuar
    let (page6, layer6) = doc.add_page(Mm(PAGE_WIDTH_MM), Mm(PAGE_HEIGHT_MM), "Layer 6");
    let current_layer = doc.get_page(page6).get_layer(layer6);
    
    // Resetar posição para nova página
    y_pos = PAGE_HEIGHT_MM - MARGIN_MM - 8.0;

    // Codabar
    y_pos = add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        y_pos,
        "Codabar",
        BarcodeType::Codabar,
        "A1234567890B",
        BARCODE_1D_WIDTH,
        BARCODE_1D_HEIGHT,
    )?;

    // Criar nova página para continuar
    let (page7, layer7) = doc.add_page(Mm(PAGE_WIDTH_MM), Mm(PAGE_HEIGHT_MM), "Layer 7");
    let current_layer = doc.get_page(page7).get_layer(layer7);
    
    // Resetar posição para nova página
    y_pos = PAGE_HEIGHT_MM - MARGIN_MM - 8.0;

    // 2. Códigos 2D
    add_text(&current_layer, &font_bold, 5.0, "2. Códigos 2D (Matriciais)", MARGIN_MM, y_pos, false);
    y_pos -= 8.0;

    // QR Code
    y_pos = add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        y_pos,
        "QR Code",
        BarcodeType::QRCode,
        "https://github.com/marcioreck/quickcodes",
        BARCODE_2D_LARGE,
        BARCODE_2D_LARGE,
    )?;

    // Criar nova página para continuar
    let (page8, layer8) = doc.add_page(Mm(PAGE_WIDTH_MM), Mm(PAGE_HEIGHT_MM), "Layer 8");
    let current_layer = doc.get_page(page8).get_layer(layer8);
    
    // Resetar posição para nova página
    y_pos = PAGE_HEIGHT_MM - MARGIN_MM - 8.0;

    // DataMatrix
    y_pos = add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        y_pos,
        "DataMatrix",
        BarcodeType::DataMatrix,
        "010123456789012815240101",
        BARCODE_2D_MEDIUM,
        BARCODE_2D_MEDIUM,
    )?;

    // Criar nova página para continuar
    let (page9, layer9) = doc.add_page(Mm(PAGE_WIDTH_MM), Mm(PAGE_HEIGHT_MM), "Layer 9");
    let current_layer = doc.get_page(page9).get_layer(layer9);
    
    // Resetar posição para nova página
    y_pos = PAGE_HEIGHT_MM - MARGIN_MM - 8.0;

    // PDF417
    y_pos = add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        y_pos,
        "PDF417",
        BarcodeType::PDF417,
        "DRIVER LICENSE|DOE,JOHN|DOB:1990-01-01",
        BARCODE_PDF417_WIDTH,
        BARCODE_PDF417_HEIGHT,
    )?;

    // Criar nova página para continuar
    let (page10, layer10) = doc.add_page(Mm(PAGE_WIDTH_MM), Mm(PAGE_HEIGHT_MM), "Layer 10");
    let current_layer = doc.get_page(page10).get_layer(layer10);
    
    // Resetar posição para nova página
    y_pos = PAGE_HEIGHT_MM - MARGIN_MM - 8.0;

    // Aztec
    y_pos = add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        y_pos,
        "Aztec",
        BarcodeType::Aztec,
        "TKT:A12345|FROM:NYC|TO:BOS|DATE:2025-08-21",
        BARCODE_2D_LARGE,
        BARCODE_2D_LARGE,
    )?;
    y_pos -= 10.0;

    // Adicionar rodapé
    add_text(&current_layer, &font, 4.0, "Dispositivo de Leitura: _________________", MARGIN_MM, y_pos, false);
    y_pos -= 4.0;
    add_text(&current_layer, &font, 4.0, "App Utilizado: _________________", MARGIN_MM, y_pos, false);
    y_pos -= 4.0;
    add_text(&current_layer, &font, 4.0, "Data do Teste: _________________", MARGIN_MM, y_pos, false);
    y_pos -= 4.0;
    add_text(&current_layer, &font, 4.0, "Condições de Luz: □ Natural  □ Artificial  □ Baixa", MARGIN_MM, y_pos, false);

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
    let x_pos = if center {
        let text_width = points_size * 0.4 * text.len() as f32 / MM_TO_POINTS;  // Ajustado fator
        x - text_width/2.0
    } else {
        x
    };
    layer.use_text(text, points_size, Mm(x_pos), Mm(y), font);
}

fn add_barcode_section(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    font_mono: &IndirectFontRef,
    y_pos: f32,
    title: &str,
    barcode_type: BarcodeType,
    data: &str,
    width_mm: f32,
    height_mm: f32,
) -> Result<f32, Box<dyn std::error::Error>> {
    let mut y = y_pos;
    
    // Título e dados
    add_text(layer, font, 4.0, &format!("{}:", title), MARGIN_MM, y, false);
    y -= 4.0;
    add_text(layer, font_mono, 3.5, &format!("Dados: {}", data), MARGIN_MM + 2.0, y, false);
    y -= 6.0;

    // Gerar código de barras
    let barcode_svg = String::from_utf8(generate(barcode_type, data, ExportFormat::SVG)?)?;
    
    // Configurar renderização SVG
    let mut opt = Options::default();
    opt.font_size = 8.0;
    opt.dpi = 300.0;
    opt.shape_rendering = usvg::ShapeRendering::CrispEdges;
    
    // Converter SVG para imagem
    let tree = Tree::from_str(&barcode_svg, &opt)?;
    let tree_size = tree.size();
    
    // Verificar se a largura excede o máximo disponível
    let final_width = if width_mm > MAX_WIDTH_MM {
        let scale_down = MAX_WIDTH_MM / width_mm;
        width_mm * scale_down
    } else {
        width_mm
    };
    let final_height = height_mm * (final_width / width_mm);
    
    // Calcular dimensões para alta resolução
    let render_scale = 4.0; // Boa qualidade, arquivo menor
    let width_px = (final_width * MM_TO_POINTS * render_scale) as u32;
    let height_px = (final_height * MM_TO_POINTS * render_scale) as u32;
    
    // Criar pixmap com fundo branco
    let mut pixmap = Pixmap::new(width_px, height_px)
        .ok_or("Failed to create pixmap")?;
    pixmap.fill(tiny_skia::Color::WHITE);
    
    // Renderizar SVG para pixmap mantendo proporções
    let transform = Transform::from_scale(
        width_px as f32 / tree_size.width(),
        height_px as f32 / tree_size.height(),
    );
    
    render(
        &tree,
        transform,
        &mut pixmap.as_mut(),
    );
    
    // Converter para XObject
    let image = Image::from(ImageXObject {
        width: Px(width_px as usize),
        height: Px(height_px as usize),
        color_space: ColorSpace::Rgb,
        bits_per_component: ColorBits::Bit8,
        interpolate: false,
        image_data: pixmap.data().to_vec(),
        image_filter: None,
        clipping_bbox: None,
        smask: None,
    });
    
    // Adicionar ao PDF com dimensões proporcionais
    image.add_to_layer(
        layer.clone(),
        ImageTransform {
            translate_x: Some(Mm(MARGIN_MM + INNER_MARGIN_MM)),
            translate_y: Some(Mm(y - final_height)),
            scale_x: Some(final_width / (width_px as f32 / MM_TO_POINTS)),
            scale_y: Some(final_height / (height_px as f32 / MM_TO_POINTS)),
            ..Default::default()
        },
    );
    
    // Campos de resultado
    y -= final_height + 8.0;
    add_text(layer, font, 3.5, "Resultado: □ OK  □ Falha", MARGIN_MM + 2.0, y, false);
    y -= 3.0;
    add_text(layer, font, 3.5, "Observações: _________________________", MARGIN_MM + 2.0, y, false);

    Ok(y)
}