use super::utils::*;
use crate::internals::mmap_ext::*;
use crate::types::*;
use memmap2::MmapOptions;
use std::fs;
use std::fs::File;
use std::vec::Vec;

impl AnnoyIndex {
    pub fn load(dimension: i32, index_file_path: &str, index_type: IndexType) -> AnnoyIndex {
        let (index_type_offset, k_node_header_style): (i32, i32) = match index_type {
            IndexType::Angular => (4, 12),
            IndexType::Euclidean => (8, 16),
            _ => panic!("Not supported"),
        };
        let min_leaf_size = dimension + 2;
        let node_size = k_node_header_style as i64 + FLOAT32_SIZE as i64 * dimension as i64;
        let file = File::open(index_file_path).expect("fail to open file");
        let file_metadata = fs::metadata(index_file_path).expect("failed to load file");
        let file_size = file_metadata.len();
        let mmap = unsafe {
            MmapOptions::new()
                .map(&file)
                .expect("failed to map the file")
        };

        let mut roots: Vec<i64> = Vec::new();
        let mut m: i32 = -1;
        let mut i = file_size as i64 - node_size as i64;
        while i >= 0 {
            let k = mmap.read_i32(i as usize);
            if m == -1 || k == m {
                roots.push(i);
                m = k;
            } else {
                break;
            }

            i -= node_size as i64;
        }

        // hacky fix: since the last root precedes the copy of all roots, delete it
        if roots.len() > 1
            && get_l_child_offset(&mmap, *roots.first().unwrap(), node_size, index_type_offset)
                == get_r_child_offset(&mmap, *roots.last().unwrap(), node_size, index_type_offset)
        {
            let last_index = roots.len() - 1;
            roots.remove(last_index);
        }

        let index = AnnoyIndex {
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
