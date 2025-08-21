"""
QuickCodes - Universal Barcode & QR Toolkit

A high-performance library for generating and reading barcodes and QR codes.
Built with Rust for maximum performance and Python bindings for ease of use.

Examples:
    Generate a QR Code:
    >>> import quickcodes
    >>> qr_data = quickcodes.generate("QRCode", "Hello, World!", "SVG")
    >>> with open("qr.svg", "wb") as f:
    ...     f.write(qr_data)
    
    Generate an EAN-13 barcode:
    >>> quickcodes.generate_to_file("EAN13", "123456789012", "barcode.png")
    
    Generate with custom settings:
    >>> qr_data = quickcodes.generate(
    ...     "QRCode", 
    ...     "Custom QR", 
    ...     "PNG",
    ...     width=200,
    ...     height=200,
    ...     margin=20,
    ...     error_correction="High"
    ... )
"""

from .quickcodes import *

__version__ = "0.1.0"
__author__ = "Márcio Reck"
__email__ = "marcio@fazmercado.com"

# Convenience constants
BARCODE_TYPES = {
    'QR_CODE': 'QRCode',
    'EAN13': 'EAN13', 
    'UPC_A': 'UPCA',
    'CODE128': 'Code128',
}

EXPORT_FORMATS = {
    'SVG': 'SVG',
    'PNG': 'PNG',
}

ERROR_CORRECTION_LEVELS = {
    'LOW': 'Low',
    'MEDIUM': 'Medium', 
    'QUARTILE': 'Quartile',
    'HIGH': 'High',
}

__all__ = [
    'generate',
    'generate_to_file',
    'BarcodeType',
    'ExportFormat', 
    'QRErrorCorrection',
    'BARCODE_TYPES',
    'EXPORT_FORMATS',
    'ERROR_CORRECTION_LEVELS',
]
