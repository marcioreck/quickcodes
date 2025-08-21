//! Barcode generators for different formats

pub mod code128;
pub mod ean13;
pub mod qr;
pub mod upc;

// Re-export generator functions
pub use code128::generate_code128;
pub use ean13::generate_ean13;
pub use qr::generate_qr;
pub use upc::generate_upc_a;
