use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::slice;
use std::str::FromStr;

use crate::{generate, generate_to_file, read_from_file, read_from_bytes};
use crate::types::{BarcodeType, ExportFormat};

#[repr(C)]
pub struct QuickCodesError {
    message: *mut c_char,
    code: c_int,
}

#[repr(C)]
pub struct QuickCodesResult {
    data: *mut u8,
    len: usize,
    error: *mut QuickCodesError,
}

impl Drop for QuickCodesResult {
    fn drop(&mut self) {
        unsafe {
            if !self.data.is_null() {
                let _ = Box::from_raw(slice::from_raw_parts_mut(self.data, self.len));
            }
            if !self.error.is_null() {
                let error = Box::from_raw(self.error);
                if !error.message.is_null() {
                    let _ = CString::from_raw(error.message);
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn quickcodes_generate(
    barcode_type: *const c_char,
    data: *const c_char,
) -> *mut QuickCodesResult {
    let result = catch_unwind_result(|| {
        let barcode_type = unsafe { CStr::from_ptr(barcode_type) }.to_str()?;
        let data = unsafe { CStr::from_ptr(data) }.to_str()?;
        
        let barcode_type = BarcodeType::from_str(barcode_type)?;
        let output = generate(barcode_type, data, ExportFormat::PNG)?;
        
        Ok(output)
    });

    Box::into_raw(Box::new(result))
}

#[no_mangle]
pub extern "C" fn quickcodes_generate_to_file(
    barcode_type: *const c_char,
    data: *const c_char,
    output_path: *const c_char,
) -> *mut QuickCodesResult {
    let result = catch_unwind_result(|| {
        let barcode_type = unsafe { CStr::from_ptr(barcode_type) }.to_str()?;
        let data = unsafe { CStr::from_ptr(data) }.to_str()?;
        let output_path = unsafe { CStr::from_ptr(output_path) }.to_str()?;
        
        let barcode_type = BarcodeType::from_str(barcode_type)?;
        generate_to_file(barcode_type, data, output_path)?;
        
        Ok(Vec::new()) // Empty success result
    });

    Box::into_raw(Box::new(result))
}

#[no_mangle]
pub extern "C" fn quickcodes_read_from_file(
    file_path: *const c_char,
) -> *mut QuickCodesResult {
    let result = catch_unwind_result(|| {
        let file_path = unsafe { CStr::from_ptr(file_path) }.to_str()?;
        let output = read_from_file(file_path)?;
        
        let data = CString::new(output.data)?;
        Ok(data.into_bytes_with_nul())
    });

    Box::into_raw(Box::new(result))
}

#[no_mangle]
pub extern "C" fn quickcodes_read_from_bytes(
    data: *const u8,
    len: usize,
) -> *mut QuickCodesResult {
    let result = catch_unwind_result(|| {
        let data = unsafe { slice::from_raw_parts(data, len) };
        let output = read_from_bytes(data, None)?;
        
        let data = CString::new(output.data)?;
        Ok(data.into_bytes_with_nul())
    });

    Box::into_raw(Box::new(result))
}

#[no_mangle]
pub extern "C" fn quickcodes_free_result(result: *mut QuickCodesResult) {
    if !result.is_null() {
        unsafe {
            drop(Box::from_raw(result));
        }
    }
}

fn catch_unwind_result<F>(f: F) -> QuickCodesResult
where
    F: FnOnce() -> anyhow::Result<Vec<u8>>,
{
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(Ok(data)) => {
            let data_box = data.into_boxed_slice();
            let len = data_box.len();
            let ptr = Box::into_raw(data_box) as *mut u8;
            QuickCodesResult {
                data: ptr,
                len,
                error: std::ptr::null_mut(),
            }
        }
        Ok(Err(e)) => {
            let message = CString::new(e.to_string()).unwrap_or_else(|_| {
                CString::new("Error converting error message").unwrap()
            });
            QuickCodesResult {
                data: std::ptr::null_mut(),
                len: 0,
                error: Box::into_raw(Box::new(QuickCodesError {
                    message: message.into_raw(),
                    code: 1,
                })),
            }
        }
        Err(_) => QuickCodesResult {
            data: std::ptr::null_mut(),
            len: 0,
            error: Box::into_raw(Box::new(QuickCodesError {
                message: CString::new("Panic in Rust code")
                    .unwrap()
                    .into_raw(),
                code: 2,
            })),
        },
    }
}