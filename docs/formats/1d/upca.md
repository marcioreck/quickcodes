# UPC-A

## 📝 Descrição
O UPC-A (Universal Product Code) é o formato padrão para produtos comerciais nos Estados Unidos e Canadá. É um subconjunto do EAN-13, usando 12 dígitos (o EAN-13 começa com '0' para códigos UPC).

## 🔍 Especificação Técnica
- **Tipo**: Linear (1D)
- **Caracteres**: Apenas numéricos (0-9)
- **Comprimento**: 12 dígitos (11 + 1 verificador)
- **Estrutura**:
  - Prefixo do sistema (1 dígito)
  - Código do fabricante (5 dígitos)
  - Código do produto (5 dígitos)
  - Dígito verificador (1 dígito)

## 🚀 Uso no QuickCodes

### Rust
```rust
use quickcodes::{generate_to_file, BarcodeType};

// Gerar UPC-A (11 dígitos, o 12º será calculado)
generate_to_file(BarcodeType::UPCA, "03600029145", "produto.svg")?;

// Ou com dígito verificador já incluído
generate_to_file(BarcodeType::UPCA, "036000291452", "produto.png")?;
```

### Python
```python
from quickcodes import generate_to_file

# Gerar UPC-A
generate_to_file("UPCA", "03600029145", "produto.svg")
```

### Go
```go
import "github.com/marcioreck/quickcodes/go/quickcodes"

// Gerar UPC-A
err := quickcodes.GenerateToFile(quickcodes.UPCA, "03600029145", "produto.svg")
```

### .NET
```csharp
using QuickCodes;

// Gerar UPC-A
BarcodeGenerator.GenerateToFile(BarcodeType.UPCA, "03600029145", "produto.svg");
```

### C++
```cpp
#include <quickcodes/quickcodes.h>

// Gerar UPC-A
quickcodes::generate_to_file(quickcodes::BarcodeType::UPCA, "03600029145", "produto.svg");
```

## ⚠️ Validações
- Comprimento deve ser 11 ou 12 dígitos
- Apenas caracteres numéricos são permitidos
- Se fornecido com 12 dígitos, o último deve ser um dígito verificador válido
- Se fornecido com 11 dígitos, o dígito verificador será calculado automaticamente

## 🔗 Links Úteis
- [Especificação Oficial GS1](https://www.gs1.org/standards/barcodes/ean-upc)
- [Documentação QuickCodes](https://docs.rs/quickcodes)
- [Calculadora de Dígito Verificador](https://www.gs1us.org/tools/check-digit-calculator)

## 📊 Casos de Uso
1. **Varejo EUA/Canadá**: Identificação de produtos no ponto de venda
2. **E-commerce**: Integração com marketplaces americanos
3. **Exportação**: Produtos destinados ao mercado norte-americano
4. **Logística**: Rastreamento na cadeia de suprimentos dos EUA

## 💡 Dicas
- Registre seu prefixo junto à GS1 US
- Mantenha uma margem silenciosa (quiet zone) ao imprimir
- Para produtos pequenos, considere o UPC-E (em desenvolvimento)
- Para uso internacional, prefira o EAN-13
