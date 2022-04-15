use super::utils::*;
use super::{AnnoyIndex, IndexType};
use crate::internals::mmap_ext::*;
use crate::types::node::*;
use memmap2::MmapOptions;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::vec::Vec;

impl AnnoyIndex {
    pub fn load(
        dimension: usize,
        index_file_path: &str,
        index_type: IndexType,
    ) -> Result<AnnoyIndex, Box<dyn Error>> {
        let (offset_before_children, node_header_size, max_descendants): (usize, usize, usize) =
            match index_type {
                IndexType::Angular => (4, NodeHeaderAngular::header_size(), dimension + 2),
                IndexType::Euclidean | IndexType::Manhattan => {
                    (8, NodeHeaderMinkowski::header_size(), dimension + 2)
                }
                // IndexType::Hamming => (4, 12),
                IndexType::Dot => (4, NodeHeaderDot::header_size(), dimension + 3),
                _ => unimplemented!("Index type not supported"),
            };

        let node_size = node_header_size as i64 + (FLOAT32_SIZE * dimension) as i64;
        let file = File::open(index_file_path)?;
        let file_metadata = fs::metadata(index_file_path)?;
        let file_size = file_metadata.len() as i64;
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
            dimension,
            index_type,
            offset_before_children,
            node_header_size,
            max_descendants: max_descendants as i32,
            node_size: node_size as usize,
            mmap: Box::new(mmap),
            roots,
            size: m as usize,
        };

        Ok(index)
    }

    pub(crate) fn get_node_from_id(&self, id: usize) -> Node {
        Node::new_with_id(id, self.node_size, self.index_type, &self.mmap)
    }

    pub(crate) fn get_descendant_id_slice(&self, node_offset: usize, n: usize) -> &[i32] {
        self.mmap
            .read_slice(node_offset + self.offset_before_children, n)
    }

    pub(crate) fn get_margin(&self, v1: &[f32], v2: &[f32], node_offset: usize) -> f32 {
        match self.index_type {
            IndexType::Angular => dot_product(v1, v2),
            IndexType::Euclidean | IndexType::Manhattan => {
                minkowski_margin(v1, v2, self.mmap.read_f32(node_offset + 4))
            }
            IndexType::Dot => dot_product(v1, v2) + self.mmap.read_f32(node_offset + 12).powi(2),
            _ => unimplemented!("Index type not supported"),
        }
    }

    pub(crate) fn get_distance_no_norm(&self, v1: &[f32], v2: &[f32]) -> f32 {
        match self.index_type {
            IndexType::Angular => cosine_distance(v1, v2),
            IndexType::Euclidean => euclidean_distance(v1, v2),
            IndexType::Manhattan => manhattan_distance(v1, v2),
            IndexType::Dot => -dot_product(v1, v2),
            _ => unimplemented!("Index type not supported"),
        }
    }

    pub(crate) fn normalized_distance(&self, d: f32) -> f32 {
        match self.index_type {
            IndexType::Angular | IndexType::Euclidean => d.sqrt(),
            IndexType::Dot => -d,
            _ => d,
        }
    }

    pub(crate) fn get_node_slice_with_offset(&self, node_offset: usize) -> &[f32] {
        let dimension = self.dimension as usize;
        let offset = node_offset + self.node_header_size as usize;
        self.mmap.read_slice::<f32>(offset, dimension)
    }
}
