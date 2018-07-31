use super::IndexType;
use super::AnnoyIndex;
use super::AnnoyIndexSearchApi;

use libc::{c_char,size_t};
use std::mem;
use std::ffi::CStr;
use std::slice;

ffi_fn! {
    fn load_annoy_index(path: *const c_char, dimension: i32, index_type: u8) -> *const AnnoyIndex {
        //let ru_dimension = dimension;
        let c_str_path =  unsafe { CStr::from_ptr(path) };
        let ru_path = c_str_path.to_str().unwrap();
        let ru_index_type:IndexType = unsafe { mem::transmute(index_type) };
        let index = AnnoyIndex::load(dimension, ru_path, ru_index_type);
        return Box::into_raw(Box::new(index));
    }
}

ffi_fn! {
    fn free_annoy_index(index: *const AnnoyIndex){
        unsafe { Box::from_raw(index as *mut AnnoyIndex); }
    }
}

ffi_fn! {
    fn get_dimension(index_ptr: *const AnnoyIndex) -> i32{
        let index = unsafe { &*index_ptr };
        return index.dimension;
    }
}

ffi_fn! {
    fn get_item_vector(index_ptr: *const AnnoyIndex, item_index: i64) -> *const f32{
        let index = unsafe{&*index_ptr};
        let item_vec = index.get_item_vector(item_index);
        return item_vec.as_ptr();
    }
}

#[repr(C)]
pub struct AnnoyIndexSearchResult_FFI {
    pub count: size_t,
    pub id_list: Box<[i64]>,
    pub distance_list: Box<[f32]>,
}

ffi_fn! {
    fn get_nearest(
        index_ptr: *const AnnoyIndex,
        query_vector_ptr: *const f32,
        n_results: size_t,
        search_k: i32,
        should_include_distance: bool) -> *const AnnoyIndexSearchResult_FFI
    {
        let index = unsafe{&*index_ptr};
        let query_vector = unsafe { slice::from_raw_parts(query_vector_ptr, index.dimension as usize) };
        let results = index.get_nearest(query_vector, n_results, search_k, should_include_distance);
        let results_len = results.len();
        let mut id_list = Vec::<i64>::with_capacity(results_len);
        let mut distance_list:Vec<f32> = Vec::with_capacity(results_len);
        for result in results{
            id_list.push(result.id);
            distance_list.push(result.distance);
        }

        let result_ffi = AnnoyIndexSearchResult_FFI{
            count: results_len,
            id_list: id_list.into_boxed_slice(),
            distance_list: distance_list.into_boxed_slice(),
        };

        return Box::into_raw(Box::new(result_ffi));;
    }
}

ffi_fn! {
    fn free_search_result(search_result_ptr: *const AnnoyIndexSearchResult_FFI){
        unsafe { Box::from_raw(search_result_ptr as *mut AnnoyIndexSearchResult_FFI); }
    }
}

ffi_fn! {
    fn get_result_count(search_result_ptr: *const AnnoyIndexSearchResult_FFI) -> usize{
        let search_result = unsafe{&*search_result_ptr};
        return search_result.count;
    }
}

ffi_fn! {
    fn get_id_list(search_result_ptr: *const AnnoyIndexSearchResult_FFI)->*const i64{
        let search_result = unsafe{&*search_result_ptr};
        return search_result.id_list.as_ptr();
    }
}

ffi_fn! {
    fn get_distance_list(search_result_ptr: *const AnnoyIndexSearchResult_FFI)->*const f32{
        let search_result = unsafe{&*search_result_ptr};
        return search_result.distance_list.as_ptr();
    }
}
