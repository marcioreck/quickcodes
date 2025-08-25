# Aztec Code

## 📝 Descrição
O Aztec Code é um código de barras matricial (2D) projetado para ser facilmente lido em dispositivos móveis e otimizado para bilhetes eletrônicos. Seu nome vem do padrão central que lembra uma pirâmide asteca vista de cima.

## 🔍 Especificação Técnica
- **Tipo**: Matricial (2D)
- **Caracteres**: ASCII completo e dados binários
- **Capacidade**:
  - Texto: até 3.832 caracteres
  - Binário: até 3.067 bytes
  - Numérico: até 4.296 dígitos
- **Formatos**:
  - Compacto (15×15 até 27×27)
  - Completo (19×19 até 151×151)
- **Correção de Erro**: 
  - 5% a 95% do total de dados
  - Nível configurável

## 🚀 Uso no QuickCodes

### Rust
```rust
use quickcodes::{generate_to_file, BarcodeType};

// Aztec simples
generate_to_file(BarcodeType::Aztec, "Hello, QuickCodes!", "aztec.svg")?;

// Bilhete de transporte
let ticket = "TKT:A12345|FROM:NYC|TO:BOS|DATE:2025-08-21|SEAT:12A";
generate_to_file(BarcodeType::Aztec, ticket, "ticket.png")?;

// Ingresso de evento
let event = "EVENT:Rock Concert|VENUE:Stadium|DATE:2025-08-21|SEAT:A123";
generate_to_file(BarcodeType::Aztec, event, "event.svg")?;
```

### Python
```python
from quickcodes import generate_to_file

# Gerar Aztec
generate_to_file("Aztec", "Hello, QuickCodes!", "aztec.svg")
```

### Go
```go
import "github.com/marcioreck/quickcodes/go/quickcodes"

// Gerar Aztec
err := quickcodes.GenerateToFile(quickcodes.Aztec, "Hello, QuickCodes!", "aztec.svg")
```

### .NET
```csharp
using QuickCodes;

// Gerar Aztec
BarcodeGenerator.GenerateToFile(BarcodeType.Aztec, "Hello, QuickCodes!", "aztec.svg");
```

### C++
```cpp
#include <quickcodes/quickcodes.h>

// Gerar Aztec
quickcodes::generate_to_file(quickcodes::BarcodeType::Aztec, "Hello, QuickCodes!", "aztec.svg");
```

## ⚠️ Validações
- Aceita qualquer tipo de dados
- Tamanho ajustável automaticamente
- Nível de correção de erro configurável
- Não requer zona silenciosa (quiet zone)

## 🔗 Links Úteis
- [Especificação ISO/IEC 24778](https://www.iso.org/standard/41548.html)
- [Documentação QuickCodes](https://docs.rs/quickcodes)
- [Tutorial Aztec Code](https://www.barcodefaq.com/2d/aztec-code/)

## 📊 Casos de Uso
1. **Transporte**:
   - Bilhetes de trem
   - Passagens aéreas
   - Tickets de metrô
2. **Eventos**:
   - Ingressos de shows
   - Entradas de cinema
   - Credenciais
3. **Documentos**:
   - Carteiras de identidade
   - Vistos eletrônicos
   - Documentos oficiais
4. **Logística**:
   - Etiquetas de bagagem
   - Rastreamento de pacotes
   - Controle de acesso

## 💡 Dicas
- Não precisa de margem branca
- Ideal para telas e dispositivos móveis
- Use estrutura de dados padronizada
- Teste em diferentes resoluções
- Considere o nível de correção baseado no meio
- Ótimo para dados dinâmicos/atualizados frequentemente
