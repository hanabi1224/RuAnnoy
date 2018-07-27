mod annoy_index_search_result;
mod pqentry;

extern crate memmap;

use std::f32;
use std::fs;
use std::fs::{File};
use std::vec::Vec;
use std::mem;
use memmap::{Mmap, MmapOptions};

use pqentry::PriorityQueueEntry;
use annoy_index_search_result::AnnoyIndexSearchResult;

const INT32_SIZE:i32 = 4;
const FLOAT32_SIZE:i32 = 4;

#[derive(PartialEq)]
pub enum IndexType {
    Angular,
    Euclidean,
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

pub trait AnnoyIndexSearchApi {
    fn get_item_vector(&self, item_index: i64) ->Vec<f32> ;
    fn get_nearest(&self, query_vector: &[f32], n_results: usize, search_k: i32, should_include_distance: bool) -> Vec<AnnoyIndexSearchResult>;
}

trait MmapExtensions{
    fn read_i32(&self, idx: usize)->i32;
    fn read_f32(&self, idx: usize)->f32;
}

impl MmapExtensions for Mmap{
    fn read_i32(&self, idx: usize)->i32{
        let array = [*&self[idx], *&self[idx+1],*&self[idx+2],*&self[idx+3]];
        return unsafe { mem::transmute::<[u8;4],i32>(array) };
    }

    fn read_f32(&self, idx: usize)->f32{
        let array = [*&self[idx], *&self[idx+1],*&self[idx+2],*&self[idx+3]];
        return unsafe { mem::transmute::<[u8;4],f32>(array) };
    }
}

impl AnnoyIndex {
    pub fn load(dimension: i32, index_file_path: &str, index_type: IndexType) -> AnnoyIndex {
        let index_type_offset:i32 = if index_type == IndexType::Angular {4} else {8};
        let k_node_header_style:i32 = if index_type == IndexType::Angular {12} else {16};
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

fn get_node_vector(index: &AnnoyIndex, node_offset:i64)-> Vec<f32>{
    let mut vec:Vec<f32> = Vec::with_capacity(index.dimension as usize);
    for i in 0..index.dimension as usize {
        let idx = node_offset as usize + index.k_node_header_style as usize + i * (FLOAT32_SIZE as usize);
        let value = index.mmap.read_f32(idx);
        vec.push(value);
    }

    return vec;
}

impl AnnoyIndexSearchApi for AnnoyIndex {
    fn get_item_vector(&self, item_index: i64) -> Vec<f32> {
        let node_offset = item_index * self.node_size;
        return get_node_vector(self, node_offset);
    }

    fn get_nearest(&self, query_vector: &[f32], n_results: usize, search_k: i32, should_include_distance: bool) -> Vec<AnnoyIndexSearchResult> {
        let mut search_k_mut = search_k;
        if search_k <=0{
            search_k_mut = n_results as i32 * (self.roots.len() as i32);
        }

        let mut pq = Vec::<PriorityQueueEntry>::with_capacity(self.roots.len() * (FLOAT32_SIZE as usize));
        for r in &self.roots {
            pq.push(PriorityQueueEntry::new(std::f32::MAX, *r));
        }

        let mut nearest_neighbors = std::collections::HashSet::<i64>::new();
        while nearest_neighbors.len() < search_k_mut as usize && !pq.is_empty(){
            pq.sort_by(|a, b| b.margin.partial_cmp(&a.margin).unwrap());
            let top = pq.remove(0);
            let top_node_offset = top.node_offset;
            let n_descendants = self.mmap.read_i32(top_node_offset as usize);
            let v = get_node_vector(self, top_node_offset);
            if n_descendants == 1{
                if is_zero_vec(&v){
                    continue;
                }

                nearest_neighbors.insert(top_node_offset / self.node_size);
            }
            else if n_descendants <= self.min_leaf_size{
                for i in 0..n_descendants as usize{
                    let j = self.mmap.read_i32(top_node_offset as usize + i * INT32_SIZE as usize) as i64;
                    if is_zero_vec(&get_node_vector(self, j)){
                        continue;
                    }

                    nearest_neighbors.insert(j);
                }
            }
            else{
                let margin = if self.index_type == IndexType::Angular {cosine_margin_no_norm(v.as_slice(), query_vector)} else {euclidean_margin(v.as_slice(), query_vector, get_node_bias(self, top_node_offset))};
                let l_child = get_l_child_offset(&self.mmap, top_node_offset, self.node_size, self.index_type_offset);
                let r_child = get_r_child_offset(&self.mmap, top_node_offset, self.node_size, self.index_type_offset);
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

        let mut sorted_nns:Vec<PriorityQueueEntry> = Vec::new();
        for nn in nearest_neighbors{
            let mut v = self.get_item_vector(nn);
            if !is_zero_vec(&v){
                let param1 = v.as_slice();
                let param2 = query_vector;
                sorted_nns.push(PriorityQueueEntry {
                    margin: if self.index_type == IndexType::Angular {cosine_distance(param1, param2)} else {euclidean_distance(param1, param2)},
                    node_offset: nn,
                });
            }
        }

        sorted_nns.sort_by(|a,b|a.margin.partial_cmp(&b.margin).unwrap());

        let mut results: Vec<AnnoyIndexSearchResult> = Vec::with_capacity(n_results);
        for i in 0..n_results{
            let nn = &sorted_nns[i];
            results.push(AnnoyIndexSearchResult{
                id: nn.node_offset,
                distance: if should_include_distance {nn.margin.sqrt()} else {0.0},
            });
        }

        return  results;
    }
}

fn is_zero_vec(v:&Vec<f32>)->bool{
    for item in v{
        if *item != 0.0{
            return false;
        }
    }

    return true;
}

fn cosine_margin_no_norm(u:&[f32], v:&[f32])->f32{
    let mut d:f32 = 0.0;
    for i in 0..u.len(){
        d += u[i] * v[i];
    }

    return d;
}

fn euclidean_margin(u:&[f32], v:&[f32], bias:f32)->f32{
    let mut d:f32 = bias;
    for i in 0..u.len(){
        d += u[i] * v[i];
    }

    return d;
}

fn cosine_distance(u:&[f32], v:&[f32])->f32{
    // want to calculate (a/|a| - b/|b|)^2
    // = a^2 / a^2 + b^2 / b^2 - 2ab/|a||b|
    // = 2 - 2cos
    let mut pp:f32 = 0.0;
    let mut qq:f32 = 0.0;
    let mut pq:f32 = 0.0;

    for i in 0..u.len(){
        let _u = u[i];
        let _v = v[i];
        pp += _u.powi(2);
        qq += _v.powi(2);
        pq += _u * _v;
    }

    let ppqq = pp * qq;
    return if ppqq > 0.0 {2.0-2.0*pq / ppqq.sqrt()} else {2.0};
}

fn euclidean_distance(u:&[f32], v:&[f32])->f32{
    let mut diff: Vec<f32> = Vec::with_capacity(u.len());
    for i in 0..u.len(){
        diff[i] = u[i] - v[i];
    }

    let mut n:f32 = 0.0;
    for item in diff{
        n += item.powi(2);
    }

    return n.sqrt();
}

fn get_node_bias(index:&AnnoyIndex, node_offset:i64)->f32{
    return index.mmap.read_f32(node_offset as usize+4);
}
