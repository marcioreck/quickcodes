use crate::types::{Barcode, BarcodeModules, BarcodeType, BarcodeConfig};
use anyhow::Result;

// Tabela de codificação Code39
// Cada caractere é representado por 9 módulos (5 barras e 4 espaços)
const CODE39_CHARS: &[(&str, &str)] = &[
    ("0", "000110100"), ("1", "100100001"), ("2", "001100001"), ("3", "101100000"),
    ("4", "000110001"), ("5", "100110000"), ("6", "001110000"), ("7", "000100101"),
    ("8", "100100100"), ("9", "001100100"), ("A", "100001001"), ("B", "001001001"),
    ("C", "101001000"), ("D", "000011001"), ("E", "100011000"), ("F", "001011000"),
    ("G", "000001101"), ("H", "100001100"), ("I", "001001100"), ("J", "000011100"),
    ("K", "100000011"), ("L", "001000011"), ("M", "101000010"), ("N", "000010011"),
    ("O", "100010010"), ("P", "001010010"), ("Q", "000000111"), ("R", "100000110"),
    ("S", "001000110"), ("T", "000010110"), ("U", "110000001"), ("V", "011000001"),
    ("W", "111000000"), ("X", "010010001"), ("Y", "110010000"), ("Z", "011010000"),
    ("-", "010000101"), (".", "110000100"), (" ", "011000100"), ("$", "010101000"),
    ("/", "010100010"), ("+", "010001010"), ("%", "000101010"), ("*", "010010100"),
];

/// Gera um código de barras Code39
///
/// O Code39 pode codificar caracteres alfanuméricos (A-Z, 0-9) e alguns símbolos especiais.
/// Cada caractere é representado por 5 barras e 4 espaços, onde 3 são largos e 6 são estreitos.
///
/// # Arguments
///
/// * `data` - Os dados a serem codificados
///
/// # Returns
///
/// Retorna um `Result` contendo o código de barras gerado ou um erro
pub fn generate_code39(data: &str) -> Result<Barcode> {
    // Validar dados de entrada
    if data.is_empty() {
        return Err(anyhow::anyhow!("Data cannot be empty"));
    }

    // Converter para maiúsculas e validar caracteres
    let data = data.to_uppercase();
    for c in data.chars() {
        if !CODE39_CHARS.iter().any(|(ch, _)| *ch == c.to_string()) {
            return Err(anyhow::anyhow!(
                "Invalid character '{}' for Code39. Only A-Z, 0-9, and -. $/+% are allowed.",
                c
            ));
        }
    }

    // Adicionar asteriscos de início/fim
    let data_with_markers = format!("*{}*", data);

    // Gerar módulos
    let mut modules = String::with_capacity(data_with_markers.len() * 10 + data_with_markers.len() - 1);
    
    for (i, c) in data_with_markers.chars().enumerate() {
        // Adicionar espaço entre caracteres
        if i > 0 {
            modules.push('0'); // espaço entre caracteres
        }

        // Encontrar o padrão de barras para o caractere
        if let Some((_, pattern)) = CODE39_CHARS.iter().find(|(ch, _)| *ch == c.to_string()) {
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
        barcode_type: BarcodeType::Code39,
        data: data.to_string(),
        modules: BarcodeModules::Linear(modules),
        config,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code39_empty_data() {
        let result = generate_code39("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("empty"));
    }

    #[test]
    fn test_code39_invalid_chars() {
        let result = generate_code39("ABC123!");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid character"));
    }

    #[test]
    fn test_code39_valid_numeric() {
        let result = generate_code39("1234567890");
        assert!(result.is_ok());
        let barcode = result.unwrap();
        assert!(!barcode.modules.as_linear().unwrap().is_empty());
    }

    #[test]
    fn test_code39_valid_alpha() {
        let result = generate_code39("ABCDEFGHIJ");
        assert!(result.is_ok());
        let barcode = result.unwrap();
        assert!(!barcode.modules.as_linear().unwrap().is_empty());
    }

    #[test]
    fn test_code39_valid_special_chars() {
        let result = generate_code39("-. $/+%");
        assert!(result.is_ok());
        let barcode = result.unwrap();
        assert!(!barcode.modules.as_linear().unwrap().is_empty());
    }

    #[test]
    fn test_code39_case_insensitive() {
        let result = generate_code39("abc");
        assert!(result.is_ok());
        let barcode = result.unwrap();
        assert!(!barcode.modules.as_linear().unwrap().is_empty());
    }
}