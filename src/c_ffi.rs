use super::AnnoyIndex;

/*
use libc::{c_char, size_t};

ffi_fn! {
    fn load_annoy_index(path: *const u8, dimension: *const dim) -> *const AnnoyIndex {
        let len = unsafe { CStr::from_ptr(pattern).to_bytes().len() };
        let pat = pattern as *const u8;
        let mut err = Error::new(ErrorKind::None);
        let re = rure_compile(
            pat, len, RURE_DEFAULT_FLAGS, ptr::null(), &mut err);
        if err.is_err() {
            let _ = writeln!(&mut io::stderr(), "{}", err);
            let _ = writeln!(
                &mut io::stderr(), "aborting from rure_compile_must");
            unsafe { abort() }
        }
        re
    }
}
*/

ffi_fn! {
    fn free_annoy_index(index: *const AnnoyIndex){
        unsafe { Box::from_raw(index as *mut AnnoyIndex); }
    }
}
