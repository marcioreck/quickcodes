# 📖 Plano de Implementação: Leitura e Decodificação

## 🎯 Objetivos

1. Implementar leitura de códigos de barras 1D e 2D
2. Suportar múltiplos formatos em uma única imagem
3. Garantir alta taxa de acerto em condições reais
4. Manter a API simples e consistente

## 📋 Etapas de Implementação

### 1. Preparação da Imagem
- [ ] Conversão para escala de cinza
- [ ] Binarização adaptativa (Otsu/local)
- [ ] Correção de perspectiva
- [ ] Redução de ruído
- [ ] Detecção de bordas

### 2. Detecção de Regiões
- [ ] Detecção de padrões finder (QR, DataMatrix, Aztec)
- [ ] Detecção de linhas paralelas (1D)
- [ ] Segmentação de regiões de interesse
- [ ] Classificação inicial do tipo de código

### 3. Decodificação por Formato
#### 1D (Lineares)
- [ ] Amostragem de linhas de varredura
- [ ] Detecção de barras e espaços
- [ ] Decodificação de padrões
- [ ] Validação de checksums

#### 2D (Matriciais)
- [ ] Extração da matriz de bits
- [ ] Correção de erros
- [ ] Decodificação de dados
- [ ] Validação de formato

### 4. Pós-processamento
- [ ] Validação de dados
- [ ] Formatação de saída
- [ ] Cálculo de confiança
- [ ] Agregação de resultados múltiplos

## 🧪 Estratégia de Testes

### Testes Unitários
1. **Processamento de Imagem**
   - Binarização
   - Detecção de bordas
   - Correção de perspectiva

2. **Detecção**
   - Padrões finder
   - Linhas paralelas
   - Regiões de interesse

3. **Decodificação**
   - Amostragem
   - Extração de bits
   - Correção de erros

### Testes de Integração
1. **Pipeline Completo**
   - Imagem → Detecção → Decodificação
   - Múltiplos códigos
   - Diferentes formatos

2. **Casos Especiais**
   - Rotação
   - Perspectiva
   - Iluminação
   - Danos parciais

### Testes Manuais

#### Dispositivos para Teste
1. **Smartphones** (câmeras mais comuns)
   - Android nativo (Google Camera)
   - iPhone câmera nativa
   - WhatsApp câmera
   - Apps bancários

2. **Webcams**
   - Webcam laptop
   - Webcam USB básica

3. **Scanners**
   - Scanner de mesa doméstico
   - App de scanner (ex: Adobe Scan)

#### Cenários de Teste
1. **Condições de Iluminação**
   - Luz natural
   - Luz artificial
   - Baixa luminosidade
   - Reflexos

2. **Ângulos e Distâncias**
   - Frontal (90°)
   - Ângulos variados (45°-90°)
   - Distâncias variadas
   - Em movimento

3. **Superfícies**
   - Papel branco
   - Papel colorido
   - Tela (display)
   - Superfícies reflexivas

4. **Qualidade do Código**
   - Impressão profissional
   - Impressão caseira
   - Display digital
   - Códigos danificados

## 🛠️ Ferramentas e Bibliotecas

### Processamento de Imagem
- **image** (Rust) - Operações básicas
- **imageproc** - Algoritmos de processamento
- **opencv** (opcional) - Algoritmos avançados

### Decodificação
- Implementação própria dos algoritmos
- Possível uso de **zxing-rs** como referência
- **reed-solomon** para correção de erros

### Testes e Benchmarking
- **criterion** - Benchmarks
- **proptest** - Testes baseados em propriedades
- **test-case** - Testes parametrizados

## ⚠️ Pontos Críticos

1. **Performance**
   - Otimização de algoritmos de processamento
   - Paralelização quando possível
   - Cache de resultados intermediários

2. **Precisão**
   - Balanceamento entre falsos positivos/negativos
   - Validação rigorosa de checksums
   - Múltiplas tentativas de decodificação

3. **Recursos**
   - Uso de memória em imagens grandes
   - Tempo de processamento
   - Compatibilidade com dispositivos limitados

4. **Compatibilidade**
   - Diferentes formatos de imagem
   - Variações de implementação dos códigos
   - Diferentes ambientes de execução

## 📊 Métricas de Sucesso

1. **Taxa de Acerto**
   - \>95% em condições ideais
   - \>80% em condições normais
   - \>50% em condições adversas

2. **Performance**
   - <500ms para códigos únicos
   - <2s para múltiplos códigos
   - <100MB de uso de memória

3. **Cobertura de Testes**
   - 100% de cobertura de código
   - Todos os casos de erro testados
   - Todos os formatos testados

## 📅 Fases de Lançamento

### Fase 1: Leitura Básica
1. Implementação inicial para QR Code
2. Testes unitários básicos
3. Documentação inicial

### Fase 2: Expansão de Formatos
1. Adição de formatos 1D
2. Adição de outros formatos 2D
3. Testes de integração

### Fase 3: Otimização
1. Melhorias de performance
2. Tratamento de casos especiais
3. Documentação completa

### Fase 4: Lançamento
1. Testes finais
2. Exemplos e tutoriais
3. Release público

## 📚 Documentação

1. **Guia de Uso**
   - API de leitura
   - Configurações
   - Exemplos

2. **Guia Técnico**
   - Algoritmos utilizados
   - Estrutura interna
   - Pontos de extensão

3. **Troubleshooting**
   - Problemas comuns
   - Soluções
   - Limitações conhecidas

## 📄 Folha de Testes Impressa

### Arquivo `test_sheet.pdf`
Arquivo PDF em A4 com todos os códigos para teste de leitura manual.

### Layout
- Orientação: Retrato (vertical)
- Tamanho: A4 (210 × 297 mm)
- Margens: 15mm em todos os lados
- Fonte: Arial (texto normal) e Courier New (dados)

### Conteúdo por Seção

#### 1. Códigos 1D (Lineares)

1. **EAN-13**
   - Código: [▮▮▮ 1234567890128 ▮▮▮]
   - Texto esperado: "1234567890128"
   - Tamanho: 50mm × 20mm

2. **UPC-A**
   - Código: [▮▮▮ 036000291452 ▮▮▮]
   - Texto esperado: "036000291452"
   - Tamanho: 50mm × 20mm

3. **Code128**
   - Código: [▮▮▮ HELLO123 ▮▮▮]
   - Texto esperado: "HELLO123"
   - Tamanho: 60mm × 20mm

4. **Code39**
   - Código: [▮▮▮ SERIAL-123ABC ▮▮▮]
   - Texto esperado: "SERIAL-123ABC"
   - Tamanho: 70mm × 20mm

5. **ITF-14**
   - Código: [▮▮▮ 12345678901231 ▮▮▮]
   - Texto esperado: "12345678901231"
   - Tamanho: 70mm × 25mm

6. **Codabar**
   - Código: [▮▮▮ A1234567890B ▮▮▮]
   - Texto esperado: "A1234567890B"
   - Tamanho: 60mm × 20mm

#### 2. Códigos 2D (Matriciais)

7. **QR Code**
   - Código: [▮▮▮ QR ▮▮▮]
   - Texto esperado: "https://github.com/marcioreck/quickcodes"
   - Tamanho: 30mm × 30mm

8. **DataMatrix**
   - Código: [▮▮▮ DM ▮▮▮]
   - Texto esperado: "010123456789012815240101"
   - Tamanho: 25mm × 25mm

9. **PDF417**
   - Código: [▮▮▮ PDF ▮▮▮]
   - Texto esperado: "DRIVER LICENSE|DOE,JOHN|DOB:1990-01-01"
   - Tamanho: 60mm × 20mm

10. **Aztec**
    - Código: [▮▮▮ AZ ▮▮▮]
    - Texto esperado: "TKT:A12345|FROM:NYC|TO:BOS|DATE:2025-08-21"
    - Tamanho: 30mm × 30mm

### Instruções de Teste na Folha

1. **Cabeçalho**
   ```
   QuickCodes - Folha de Teste de Leitura v1.0
   Data de Geração: YYYY-MM-DD
   ```

2. **Para cada código**
   ```
   Tipo: [Nome do Código]
   Dados Esperados: [Texto que deve ser lido]
   Resultado: □ OK  □ Falha
   Observações: _________________________
   ```

3. **Rodapé**
   ```
   Dispositivo de Leitura: _________________
   App Utilizado: _________________
   Data do Teste: _________________
   Condições de Luz: □ Natural  □ Artificial  □ Baixa
   ```

### Geração do PDF

1. **Script de Geração**
   - Nome: `generate_test_sheet.rs`
   - Localização: `examples/`
   - Dependências: 
     - `printpdf` para geração do PDF
     - `quickcodes` para geração dos códigos

2. **Execução**
   ```bash
   cargo run --example generate_test_sheet
   ```

3. **Saída**
   - Arquivo: `examples/output/test_sheet.pdf`
   - Resolução: 300 DPI para melhor qualidade de impressão

### Instruções de Uso

1. **Impressão**
   - Usar impressora laser de boa qualidade
   - Papel branco A4 75g/m²
   - Configurar para "Tamanho Real" (não ajustar à página)
   - Verificar qualidade da impressão

2. **Teste**
   - Usar diferentes apps de leitura
   - Testar em diferentes condições de luz
   - Anotar resultados na própria folha
   - Fotografar folha preenchida para documentação

3. **Documentação**
   - Criar planilha com resultados
   - Registrar problemas encontrados
   - Sugerir melhorias no formato/tamanho