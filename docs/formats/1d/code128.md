# Code 128

## 📝 Descrição
O Code 128 é um código de barras de alta densidade que pode codificar texto completo ASCII. É amplamente usado em logística e indústria devido à sua capacidade de codificar muitos dados em um espaço compacto.

## 🔍 Especificação Técnica
- **Tipo**: Linear (1D)
- **Caracteres**: ASCII completo (128 caracteres)
- **Comprimento**: Variável
- **Subtipos**:
  - Code 128A: Maiúsculas, números, símbolos e controles
  - Code 128B: Maiúsculas, minúsculas, números e símbolos
  - Code 128C: Pares de dígitos (otimizado para números)

## 🚀 Uso no QuickCodes

### Rust
```rust
use quickcodes::{generate_to_file, BarcodeType};

// Gerar Code 128
generate_to_file(BarcodeType::Code128, "HELLO123", "codigo.svg")?;

// Dados mistos (texto, números, símbolos)
generate_to_file(BarcodeType::Code128, "Item#123-A/B", "produto.png")?;
```

### Python
```python
from quickcodes import generate_to_file

# Gerar Code 128
generate_to_file("Code128", "HELLO123", "codigo.svg")
```

### Go
```go
import "github.com/marcioreck/quickcodes/go/quickcodes"

// Gerar Code 128
err := quickcodes.GenerateToFile(quickcodes.Code128, "HELLO123", "codigo.svg")
```

### .NET
```csharp
using QuickCodes;

// Gerar Code 128
BarcodeGenerator.GenerateToFile(BarcodeType.Code128, "HELLO123", "codigo.svg");
```

### C++
```cpp
#include <quickcodes/quickcodes.h>

// Gerar Code 128
quickcodes::generate_to_file(quickcodes::BarcodeType::Code128, "HELLO123", "codigo.svg");
```

## ⚠️ Validações
- Aceita qualquer caractere ASCII
- Comprimento variável (limitado pelo espaço físico)
- Otimização automática entre subtipos A, B e C
- Cálculo automático do caractere de verificação

## 🔗 Links Úteis
- [Especificação ISO/IEC 15417](https://www.iso.org/standard/43896.html)
- [Documentação QuickCodes](https://docs.rs/quickcodes)
- [Tutorial Code 128](https://www.barcodefaq.com/1d/code-128/)

## 📊 Casos de Uso
1. **Logística**: Rastreamento de pacotes e contêineres
2. **Indústria**: Identificação de peças e produtos
3. **Saúde**: Identificação de amostras e equipamentos
4. **Varejo**: Códigos seriais e SKUs personalizados

## 💡 Dicas
- Use Code 128C para dados puramente numéricos (mais compacto)
- Evite caracteres de controle desnecessários
- Para dados muito longos, considere um código 2D
- Teste a leitura em diferentes orientações
