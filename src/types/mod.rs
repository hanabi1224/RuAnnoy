mod annoy_index_impl;
mod utils;

pub mod serving;
pub use serving::AnnoyIndexSearchApi;

use memmap2::Mmap;

#[repr(C)]
pub struct AnnoyIndexSearchResult {
    pub id: i64,
    pub distance: f32,
}

#[repr(u8)]
#[derive(PartialEq)]
pub enum IndexType {
    Angular = 0,
    Euclidean = 1,
    Manhattan = 2,
    Hamming = 3,
    Dot = 4,
}

pub struct AnnoyIndex {
    pub dimension: i32,
    pub index_type: IndexType,
    index_type_offset: i32,
    k_node_header_style: i32,
    min_leaf_size: i32,
    node_size: i64,
    mmap: Mmap,
    roots: Vec<i64>,
}
