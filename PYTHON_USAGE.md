# 🐍 QuickCodes - Uso Python

## 📦 Instalação

```bash
# Instalar via pip (quando publicado)
pip install quickcodes

# Ou build local
cd quickcodes
pip install maturin
maturin develop --features python
```

## 🚀 Uso Básico

### Importação
```python
import quickcodes

# Gerar QR Code
qr_data = quickcodes.generate("QRCode", "Hello, Python!", "SVG")
with open("qr.svg", "wb") as f:
    f.write(qr_data)

# Gerar EAN-13
quickcodes.generate_to_file("EAN13", "123456789012", "barcode.png")
```

### Constantes de Conveniência
```python
import quickcodes

# Usar constantes para melhor legibilidade
qr_data = quickcodes.generate(
    quickcodes.BARCODE_TYPES['QR_CODE'],
    "Using constants",
    quickcodes.EXPORT_FORMATS['SVG']
)
```

## 📋 Tipos Suportados

### Códigos de Barras
```python
# 1D Barcodes
quickcodes.generate("EAN13", "123456789012", "PNG")      # EAN-13
quickcodes.generate("UPCA", "03600029145", "SVG")        # UPC-A  
quickcodes.generate("Code128", "HELLO123", "SVG")        # Code128

# 2D Codes
quickcodes.generate("QRCode", "Hello, World!", "PNG")    # QR Code
```

### Formatos de Saída
```python
# SVG (vetorial, escalável)
svg_data = quickcodes.generate("QRCode", "SVG test", "SVG")

# PNG (raster, alta qualidade)
png_data = quickcodes.generate("QRCode", "PNG test", "PNG")
```

## ⚙️ Configurações Avançadas

### QR Code com Correção de Erro
```python
# Níveis de correção de erro
qr_low = quickcodes.generate("QRCode", "Low EC", "SVG", error_correction="Low")
qr_medium = quickcodes.generate("QRCode", "Medium EC", "SVG", error_correction="Medium") 
qr_high = quickcodes.generate("QRCode", "High EC", "SVG", error_correction="High")
```

### Margens Customizadas
```python
# QR Code com margem maior
qr_big_margin = quickcodes.generate("QRCode", "Big margins", "SVG", margin=30)
```

## 🎯 Casos de Uso Práticos

### 1. Pagamento Pix
```python
import quickcodes

pix_data = "00020126580014BR.GOV.BCB.PIX0114+5551999999995204000053039865405100.005802BR5920Python Example6009SAO PAULO62070503***6304ABCD"

quickcodes.generate_to_file("QRCode", pix_data, "pix_payment.png")
print("✅ QR Code Pix gerado: pix_payment.png")
```

### 2. Código de Produto EAN-13
```python
import quickcodes

# O dígito verificador é calculado automaticamente
produto_code = "123456789012"  # 12 dígitos
quickcodes.generate_to_file("EAN13", produto_code, "produto.svg")
print("✅ Código EAN-13 gerado: produto.svg")
# Resultado: 1234567890128 (com dígito verificador 8)
```

### 3. QR Code para WiFi
```python
import quickcodes

wifi_data = "WIFI:T:WPA;S:MinhaRede;P:senha123;H:false;;"
quickcodes.generate_to_file("QRCode", wifi_data, "wifi_qr.png")
print("✅ QR Code WiFi gerado: wifi_qr.png")
```

### 4. Cartão de Visita (vCard)
```python
import quickcodes

vcard = """BEGIN:VCARD
VERSION:3.0
FN:João Silva
ORG:Empresa ABC
TEL:+5511999999999
EMAIL:joao@empresa.com
URL:https://empresa.com
END:VCARD"""

quickcodes.generate_to_file("QRCode", vcard, "cartao_visita.svg")
print("✅ QR Code vCard gerado: cartao_visita.svg")
```

### 5. Geração em Lote
```python
import quickcodes
import os

# Criar diretório de saída
os.makedirs("codigos_gerados", exist_ok=True)

produtos = [
    ("123456789012", "Produto A"),
    ("234567890123", "Produto B"), 
    ("345678901234", "Produto C"),
]

for codigo, nome in produtos:
    filename = f"codigos_gerados/{nome.replace(' ', '_')}.png"
    quickcodes.generate_to_file("EAN13", codigo, filename)
    print(f"✅ Gerado: {filename}")
```

## 🔧 Tratamento de Erros

```python
import quickcodes

try:
    # Dados inválidos
    quickcodes.generate("EAN13", "123", "SVG")
except ValueError as e:
    print(f"Erro: {e}")

try:
    # Formato não suportado
    quickcodes.generate("QRCode", "test", "PDF")
except ValueError as e:
    print(f"Erro: {e}")

try:
    # Checksum inválido
    quickcodes.generate("EAN13", "1234567890127", "SVG")  # Checksum errado
except ValueError as e:
    print(f"Erro: {e}")
```

## 🧪 Testes

```python
import quickcodes
import tempfile
import os

def test_qr_generation():
    """Teste básico de geração QR"""
    result = quickcodes.generate("QRCode", "test", "SVG")
    assert isinstance(result, bytes)
    # Verifica se o resultado contém a tag SVG, indicando que é um arquivo SVG válido
    assert b"<svg" in result
    print("✅ Teste QR Code passou")

def test_ean13_generation():
    """Teste geração EAN-13"""
    with tempfile.TemporaryDirectory() as tmpdir:
        filepath = os.path.join(tmpdir, "test.png")
        quickcodes.generate_to_file("EAN13", "123456789012", filepath)
        assert os.path.exists(filepath)
        print("✅ Teste EAN-13 passou")

if __name__ == "__main__":
    test_qr_generation()
    test_ean13_generation()
    print("🎉 Todos os testes passaram!")
```

## 📊 Performance

```python
import quickcodes
import time

def benchmark_generation():
    """Benchmark de geração"""
    formats = [
        ("QRCode", "Benchmark test"),
        ("EAN13", "123456789012"),
        ("UPCA", "03600029145"),
        ("Code128", "BENCHMARK"),
    ]
    
    for barcode_type, data in formats:
        start = time.time()
        result = quickcodes.generate(barcode_type, data, "SVG")
        end = time.time()
        
        print(f"{barcode_type}: {(end-start)*1000:.2f}ms")

if __name__ == "__main__":
    benchmark_generation()
```

## 📚 Referência da API

### `generate(barcode_type, data, format, **kwargs)`
Gera um código de barras e retorna os bytes.

**Parâmetros:**
- `barcode_type`: "QRCode", "EAN13", "UPCA", "Code128"
- `data`: Dados a codificar
- `format`: "SVG", "PNG"
- `error_correction`: "Low", "Medium", "Quartile", "High" (apenas QR)
- `margin`: Tamanho da margem em pixels

**Retorna:** `bytes`

### `generate_to_file(barcode_type, data, output_path)`
Gera um código de barras e salva em arquivo.

**Parâmetros:**
- `barcode_type`: Tipo do código
- `data`: Dados a codificar  
- `output_path`: Caminho do arquivo (extensão determina formato)

**Retorna:** `None`

## 🎯 Próximos Passos

O binding Python está funcional para a Fase 1. Na Fase 2 serão adicionados:
- Leitura/decodificação de códigos
- Mais tipos de código (DataMatrix, PDF417, Aztec)
- Configurações avançadas
- Processamento em lote otimizado

---

**QuickCodes Python - Desenvolvido por Márcio Reck**
