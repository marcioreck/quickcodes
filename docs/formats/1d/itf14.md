# ITF-14

## 📝 Descrição
O ITF-14 (Interleaved Two of Five) é um formato de código de barras especialmente projetado para embalagens de papelão na cadeia de suprimentos. É baseado no Code 2 of 5 e otimizado para impressão direta em superfícies corrugadas.

## 🔍 Especificação Técnica
- **Tipo**: Linear (1D)
- **Caracteres**: Apenas numéricos (0-9)
- **Comprimento**: 14 dígitos (13 + 1 verificador)
- **Estrutura**:
  - Indicador logístico (1 dígito)
  - Código do produto (12 dígitos, geralmente um EAN/UPC)
  - Dígito verificador (1 dígito)
- **Características**:
  - Barras mais largas que outros formatos
  - Moldura de proteção (bearer bar)
  - Codificação entrelaçada para maior densidade

## 🚀 Uso no QuickCodes

### Rust
```rust
use quickcodes::{generate_to_file, BarcodeType};

// Gerar ITF-14 (13 dígitos, o 14º será calculado)
generate_to_file(BarcodeType::ITF14, "1234567890123", "caixa.svg")?;

// Ou com dígito verificador já incluído
generate_to_file(BarcodeType::ITF14, "12345678901231", "caixa.png")?;
```

### Python
```python
from quickcodes import generate_to_file

# Gerar ITF-14
generate_to_file("ITF14", "1234567890123", "caixa.svg")
```

### Go
```go
import "github.com/marcioreck/quickcodes/go/quickcodes"

// Gerar ITF-14
err := quickcodes.GenerateToFile(quickcodes.ITF14, "1234567890123", "caixa.svg")
```

### .NET
```csharp
using QuickCodes;

// Gerar ITF-14
BarcodeGenerator.GenerateToFile(BarcodeType.ITF14, "1234567890123", "caixa.svg");
```

### C++
```cpp
#include <quickcodes/quickcodes.h>

// Gerar ITF-14
quickcodes::generate_to_file(quickcodes::BarcodeType::ITF14, "1234567890123", "caixa.svg");
```

## ⚠️ Validações
- Comprimento deve ser 13 ou 14 dígitos
- Apenas caracteres numéricos são permitidos
- Se fornecido com 14 dígitos, o último deve ser um dígito verificador válido
- Se fornecido com 13 dígitos, o dígito verificador será calculado automaticamente

## 🔗 Links Úteis
- [Especificação GS1](https://www.gs1.org/standards/barcodes/itf-14)
- [Documentação QuickCodes](https://docs.rs/quickcodes)
- [Guia de Aplicação ITF-14](https://www.gs1.org/docs/barcodes/GS1_ITF-14_Implementation_Guide.pdf)

## 📊 Casos de Uso
1. **Logística**: Identificação de caixas e embalagens
2. **Distribuição**: Rastreamento de paletes
3. **Armazenagem**: Gestão de estoque em alto nível
4. **Indústria**: Embalagens de transporte

## 💡 Dicas
- Use impressão direta para caixas de papelão
- Mantenha a moldura de proteção (bearer bar)
- Ideal para leitura a distância
- Considere o tamanho mínimo recomendado (142% do tamanho nominal)
