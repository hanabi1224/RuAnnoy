mod annoy_index_impl;
mod utils;

pub mod serving;
pub use serving::AnnoyIndexSearchApi;
use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;

use memmap2::Mmap;

#[repr(C)]
pub struct AnnoyIndexSearchResult {
    pub id: u64,
    pub distance: f32,
}

#[repr(u8)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum IndexType {
    Angular = 0,
    Euclidean = 1,
    Manhattan = 2,
    Hamming = 3,
    Dot = 4,
}

impl Display for IndexType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let r = format!("{:?}", self).to_lowercase();
        f.write_str(&r)
    }
}

pub struct AnnoyIndex {
    pub dimension: usize,
    pub index_type: IndexType,
    pub node_size: usize,
    pub node_count: usize,
    pub max_descendants: i32,
    offset_before_children: usize,
    k_node_header_style: usize,
    min_leaf_size: i32,
    mmap: Rc<Mmap>,
    pub roots: Vec<usize>,
    pub degree: usize,
}
