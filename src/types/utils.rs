use crate::internals::mmap_ext::*;
use memmap2::Mmap;

pub const INT32_SIZE: i32 = 4;
pub const FLOAT32_SIZE: i32 = 4;

pub fn is_zero_vec(v: &Vec<f32>) -> bool {
    for item in v {
        if *item != 0.0 {
            return false;
        }
    }

    return true;
}

pub fn cosine_margin_no_norm(u: &[f32], v: &[f32]) -> f32 {
    let mut d: f32 = 0.0;
    for i in 0..u.len() {
        d += u[i] * v[i];
    }

    return d;
}

pub fn euclidean_margin(u: &[f32], v: &[f32], bias: f32) -> f32 {
    let mut d: f32 = bias;
    for i in 0..u.len() {
        d += u[i] * v[i];
    }

    return d;
}

pub fn cosine_distance(u: &[f32], v: &[f32]) -> f32 {
    // want to calculate (a/|a| - b/|b|)^2
    // = a^2 / a^2 + b^2 / b^2 - 2ab/|a||b|
    // = 2 - 2cos
    let mut pp: f32 = 0.0;
    let mut qq: f32 = 0.0;
    let mut pq: f32 = 0.0;

    for i in 0..u.len() {
        let _u = u[i];
        let _v = v[i];
        pp += _u.powi(2);
        qq += _v.powi(2);
        pq += _u * _v;
    }

    let ppqq = pp * qq;
    return if ppqq > 0.0 {
        2.0 - 2.0 * pq / ppqq.sqrt()
    } else {
        2.0
    };
}

pub fn euclidean_distance(u: &[f32], v: &[f32]) -> f32 {
    let mut diff: Vec<f32> = Vec::with_capacity(u.len());
    for i in 0..u.len() {
        diff[i] = u[i] - v[i];
    }

    let mut n: f32 = 0.0;
    for item in diff {
        n += item.powi(2);
    }

    return n.sqrt();
}

pub fn get_l_child_offset(
    mmap: &Mmap,
    top_node_offset: i64,
    node_size: i64,
    index_type_offset: i32,
) -> i64 {
    let child_offset = top_node_offset as usize + index_type_offset as usize;
    let child = mmap.read_i32(child_offset) as i64;
    return node_size * child;
}

pub fn get_r_child_offset(
    mmap: &Mmap,
    top_node_offset: i64,
    node_size: i64,
    index_type_offset: i32,
) -> i64 {
    let child_offset = top_node_offset as usize + index_type_offset as usize + 4;
    let child = mmap.read_i32(child_offset) as i64;
    return node_size * child;
}
