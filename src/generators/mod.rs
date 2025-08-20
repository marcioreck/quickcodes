//! Barcode generators for different formats

pub mod qr;
pub mod ean13;
pub mod upc;
pub mod code128;

// Re-export generator functions
pub use qr::generate_qr;
pub use ean13::generate_ean13;
pub use upc::generate_upc_a;
pub use code128::generate_code128;
