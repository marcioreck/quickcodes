use printpdf::*;
use quickcodes::{generate, BarcodeType, ExportFormat};
use std::fs::File;
use std::io::BufWriter;
use chrono::Local;
use usvg::{Options, Tree};
use resvg::render;
use tiny_skia::{Pixmap, Transform};
use tiny_skia::{Paint, Stroke, PathBuilder};
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

const PAGE_WIDTH_MM: f32 = 210.0;  // A4 width in mm
const PAGE_HEIGHT_MM: f32 = 297.0; // A4 height in mm
const MARGIN_MM: f32 = 15.0;       // Margem externa em mm
const INNER_MARGIN_MM: f32 = 0.0;  // Margem interna em mm
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
    y_pos -= 4.5;
    add_text(&current_layer, &font, 4.0, &format!("Data de Geração: {}", Local::now().format("%Y-%m-%d %H:%M:%S")), PAGE_WIDTH_MM/2.0, y_pos, true);
    y_pos -= 10.0;

    // 1. Códigos 1D
    add_text(&current_layer, &font_bold, 5.0, "1. Códigos 1D (Lineares)", MARGIN_MM, y_pos, false);
    y_pos -= 8.0;

    // EAN-13
    add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        MARGIN_MM,
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
    add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        MARGIN_MM,
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
    add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        MARGIN_MM,
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
    add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        MARGIN_MM,
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
    add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        MARGIN_MM,
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
    add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        MARGIN_MM,
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
    add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        MARGIN_MM,
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
    add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        MARGIN_MM,
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
    add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        MARGIN_MM,
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
    add_barcode_section(
        &current_layer,
        &font,
        &font_mono,
        MARGIN_MM,
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

    // Improved center calculation using font metrics
    let text_width_pts = FontTextWidth::get_text_width(font, text, points_size)
        .unwrap_or(points_size * text.len() as f32 * 0.5); // fallback estimate

    let text_width_mm = text_width_pts / MM_TO_POINTS;
    let x_pos = if center {
        x - text_width_mm / 2.0
    } else {
        x
    };

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
    width_mm: f32,
    height_mm: f32,
) -> Result<f32, Box<dyn std::error::Error>> {
    let mut y = y_pos;

    // Título e dados
    add_text(layer, font, 4.0, &format!("{}:", title), x_pos, y, false);
    y -= 4.0;
    add_text(layer, font_mono, 3.5, &format!("Dados: {}", data), x_pos + 2.0, y, false);
    y -= 6.0;

    // Gerar código de barras SVG
    let barcode_svg = String::from_utf8(generate(barcode_type, data, ExportFormat::SVG)?)?;

    // Salvar SVG
    let output_dir = "examples/output";
    std::fs::create_dir_all(output_dir)?;
    let svg_filename = format!("{}/barcode_{:?}_{}.svg", output_dir, barcode_type, data.replace('/', "_"));
    std::fs::write(&svg_filename, &barcode_svg)?;

    // Configurar renderização SVG
    let mut opt = Options::default();
    opt.font_size = 8.0;
    opt.dpi = 300.0;
    opt.shape_rendering = usvg::ShapeRendering::CrispEdges;

    // Converter SVG para imagem
    let tree = Tree::from_str(&barcode_svg, &opt)?;
    let tree_size = tree.size();

    // Verificar se a largura excede o máximo disponível para PDF
    let final_width = if width_mm > MAX_WIDTH_MM {
        let scale_down = MAX_WIDTH_MM / width_mm;
        width_mm * scale_down
    } else {
        width_mm
    };
    let final_height = height_mm * (final_width / width_mm);

    // Calcular dimensões para PNG (usar largura/altura originais)
    let png_render_scale = 8.0; // Alta qualidade para PNG
    let png_width_px = (width_mm * MM_TO_POINTS * png_render_scale) as u32;
    let png_height_px = (height_mm * MM_TO_POINTS * png_render_scale) as u32;

    let mut png_pixmap = Pixmap::new(png_width_px, png_height_px)
        .ok_or("Failed to create PNG pixmap")?;
    png_pixmap.fill(tiny_skia::Color::WHITE);
    let png_transform = Transform::from_scale(
        png_width_px as f32 / tree_size.width(),
        png_height_px as f32 / tree_size.height(),
    );
    render(&tree, png_transform, &mut png_pixmap.as_mut());

    // Salvar PNG
    //let png_filename = format!("{}/barcode_{:?}_{}.png", output_dir, barcode_type, data.replace('/', "_"));
    //png_pixmap.save_png(&png_filename).map_err(|e| format!("PNG save error: {:?}", e))?;

    // Calcular dimensões para PDF (usar largura/altura ajustados em pontos)
    let pdf_width_pt = (final_width * MM_TO_POINTS).round() as u32;
    let pdf_height_pt = (final_height * MM_TO_POINTS).round() as u32;
    println!("PDF image dimensions: width = {} pt, height = {} pt", pdf_width_pt, pdf_height_pt);

    if pdf_width_pt == 0 || pdf_height_pt == 0 {
        return Err("PDF image dimensions are zero; cannot create pixmap".into());
    }
    let mut pdf_pixmap = Pixmap::new(pdf_width_pt, pdf_height_pt)
        .ok_or("Failed to create PDF pixmap")?;
    // Preencher o fundo com branco
    pdf_pixmap.fill(tiny_skia::Color::WHITE);

    // Renderizar SVG para pixmap
    let pdf_transform = Transform::from_scale(
        pdf_width_pt as f32 / tree_size.width(),
        pdf_height_pt as f32 / tree_size.height(),
    );
    render(&tree, pdf_transform, &mut pdf_pixmap.as_mut());

    // Desenhar uma borda preta de 3px ao redor da imagem (após renderizar SVG)
    let mut pb = PathBuilder::new();
    let border_offset = 1.5;
    pb.move_to(border_offset, border_offset);
    pb.line_to(pdf_width_pt as f32 - border_offset, border_offset);
    pb.line_to(pdf_width_pt as f32 - border_offset, pdf_height_pt as f32 - border_offset);
    pb.line_to(border_offset, pdf_height_pt as f32 - border_offset);
    pb.close();

    let path = pb.finish().unwrap();
    let mut paint = Paint::default();
    paint.set_color(tiny_skia::Color::BLACK);
    let stroke = Stroke {
        width: 3.0,
        ..Default::default()
    };
    pdf_pixmap.stroke_path(&path, &paint, &stroke, tiny_skia::Transform::identity(), None);

    // Converter para XObject
    let image = Image::from(ImageXObject {
        width: Px(pdf_width_pt as usize),
        height: Px(pdf_height_pt as usize),
        color_space: ColorSpace::Rgb,
        bits_per_component: ColorBits::Bit8,
        interpolate: false,
        image_data: pdf_pixmap.data().to_vec(),
        image_filter: None,
        clipping_bbox: None,
        smask: None,
    });

    // Adicionar ao PDF com dimensões exatas em mm
    image.add_to_layer(
        layer.clone(),
        ImageTransform {
            translate_x: Some(Mm(x_pos + INNER_MARGIN_MM)),
            translate_y: Some(Mm(y - final_height)),
            scale_x: None,
            scale_y: None,
            ..Default::default()
        },
    );

    // Campos de resultado
    y -= final_height + 8.0;
    add_text(layer, font, 3.5, "Resultado: □ OK  □ Falha", x_pos + 2.0, y, false);
    y -= 3.0;
    add_text(layer, font, 3.5, "Observações: _________________________", x_pos + 2.0, y, false);

    Ok(y)
}