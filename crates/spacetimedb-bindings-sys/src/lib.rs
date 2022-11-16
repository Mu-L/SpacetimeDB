#![no_std]

extern crate alloc as alloc_crate;

use alloc_crate::alloc;

use alloc_crate::boxed::Box;

pub const ABI_VERSION: u16 = 0;

pub mod raw {
    // breaking changes to these function signatures should bump the v in the module symbol
    // as well as in ABI_VERSION above
    #[link(wasm_import_module = "spacetime_v0")]
    extern "C" {
        pub fn _create_table(ptr: *const u8, len: usize) -> u32;
        pub fn _get_table_id(ptr: *const u8, len: usize) -> u32;
        pub fn _create_index(table_id: u32, col_id: u32, index_type: u8);

        pub fn _insert(table_id: u32, ptr: *const u8, len: usize) -> u8;

        pub fn _delete_pk(table_id: u32, ptr: *const u8, len: usize) -> u8;
        pub fn _delete_value(table_id: u32, ptr: *const u8, len: usize) -> u8;
        pub fn _delete_eq(table_id: u32, col_id: u32, ptr: *const u8, len: usize) -> u32;
        pub fn _delete_range(table_id: u32, col_id: u32, ptr: *const u8, len: usize) -> u32;

        // pub fn _filter_eq(table_id: u32, col_id: u32, src_ptr: *const u8, result_ptr: *const u8);

        pub fn _iter(table_id: u32) -> u64;
        pub fn _console_log(level: u8, ptr: *const u8, len: usize);
    }
}

pub struct BindingError {}

fn cvt_count(x: u32) -> Result<u32, BindingError> {
    if x == u32::MAX {
        Err(BindingError {})
    } else {
        Ok(x)
    }
}
fn cvt(x: u8) -> Result<(), BindingError> {
    if x == 0 {
        Err(BindingError {})
    } else {
        Ok(())
    }
}

#[inline]
pub fn create_table(data: &[u8]) -> u32 {
    unsafe { raw::_create_table(data.as_ptr(), data.len()) }
}
#[inline]
pub fn get_table_id(data: &[u8]) -> u32 {
    unsafe { raw::_get_table_id(data.as_ptr(), data.len()) }
}
#[inline]
pub fn create_index(table_id: u32, col_id: u32, index_type: u8) {
    unsafe { raw::_create_index(table_id, col_id, index_type) }
}

#[inline]
pub fn insert(table_id: u32, data: &[u8]) -> Result<(), BindingError> {
    cvt(unsafe { raw::_insert(table_id, data.as_ptr(), data.len()) })
}

#[inline]
pub fn delete_pk(table_id: u32, data: &[u8]) -> Result<(), BindingError> {
    cvt(unsafe { raw::_delete_pk(table_id, data.as_ptr(), data.len()) })
}
#[inline]
pub fn delete_value(table_id: u32, data: &[u8]) -> Result<(), BindingError> {
    cvt(unsafe { raw::_delete_value(table_id, data.as_ptr(), data.len()) })
}
#[inline]
pub fn delete_eq(table_id: u32, col_id: u32, data: &[u8]) -> Result<u32, BindingError> {
    cvt_count(unsafe { raw::_delete_eq(table_id, col_id, data.as_ptr(), data.len()) })
}
#[inline]
pub fn delete_range(table_id: u32, col_id: u32, data: &[u8]) -> Result<u32, BindingError> {
    cvt_count(unsafe { raw::_delete_eq(table_id, col_id, data.as_ptr(), data.len()) })
}

// not yet implemented
// #[inline]
// pub fn filter_eq(table_id: u32, col_id: u32, src_ptr: *mut u8, result_ptr: *mut u8) {}

#[inline]
pub fn iter(table_id: u32) -> Box<[u8]> {
    unsafe { unpack_slice(raw::_iter(table_id)) }
}
#[inline]
pub fn console_log(level: u8, data: &[u8]) {
    unsafe { raw::_console_log(level, data.as_ptr(), data.len()) }
}

pub fn pack_slice(b: Box<[u8]>) -> u64 {
    let len = b.len();
    let ptr = Box::into_raw(b) as *mut u8;
    (ptr as usize as u64) << 32 | (len as u64)
}

pub unsafe fn unpack_slice(packed: u64) -> Box<[u8]> {
    let ptr = (packed >> 32) as u32 as *mut u8;
    let len = packed as usize;
    Box::from_raw(core::ptr::slice_from_raw_parts_mut(ptr, len))
}

const ALLOC_ALIGN: usize = core::mem::align_of::<usize>();

#[no_mangle]
extern "C" fn alloc(size: usize) -> *mut u8 {
    unsafe {
        let layout = alloc::Layout::from_size_align_unchecked(size, ALLOC_ALIGN);
        alloc::alloc(layout)
    }
}

#[no_mangle]
extern "C" fn dealloc(ptr: *mut u8, size: usize) {
    unsafe {
        let layout = alloc::Layout::from_size_align_unchecked(size, ALLOC_ALIGN);
        alloc::dealloc(ptr, layout);
    }
}

// TODO: eventually there should be a way to set a consistent random seed for a module
#[cfg(feature = "getrandom")]
fn fake_random(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    for i in 0..buf.len() {
        let start = match i % 4 {
            0 => 0x64,
            1 => 0xe9,
            2 => 0x48,
            _ => 0xb5,
        };
        buf[i] = (start ^ i) as u8;
    }

    Result::Ok(())
}
#[cfg(feature = "getrandom")]
getrandom::register_custom_getrandom!(fake_random);
