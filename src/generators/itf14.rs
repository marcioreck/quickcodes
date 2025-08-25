use crate::types::{Barcode, BarcodeModules, BarcodeType, BarcodeConfig};
use anyhow::Result;

// Padrões de codificação ITF-14
const ITF_PATTERNS: &[(&str, &str)] = &[
    ("0", "00110"), ("1", "10001"), ("2", "01001"), ("3", "11000"),
    ("4", "00101"), ("5", "10100"), ("6", "01100"), ("7", "00011"),
    ("8", "10010"), ("9", "01010"),
];

/// Calcula o dígito verificador ITF-14
fn calculate_check_digit(data: &str) -> u8 {
    let mut sum = 0;
    for (i, c) in data.chars().enumerate() {
        let digit = c.to_digit(10).unwrap() as u8;
        // Posições pares têm peso 3, ímpares têm peso 1
        sum += if i % 2 == 0 { digit * 3 } else { digit };
    }
    ((10 - (sum % 10)) % 10) as u8
}

/// Gera um código de barras ITF-14
///
/// O ITF-14 é usado principalmente em embalagens de produtos e logística.
/// Ele codifica exatamente 14 dígitos numéricos, incluindo um dígito verificador.
///
/// # Arguments
///
/// * `data` - Os dados a serem codificados (13 dígitos, o 14º será calculado)
///
/// # Returns
///
/// Retorna um `Result` contendo o código de barras gerado ou um erro
pub fn generate_itf14(data: &str) -> Result<Barcode> {
    // Validar comprimento
    if data.len() != 13 {
        return Err(anyhow::anyhow!(
            "ITF-14 data must be exactly 13 digits (check digit will be calculated)"
        ));
    }

    // Validar que são apenas dígitos
    if !data.chars().all(|c| c.is_ascii_digit()) {
        return Err(anyhow::anyhow!("ITF-14 data must contain only digits"));
    }

    // Calcular dígito verificador
    let check_digit = calculate_check_digit(data);
    let data = format!("{}{}", data, check_digit);

    // Padrão de início
    let mut modules = String::from("1010"); // start pattern

    // Codificar pares de dígitos
    for chunk in data.as_bytes().chunks(2) {
        let first = (chunk[0] - b'0') as usize;
        let second = (chunk[1] - b'0') as usize;

        // Obter os padrões para os dois dígitos
        let (_, pattern1) = ITF_PATTERNS[first];
        let (_, pattern2) = ITF_PATTERNS[second];

        // Entrelaçar os padrões
        for i in 0..5 {
            // Primeiro dígito gera barras, segundo gera espaços
            if pattern1.as_bytes()[i] == b'1' {
                modules.push_str("11"); // barra larga
            } else {
                modules.push('1'); // barra estreita
            }
            if pattern2.as_bytes()[i] == b'1' {
                modules.push_str("00"); // espaço largo
            } else {
                modules.push('0'); // espaço estreito
            }
        }
    }

    // Padrão de fim
    modules.push_str("11101"); // stop pattern

    // Converter string de módulos em vetor de booleanos
    let modules: Vec<bool> = modules.chars().map(|c| c == '1').collect();

    // Criar configuração padrão
    let mut config = BarcodeConfig::default();
    config.width = modules.len() as u32;
    config.height = config.width / 2;

    Ok(Barcode {
        barcode_type: BarcodeType::ITF14,
        data: data.to_string(),
        modules: BarcodeModules::Linear(modules),
        config,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_itf14_check_digit() {
        assert_eq!(calculate_check_digit("0123456789012"), 8);
        assert_eq!(calculate_check_digit("1234567890123"), 1);
    }

    #[test]
    fn test_itf14_invalid_length() {
        let result = generate_itf14("12345");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("13 digits"));
    }

    #[test]
    fn test_itf14_non_numeric() {
        let result = generate_itf14("12345678901AB");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("only digits"));
    }

    #[test]
    fn test_itf14_valid_data() {
        let result = generate_itf14("0123456789012");
        assert!(result.is_ok());
        let barcode = result.unwrap();
        assert!(!barcode.modules.as_linear().unwrap().is_empty());
        assert_eq!(barcode.data.len(), 14);
    }

    #[test]
    fn test_itf14_check_digit_validation() {
        let result = generate_itf14("0123456789012");
        assert!(result.is_ok());
        let barcode = result.unwrap();
        assert_eq!(barcode.data, "01234567890128");
    }
}