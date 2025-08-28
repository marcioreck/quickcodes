# Biblioteca de Detecção de Códigos de Barras - Requisitos Técnicos

## Objetivo: 90%+ Precisão Inspirada em ZXing/Bardecode

### 1. Algoritmos de Detecção Core

#### A) Pré-processamento de Imagem
- **Binarização Adaptiva**: Threshold local baseado em histograma
- **Filtros de Ruído**: Gaussian blur + morphological operations
- **Correção de Contraste**: Histogram equalization
- **Edge Enhancement**: Sobel/Canny edge detection

#### B) Pattern Recognition (QR Code)
```rust
struct FinderPattern {
    ratios: [u32; 5], // 1:1:3:1:1
    center: Point2D,
    size: f32,
    confidence: f32,
}

// Implementar validação geométrica rigorosa
fn validate_finder_triangle(a: &FinderPattern, b: &FinderPattern, c: &FinderPattern) -> bool {
    // Verificar ângulos 90° ± tolerância
    // Verificar proporções de distância
    // Verificar tamanhos similares
}
```

#### C) Pattern Recognition (DataMatrix)
```rust
struct LBorder {
    vertical_line: Line2D,
    horizontal_line: Line2D,
    corner: Point2D,
    timing_patterns: Vec<bool>,
}

// Detectar bordas em L com timing patterns
fn detect_l_border(image: &BitMatrix) -> Vec<LBorder> {
    // Hough transform para linhas
    // Validação de timing patterns alternados
    // Verificação de ângulos perpendiculares
}
```

#### D) Multi-Scale Detection
```rust
struct ScaleParams {
    min_module_size: f32,
    max_module_size: f32,
    scale_steps: usize,
}

// Detectar em múltiplas escalas
fn multi_scale_detection(image: &BitMatrix, params: ScaleParams) -> Vec<Detection> {
    // Pyramid scaling
    // Detection em cada escala
    // Non-maximum suppression
}
```

### 2. Validação e Filtragem Rigorosa

#### A) Validação Geométrica
- **Aspect Ratio Check**: Proporções corretas para cada formato
- **Size Consistency**: Consistência de tamanho entre patterns
- **Angle Validation**: Ângulos esperados (90° para QR, etc.)
- **Distance Ratios**: Proporções de distância entre elementos

#### B) Validação de Conteúdo
- **Pattern Integrity**: Verificação de integridade dos padrões
- **Checksum Validation**: Validação de checksums quando disponível
- **Format Validation**: Validação de formato específico
- **Error Correction**: Uso de códigos de correção de erro

#### C) Sistema de Confiança
```rust
struct DetectionConfidence {
    geometric_score: f32,    // 0.0 - 1.0
    pattern_score: f32,      // 0.0 - 1.0
    contrast_score: f32,     // 0.0 - 1.0
    noise_score: f32,        // 0.0 - 1.0
    overall: f32,           // Combined score
}

const MIN_CONFIDENCE_THRESHOLD: f32 = 0.85; // Para 90%+ accuracy
```

### 3. Estratégias Anti-False Positive

#### A) Multi-Stage Filtering
1. **Pre-filter**: Pattern básico (rápido, baixa precisão)
2. **Geometric Filter**: Validação geométrica rigorosa
3. **Content Filter**: Tentativa de decodificação
4. **Final Validation**: Verificação completa

#### B) Contextual Analysis
```rust
struct ImageContext {
    overall_contrast: f32,
    noise_level: f32,
    edge_density: f32,
    brightness_distribution: Histogram,
}

// Ajustar thresholds baseado no contexto
fn adaptive_thresholds(context: &ImageContext) -> DetectionParams {
    // Thresholds mais rigorosos para imagens ruidosas
    // Sensibilidade ajustada por contraste
}
```

### 4. Implementação em Rust

#### A) Estrutura de Módulos
```
src/
├── detection/
│   ├── qr/
│   │   ├── finder_pattern.rs
│   │   ├── alignment_pattern.rs
│   │   └── detector.rs
│   ├── datamatrix/
│   │   ├── l_border.rs
│   │   ├── timing_pattern.rs
│   │   └── detector.rs
│   └── common/
│       ├── pattern_matching.rs
│       ├── geometric_validation.rs
│       └── confidence.rs
├── preprocessing/
│   ├── binarization.rs
│   ├── noise_filter.rs
│   └── contrast.rs
└── validation/
    ├── anti_false_positive.rs
    └── confidence_scoring.rs
```

#### B) Dependencies Necessárias
```toml
[dependencies]
image = "0.24"           # Image processing
imageproc = "0.23"       # Advanced image operations
nalgebra = "0.32"        # Linear algebra
num-traits = "0.2"       # Numeric traits
rayon = "1.7"            # Parallel processing
thiserror = "1.0"        # Error handling
serde = { version = "1.0", features = ["derive"] }
```

### 5. Métricas de Validação

#### A) Benchmarks Necessários
- **True Positive Rate**: > 90%
- **False Positive Rate**: < 1%
- **Processing Time**: < 100ms per image (1080p)
- **Memory Usage**: < 50MB peak

#### B) Test Dataset
- Imagens com ruído
- Diferentes ângulos e distorções
- Múltiplos códigos por imagem
- Códigos parcialmente obstruídos
- Diferentes condições de iluminação

### 6. Integração WASM

```rust
#[wasm_bindgen]
pub struct AdvancedBarcodeDetector {
    detector: DetectionEngine,
    config: DetectionConfig,
}

#[wasm_bindgen]
impl AdvancedBarcodeDetector {
    #[wasm_bindgen(constructor)]
    pub fn new(config: JsValue) -> Result<AdvancedBarcodeDetector, JsValue> {
        // Configuração customizável
    }
    
    #[wasm_bindgen]
    pub fn detect_with_confidence(&self, image_data: &[u8]) -> JsValue {
        // Retornar detecções com scores de confiança
    }
}
```

### 7. Timeline de Implementação

#### Fase 1 (2-3 semanas): Foundation
- [ ] Sistema de pré-processamento
- [ ] Pattern matching básico
- [ ] Validação geométrica core

#### Fase 2 (3-4 semanas): Detection Algorithms
- [ ] QR Code finder patterns
- [ ] DataMatrix L-border detection
- [ ] PDF417 guard patterns
- [ ] Code128 bar patterns

#### Fase 3 (2-3 semanas): Anti-False Positive
- [ ] Sistema de confiança
- [ ] Multi-stage filtering
- [ ] Contextual analysis

#### Fase 4 (1-2 semanas): Integration & Testing
- [ ] WASM bindings
- [ ] Performance optimization
- [ ] Extensive testing

**Total: 8-12 semanas para biblioteca completa**

### 8. Recursos de Estudo Recomendados

1. **ZXing-CPP Source Code**: https://github.com/zxing-cpp/zxing-cpp
2. **ISO/IEC Standards**: 
   - ISO/IEC 18004 (QR Code)
   - ISO/IEC 16022 (DataMatrix)
   - ISO/IEC 15438 (PDF417)
3. **Computer Vision Papers**:
   - "Robust QR Code Detection" research papers
   - "Barcode Localization in Natural Images"

## Conclusão

**É VIÁVEL** implementar uma biblioteca com 90%+ de precisão, mas requer:
- Implementação de algoritmos robustos inspirados no ZXing
- Sistema rigoroso de validação multi-estágio
- Extensive testing e tuning de parâmetros
- Investimento significativo de tempo (2-3 meses)

A chave é combinar técnicas comprovadas (ZXing) com validação rigorosa e sistema de confiança bem calibrado.
