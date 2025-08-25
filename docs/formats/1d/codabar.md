# Codabar

## 📝 Descrição
O Codabar (também conhecido como NW-7, Monarch ou Code 2 of 7) é um formato especializado usado principalmente em bibliotecas, bancos de sangue e laboratórios de revelação de fotos. É conhecido por sua facilidade de impressão e alta tolerância a variações.

## 🔍 Especificação Técnica
- **Tipo**: Linear (1D)
- **Caracteres**: 0-9, -$:/.+ e A-D (start/stop)
- **Comprimento**: Variável
- **Estrutura**:
  - Caractere start (A, B, C ou D)
  - Dados (dígitos e símbolos)
  - Caractere stop (A, B, C ou D)
- **Características**:
  - Auto-verificação (sem dígito verificador)
  - Tolerante a variações de impressão
  - Leitura bidirecional

## 🚀 Uso no QuickCodes

### Rust
```rust
use quickcodes::{generate_to_file, BarcodeType};

// Gerar Codabar (com start/stop A)
generate_to_file(BarcodeType::Codabar, "A1234567890B", "biblioteca.svg")?;

// Com símbolos especiais
generate_to_file(BarcodeType::Codabar, "A123-456/789B", "amostra.png")?;
```

### Python
```python
from quickcodes import generate_to_file

# Gerar Codabar
generate_to_file("Codabar", "A1234567890B", "biblioteca.svg")
```

### Go
```go
import "github.com/marcioreck/quickcodes/go/quickcodes"

// Gerar Codabar
err := quickcodes.GenerateToFile(quickcodes.Codabar, "A1234567890B", "biblioteca.svg")
```

### .NET
```csharp
using QuickCodes;

// Gerar Codabar
BarcodeGenerator.GenerateToFile(BarcodeType.Codabar, "A1234567890B", "biblioteca.svg");
```

### C++
```cpp
#include <quickcodes/quickcodes.h>

// Gerar Codabar
quickcodes::generate_to_file(quickcodes::BarcodeType::Codabar, "A1234567890B", "biblioteca.svg");
```

## ⚠️ Validações
- Deve começar e terminar com A, B, C ou D
- Caracteres permitidos: 0-9, -$:/.+
- Comprimento variável
- Caracteres start/stop devem ser diferentes (recomendado)

## 🔗 Links Úteis
- [Especificação ANSI/AIM BC3-1995](https://www.aimglobal.org/)
- [Documentação QuickCodes](https://docs.rs/quickcodes)
- [Tutorial Codabar](https://www.barcodefaq.com/1d/codabar/)

## 📊 Casos de Uso
1. **Bibliotecas**: Cartões de membros e livros
2. **Saúde**: Etiquetas de bolsas de sangue
3. **Laboratórios**: Identificação de amostras
4. **Fotos**: Envelopes de revelação
5. **Correios**: Etiquetas de entrega

## 💡 Dicas
- Use caracteres start/stop diferentes (ex: A...B)
- Ideal para impressão matricial
- Bom para aplicações que exigem leitura em condições adversas
- Considere Code 128 para dados mais densos
