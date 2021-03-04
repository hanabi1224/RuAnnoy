use memmap2::Mmap;
use std::mem;
use std::slice;

pub trait NodeHeader {
    fn get_n_descendants(&self) -> i32;
    fn get_children(&self) -> [i32; 2];
}

#[repr(C)]
pub struct NodeHeaderAngular {
    n_descendants: i32,
    children: [i32; 2],
}

#[repr(C)]
pub struct NodeHeaderMinkowski {
    n_descendants: i32,
    bias: f32,
    children: [i32; 2],
}

#[repr(C)]
pub struct NodeHeaderDot {
    n_descendants: i32,
    children: [i32; 2],
    dot_factor: f32,
}

pub trait MmapExtensions {
    fn read_i32(&self, idx: usize) -> i32;
    fn read_f32(&self, idx: usize) -> f32;
    fn read_slice<T: Sized>(&self, idx: usize, len: usize) -> &[T];
    fn read_angular_header(&self, idx: usize) -> *const NodeHeaderAngular;
    fn read_minkowski_header(&self, idx: usize) -> *const NodeHeaderMinkowski;
    fn read_dot_header(&self, idx: usize) -> *const NodeHeaderDot;
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

    fn read_angular_header(&self, idx: usize) -> *const NodeHeaderAngular {
        unsafe { mem::transmute(&self[idx]) }
    }

    fn read_minkowski_header(&self, idx: usize) -> *const NodeHeaderMinkowski {
        unsafe { mem::transmute(&self[idx]) }
    }

    fn read_dot_header(&self, idx: usize) -> *const NodeHeaderDot {
        unsafe { mem::transmute(&self[idx]) }
    }
}
