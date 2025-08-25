# QR Code

## 📝 Descrição
O QR Code (Quick Response Code) é um código de barras matricial (2D) desenvolvido pela Denso Wave. É extremamente versátil, podendo armazenar diferentes tipos de dados e sendo otimizado para leitura rápida, mesmo com danos parciais.

## 🔍 Especificação Técnica
- **Tipo**: Matricial (2D)
- **Caracteres**: Qualquer (suporte Unicode completo)
- **Capacidade**:
  - Numérico: até 7.089 caracteres
  - Alfanumérico: até 4.296 caracteres
  - Binário: até 2.953 bytes
  - Kanji: até 1.817 caracteres
- **Níveis de Correção de Erro**:
  - L (Low): 7% de recuperação
  - M (Medium): 15% de recuperação
  - Q (Quartile): 25% de recuperação
  - H (High): 30% de recuperação

## 🚀 Uso no QuickCodes

### Rust
```rust
use quickcodes::{generate_to_file, BarcodeType};

// QR Code simples
generate_to_file(BarcodeType::QRCode, "Hello, QuickCodes!", "qr.svg")?;

// URL
generate_to_file(BarcodeType::QRCode, "https://example.com", "url.png")?;

// Wi-Fi
let wifi = "WIFI:T:WPA;S:MyNetwork;P:password123;;";
generate_to_file(BarcodeType::QRCode, wifi, "wifi.svg")?;

// PIX (Brasil)
let pix = "00020126580014BR.GOV.BCB.PIX0114+5551999999995204000053039865405100.005802BR5920Test User6009SAO PAULO62070503***6304ABCD";
generate_to_file(BarcodeType::QRCode, pix, "pix.png")?;
```

### Python
```python
from quickcodes import generate_to_file

# Gerar QR Code
generate_to_file("QRCode", "Hello, QuickCodes!", "qr.svg")
```

### Go
```go
import "github.com/marcioreck/quickcodes/go/quickcodes"

// Gerar QR Code
err := quickcodes.GenerateToFile(quickcodes.QRCode, "Hello, QuickCodes!", "qr.svg")
```

### .NET
```csharp
using QuickCodes;

// Gerar QR Code
BarcodeGenerator.GenerateToFile(BarcodeType.QRCode, "Hello, QuickCodes!", "qr.svg");
```

### C++
```cpp
#include <quickcodes/quickcodes.h>

// Gerar QR Code
quickcodes::generate_to_file(quickcodes::BarcodeType::QRCode, "Hello, QuickCodes!", "qr.svg");
```

## ⚠️ Validações
- Aceita qualquer tipo de dados
- Tamanho máximo depende do tipo de dados
- Nível de correção de erro configurável
- Otimização automática de versão e modo

## 🔗 Links Úteis
- [Especificação ISO/IEC 18004](https://www.iso.org/standard/62021.html)
- [Documentação QuickCodes](https://docs.rs/quickcodes)
- [Gerador de URLs para QR Code](https://www.qr-code-generator.com/)

## 📊 Casos de Uso
1. **Marketing**: URLs e cartões de visita
2. **Pagamentos**: PIX, Bitcoin, outros
3. **Wi-Fi**: Configuração automática
4. **Documentos**: Links para versões digitais
5. **Eventos**: Ingressos e credenciais
6. **Produtos**: Links para manuais e suporte

## 💡 Dicas
- Use nível H de correção para QR codes impressos
- Adicione logo no centro (com cuidado)
- Teste em diferentes leitores
- Considere usar URLs curtos
- Para dados estruturados, use formatos padrão (vCard, EMV, etc)
