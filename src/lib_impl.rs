use std::mem;
use memmap::{Mmap};

pub trait MmapExtensions{
    fn read_i32(&self, idx: usize)->i32;
    fn read_f32(&self, idx: usize)->f32;
}

impl MmapExtensions for Mmap{
    fn read_i32(&self, idx: usize)->i32{
        let array = [*&self[idx], *&self[idx+1],*&self[idx+2],*&self[idx+3]];
        return unsafe { mem::transmute::<[u8;4],i32>(array) };
    }

    fn read_f32(&self, idx: usize)->f32{
        let array = [*&self[idx], *&self[idx+1],*&self[idx+2],*&self[idx+3]];
        return unsafe { mem::transmute::<[u8;4],f32>(array) };
    }
}
