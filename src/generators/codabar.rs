use crate::types::{Barcode, BarcodeModules, BarcodeType, BarcodeConfig};
use anyhow::Result;

// Padrões de codificação Codabar
const CODABAR_PATTERNS: &[(&str, &str)] = &[
    ("0", "0000011"), ("1", "0000110"), ("2", "0001001"), ("3", "1100000"),
    ("4", "0010010"), ("5", "1000010"), ("6", "0100001"), ("7", "0100100"),
    ("8", "0110000"), ("9", "1001000"), ("-", "0001100"), ("$", "0011000"),
    (":", "1000101"), ("/", "1010001"), (".", "1010100"), ("+", "0010101"),
    ("A", "1011001"), ("B", "1001011"), ("C", "0011011"), ("D", "0011101"),
];

/// Gera um código de barras Codabar
///
/// O Codabar é usado principalmente em bibliotecas, bancos de sangue e etiquetas de remessa.
/// Ele pode codificar dígitos, alguns símbolos especiais e as letras A-D (usadas como start/stop).
///
/// # Arguments
///
/// * `data` - Os dados a serem codificados
///
/// # Returns
///
/// Retorna um `Result` contendo o código de barras gerado ou um erro
pub fn generate_codabar(data: &str) -> Result<Barcode> {
    // Validar dados de entrada
    if data.is_empty() {
        return Err(anyhow::anyhow!("Data cannot be empty"));
    }

    // Converter para maiúsculas
    let data = data.to_uppercase();

    // Validar caracteres
    for c in data.chars() {
        if !CODABAR_PATTERNS.iter().any(|(ch, _)| *ch == c.to_string()) {
            return Err(anyhow::anyhow!(
                "Invalid character '{}' for Codabar. Only 0-9, -$:/.+ and A-D are allowed.",
                c
            ));
        }
    }

    // Verificar se começa e termina com A-D
    let first = data.chars().next().unwrap();
    let last = data.chars().last().unwrap();
    if !matches!(first, 'A'..='D') || !matches!(last, 'A'..='D') {
        return Err(anyhow::anyhow!(
            "Codabar data must start and end with A, B, C, or D"
        ));
    }

    // Gerar módulos
    let mut modules = String::new();

    for (i, c) in data.chars().enumerate() {
        // Adicionar espaço entre caracteres
        if i > 0 {
            modules.push('0'); // espaço entre caracteres
        }

        // Encontrar o padrão de barras para o caractere
        if let Some((_, pattern)) = CODABAR_PATTERNS.iter().find(|(ch, _)| *ch == c.to_string()) {
            modules.push_str(pattern);
        }
    }

    // Converter string de módulos em vetor de booleanos
    let modules: Vec<bool> = modules.chars().map(|c| c == '1').collect();

    // Criar configuração padrão
    let mut config = BarcodeConfig::default();
    config.width = modules.len() as u32;
    config.height = config.width / 2;

    Ok(Barcode {
        barcode_type: BarcodeType::Codabar,
        data: data.to_string(),
        modules: BarcodeModules::Linear(modules),
        config,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codabar_empty_data() {
        let result = generate_codabar("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("empty"));
    }

    #[test]
    fn test_codabar_invalid_chars() {
        let result = generate_codabar("A123X456B");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid character"));
    }

    #[test]
    fn test_codabar_missing_start_stop() {
        let result = generate_codabar("12345");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("start and end"));
    }

    #[test]
    fn test_codabar_valid_numeric() {
        let result = generate_codabar("A1234567890B");
        assert!(result.is_ok());
        let barcode = result.unwrap();
        assert!(!barcode.modules.as_linear().unwrap().is_empty());
    }

    #[test]
    fn test_codabar_valid_special_chars() {
        let result = generate_codabar("A-$:/.+B");
        assert!(result.is_ok());
        let barcode = result.unwrap();
        assert!(!barcode.modules.as_linear().unwrap().is_empty());
    }

    #[test]
    fn test_codabar_case_insensitive() {
        let result = generate_codabar("a123b");
        assert!(result.is_ok());
        let barcode = result.unwrap();
        assert!(!barcode.modules.as_linear().unwrap().is_empty());
    }

    #[test]
    fn test_codabar_different_start_stop() {
        let result = generate_codabar("A1234C");
        assert!(result.is_ok());
        let barcode = result.unwrap();
        assert!(!barcode.modules.as_linear().unwrap().is_empty());
    }
}