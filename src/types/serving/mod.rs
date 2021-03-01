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
        return get_node_vector(self, node_offset as usize);
    }

    /*
    void _get_all_nns(const T* v, size_t n, int search_k, vector<S>* result, vector<T>* distances) const {
        Node* v_node = (Node *)alloca(_s);
        D::template zero_value<Node>(v_node);
        memcpy(v_node->v, v, sizeof(T) * _f);
        D::init_node(v_node, _f);

        std::priority_queue<pair<T, S> > q;

        if (search_k == -1) {
          search_k = n * _roots.size();
        }

        for (size_t i = 0; i < _roots.size(); i++) {
          q.push(make_pair(Distance::template pq_initial_value<T>(), _roots[i]));
        }

        std::vector<S> nns;
        while (nns.size() < (size_t)search_k && !q.empty()) {
          const pair<T, S>& top = q.top();
          T d = top.first;
          S i = top.second;
          Node* nd = _get(i);
          q.pop();
          if (nd->n_descendants == 1 && i < _n_items) {
            nns.push_back(i);
          } else if (nd->n_descendants <= _K) {
            const S* dst = nd->children;
            nns.insert(nns.end(), dst, &dst[nd->n_descendants]);
          } else {
            T margin = D::margin(nd, v, _f);
            q.push(make_pair(D::pq_distance(d, margin, 1), static_cast<S>(nd->children[1])));
            q.push(make_pair(D::pq_distance(d, margin, 0), static_cast<S>(nd->children[0])));
          }
        }

        // Get distances for all items
        // To avoid calculating distance multiple times for any items, sort by id
        std::sort(nns.begin(), nns.end());
        vector<pair<T, S> > nns_dist;
        S last = -1;
        for (size_t i = 0; i < nns.size(); i++) {
          S j = nns[i];
          if (j == last)
            continue;
          last = j;
          if (_get(j)->n_descendants == 1)  // This is only to guard a really obscure case, #284
            nns_dist.push_back(make_pair(D::distance(v_node, _get(j), _f), j));
        }

        size_t m = nns_dist.size();
        size_t p = n < m ? n : m; // Return this many items
        std::partial_sort(nns_dist.begin(), nns_dist.begin() + p, nns_dist.end());
        for (size_t i = 0; i < p; i++) {
          if (distances)
            distances->push_back(D::normalized_distance(nns_dist[i].first));
          result->push_back(nns_dist[i].second);
        }
      }
    */

    fn get_nearest(
        &self,
        query_vector: &[f32],
        n_results: usize,
        search_k: i32,
        should_include_distance: bool,
    ) -> Vec<AnnoyIndexSearchResult> {
        let result_capcity = if n_results < self.node_count {
            n_results
        } else {
            self.node_count
        };
        let search_k_fixed = if search_k > 0 {
            search_k as usize
        } else {
            result_capcity * self.roots.len()
        };

        let mut pq = Vec::<PriorityQueueEntry>::with_capacity(self.roots.len() * FLOAT32_SIZE);
        for i in 0..self.roots.len() {
            let r = self.roots[i];
            pq.push(PriorityQueueEntry::new(std::f32::MAX, r));
        }

        let mut nearest_neighbors =
            std::collections::HashSet::<usize>::with_capacity(search_k_fixed);
        while nearest_neighbors.len() < search_k_fixed && !pq.is_empty() {
            pq.sort_by(|a, b| b.margin.partial_cmp(&a.margin).unwrap());
            let top = pq.remove(0);
            let top_node_offset = top.node_offset as usize;
            let n_descendants = self.mmap.read_i32(top_node_offset);
            let v = get_node_vector(self, top_node_offset);
            if n_descendants == 1 {
                if is_zero_vec(&v) {
                    continue;
                }

                nearest_neighbors.insert(top_node_offset / self.node_size as usize);
            } else if n_descendants <= self.min_leaf_size {
                for i in 0..n_descendants as usize {
                    let j = self.mmap.read_i32(top_node_offset + i * INT32_SIZE) as usize;
                    if is_zero_vec(&get_node_vector(self, j)) {
                        continue;
                    }

                    nearest_neighbors.insert(j);
                }
            } else {
                let margin = match self.index_type {
                    IndexType::Angular => dot_product(v.as_slice(), query_vector),
                    IndexType::Euclidean | IndexType::Manhattan => minkowski_margin(
                        v.as_slice(),
                        query_vector,
                        self.mmap.read_f32(top_node_offset + 4),
                    ),
                    IndexType::Dot => {
                        dot_product(v.as_slice(), query_vector)
                            + self.mmap.read_f32(top_node_offset + 12).powi(2)
                    }
                    _ => panic!("Not supported"),
                };
                let l_child = get_l_child_offset(
                    &self.mmap,
                    top_node_offset as i64,
                    self.node_size,
                    self.index_type_offset,
                );
                let r_child = get_r_child_offset(
                    &self.mmap,
                    top_node_offset as i64,
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
            let v = self.get_item_vector(nn as i64);
            if !is_zero_vec(&v) {
                let param1 = v.as_slice();
                let param2 = query_vector;
                sorted_nns.push(PriorityQueueEntry {
                    margin: match self.index_type {
                        IndexType::Angular => cosine_distance(param1, param2),
                        IndexType::Euclidean => euclidean_distance(param1, param2),
                        IndexType::Manhattan => manhattan_distance(param1, param2),
                        IndexType::Dot => -dot_product(param1, param2),
                        _ => panic!("Not supported"),
                    },
                    node_offset: nn as i64,
                });
            }
        }

        sorted_nns.sort_by(|a, b| a.margin.partial_cmp(&b.margin).unwrap());

        let mut results: Vec<AnnoyIndexSearchResult> = Vec::with_capacity(result_capcity);
        for i in 0..n_results.min(sorted_nns.len()) {
            let nn = &sorted_nns[i];
            results.push(AnnoyIndexSearchResult {
                id: nn.node_offset,
                distance: if should_include_distance {
                    match self.index_type {
                        IndexType::Angular => nn.margin.sqrt(),
                        IndexType::Dot => -nn.margin,
                        _ => nn.margin,
                    }
                } else {
                    0.0
                },
            });
        }

        return results;
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
