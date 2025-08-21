# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-01-20

### Added
- **Core Barcode Generation**: Support for QR Code, EAN-13, UPC-A, and Code128
- **Multiple Export Formats**: SVG (vector) and PNG (raster) output
- **High-Quality Implementation**: 
  - Automatic checksum calculation for EAN-13 and UPC-A
  - Multiple QR Code error correction levels (Low, Medium, Quartile, High)
  - Industry-standard barcode patterns
- **Rust API**: Complete native Rust library with clean, ergonomic API
- **Python Bindings**: PyO3-based bindings for Python integration
- **Comprehensive Testing**: 40 tests (25 unit + 12 integration + 3 doctests)
- **Documentation**: Complete API documentation with working examples
- **CI/CD Pipeline**: Multi-platform testing (Linux, Windows, macOS) with multiple Rust versions
- **Code Quality**: Zero warnings, Clippy-approved, auto-formatted
- **Examples**: Practical examples including Brazilian Pix payments, product codes, URLs
- **Performance**: Sub-second generation times with thread safety
- **Cross-Platform**: Works on all major operating systems

### Features
- Generate QR Codes with customizable error correction
- Generate EAN-13 barcodes with automatic checksum
- Generate UPC-A barcodes with validation
- Generate Code128 barcodes for alphanumeric data
- Export to SVG (scalable vector format)
- Export to PNG (high-quality raster format)
- Python bindings for easy integration
- Thread-safe concurrent generation
- Comprehensive error handling
- File extension auto-detection

### Documentation
- Complete API documentation
- Working code examples in documentation
- Practical usage examples (Pix payments, product codes, WiFi QR codes)
- Performance benchmarks and testing reports
- Python usage guide with examples

### Quality Assurance
- 100% test coverage of public API
- Multi-platform CI/CD testing
- Security audit integration
- Dependency vulnerability monitoring
- Code formatting and linting enforcement
- Performance regression testing

[Unreleased]: https://github.com/marcioreck/quickcodes/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/marcioreck/quickcodes/releases/tag/v0.1.0
