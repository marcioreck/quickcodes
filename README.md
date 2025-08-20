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

---

## 📌 Roadmap

* [x] Núcleo em Rust
* [x] Bindings para Python e Node.js
* [ ] Suporte a DataMatrix, PDF417 e Aztec
* [ ] Exportação para PDF nativo
* [ ] Bindings para Go e .NET
* [ ] API REST em Docker

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

