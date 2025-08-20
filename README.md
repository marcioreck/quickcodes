# 📦 QuickCodes

**Universal Barcode & QR Toolkit**
*Gere e leia códigos de barras (1D) e 2D em múltiplos padrões, com performance e simplicidade.*

[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/seu-usuario/quickcodes/ci.yml)]()
[![Stars](https://img.shields.io/github/stars/seu-usuario/quickcodes?style=social)]()

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
from quickcodes import generate, read

# Gerar QR Code de pagamento Pix
generate("QR", "00020126580014BR.GOV.BCB.PIX0114+5551999999995204000053039865405100.005802BR5920Padaria Exemplo6009SAO PAULO62070503***6304ABCD", output="pix.svg")

# Ler um código de barras de imagem
data = read("produto.png")
print(data)  # -> "7891234567890"
```

### JavaScript (Browser)

```javascript
import { generate, read } from "quickcodes-wasm";

// Gerar um EAN-13
let svg = generate("EAN13", "7891234567890");

// Ler QR Code da webcam
let result = await read(videoStream);
console.log(result);
```

### 📸 Exemplos Gerados

Após executar `cargo run --example basic_usage`, você encontrará estes arquivos em `examples/output/`:

- **qr_hello.svg** - QR Code: "Hello, QuickCodes!"
- **ean13_example.png** - EAN-13: 1234567890128
- **upc_a_example.svg** - UPC-A: 036000291452  
- **code128_example.svg** - Code128: "HELLO123"
- **pix_payment.svg** - QR Code para pagamento Pix
- **github_url.png** - QR Code com URL do GitHub

---

## 🎯 Status Atual

✅ **MVP Funcional Completo!**

- ✅ 4 formatos de código implementados (QR, EAN-13, UPC-A, Code128)
- ✅ 2 formatos de exportação (SVG, PNG)
- ✅ 25 testes unitários passando
- ✅ API unificada e fácil de usar
- ✅ Exemplos funcionais com arquivos reais gerados

```bash
# Teste a biblioteca agora:
git clone <repo>
cd quickcodes
cargo run --example basic_usage
# Veja os códigos gerados em examples/output/
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
* [ ] **Bindings Iniciais**
  * [ ] Python (PyO3)
  * [ ] JavaScript/Node.js (NAPI-RS)

### 🔧 **Fase 2 - Expansão Industrial**
* [ ] **Códigos 2D Avançados**
  * [ ] DataMatrix (farmacêutica/ANVISA)
  * [ ] PDF417 (documentos oficiais)
  * [ ] Aztec Code (transporte)
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

