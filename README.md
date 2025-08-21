# 📦 QuickCodes

**Universal Barcode & QR Toolkit**

*Gere e leia códigos de barras (1D) e 2D em múltiplos padrões, com performance e simplicidade.*

[![CI](https://github.com/marcioreck/quickcodes/actions/workflows/ci.yml/badge.svg)](https://github.com/marcioreck/quickcodes/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE.md)
[![Crates.io](https://img.shields.io/crates/v/quickcodes.svg)](https://crates.io/crates/quickcodes)
[![Documentation](https://docs.rs/quickcodes/badge.svg)](https://docs.rs/quickcodes)
[![codecov](https://codecov.io/gh/marcioreck/quickcodes/branch/main/graph/badge.svg)](https://codecov.io/gh/marcioreck/quickcodes)
[![Stars](https://img.shields.io/github/stars/marcioreck/quickcodes?style=social)](https://github.com/marcioreck/quickcodes)

> 🎉 **QuickCodes está agora disponível no [crates.io](https://crates.io/crates/quickcodes)!**

---

## ✨ Features

* ✅ Geração e leitura de códigos **1D**: EAN-13, UPC-A, Code128, Code39, ITF-14, Codabar
* ✅ Geração e leitura de códigos **2D**: QR Code, DataMatrix, PDF417, Aztec
* ✅ Saída em **PNG, SVG, PDF, Canvas**
* ✅ **Bindings** para Python, JavaScript (Node.js + WASM), e futuro suporte a Go e .NET
* ✅ **Leitura em tempo real** de câmera (via WebAssembly no browser)
* ✅ API simples e moderna

---

## 🚀 Exemplos de Uso

### Python

```python
from quickcodes import generate_to_file
import quickcodes as qc

# Gerar QR Code de pagamento Pix
generate_to_file("QRCode", "00020126580014BR.GOV.BCB.PIX0114+5551999999995204000053039865405100.005802BR5920Padaria Exemplo6009SAO PAULO62070503***6304ABCD", "pix.svg")

# Gerar DataMatrix para rastreamento farmacêutico (ANVISA)
generate_to_file("DataMatrix", "010123456789012815240101", "pharma.svg")

# Gerar PDF417 para documentos oficiais
generate_to_file("PDF417", "DRIVER LICENSE|DOE,JOHN|DOB:1990-01-01", "document.svg")

# Gerar Aztec para tickets de transporte
generate_to_file("Aztec", "TKT:12345|FROM:NYC|TO:BOS|DATE:2024-01-15", "ticket.svg")

# Formatos disponíveis: QRCode, EAN13, UPCA, Code128, DataMatrix, PDF417, Aztec
```

### JavaScript (Browser) *[Planejado - WASM em desenvolvimento]*

```javascript
import { generate } from "quickcodes-wasm";

// Gerar um EAN-13
let svg = generate("EAN13", "7891234567890");

// Gerar DataMatrix para farmácia
let datamatrix = generate("DataMatrix", "010123456789012815240101");

// Gerar PDF417 para documentos
let pdf417 = generate("PDF417", "DRIVER LICENSE|DOE,JOHN|1990-01-01");

// Gerar Aztec para tickets
let aztec = generate("Aztec", "TKT:12345|FROM:NYC|TO:BOS");
```

### 📸 Exemplos Gerados

Após executar os exemplos, você encontrará estes arquivos em `examples/output/`:

**Fase 1 (Básicos):**
- **qr_hello.svg** - QR Code: "Hello, QuickCodes!"
- **ean13_example.png** - EAN-13: 1234567890128
- **upc_a_example.svg** - UPC-A: 036000291452  
- **code128_example.svg** - Code128: "HELLO123"
- **pix_payment.svg** - QR Code para pagamento Pix
- **github_url.png** - QR Code com URL do GitHub

**Fase 2 (Avançados):**
- **datamatrix_pharma.svg** - DataMatrix farmacêutico (ANVISA)
- **datamatrix_industrial.png** - DataMatrix industrial
- **pdf417_document.svg** - PDF417 para documentos
- **pdf417_invoice.png** - PDF417 com dados grandes
- **aztec_transport.svg** - Aztec para transporte
- **aztec_event.png** - Aztec para eventos
- **datamatrix_unicode.svg** - DataMatrix com Unicode

---

## 🎯 Status Atual

✅ **Fase 2 - Códigos 2D Avançados Implementados!**

- ✅ 7 formatos de código implementados (QR, EAN-13, UPC-A, Code128, DataMatrix, PDF417, Aztec)
- ✅ 2 formatos de exportação (SVG, PNG)
- ✅ 56 testes passando (41 unitários + 12 integração + 3 doctests)
- ✅ API unificada Rust e Python para todos os formatos
- ✅ Bindings Python com PyO3 atualizados
- ✅ Código 100% limpo (0 warnings, clippy aprovado)
- ✅ Exemplos funcionais da Fase 2 e documentação completa
- ✅ Suporte completo para casos de uso farmacêuticos (DataMatrix/ANVISA)
- ✅ Suporte para documentos oficiais (PDF417)
- ✅ Suporte para tickets de transporte (Aztec)

```bash
# Teste a biblioteca agora:
git clone https://github.com/marcioreck/quickcodes
cd quickcodes

# Exemplos da Fase 1 (formatos básicos)
cargo run --example basic_usage

# Exemplos da Fase 2 (códigos 2D avançados)
cargo run --example phase2_usage

# Veja todos os códigos gerados em examples/output/
```

---

## 📌 Roadmap

### 🚀 **Fase 1 - MVP (Núcleo Funcional)** ✅ CONCLUÍDA
* [x] **Configuração do Projeto Rust**
  * [x] Estrutura modular do projeto
  * [x] Sistema de build e testes (25 testes unitários passando)
  * [x] Documentação automática (docs.rs ready)
* [x] **Geradores de Código 1D**
  * [x] Code128 (implementação básica)
  * [x] EAN-13 com checksum automático
  * [x] UPC-A
* [x] **Geradores de Código 2D**
  * [x] QR Code (Low, Medium, Quartile, High error correction)
* [x] **Sistema de Exportação**
  * [x] SVG (vetorial, escalável)
  * [x] PNG (raster, alta qualidade)
  * [x] Configurações de tamanho e DPI
* [x] **Bindings Iniciais**
  * [x] Python (PyO3) - Implementado e testado
  * [ ] JavaScript/Node.js (NAPI-RS) [postergado para realizar após API estar completa, toda a fase 2]

### 🔧 **Fase 2 - Expansão Industrial** ⚡ EM PROGRESSO
* [x] **Códigos 2D Avançados** ✅ CONCLUÍDO
  * [x] DataMatrix (farmacêutica/ANVISA)
  * [x] PDF417 (documentos oficiais)
  * [x] Aztec Code (transporte)
* [ ] **Leitura/Decodificação**
  * [ ] Leitor de imagens estáticas
  * [ ] Algoritmos de detecção e correção
  * [ ] Suporte a múltiplos códigos por imagem
* [ ] **Exportação Avançada**
  * [ ] PDF nativo
  * [ ] Canvas/HTML5 integration
  * [ ] Batch processing
* [ ] **WebAssembly**
  * [ ] Build WASM otimizado
  * [ ] API JavaScript para browser
  * [ ] Leitura de webcam em tempo real

### 🌐 **Fase 3 - Ecossistema Completo**
* [ ] **Bindings Adicionais**
  * [ ] Go (CGO)
  * [ ] .NET (P/Invoke)
  * [ ] C/C++ headers
* [ ] **Formatos de Legado**
  * [ ] Code39
  * [ ] ITF-14
  * [ ] Codabar
* [ ] **Ferramentas e Utilitários**
  * [ ] CLI tool (`quickcodes generate`, `quickcodes read`)
  * [ ] API REST em Docker
  * [ ] Benchmarks e performance tests
* [ ] **Recursos Avançados**
  * [ ] Sistema de plugins
  * [ ] Configurações avançadas de renderização
  * [ ] Suporte a fontes customizadas
  * [ ] Watermarks e branding

---

## 📜 Legal Disclaimer

QuickCodes é uma biblioteca de software open source para geração e leitura de códigos de barras lineares (1D) e bidimensionais (2D).

Todos os padrões suportados (EAN, UPC, Code128, Code39, ITF-14, Codabar, QR Code, DataMatrix, PDF417, Aztec, entre outros) são especificados por normas ISO/IEC ou por organizações de padronização (como GS1). Esses padrões são de **uso livre e isentos de royalties para implementação em software**, conforme documentação pública.

⚠️ Observações importantes:

* O termo **"QR Code"** é uma marca registrada da Denso Wave Inc. O uso nesta biblioteca é apenas descritivo, não implica afiliação ou endosso pela Denso Wave.
* Para a **atribuição oficial de códigos de produto (EAN/UPC)** em aplicações comerciais (ex.: venda em supermercados), as empresas devem obter prefixos de código junto à organização **GS1** em seu país, o que pode envolver taxas e anuidades.
* QuickCodes não fornece números de código de barras oficiais, apenas ferramentas de **geração e leitura de imagens** conforme os padrões abertos.

---

## 📄 Licença

Distribuído sob a licença MIT. Consulte o arquivo `LICENSE` para mais detalhes.

---

## Autor

**Márcio Reck**
- Portfólio: [https://fazmercado.com](https://fazmercado.com)
- GitHub: [@marcioreck](https://github.com/marcioreck)

## Agradecimentos

- **Comunidade**: Pelos feedbacks e contribuições

---

*QuickCodes - Desenvolvido por Márcio Reck, AI-Augmented Coder*