//! Barcode generators for different formats

pub mod code128;
pub mod ean13;
pub mod qr;
pub mod upc;

// Phase 2: Advanced 2D codes
pub mod aztec;
pub mod datamatrix;
pub mod pdf417;

// Phase 3: Legacy formats
pub mod code39;
pub mod codabar;
pub mod itf14;

// Re-export generator functions
pub use code128::generate_code128;
pub use ean13::generate_ean13;
pub use qr::generate_qr;
pub use upc::generate_upc_a;

// Phase 2 generators
pub use aztec::generate_aztec;
pub use datamatrix::generate_datamatrix;
pub use pdf417::generate_pdf417;

// Phase 3 generators
pub use codabar::generate_codabar;
pub use code39::generate_code39;
pub use itf14::generate_itf14;