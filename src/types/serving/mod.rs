use super::*;
use crate::internals::priority_queue::PriorityQueue;

pub trait AnnoyIndexSearchApi {
    fn get_item_vector(&self, item_index: i64) -> Vec<f32>;
    fn get_nearest(
        &self,
        query_vector: &[f32],
        n_results: usize,
        search_k: i32,
        should_include_distance: bool,
    ) -> AnnoyIndexSearchResult;
    fn get_nearest_to_item(
        &self,
        item_index: i64,
        n_results: usize,
        search_k: i32,
        should_include_distance: bool,
    ) -> AnnoyIndexSearchResult;
}

impl AnnoyIndexSearchApi for AnnoyIndex {
    fn get_item_vector(&self, item_index: i64) -> Vec<f32> {
        let node_offset = item_index as usize * self.node_size;
        let slice = self.get_node_slice_with_offset(node_offset);
        slice.iter().map(|&a| a).collect()
    }

    fn get_nearest(
        &self,
        query_vector: &[f32],
        n_results: usize,
        search_k: i32,
        should_include_distance: bool,
    ) -> AnnoyIndexSearchResult {
        let result_capacity = n_results.min(self.degree).max(1);
        let search_k_fixed = if search_k > 0 {
            search_k as usize
        } else {
            result_capacity * self.roots.len()
        };

        let mut pq = PriorityQueue::with_capacity(result_capacity);
        for i in 0..self.roots.len() {
            let id = self.roots[i];
            pq.push(id, f32::MAX);
        }

        let mut nearest_neighbors = std::collections::HashSet::<usize>::new();
        while pq.len() > 0 && nearest_neighbors.len() < search_k_fixed {
            if let Some((top_node_id, top_node_margin)) = pq.pop() {
                let top_node = self.get_node_from_id(top_node_id);
                let top_node_header = top_node.header;
                let top_node_offset = top_node.offset;
                let n_descendants = top_node_header.get_n_descendant();
                if n_descendants == 1 && top_node_id < self.degree {
                    nearest_neighbors.insert(top_node_id);
                } else if n_descendants <= self.min_leaf_size {
                    let children_id_slice =
                        self.get_descendant_id_slice(top_node_offset, n_descendants as usize);
                    for &child_id in children_id_slice {
                        nearest_neighbors.insert(child_id as usize);
                    }
                } else {
                    let v = self.get_node_slice_with_offset(top_node_offset);
                    let margin = self.get_margin(v, query_vector, top_node_offset);
                    let children_id = top_node_header.get_children_id_slice();
                    // NOTE: Hamming has different logic to calculate margin
                    pq.push(children_id[1] as usize, top_node_margin.min(margin));
                    pq.push(children_id[0] as usize, top_node_margin.min(-margin));
                }
            }
        }

        let mut sorted_nns = Vec::with_capacity(nearest_neighbors.len());
        for nn_id in nearest_neighbors {
            let node = self.get_node_from_id(nn_id);
            let n_descendants = node.header.get_n_descendant();
            if n_descendants != 1 {
                continue;
            }

            let s = self.get_node_slice_with_offset(nn_id * self.node_size);
            sorted_nns.push((nn_id, self.get_distance_no_norm(s, query_vector)));
        }

        sorted_nns.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let final_result_capcity = n_results.min(sorted_nns.len());
        let mut id_list = Vec::with_capacity(final_result_capcity);
        let mut distance_list = Vec::with_capacity(if should_include_distance {
            final_result_capcity
        } else {
            0
        });
        for i in 0..final_result_capcity {
            let nn = &sorted_nns[i];
            id_list.push(nn.0 as u64);
            if should_include_distance {
                distance_list.push(self.normalized_distance(nn.1));
            }
        }
        return AnnoyIndexSearchResult {
            count: final_result_capcity,
            is_distance_included: should_include_distance,
            id_list: id_list,
            distance_list: distance_list,
        };
    }

    fn get_nearest_to_item(
        &self,
        item_index: i64,
        n_results: usize,
        search_k: i32,
        should_include_distance: bool,
    ) -> AnnoyIndexSearchResult {
        let item_vector = self.get_item_vector(item_index);
        self.get_nearest(
            item_vector.as_slice(),
            n_results,
            search_k,
            should_include_distance,
        )
    }
}
