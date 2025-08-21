//! Barcode generators for different formats

pub mod code128;
pub mod ean13;
pub mod qr;
pub mod upc;

// Phase 2: Advanced 2D codes
pub mod datamatrix;
pub mod pdf417;
pub mod aztec;

// Re-export generator functions
pub use code128::generate_code128;
pub use ean13::generate_ean13;
pub use qr::generate_qr;
pub use upc::generate_upc_a;

// Phase 2 generators
pub use datamatrix::generate_datamatrix;
pub use pdf417::generate_pdf417;
pub use aztec::generate_aztec;
