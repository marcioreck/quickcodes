# EAN-13

## 📝 Descrição
O EAN-13 (European Article Number) é o formato de código de barras mais comum para produtos comerciais em todo o mundo. Cada código EAN-13 contém 13 dígitos numéricos, incluindo um dígito verificador calculado automaticamente.

## 🔍 Especificação Técnica
- **Tipo**: Linear (1D)
- **Caracteres**: Apenas numéricos (0-9)
- **Comprimento**: 13 dígitos (12 + 1 verificador)
- **Estrutura**:
  - Prefixo do país (2-3 dígitos)
  - Código do fabricante (4-5 dígitos)
  - Código do produto (5 dígitos)
  - Dígito verificador (1 dígito)

## 🚀 Uso no QuickCodes

### Rust
```rust
use quickcodes::{generate_to_file, BarcodeType};

// Gerar EAN-13 (12 dígitos, o 13º será calculado)
generate_to_file(BarcodeType::EAN13, "123456789012", "produto.svg")?;

// Ou com dígito verificador já incluído
generate_to_file(BarcodeType::EAN13, "1234567890128", "produto.png")?;
```

### Python
```python
from quickcodes import generate_to_file

# Gerar EAN-13
generate_to_file("EAN13", "123456789012", "produto.svg")
```

### Go
```go
import "github.com/marcioreck/quickcodes/go/quickcodes"

// Gerar EAN-13
err := quickcodes.GenerateToFile(quickcodes.EAN13, "123456789012", "produto.svg")
```

### .NET
```csharp
using QuickCodes;

// Gerar EAN-13
BarcodeGenerator.GenerateToFile(BarcodeType.EAN13, "123456789012", "produto.svg");
```

### C++
```cpp
#include <quickcodes/quickcodes.h>

// Gerar EAN-13
quickcodes::generate_to_file(quickcodes::BarcodeType::EAN13, "123456789012", "produto.svg");
```

## ⚠️ Validações
- Comprimento deve ser 12 ou 13 dígitos
- Apenas caracteres numéricos são permitidos
- Se fornecido com 13 dígitos, o último deve ser um dígito verificador válido
- Se fornecido com 12 dígitos, o dígito verificador será calculado automaticamente

## 🔗 Links Úteis
- [Especificação Oficial GS1](https://www.gs1.org/standards/barcodes/ean-upc)
- [Documentação QuickCodes](https://docs.rs/quickcodes)
- [Calculadora de Dígito Verificador](https://www.gs1.org/services/check-digit-calculator)

## 📊 Casos de Uso
1. **Varejo**: Identificação de produtos no ponto de venda
2. **Inventário**: Controle de estoque e rastreamento
3. **E-commerce**: Integração com sistemas de venda online
4. **Logística**: Rastreamento de produtos na cadeia de suprimentos

## 💡 Dicas
- Use o prefixo correto do seu país (disponível via GS1 local)
- Mantenha uma margem silenciosa (quiet zone) ao imprimir
- Para produtos pequenos, considere o EAN-8 (em desenvolvimento)
- Teste a leitura em diferentes scanners antes do uso em produção
