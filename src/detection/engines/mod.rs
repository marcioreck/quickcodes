// Detection Engines Module
// Format-specific detection algorithms inspired by ZXing

pub mod qr;
pub mod datamatrix;
pub mod pdf417;
pub mod aztec;
pub mod linear;

// Re-export main detection functions
pub use qr::detect_qr_candidates;
pub use datamatrix::detect_datamatrix_candidates;
pub use pdf417::detect_pdf417_candidates;
pub use aztec::detect_aztec_candidates;
pub use linear::detect_linear_candidates;
