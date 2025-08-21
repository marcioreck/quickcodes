//! Python bindings for QuickCodes using PyO3

use crate::types::{BarcodeConfig, BarcodeType, ExportFormat, QRErrorCorrection};
use crate::{generate as rust_generate, generate_to_file as rust_generate_to_file};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyModule};

/// Python wrapper for BarcodeType enum
#[pyclass(name = "BarcodeType")]
#[derive(Clone)]
pub struct PyBarcodeType {
    inner: BarcodeType,
}

#[pymethods]
impl PyBarcodeType {
    #[classattr]
    const QR_CODE: &'static str = "QRCode";

    #[classattr]
    const EAN13: &'static str = "EAN13";

    #[classattr]
    const UPC_A: &'static str = "UPCA";

    #[classattr]
    const CODE128: &'static str = "Code128";

    #[classattr]
    const DATA_MATRIX: &'static str = "DataMatrix";

    #[classattr]
    const PDF417: &'static str = "PDF417";

    #[classattr]
    const AZTEC: &'static str = "Aztec";

    #[new]
    fn new(barcode_type: &str) -> PyResult<Self> {
        let inner = match barcode_type {
            "QRCode" => BarcodeType::QRCode,
            "EAN13" => BarcodeType::EAN13,
            "UPCA" => BarcodeType::UPCA,
            "Code128" => BarcodeType::Code128,
            "DataMatrix" => BarcodeType::DataMatrix,
            "PDF417" => BarcodeType::PDF417,
            "Aztec" => BarcodeType::Aztec,
            _ => {
                return Err(PyValueError::new_err(format!(
                    "Unknown barcode type: {}",
                    barcode_type
                )))
            }
        };
        Ok(PyBarcodeType { inner })
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.inner)
    }

    fn __repr__(&self) -> String {
        format!("BarcodeType('{:?}')", self.inner)
    }
}

/// Python wrapper for ExportFormat enum
#[pyclass(name = "ExportFormat")]
#[derive(Clone)]
pub struct PyExportFormat {
    inner: ExportFormat,
}

#[pymethods]
impl PyExportFormat {
    #[classattr]
    const SVG: &'static str = "SVG";

    #[classattr]
    const PNG: &'static str = "PNG";

    #[classattr]
    const PDF: &'static str = "PDF";

    #[new]
    fn new(format: &str) -> PyResult<Self> {
        let inner = match format {
            "SVG" => ExportFormat::SVG,
            "PNG" => ExportFormat::PNG,
            "PDF" => ExportFormat::PDF,
            _ => {
                return Err(PyValueError::new_err(format!(
                    "Unknown export format: {}",
                    format
                )))
            }
        };
        Ok(PyExportFormat { inner })
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.inner)
    }

    fn __repr__(&self) -> String {
        format!("ExportFormat('{:?}')", self.inner)
    }
}

/// Python wrapper for QRErrorCorrection enum
#[pyclass(name = "QRErrorCorrection")]
#[derive(Clone)]
pub struct PyQRErrorCorrection {
    inner: QRErrorCorrection,
}

#[pymethods]
impl PyQRErrorCorrection {
    #[classattr]
    const LOW: &'static str = "Low";

    #[classattr]
    const MEDIUM: &'static str = "Medium";

    #[classattr]
    const QUARTILE: &'static str = "Quartile";

    #[classattr]
    const HIGH: &'static str = "High";

    #[new]
    fn new(level: &str) -> PyResult<Self> {
        let inner = match level {
            "Low" => QRErrorCorrection::Low,
            "Medium" => QRErrorCorrection::Medium,
            "Quartile" => QRErrorCorrection::Quartile,
            "High" => QRErrorCorrection::High,
            _ => {
                return Err(PyValueError::new_err(format!(
                    "Unknown error correction level: {}",
                    level
                )))
            }
        };
        Ok(PyQRErrorCorrection { inner })
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.inner)
    }

    fn __repr__(&self) -> String {
        format!("QRErrorCorrection('{:?}')", self.inner)
    }
}

/// Generate a barcode and return the bytes
#[pyfunction]
#[pyo3(signature = (barcode_type, data, format, error_correction=None, margin=None))]
fn generate(
    py: Python,
    barcode_type: &str,
    data: &str,
    format: &str,
    error_correction: Option<&str>,
    margin: Option<u32>,
) -> PyResult<PyObject> {
    // Parse barcode type
    let bt = match barcode_type {
        // Phase 1: Core formats
        "QRCode" => BarcodeType::QRCode,
        "EAN13" => BarcodeType::EAN13,
        "UPCA" => BarcodeType::UPCA,
        "Code128" => BarcodeType::Code128,

        // Phase 2: Advanced 2D codes
        "DataMatrix" => BarcodeType::DataMatrix,
        "PDF417" => BarcodeType::PDF417,
        "Aztec" => BarcodeType::Aztec,
        _ => {
            return Err(PyValueError::new_err(format!(
                "Unsupported barcode type: {}",
                barcode_type
            )))
        }
    };

    // Parse export format
    let fmt = match format {
        "SVG" => ExportFormat::SVG,
        "PNG" => ExportFormat::PNG,
        "PDF" => ExportFormat::PDF,
        _ => {
            return Err(PyValueError::new_err(format!(
                "Unsupported export format: {}. Supported formats: SVG, PNG, PDF",
                format
            )))
        }
    };

    // Create config with optional parameters
    let mut config = BarcodeConfig::default();
    if let Some(m) = margin {
        config.margin = m;
    }
    if let Some(ec) = error_correction {
        config.qr_config.error_correction = match ec {
            "Low" => QRErrorCorrection::Low,
            "Medium" => QRErrorCorrection::Medium,
            "Quartile" => QRErrorCorrection::Quartile,
            "High" => QRErrorCorrection::High,
            _ => {
                return Err(PyValueError::new_err(format!(
                    "Unknown error correction: {}",
                    ec
                )))
            }
        };
    }

    // Generate barcode
    match rust_generate(bt, data, fmt) {
        Ok(bytes) => Ok(PyBytes::new(py, &bytes).into()),
        Err(e) => Err(PyValueError::new_err(format!("Generation failed: {}", e))),
    }
}

/// Read barcode from image file
#[pyfunction]
fn read_from_file(image_path: &str) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        #[cfg(feature = "readers")]
        {
            use crate::readers;
            match readers::read_from_file(image_path) {
                Ok(result) => {
                    let dict = pyo3::types::PyDict::new(py);
                    dict.set_item("barcode_type", format!("{:?}", result.barcode_type))?;
                    dict.set_item("data", result.data)?;
                    dict.set_item("confidence", result.confidence)?;
                    if let Some((x, y, w, h)) = result.position {
                        let pos_dict = pyo3::types::PyDict::new(py);
                        pos_dict.set_item("x", x)?;
                        pos_dict.set_item("y", y)?;
                        pos_dict.set_item("width", w)?;
                        pos_dict.set_item("height", h)?;
                        dict.set_item("position", pos_dict)?;
                    } else {
                        dict.set_item("position", py.None())?;
                    }
                    Ok(dict.into())
                }
                Err(e) => Err(PyValueError::new_err(format!("Read failed: {}", e))),
            }
        }

        #[cfg(not(feature = "readers"))]
        {
            Err(PyValueError::new_err(
                "Reader functionality not available - enable the 'readers' feature",
            ))
        }
    })
}

/// Read all barcodes from image file
#[pyfunction]
fn read_all_from_file(image_path: &str) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        #[cfg(feature = "readers")]
        {
            use crate::readers;
            match readers::read_all_from_file(image_path) {
                Ok(results) => {
                    let list = pyo3::types::PyList::empty(py);
                    for result in results {
                        let dict = pyo3::types::PyDict::new(py);
                        dict.set_item("barcode_type", format!("{:?}", result.barcode_type))?;
                        dict.set_item("data", result.data)?;
                        dict.set_item("confidence", result.confidence)?;
                        if let Some((x, y, w, h)) = result.position {
                            let pos_dict = pyo3::types::PyDict::new(py);
                            pos_dict.set_item("x", x)?;
                            pos_dict.set_item("y", y)?;
                            pos_dict.set_item("width", w)?;
                            pos_dict.set_item("height", h)?;
                            dict.set_item("position", pos_dict)?;
                        } else {
                            dict.set_item("position", py.None())?;
                        }
                        list.append(dict)?;
                    }
                    Ok(list.into())
                }
                Err(e) => Err(PyValueError::new_err(format!("Read failed: {}", e))),
            }
        }

        #[cfg(not(feature = "readers"))]
        {
            Err(PyValueError::new_err(
                "Reader functionality not available - enable the 'readers' feature",
            ))
        }
    })
}

/// Generate a barcode and save to file
#[pyfunction]
fn generate_to_file(barcode_type: &str, data: &str, output_path: &str) -> PyResult<()> {
    // Parse barcode type
    let bt = match barcode_type {
        // Phase 1: Core formats
        "QRCode" => BarcodeType::QRCode,
        "EAN13" => BarcodeType::EAN13,
        "UPCA" => BarcodeType::UPCA,
        "Code128" => BarcodeType::Code128,

        // Phase 2: Advanced 2D codes
        "DataMatrix" => BarcodeType::DataMatrix,
        "PDF417" => BarcodeType::PDF417,
        "Aztec" => BarcodeType::Aztec,
        _ => {
            return Err(PyValueError::new_err(format!(
                "Unsupported barcode type: {}",
                barcode_type
            )))
        }
    };

    // Generate and save
    match rust_generate_to_file(bt, data, output_path) {
        Ok(_) => Ok(()),
        Err(e) => Err(PyValueError::new_err(format!("Generation failed: {}", e))),
    }
}

/// QuickCodes Python module
#[pymodule]
fn quickcodes(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", "0.1.0")?;
    m.add("__author__", "Márcio Reck")?;

    // Add classes
    m.add_class::<PyBarcodeType>()?;
    m.add_class::<PyExportFormat>()?;
    m.add_class::<PyQRErrorCorrection>()?;

    // Add functions
    m.add_function(wrap_pyfunction!(generate, m)?)?;
    m.add_function(wrap_pyfunction!(generate_to_file, m)?)?;
    m.add_function(wrap_pyfunction!(read_from_file, m)?)?;
    m.add_function(wrap_pyfunction!(read_all_from_file, m)?)?;

    // Add constants for convenience
    // Phase 1: Core formats
    m.add("QR_CODE", "QRCode")?;
    m.add("EAN13", "EAN13")?;
    m.add("UPC_A", "UPCA")?;
    m.add("CODE128", "Code128")?;

    // Phase 2: Advanced 2D codes
    m.add("DATA_MATRIX", "DataMatrix")?;
    m.add("PDF417", "PDF417")?;
    m.add("AZTEC", "Aztec")?;

    m.add("SVG", "SVG")?;
    m.add("PNG", "PNG")?;
    m.add("PDF", "PDF")?;

    m.add("LOW", "Low")?;
    m.add("MEDIUM", "Medium")?;
    m.add("QUARTILE", "Quartile")?;
    m.add("HIGH", "High")?;

    Ok(())
}
