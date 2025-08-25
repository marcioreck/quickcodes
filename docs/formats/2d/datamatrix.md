# DataMatrix

## 📝 Descrição
O DataMatrix é um código de barras matricial (2D) de alta densidade, especialmente popular na indústria e no setor farmacêutico. É capaz de codificar grandes quantidades de dados em um espaço muito pequeno e oferece excelente correção de erros.

## 🔍 Especificação Técnica
- **Tipo**: Matricial (2D)
- **Caracteres**: ASCII completo e dados binários
- **Capacidade**:
  - Numérico: até 3.116 dígitos
  - Alfanumérico: até 2.335 caracteres
  - Binário: até 1.556 bytes
- **Correção de Erro**: Reed-Solomon (até 25%)
- **Formatos**:
  - Quadrado: 10×10 até 144×144
  - Retangular: 8×18 até 16×48

## 🚀 Uso no QuickCodes

### Rust
```rust
use quickcodes::{generate_to_file, BarcodeType};

// DataMatrix simples
generate_to_file(BarcodeType::DataMatrix, "Hello, QuickCodes!", "dm.svg")?;

// Rastreamento farmacêutico (ANVISA)
let pharma = "010123456789012815240101";
generate_to_file(BarcodeType::DataMatrix, pharma, "pharma.png")?;

// Rastreamento industrial
let industrial = "PART:ABC123|LOT:20250821|SN:001234567";
generate_to_file(BarcodeType::DataMatrix, industrial, "part.svg")?;
```

### Python
```python
from quickcodes import generate_to_file

# Gerar DataMatrix
generate_to_file("DataMatrix", "Hello, QuickCodes!", "dm.svg")
```

### Go
```go
import "github.com/marcioreck/quickcodes/go/quickcodes"

// Gerar DataMatrix
err := quickcodes.GenerateToFile(quickcodes.DataMatrix, "Hello, QuickCodes!", "dm.svg")
```

### .NET
```csharp
using QuickCodes;

// Gerar DataMatrix
BarcodeGenerator.GenerateToFile(BarcodeType.DataMatrix, "Hello, QuickCodes!", "dm.svg");
```

### C++
```cpp
#include <quickcodes/quickcodes.h>

// Gerar DataMatrix
quickcodes::generate_to_file(quickcodes::BarcodeType::DataMatrix, "Hello, QuickCodes!", "dm.svg");
```

## ⚠️ Validações
- Aceita qualquer caractere ASCII
- Suporte a dados binários
- Otimização automática de tamanho
- Correção de erros embutida

## 🔗 Links Úteis
- [Especificação ISO/IEC 16022](https://www.iso.org/standard/44230.html)
- [Documentação QuickCodes](https://docs.rs/quickcodes)
- [GS1 DataMatrix Guideline](https://www.gs1.org/docs/barcodes/GS1_DataMatrix_Guideline.pdf)

## 📊 Casos de Uso
1. **Farmacêutico**: Rastreamento ANVISA
2. **Industrial**: Marcação de peças pequenas
3. **Automotivo**: Identificação de componentes
4. **Eletrônico**: Placas de circuito
5. **Logística**: Rastreamento de pacotes
6. **Saúde**: Instrumentos cirúrgicos

## 💡 Dicas
- Ideal para marcação direta de peças (DPM)
- Use GS1 DataMatrix para padrões da indústria
- Mantenha zona silenciosa
- Teste em diferentes superfícies e materiais
- Considere o tamanho mínimo para a aplicação
