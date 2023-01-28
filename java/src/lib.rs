#[macro_use]
mod macros;

use annoy_rs::*;
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jbyte, jclass, jfloatArray, jint, jlong, jlongArray};
use jni::JNIEnv;
use std::error::Error;
use std::mem;

/*
 * Class:     com_github_hanabi1224_RuAnnoy_NativeMethods
 * Method:    loadIndex
 * Signature: (Ljava/lang/String;IB)J
 */
// JNIEXPORT jlong JNICALL Java_com_github_hanabi1224_RuAnnoy_NativeMethods_loadIndex
//   (JNIEnv *, jclass, jstring, jint, jbyte);
ffi_fn! {
    fn Java_com_github_hanabi1224_RuAnnoy_NativeMethods_loadIndex(
        env: JNIEnv,
        class: JClass,
        path: JString,
        dimension: jint,
        index_type: jbyte,
    ) -> jlong {
        let result = Java_com_github_hanabi1224_RuAnnoy_NativeMethods_loadIndex_inner(
            env, class, path, dimension, index_type,
        );
        match result {
            Ok(pointer) => pointer,
            _ => 0,
        }
    }
}

#[allow(non_snake_case)]
fn Java_com_github_hanabi1224_RuAnnoy_NativeMethods_loadIndex_inner(
    env: JNIEnv,
    _class: JClass,
    path: JString,
    dimension: jint,
    index_type: jbyte,
) -> Result<jlong, Box<dyn Error>> {
    let ru_path: String = env.get_string(path)?.into();
    let ru_index_type: IndexType = unsafe { mem::transmute(index_type) };
    let index = AnnoyIndex::load(dimension as usize, ru_path.as_str(), ru_index_type)?;
    let ptr = Box::into_raw(Box::new(index));
    Ok(ptr as jlong)
}

/*
 * Class:     com_github_hanabi1224_RuAnnoy_NativeMethods
 * Method:    freeIndex
 * Signature: (J)V
 */
// JNIEXPORT void JNICALL Java_com_github_hanabi1224_RuAnnoy_NativeMethods_freeIndex
//   (JNIEnv *, jclass, jlong);
ffi_fn! {
    fn Java_com_github_hanabi1224_RuAnnoy_NativeMethods_freeIndex(
        env: JNIEnv,
        class: JClass,
        pointer: jlong,
    ) {
        unsafe {
            drop(Box::from_raw(pointer as *mut AnnoyIndex));
        }
    }
}

/*
 * Class:     com_github_hanabi1224_RuAnnoy_NativeMethods
 * Method:    getIndexSize
 * Signature: (J)J
 */
// JNIEXPORT jlong JNICALL Java_com_github_hanabi1224_RuAnnoy_NativeMethods_getIndexSize
//   (JNIEnv *, jclass, jlong);
ffi_fn! {
    fn Java_com_github_hanabi1224_RuAnnoy_NativeMethods_getIndexSize(
        env: JNIEnv,
        class: JClass,
        pointer: jlong,
    ) -> jlong {
        let index = unsafe { &*(pointer as *const AnnoyIndex) };
        index.size as jlong
    }
}

/*
 * Class:     com_github_hanabi1224_RuAnnoy_NativeMethods
 * Method:    getItemVector
 * Signature: (JJ)[F
 */
// JNIEXPORT jfloatArray JNICALL Java_com_github_hanabi1224_RuAnnoy_NativeMethods_getItemVector
//   (JNIEnv *, jclass, jlong, jlong);
ffi_fn! {
    fn Java_com_github_hanabi1224_RuAnnoy_NativeMethods_getItemVector(
        env: JNIEnv,
        _class: jclass,
        pointer: jlong,
        item_index: jlong,
    ) -> jfloatArray {
        let index = unsafe { &*(pointer as *const AnnoyIndex) };
        let vector = index.get_item_vector(item_index as u64);
        let result = env.new_float_array(index.dimension as i32).unwrap();
        let _ = env.set_float_array_region(result, 0, vector.as_slice());
        result
    }
}

/*
 * Class:     com_github_hanabi1224_RuAnnoy_NativeMethods
 * Method:    getNearestToItem
 * Signature: (JJIIZ[J[F)I
 */
// JNIEXPORT jint JNICALL Java_com_github_hanabi1224_RuAnnoy_NativeMethods_getNearestToItem
//   (JNIEnv *, jclass, jlong, jlong, jint, jint, jboolean, jlongArray, jfloatArray);
ffi_fn! {
    fn Java_com_github_hanabi1224_RuAnnoy_NativeMethods_getNearestToItem(
        env: JNIEnv,
        _class: jclass,
        pointer: jlong,
        item_index: jlong,
        n_results: jint,
        search_k: jint,
        should_include_distance: jboolean,
        id_list: jlongArray,
        distance_list: jfloatArray,
    ) -> jint {
        let index = unsafe { &*(pointer as *const AnnoyIndex) };
        let r = index.get_nearest_to_item(
            item_index as u64,
            n_results as usize,
            search_k,
            should_include_distance != 0,
        );
        let r_id_list: Vec<i64> = r.id_list.iter().map(|&i| i as i64).collect();
        let _ = env.set_long_array_region(id_list, 0, r_id_list.as_slice());
        if should_include_distance != 0 {
            let _ = env.set_float_array_region(distance_list, 0, r.distance_list.as_slice());
        }
        r.count as jint
    }
}

/*
 * Class:     com_github_hanabi1224_RuAnnoy_NativeMethods
 * Method:    getNearest
 * Signature: (J[FIIZ[J[F)I
 */
// JNIEXPORT jint JNICALL Java_com_github_hanabi1224_RuAnnoy_NativeMethods_getNearest
//   (JNIEnv *, jclass, jlong, jfloatArray, jint, jint, jboolean, jlongArray, jfloatArray);
ffi_fn! {
    fn Java_com_github_hanabi1224_RuAnnoy_NativeMethods_getNearest(
        env: JNIEnv,
        _class: jclass,
        pointer: jlong,
        query_vector_j: jfloatArray,
        n_results: jint,
        search_k: jint,
        should_include_distance: jboolean,
        id_list: jlongArray,
        distance_list: jfloatArray,
    ) -> jint {
        let index = unsafe { &*(pointer as *const AnnoyIndex) };
        let dim = index.dimension;

        let mut query_vector = vec![0_f32; dim];
        match env.get_float_array_region(query_vector_j, 0, query_vector.as_mut_slice()) {
            Err(_) => 0,
            Ok(_) => {
                let r = index.get_nearest(
                    query_vector.as_slice(),
                    n_results as usize,
                    search_k,
                    should_include_distance != 0,
                );
                let r_id_list: Vec<i64> = r.id_list.iter().map(|&i| i as i64).collect();
                let _ = env.set_long_array_region(id_list, 0, r_id_list.as_slice());
                if should_include_distance != 0 {
                    let _ = env.set_float_array_region(distance_list, 0, r.distance_list.as_slice());
                }
                r.count as jint
            }
        }
    }
}
