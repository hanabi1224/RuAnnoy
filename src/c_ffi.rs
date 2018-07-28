use super::AnnoyIndex;
use super::IndexType;

use std::mem;
use libc::c_char;
use std::ffi::CStr;

ffi_fn! {
    fn load_annoy_index(path: *const c_char, dimension: *const i32, index_type: *const u8) -> *const AnnoyIndex {
        let ru_dimension = unsafe {*dimension} ;
        let c_str_path =  unsafe { CStr::from_ptr(path) };
        let ru_path = c_str_path.to_str().unwrap();
        let ru_index_type:IndexType = unsafe { mem::transmute(*index_type) };
        let index = AnnoyIndex::load(ru_dimension, ru_path, ru_index_type);
        return Box::into_raw(Box::new(index));
    }
}

ffi_fn! {
    fn free_annoy_index(index: *const AnnoyIndex){
        unsafe { Box::from_raw(index as *mut AnnoyIndex); }
    }
}
