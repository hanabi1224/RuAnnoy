pub(crate) mod annoy_index_impl;
pub(crate) mod node;
pub(crate) mod utils;

pub mod serving;
pub use serving::*;
use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;

use memmap2::Mmap;

pub struct AnnoyIndexSearchResult {
    pub count: usize,
    pub is_distance_included: bool,
    pub id_list: Vec<u64>,
    pub distance_list: Vec<f32>,
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
    pub size: usize,
    pub(crate) max_descendants: i32,
    pub(crate) offset_before_children: usize,
    pub(crate) node_header_size: usize,
    pub(crate) mmap: Rc<Mmap>,
    pub(crate) roots: Vec<usize>,
}
