mod IndexType;
mod AnnoyIndexSearchResult;

extern crate memmap;

use std::fs;
use std::fs::{File};
use std::vec::Vec;
use std::mem;
use memmap::{Mmap, MmapOptions};

const INT32_SIZE:i32 = 4;
const FLOAT32_SIZE:i32 = 4;

pub struct AnnoyIndex {
    dimension: i32,
    index_type: IndexType::IndexType,
    index_type_offset: i32,
    k_node_header_style: i32,
    min_leaf_size: i32,
    node_size: i64,
    mmap: Mmap,
    roots: Vec<i64>,
}

trait MmapExtensions{
    fn read_i32(&self, idx: usize)->i32;
}

impl MmapExtensions for Mmap{
    fn read_i32(&self, idx: usize)->i32{
        let array = [*&self[idx], *&self[idx+1],*&self[idx+2],*&self[idx+3]];
        return unsafe { mem::transmute::<[u8;4],i32>(array) };
    }
}

impl AnnoyIndex {
    pub fn new(dimension: i32, index_file_path: &str, index_type: IndexType::IndexType) -> AnnoyIndex {
        let index_type_offset:i32 = if index_type == IndexType::IndexType::Angular {4} else {8};
        let k_node_header_style:i32 = if index_type == IndexType::IndexType::Angular {12} else {16};
        let min_leaf_size = dimension + 2;
        let node_size = k_node_header_style as i64 + FLOAT32_SIZE as i64 * dimension as i64;
        let file = File::open(index_file_path).expect("fail to open file");
        let file_metadata = fs::metadata(index_file_path).expect("failed to load file");
        let file_size = file_metadata.len();
        let mmap = unsafe { MmapOptions::new().map(&file).expect("failed to map the file") };

        let mut roots: Vec<i64> = Vec::new();
        let mut m:i32 = -1;
        let mut i = file_size as i64 - node_size as i64;
        while i >= 0 {
            let k= mmap.read_i32(i as usize);
            if m == -1 || k == m
            {
                roots.push(i);
                m = k;
            }
            else
            {
                break;
            }

            i -= node_size as i64;
        }

        // hacky fix: since the last root precedes the copy of all roots, delete it
        if roots.len() > 1 && get_l_child_offset(&mmap, *roots.first().unwrap(), node_size, index_type_offset) == get_r_child_offset(&mmap, *roots.last().unwrap(), node_size, index_type_offset)
        {
            let last_index = roots.len() - 1;
            roots.remove(last_index);
        }

        let index = AnnoyIndex{
            dimension: dimension,
            index_type: index_type,
            index_type_offset: index_type_offset,
            k_node_header_style: k_node_header_style,
            min_leaf_size: min_leaf_size,
            node_size: node_size,
            mmap: mmap,
            roots: roots,
        };

        return index;
    }
}

fn get_l_child_offset(mmap:&Mmap, top_node_offset:i64, node_size:i64, index_type_offset: i32)->i64{
    let child_offset = top_node_offset as usize + index_type_offset as usize;
    let child = mmap.read_i32(child_offset) as i64;
    return node_size * child;
}

fn get_r_child_offset(mmap:&Mmap, top_node_offset:i64, node_size:i64, index_type_offset: i32)->i64{
    let child_offset = top_node_offset as usize + index_type_offset as usize + 4;
    let child = mmap.read_i32(child_offset) as i64;
    return node_size * child;
}

/*
pub trait AnnoyIndexSearchApi {
    fn getItemVector(&self, itemIndex: i64) -> &[f32];

    fn getNearest(&self, queryVector: &[f32], nResults: i32, searchK: i32, shouldIncludeDistance: bool) -> &[AnnoyIndexSearchResult::AnnoyIndexSearchResult];
}

impl AnnoyIndexSearchApi for AnnoyIndex {
    fn getItemVector(&self, itemIndex: i64) -> &[f32] {}

    fn getNearest(&self, queryVector: &[f32], nResults: i32, searchK: i32, shouldIncludeDistance: bool) -> &[AnnoyIndexSearchResult::AnnoyIndexSearchResult] {}
}
*/