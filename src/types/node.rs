use crate::IndexType;
use memmap2::Mmap;
use std::mem;
use std::rc::Rc;

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
            IndexType::Angular => {
                NodeHeader::Angular(unsafe { *NodeHeaderAngular::read(mmap, offset) })
            }
            IndexType::Euclidean | IndexType::Manhattan => {
                NodeHeader::Minkowski(unsafe { *NodeHeaderMinkowski::read(mmap, offset) })
            }
            IndexType::Dot => NodeHeader::Dot(unsafe { *NodeHeaderDot::read(mmap, offset) }),
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

impl NodeHeaderAngular {
    fn read(mmap: &Mmap, offset: usize) -> *const NodeHeaderAngular {
        unsafe { mem::transmute(&mmap[offset]) }
    }
}

impl NodeHeaderMinkowski {
    fn read(mmap: &Mmap, offset: usize) -> *const NodeHeaderMinkowski {
        unsafe { mem::transmute(&mmap[offset]) }
    }
}

impl NodeHeaderDot {
    fn read(mmap: &Mmap, offset: usize) -> *const NodeHeaderDot {
        unsafe { mem::transmute(&mmap[offset]) }
    }
}
