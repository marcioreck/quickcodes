# Code 39

## 📝 Descrição
O Code 39 (também conhecido como Code 3 of 9) é um dos primeiros formatos de código de barras alfanumérico. É amplamente usado em aplicações industriais e militares devido à sua simplicidade e confiabilidade.

## 🔍 Especificação Técnica
- **Tipo**: Linear (1D)
- **Caracteres**: A-Z, 0-9, e símbolos especiais (-. $/+%)
- **Comprimento**: Variável
- **Estrutura**:
  - Cada caractere é codificado com 9 elementos (5 barras e 4 espaços)
  - 3 elementos são largos e 6 são estreitos
  - Delimitadores * no início e fim (automáticos)

## 🚀 Uso no QuickCodes

### Rust
```rust
use quickcodes::{generate_to_file, BarcodeType};

// Gerar Code 39
generate_to_file(BarcodeType::Code39, "SERIAL123ABC", "codigo.svg")?;

// Com símbolos especiais
generate_to_file(BarcodeType::Code39, "PART-123/A", "peca.png")?;
```

### Python
```python
from quickcodes import generate_to_file

# Gerar Code 39
generate_to_file("Code39", "SERIAL123ABC", "codigo.svg")
```

### Go
```go
import "github.com/marcioreck/quickcodes/go/quickcodes"

// Gerar Code 39
err := quickcodes.GenerateToFile(quickcodes.Code39, "SERIAL123ABC", "codigo.svg")
```

### .NET
```csharp
using QuickCodes;

// Gerar Code 39
BarcodeGenerator.GenerateToFile(BarcodeType.Code39, "SERIAL123ABC", "codigo.svg");
```

### C++
```cpp
#include <quickcodes/quickcodes.h>

// Gerar Code 39
quickcodes::generate_to_file(quickcodes::BarcodeType::Code39, "SERIAL123ABC", "codigo.svg");
```

## ⚠️ Validações
- Apenas maiúsculas (conversão automática)
- Caracteres permitidos: A-Z, 0-9, -. $/+%
- Comprimento variável
- Asteriscos de início/fim são adicionados automaticamente

## 🔗 Links Úteis
- [Especificação ISO/IEC 16388](https://www.iso.org/standard/43897.html)
- [Documentação QuickCodes](https://docs.rs/quickcodes)
- [Tutorial Code 39](https://www.barcodefaq.com/1d/code-39/)

## 📊 Casos de Uso
1. **Indústria**: Identificação de peças e produtos
2. **Militar**: Rastreamento de equipamentos (DoD)
3. **Automotivo**: Etiquetas de peças e componentes
4. **Laboratórios**: Identificação de amostras

## 💡 Dicas
- Use Code 39 Extended para caracteres minúsculos (em desenvolvimento)
- Para dados densos, considere Code 128
- Mantenha uma margem silenciosa adequada
- Ideal para aplicações que precisam de leitura bidirecional
