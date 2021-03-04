use crate::IndexType;
use memmap2::Mmap;
use std::mem;
use std::rc::Rc;
use std::slice;

pub struct Node {
    pub mmap: Rc<Mmap>,
    pub id: usize,
    pub offset: usize,
    pub header: NodeHeader,
}

impl Node {
    pub fn new_with_id(id: usize, node_size: usize, index_type: IndexType, mmap: Rc<Mmap>) -> Node {
        let offset = id * node_size;
        let header = NodeHeader::new(offset, &index_type, &mmap);
        Node {
            mmap: mmap,
            id: id,
            offset: offset,
            header: header,
        }
    }

    pub fn new_with_offset(
        offset: usize,
        node_size: usize,
        index_type: IndexType,
        mmap: Rc<Mmap>,
    ) -> Node {
        let header = NodeHeader::new(offset, &index_type, &mmap);
        Node {
            mmap: mmap,
            id: offset / node_size,
            offset: offset,
            header: header,
        }
    }
}

#[repr(C)]
pub enum NodeHeader {
    Angular(NodeHeaderAngular),
    Minkowski(NodeHeaderMinkowski),
    Dot(NodeHeaderDot),
}

impl NodeHeader {
    pub fn new(offset: usize, index_type: &IndexType, mmap: &Mmap) -> NodeHeader {
        match index_type {
            IndexType::Angular => NodeHeader::Angular(unsafe { *mmap.read_angular_header(offset) }),
            IndexType::Euclidean | IndexType::Manhattan => {
                NodeHeader::Minkowski(unsafe { *mmap.read_minkowski_header(offset) })
            }
            IndexType::Dot => NodeHeader::Dot(unsafe { *mmap.read_dot_header(offset) }),
            _ => unimplemented!("Index type not supported"),
        }
    }

    pub fn get_n_descendant(&self) -> i32 {
        match self {
            NodeHeader::Angular(h) => h.n_descendants,
            NodeHeader::Minkowski(h) => h.n_descendants,
            NodeHeader::Dot(h) => h.n_descendants,
        }
    }

    pub fn get_children_id_slice(&self) -> &[i32] {
        match self {
            NodeHeader::Angular(h) => &h.children,
            NodeHeader::Minkowski(h) => &h.children,
            NodeHeader::Dot(h) => &h.children,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NodeHeaderAngular {
    n_descendants: i32,
    children: [i32; 2],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NodeHeaderMinkowski {
    n_descendants: i32,
    bias: f32,
    children: [i32; 2],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
