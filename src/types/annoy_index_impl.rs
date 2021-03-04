use super::utils::*;
use super::{AnnoyIndex, IndexType};
use crate::internals::mmap_ext::*;
use crate::types::node::*;
use memmap2::MmapOptions;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::rc::Rc;
use std::vec::Vec;

impl AnnoyIndex {
    pub fn load(
        dimension: usize,
        index_file_path: &str,
        index_type: IndexType,
    ) -> Result<AnnoyIndex, Box<dyn Error>> {
        let (offset_before_children, k_node_header_style, max_descendants): (usize, usize, usize) =
            match index_type {
                IndexType::Angular => (4, 12, 2),
                IndexType::Euclidean => (8, 16, 2),
                IndexType::Manhattan => (8, 16, 2),
                // IndexType::Hamming => (4, 12),
                IndexType::Dot => (4, 16, 3),
                _ => unimplemented!("Index type not supported"),
            };

        let min_leaf_size = dimension + max_descendants;
        let node_size = k_node_header_style as i64 + (FLOAT32_SIZE * dimension) as i64;
        let file = File::open(index_file_path)?; // .expect(format!("fail to open {}", index_file_path).as_str());
        let file_metadata = fs::metadata(index_file_path)?;
        let file_size = file_metadata.len() as i64;
        let node_count = file_size / node_size;
        let mmap = unsafe { MmapOptions::new().map(&file)? };

        let mut roots = Vec::new();
        let mut m: i32 = -1;
        let mut i = file_size - node_size;
        while i >= 0 {
            let n_descendants = mmap.read_i32(i as usize);
            if m == -1 || n_descendants == m {
                roots.push((i / node_size) as usize);
                m = n_descendants;
            } else {
                break;
            }
            i -= node_size;
        }

        // hacky fix: since the last root precedes the copy of all roots, delete it
        if roots.len() > 1
            && get_nth_descendant_id(
                &mmap,
                *roots.first().unwrap() * node_size as usize,
                offset_before_children,
                0,
            ) == get_nth_descendant_id(
                &mmap,
                *roots.last().unwrap() * node_size as usize,
                offset_before_children,
                0,
            )
        {
            roots.pop();
        }

        let index = AnnoyIndex {
            dimension: dimension,
            index_type: index_type,
            offset_before_children: offset_before_children,
            k_node_header_style: k_node_header_style,
            min_leaf_size: min_leaf_size as i32,
            node_size: node_size as usize,
            node_count: node_count as usize,
            max_descendants: max_descendants as i32,
            mmap: Rc::new(mmap),
            roots: roots,
            degree: m as usize,
        };

        return Ok(index);
    }

    pub fn get_node_from_id(&self, id: usize) -> Node {
        Node::new_with_id(id, self.node_size, self.index_type, self.mmap.clone())
    }

    pub fn get_descendant_id_slice(&self, node_offset: usize, n: usize) -> &[i32] {
        self.mmap
            .read_slice(node_offset + self.offset_before_children, n)
    }

    pub fn get_children_id_slice(&self, node_offset: usize) -> &[i32] {
        self.mmap
            .read_slice(node_offset + self.offset_before_children, 2)
    }

    pub fn get_nth_descendant_id(&self, node_offset: usize, n: usize) -> usize {
        get_nth_descendant_id(&self.mmap, node_offset, self.offset_before_children, n)
    }

    pub fn get_l_child_id(&self, node_offset: usize) -> usize {
        self.get_nth_descendant_id(node_offset, 0)
    }

    pub fn get_l_child_offset(&self, node_offset: usize) -> usize {
        self.get_l_child_id(node_offset) * self.node_size
    }

    pub fn get_r_child_id(&self, node_offset: usize) -> usize {
        self.get_nth_descendant_id(node_offset, 1)
    }

    pub fn get_r_child_offset(&self, node_offset: usize) -> usize {
        self.get_r_child_id(node_offset) * self.node_size
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

    pub fn get_node_slice_with_offset(&self, node_offset: usize) -> &[f32] {
        let dimension = self.dimension as usize;
        let offset = node_offset + self.k_node_header_style as usize;
        self.mmap.read_slice::<f32>(offset, dimension)
    }
}
