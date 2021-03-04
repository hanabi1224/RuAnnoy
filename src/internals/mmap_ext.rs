use memmap2::Mmap;
use std::mem;
use std::slice;

pub(crate) trait MmapExtensions {
    fn read_i32(&self, idx: usize) -> i32;
    fn read_f32(&self, idx: usize) -> f32;
    fn read_slice<T: Sized>(&self, idx: usize, len: usize) -> &[T];
}

impl MmapExtensions for Mmap {
    fn read_i32(&self, idx: usize) -> i32 {
        let ptr: *const i32 = unsafe { mem::transmute(&self[idx]) };
        return unsafe { *ptr };
    }

    fn read_f32(&self, idx: usize) -> f32 {
        let ptr: *const f32 = unsafe { mem::transmute(&self[idx]) };
        return unsafe { *ptr };
    }

    fn read_slice<T: Sized>(&self, idx: usize, len: usize) -> &[T] {
        let ptr: *const T = unsafe { mem::transmute(&self[idx]) };
        unsafe { slice::from_raw_parts(ptr, len) }
    }
}
