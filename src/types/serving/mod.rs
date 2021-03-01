use super::utils::*;
use super::*;
use crate::internals::mmap_ext::*;
use crate::internals::pqentry::*;

pub trait AnnoyIndexSearchApi {
    fn get_item_vector(&self, item_index: i64) -> Vec<f32>;
    fn get_nearest(
        &self,
        query_vector: &[f32],
        n_results: usize,
        search_k: i32,
        should_include_distance: bool,
    ) -> Vec<AnnoyIndexSearchResult>;
    fn get_nearest_to_item(
        &self,
        item_index: i64,
        n_results: usize,
        search_k: i32,
        should_include_distance: bool,
    ) -> Vec<AnnoyIndexSearchResult>;
}

impl AnnoyIndexSearchApi for AnnoyIndex {
    fn get_item_vector(&self, item_index: i64) -> Vec<f32> {
        let node_offset = item_index as usize * self.node_size as usize;
        return get_node_vector(self, node_offset);
    }

    fn get_nearest(
        &self,
        query_vector: &[f32],
        n_results: usize,
        search_k: i32,
        should_include_distance: bool,
    ) -> Vec<AnnoyIndexSearchResult> {
        let result_capcity = n_results.min(self.degree as usize).max(1);
        let search_k_fixed = if search_k > 0 {
            search_k as usize
        } else {
            result_capcity * self.roots.len()
        };

        let mut pq = Vec::<PriorityQueueEntry>::with_capacity(self.roots.len() * FLOAT32_SIZE);
        for i in 0..self.roots.len() {
            let offset = self.roots[i];
            pq.push(PriorityQueueEntry {
                margin: f32::MAX,
                node_offset: offset,
                node_id: 0,
            });
        }

        let mut nearest_neighbors = std::collections::HashSet::<usize>::new();
        while !pq.is_empty() && nearest_neighbors.len() < search_k_fixed {
            if let Some(top) = pq.pop() {
                let top_node_offset = top.node_offset as usize;
                let top_node_id = top_node_offset / self.node_size as usize;
                let n_descendants = self.mmap.read_i32(top_node_offset);
                if n_descendants == 1 && top_node_id < self.degree as usize {
                    nearest_neighbors.insert(top_node_id);
                } else if n_descendants <= self.min_leaf_size {
                    for i in 0..n_descendants as usize {
                        let child_id = self.get_nth_descendant_id(top_node_offset as i64, i);
                        nearest_neighbors.insert(child_id as usize);
                    }
                } else {
                    let v = get_node_vector(self, top_node_offset);
                    let margin = self.get_margin(v.as_slice(), query_vector, top_node_offset);
                    let l_child_offset = self.get_l_child_offset(top_node_offset as i64);
                    let r_child_offset = self.get_r_child_offset(top_node_offset as i64);
                    // NOTE: Hamming has different logic to calculate margin
                    pq.push(PriorityQueueEntry {
                        margin: top.margin.min(margin),
                        node_offset: r_child_offset,
                        node_id: 0,
                    });
                    pq.push(PriorityQueueEntry {
                        margin: top.margin.min(-margin),
                        node_offset: l_child_offset,
                        node_id: 0,
                    });
                    pq.sort_by(|a, b| a.margin.partial_cmp(&b.margin).unwrap());
                }
            }
        }

        let mut sorted_nns: Vec<PriorityQueueEntry> = Vec::with_capacity(nearest_neighbors.len());
        for nn_id in nearest_neighbors {
            let n_descendants = self.mmap.read_i32(nn_id * self.node_size as usize);
            if n_descendants != 1 {
                continue;
            }
            let v = self.get_item_vector(nn_id as i64);
            sorted_nns.push(PriorityQueueEntry {
                margin: self.get_distance_no_norm(v.as_slice(), query_vector),
                node_id: nn_id as i64,
                node_offset: 0,
            });
        }

        sorted_nns.sort_by(|a, b| a.margin.partial_cmp(&b.margin).unwrap());

        let final_result_capcity = n_results.min(sorted_nns.len());
        let mut results: Vec<AnnoyIndexSearchResult> = Vec::with_capacity(final_result_capcity);
        for i in 0..final_result_capcity {
            let nn = &sorted_nns[i];
            results.push(AnnoyIndexSearchResult {
                id: nn.node_id,
                distance: match should_include_distance {
                    true => self.normalized_distance(nn.margin),
                    false => 0.0,
                },
            });
        }
        return results;
    }

    fn get_nearest_to_item(
        &self,
        item_index: i64,
        n_results: usize,
        search_k: i32,
        should_include_distance: bool,
    ) -> Vec<AnnoyIndexSearchResult> {
        let item_vector = self.get_item_vector(item_index);
        return self.get_nearest(
            item_vector.as_slice(),
            n_results,
            search_k,
            should_include_distance,
        );
    }
}

fn get_node_vector(index: &AnnoyIndex, node_offset: usize) -> Vec<f32> {
    let dimension = index.dimension as usize;
    let mut vec: Vec<f32> = Vec::with_capacity(dimension);
    let mut offset = node_offset + index.k_node_header_style as usize;
    for _ in 0..dimension {
        let value = index.mmap.read_f32(offset);
        vec.push(value);
        offset += FLOAT32_SIZE;
    }
    return vec;
}

// fn is_zero_node_vector(index: &AnnoyIndex, node_offset: usize) -> bool {
//     let dimension = index.dimension as usize;
//     let mut offset = node_offset + index.k_node_header_style as usize;
//     for _ in 0..dimension {
//         let value = index.mmap.read_f32(offset);
//         if value != 0.0 {
//             return false;
//         }
//         offset += FLOAT32_SIZE;
//     }
//     return true;
// }
