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
}

impl AnnoyIndexSearchApi for AnnoyIndex {
    fn get_item_vector(&self, item_index: i64) -> Vec<f32> {
        let node_offset = item_index * self.node_size;
        return get_node_vector(self, node_offset);
    }

    fn get_nearest(
        &self,
        query_vector: &[f32],
        n_results: usize,
        search_k: i32,
        should_include_distance: bool,
    ) -> Vec<AnnoyIndexSearchResult> {
        let mut search_k_mut = search_k;
        if search_k <= 0 {
            search_k_mut = n_results as i32 * (self.roots.len() as i32);
        }

        let mut pq =
            Vec::<PriorityQueueEntry>::with_capacity(self.roots.len() * (FLOAT32_SIZE as usize));
        for r in &self.roots {
            pq.push(PriorityQueueEntry::new(std::f32::MAX, *r));
        }

        let mut nearest_neighbors = std::collections::HashSet::<i64>::new();
        while nearest_neighbors.len() < search_k_mut as usize && !pq.is_empty() {
            pq.sort_by(|a, b| b.margin.partial_cmp(&a.margin).unwrap());
            let top = pq.remove(0);
            let top_node_offset = top.node_offset;
            let n_descendants = self.mmap.read_i32(top_node_offset as usize);
            let v = get_node_vector(self, top_node_offset);
            if n_descendants == 1 {
                if is_zero_vec(&v) {
                    continue;
                }

                nearest_neighbors.insert(top_node_offset / self.node_size);
            } else if n_descendants <= self.min_leaf_size {
                for i in 0..n_descendants as usize {
                    let j = self
                        .mmap
                        .read_i32(top_node_offset as usize + i * INT32_SIZE as usize)
                        as i64;
                    if is_zero_vec(&get_node_vector(self, j)) {
                        continue;
                    }

                    nearest_neighbors.insert(j);
                }
            } else {
                let margin = if self.index_type == IndexType::Angular {
                    cosine_margin_no_norm(v.as_slice(), query_vector)
                } else {
                    euclidean_margin(
                        v.as_slice(),
                        query_vector,
                        get_node_bias(self, top_node_offset),
                    )
                };
                let l_child = get_l_child_offset(
                    &self.mmap,
                    top_node_offset,
                    self.node_size,
                    self.index_type_offset,
                );
                let r_child = get_r_child_offset(
                    &self.mmap,
                    top_node_offset,
                    self.node_size,
                    self.index_type_offset,
                );
                pq.push(PriorityQueueEntry {
                    margin: top.margin.min(-margin),
                    node_offset: l_child,
                });
                pq.push(PriorityQueueEntry {
                    margin: top.margin.min(margin),
                    node_offset: r_child,
                });
            }
        }

        let mut sorted_nns: Vec<PriorityQueueEntry> = Vec::new();
        for nn in nearest_neighbors {
            let v = self.get_item_vector(nn);
            if !is_zero_vec(&v) {
                let param1 = v.as_slice();
                let param2 = query_vector;
                sorted_nns.push(PriorityQueueEntry {
                    margin: if self.index_type == IndexType::Angular {
                        cosine_distance(param1, param2)
                    } else {
                        euclidean_distance(param1, param2)
                    },
                    node_offset: nn,
                });
            }
        }

        sorted_nns.sort_by(|a, b| a.margin.partial_cmp(&b.margin).unwrap());

        let mut results: Vec<AnnoyIndexSearchResult> = Vec::with_capacity(n_results);
        for i in 0..n_results.min(sorted_nns.len()) {
            let nn = &sorted_nns[i];
            results.push(AnnoyIndexSearchResult {
                id: nn.node_offset,
                distance: if should_include_distance {
                    nn.margin.sqrt()
                } else {
                    0.0
                },
            });
        }

        return results;
    }
}

fn get_node_vector(index: &AnnoyIndex, node_offset: i64) -> Vec<f32> {
    let mut vec: Vec<f32> = Vec::with_capacity(index.dimension as usize);
    for i in 0..index.dimension as usize {
        let idx =
            node_offset as usize + index.k_node_header_style as usize + i * (FLOAT32_SIZE as usize);
        let value = index.mmap.read_f32(idx);
        vec.push(value);
    }

    return vec;
}

fn get_node_bias(index: &AnnoyIndex, node_offset: i64) -> f32 {
    return index.mmap.read_f32(node_offset as usize + 4);
}
