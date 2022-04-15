use crate::{IndexType, Storage};
use std::mem;

pub(crate) struct Node {
    pub offset: usize,
    pub header: NodeHeader,
}

impl Node {
    pub fn new_with_id(
        id: usize,
        node_size: usize,
        index_type: &IndexType,
        storage: &Storage,
    ) -> Node {
        let offset = id * node_size;
        let header = NodeHeader::new(offset, index_type, storage);
        Node { offset, header }
    }
}

#[repr(C)]
pub(crate) enum NodeHeader {
    Angular(NodeHeaderAngular),
    Minkowski(NodeHeaderMinkowski),
    Dot(NodeHeaderDot),
}

impl NodeHeader {
    pub fn new(offset: usize, index_type: &IndexType, storage: &Storage) -> NodeHeader {
        match index_type {
            IndexType::Angular => {
                NodeHeader::Angular(unsafe { *NodeHeaderAngular::read(storage, offset) })
            }
            IndexType::Euclidean | IndexType::Manhattan => {
                NodeHeader::Minkowski(unsafe { *NodeHeaderMinkowski::read(storage, offset) })
            }
            IndexType::Dot => NodeHeader::Dot(unsafe { *NodeHeaderDot::read(storage, offset) }),
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

    pub fn get_children_id_slice(&self) -> [i32; 2] {
        match self {
            NodeHeader::Angular(h) => h.children,
            NodeHeader::Minkowski(h) => h.children,
            NodeHeader::Dot(h) => h.children,
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

// #[repr(C)]
// #[derive(Debug, Clone, Copy)]
// pub struct NodeHeaderHamming {
//     n_descendants: i32,
//     children: [i32; 2],
// }

impl NodeHeaderAngular {
    fn read(storage: &Storage, offset: usize) -> *const NodeHeaderAngular {
        unsafe { mem::transmute(&storage[offset]) }
    }

    pub const fn header_size() -> usize {
        mem::size_of::<NodeHeaderAngular>()
    }
}

impl NodeHeaderMinkowski {
    fn read(storage: &Storage, offset: usize) -> *const NodeHeaderMinkowski {
        unsafe { mem::transmute(&storage[offset]) }
    }

    pub const fn header_size() -> usize {
        mem::size_of::<NodeHeaderMinkowski>()
    }
}

impl NodeHeaderDot {
    fn read(storage: &Storage, offset: usize) -> *const NodeHeaderDot {
        unsafe { mem::transmute(&storage[offset]) }
    }

    pub const fn header_size() -> usize {
        mem::size_of::<NodeHeaderDot>()
    }
}
