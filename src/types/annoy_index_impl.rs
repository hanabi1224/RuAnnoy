use super::utils::*;
use super::{AnnoyIndex, IndexType};
use crate::internals::mmap_ext::*;
use memmap2::MmapOptions;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::vec::Vec;

impl AnnoyIndex {
    pub fn load(
        dimension: i32,
        index_file_path: &str,
        index_type: IndexType,
    ) -> Result<AnnoyIndex, Box<dyn Error>> {
        let (index_type_offset, k_node_header_style, max_descendants): (i32, i32, i32) =
            match index_type {
                IndexType::Angular => (4, 12, 2),
                IndexType::Euclidean => (8, 16, 2),
                IndexType::Manhattan => (8, 16, 2),
                // IndexType::Hamming => (4, 12),
                IndexType::Dot => (4, 16, 3),
                _ => unimplemented!("Index type not supported"),
            };

        let min_leaf_size = dimension + max_descendants;
        let node_size = k_node_header_style + FLOAT32_SIZE as i32 * dimension;
        let file = File::open(index_file_path)?; // .expect(format!("fail to open {}", index_file_path).as_str());
        let file_metadata = fs::metadata(index_file_path)?;
        let file_size = file_metadata.len() as i64;
        let node_count = file_size / node_size as i64;
        let mmap = unsafe { MmapOptions::new().map(&file)? };

        let mut roots: Vec<i64> = Vec::new();
        let mut m: i32 = -1;
        let mut i = file_size - node_size as i64;
        while i >= 0 {
            let n_descendants = mmap.read_i32(i as usize);
            if m == -1 || n_descendants == m {
                roots.push(i);
                m = n_descendants;
            } else {
                break;
            }
            i -= node_size as i64;
        }

        // hacky fix: since the last root precedes the copy of all roots, delete it
        if roots.len() > 1
            && get_nth_descendant_id(&mmap, *roots.first().unwrap(), index_type_offset, 0)
                == get_nth_descendant_id(&mmap, *roots.last().unwrap(), index_type_offset, 0)
        {
            roots.pop();
        }

        let index = AnnoyIndex {
            dimension: dimension,
            index_type: index_type,
            index_type_offset: index_type_offset,
            k_node_header_style: k_node_header_style,
            min_leaf_size: min_leaf_size,
            node_size: node_size,
            node_count: node_count as usize,
            max_descendants: max_descendants,
            mmap: mmap,
            roots: roots,
            degree: m,
        };

        return Ok(index);
    }

    pub fn get_nth_descendant_id(&self, node_offset: i64, n: usize) -> i64 {
        get_nth_descendant_id(&self.mmap, node_offset, self.index_type_offset, n)
    }

    pub fn get_l_child_id(&self, node_offset: i64) -> i64 {
        self.get_nth_descendant_id(node_offset, 0)
    }

    pub fn get_l_child_offset(&self, node_offset: i64) -> i64 {
        self.get_l_child_id(node_offset) * self.node_size as i64
    }

    pub fn get_r_child_id(&self, node_offset: i64) -> i64 {
        self.get_nth_descendant_id(node_offset, 1)
    }

    pub fn get_r_child_offset(&self, node_offset: i64) -> i64 {
        self.get_r_child_id(node_offset) * self.node_size as i64
    }

    pub fn get_margin(&self, v1: &[f32], v2: &[f32], node_offset: usize) -> f32 {
        match self.index_type {
            IndexType::Angular => dot_product(v1, v2),
            IndexType::Euclidean | IndexType::Manhattan => {
                minkowski_margin(v1, v2, self.mmap.read_f32(node_offset + 4))
            }
            IndexType::Dot => dot_product(v1, v2) + self.mmap.read_f32(node_offset + 12).powi(2),
            _ => unimplemented!("Index type not supported"),
        }
    }

    pub fn get_distance_no_norm(&self, v1: &[f32], v2: &[f32]) -> f32 {
        match self.index_type {
            IndexType::Angular => cosine_distance(v1, v2),
            IndexType::Euclidean => euclidean_distance(v1, v2),
            IndexType::Manhattan => manhattan_distance(v1, v2),
            IndexType::Dot => -dot_product(v1, v2),
            _ => unimplemented!("Index type not supported"),
        }
    }

    pub fn normalized_distance(&self, d: f32) -> f32 {
        match self.index_type {
            IndexType::Angular | IndexType::Euclidean => d.sqrt(),
            IndexType::Dot => -d,
            _ => d,
        }
    }
}
