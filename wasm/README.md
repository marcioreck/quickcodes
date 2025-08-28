# 🚀 QuickCodes WebAssembly

**Universal Barcode & QR Toolkit for the Web**

Este módulo WebAssembly permite usar QuickCodes diretamente no navegador e Node.js, oferecendo geração e leitura de códigos de barras com alta performance.

---

## ✨ Features

- ✅ **Geração de códigos** em SVG, PNG (base64)
- ✅ **Leitura de códigos** via câmera ou upload de arquivo
- ✅ **Performance nativa** via WebAssembly
- ✅ **Suporte completo** a todos os formatos do QuickCodes
- ✅ **Cross-platform**: Browser + Node.js
- ✅ **Zero dependências** externas no browser

---

## 📦 Formatos Suportados

### 📊 Códigos 1D (Lineares)
- **EAN-13** - Produtos comerciais
- **UPC-A** - Produtos EUA/Canadá  
- **Code128** - Logística
- **Code39** - Industrial
- **ITF-14** - Embalagens
- **Codabar** - Bibliotecas/Laboratórios

### 🔲 Códigos 2D
- **QR Code** - URLs, PIX, uso geral
- **DataMatrix** - Farmacêutico/Industrial
- **PDF417** - Documentos oficiais
- **Aztec** - Bilhetes de transporte

---

## 🔧 Instalação

### 1. Build do Módulo WASM

```bash
# Clone o repositório QuickCodes
git clone https://github.com/marcioreck/quickcodes
cd quickcodes/wasm

# Instale wasm-pack (se não tiver)
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build para diferentes targets
./build.sh
```

### 2. Targets Disponíveis

- `pkg/web/` - Para uso direto no browser
- `pkg/nodejs/` - Para Node.js
- `pkg/bundler/` - Para webpack, vite, rollup, etc.

---

## 🌐 Uso no Browser

### HTML + JavaScript Vanilla

```html
<!DOCTYPE html>
<html>
<head>
    <title>QuickCodes WASM Demo</title>
</head>
<body>
    <div id="output"></div>
    
    <script type="module">
        import init, { generate_svg, generate } from './pkg/web/quickcodes_wasm.js';
        
        async function demo() {
            // Inicializar WASM
            await init();
            
            // Gerar QR Code como SVG
            const qr = generate_svg('QRCode', 'https://github.com/marcioreck/quickcodes');
            if (qr.success) {
                document.getElementById('output').innerHTML = qr.data;
            }
            
            // Gerar EAN-13 como PNG (base64)
            const ean = generate('EAN13', '123456789012', 'PNG');
            if (ean.success) {
                const img = document.createElement('img');
                img.src = `data:image/png;base64,${ean.data}`;
                document.body.appendChild(img);
            }
        }
        
        demo();
    </script>
</body>
</html>
```

### React

```jsx
import { useEffect, useState } from 'react';
import init, { generate_svg } from '../pkg/web/quickcodes_wasm.js';

function BarcodeGenerator() {
    const [wasm, setWasm] = useState(null);
    const [barcode, setBarcode] = useState('');

    useEffect(() => {
        init().then(() => setWasm(true));
    }, []);

    const generateQR = () => {
        if (!wasm) return;
        
        const result = generate_svg('QRCode', 'Hello, QuickCodes!');
        if (result.success) {
            setBarcode(result.data);
        }
    };

    return (
        <div>
            <button onClick={generateQR} disabled={!wasm}>
                Generate QR Code
            </button>
            <div dangerouslySetInnerHTML={{ __html: barcode }} />
        </div>
    );
}
```

### Vue.js

```vue
<template>
  <div>
    <button @click="generateCode" :disabled="!wasmReady">
      Generate Code
    </button>
    <div v-html="barcode"></div>
  </div>
</template>

<script>
import init, { generate_svg } from '../pkg/web/quickcodes_wasm.js';

export default {
  data() {
    return {
      wasmReady: false,
      barcode: ''
    };
  },
  async mounted() {
    await init();
    this.wasmReady = true;
  },
  methods: {
    generateCode() {
      const result = generate_svg('QRCode', 'Vue.js + QuickCodes!');
      if (result.success) {
        this.barcode = result.data;
      }
    }
  }
};
</script>
```

---

## 🖥️ Uso no Node.js

```javascript
import * as wasm from '../pkg/nodejs/quickcodes_wasm.js';
import fs from 'fs';

// Gerar QR Code como SVG
const qr = wasm.generate_svg('QRCode', 'https://example.com');
if (qr.success) {
    fs.writeFileSync('qr.svg', qr.data);
}

// Gerar EAN-13 como PNG
const ean = wasm.generate('EAN13', '123456789012', 'PNG');
if (ean.success) {
    const buffer = Buffer.from(ean.data, 'base64');
    fs.writeFileSync('ean13.png', buffer);
}

// Listar tipos suportados
console.log('Supported types:', wasm.get_supported_types());
console.log('Supported formats:', wasm.get_supported_formats());
```

---

## 📱 Leitura via Câmera

```javascript
// Acessar câmera
const video = document.getElementById('video');
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

navigator.mediaDevices.getUserMedia({ video: true })
    .then(stream => {
        video.srcObject = stream;
    });

// Capturar e ler frame
function captureAndRead() {
    canvas.width = video.videoWidth;
    canvas.height = video.videoHeight;
    ctx.drawImage(video, 0, 0);
    
    const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
    const result = read_from_image_data(imageData);
    
    if (result.success) {
        console.log('Found barcode:', result.barcode_type, result.data);
    }
}
```

---

## 📝 API Reference

### Generation Functions

#### `generate(type, data, format) -> Result`
Gera código de barras no formato especificado.

- **type**: String - Tipo do código ("QRCode", "EAN13", "UPCA", "Code128", "DataMatrix", "PDF417", "Aztec", "Code39", "ITF14", "Codabar")
- **data**: String - Dados para codificar
- **format**: String - Formato de saída ("SVG", "PNG", "PDF")
- **Returns**: `{success: boolean, data?: string, error?: string}`

#### `generate_svg(type, data) -> Result`
Gera código de barras como SVG string.

- **type**: String - Tipo do código
- **data**: String - Dados para codificar  
- **Returns**: `{success: boolean, data?: string, error?: string}`

### Reading Functions (se feature "readers" habilitada)

#### `read_from_image_data(imageData) -> ReadResult`
Lê código de ImageData do Canvas.

- **imageData**: ImageData - Dados da imagem do canvas
- **Returns**: `{success: boolean, barcode_type?: string, data?: string, error?: string}`

#### `read_from_file(file) -> Promise<ReadResult>`
Lê código de arquivo de imagem.

- **file**: File - Objeto File do browser
- **Returns**: Promise com ReadResult

### Utility Functions

#### `get_supported_types() -> Array<string>`
Retorna lista de tipos de código suportados.

#### `get_supported_formats() -> Array<string>`
Retorna lista de formatos de exportação suportados.

---

## 🎯 Exemplos Práticos

### PIX QR Code
```javascript
const pixCode = "00020126580014BR.GOV.BCB.PIX0114+5551999999995204000053039865405100.005802BR5920Padaria Exemplo6009SAO PAULO62070503***6304ABCD";
const result = generate_svg('QRCode', pixCode);
```

### Código de Produto (EAN-13)
```javascript
const result = generate_svg('EAN13', '7891234567890');
```

### Rastreamento Farmacêutico (DataMatrix)
```javascript
const result = generate_svg('DataMatrix', '010123456789012815240101');
```

### Documento Oficial (PDF417)
```javascript
const result = generate_svg('PDF417', 'DRIVER LICENSE|DOE,JOHN|DOB:1990-01-01');
```

### Bilhete de Transporte (Aztec)
```javascript
const result = generate_svg('Aztec', 'TKT:12345|FROM:NYC|TO:BOS|DATE:2025-08-21');
```

---

## 🔧 Bundlers

### Webpack
```javascript
// webpack.config.js
module.exports = {
  experiments: {
    asyncWebAssembly: true
  }
};
```

### Vite
```javascript
// vite.config.js
export default {
  optimizeDeps: {
    exclude: ['quickcodes-wasm']
  }
};
```

### Rollup
```javascript
// rollup.config.js
import { wasm } from '@rollup/plugin-wasm';

export default {
  plugins: [wasm()]
};
```

---

## 📊 Performance

- **Tamanho do WASM**: ~800KB (otimizado)
- **Tempo de inicialização**: ~50ms
- **Geração QR Code**: ~1-5ms
- **Leitura de imagem**: ~10-50ms

---

## 🐛 Troubleshooting

### CORS Issues
Sirva os arquivos via HTTP server, não file://:
```bash
# Python
python -m http.server 8000

# Node.js
npx serve .

# PHP
php -S localhost:8000
```

### Memory Issues
Para aplicações com uso intensivo, chame periodicamente:
```javascript
// Força garbage collection (experimental)
if (performance.memory) {
  console.log('Memory usage:', performance.memory.usedJSHeapSize);
}
```

---

## 📄 License

MIT License - veja [LICENSE.md](../LICENSE.md) para detalhes.

---

## 🤝 Contributing

Contribuições são bem-vindas! Veja [CONTRIBUTING.md](../CONTRIBUTING.md) para guidelines.

---

## 🔗 Links

- [Repositório Principal](https://github.com/marcioreck/quickcodes)
- [Documentação](https://docs.rs/quickcodes)
- [NPM Package](https://www.npmjs.com/package/quickcodes-wasm) (em breve)
- [Crates.io](https://crates.io/crates/quickcodes)
