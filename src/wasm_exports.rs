use crate::*;
use js_sys::{Array, Error, Uint8Array};
use serde::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResultJs {
    pub id: u64,
    pub distance: Option<f32>,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct AnnoyIndexJs {
    pub dimension: usize,
    pub size: usize,

    index_ptr: *const AnnoyIndex,
}

#[wasm_bindgen]
impl AnnoyIndexJs {
    pub fn free(&self) {
        unsafe {
            Box::from_raw(self.index_ptr as *mut AnnoyIndex);
        }
    }

    pub fn get_item_vector(&self, item_index: u64) -> Array {
        let index = unsafe { &*self.index_ptr };
        let item_vec = index.get_item_vector(item_index);
        let array = Array::new();
        for v in item_vec {
            array.push(&JsValue::from_f64(v as f64));
        }
        array
    }

    pub fn get_nearest(
        &self,
        query_vector: Array,
        n_results: u32,
        search_k: i32,
        should_include_distance: bool,
    ) -> Result<Array, Error> {
        let index = unsafe { &*self.index_ptr };
        if query_vector.length() as usize != index.dimension {
            return Err(Error::new(&format!(
                "Wrong input dimension, {} expected, {} provided.",
                index.dimension,
                query_vector.length()
            )));
        }
        let mut vec = Vec::with_capacity(index.dimension);
        for i in 0..(index.dimension as i32) {
            let v = query_vector.at(i);
            if let Some(v) = v.as_f64() {
                vec.push(v as f32);
            } else {
                return Err(Error::new(&format!(
                    "Input array should be of number type.",
                )));
            }
        }
        let result = index.get_nearest(
            vec.as_slice(),
            n_results as usize,
            search_k,
            should_include_distance,
        );
        convert_result(result)
    }

    pub fn get_nearest_to_item(
        &self,
        item_index: u64,
        n_results: u32,
        search_k: i32,
        should_include_distance: bool,
    ) -> Result<Array, Error> {
        let index = unsafe { &*self.index_ptr };
        let result = index.get_nearest_to_item(
            item_index,
            n_results as usize,
            search_k,
            should_include_distance,
        );
        convert_result(result)
    }
}

#[wasm_bindgen]
pub fn load_index(
    u8a: &Uint8Array,
    dimension: usize,
    index_type: IndexType,
) -> Result<AnnoyIndexJs, Error> {
    let mut buffer = vec![0_u8; u8a.length() as usize];
    u8a.copy_to(&mut buffer);
    let index = AnnoyIndex::load_with_buffer(buffer, dimension, index_type)
        .map_err(|err| Error::new(&format!("{err}")))?;
    Ok(AnnoyIndexJs {
        dimension: index.dimension,
        size: index.size,

        index_ptr: Box::into_raw(Box::new(index)),
    })
}

fn convert_result(result: AnnoyIndexSearchResult) -> Result<Array, Error> {
    let array = Array::new();
    for i in 0..result.count {
        let id = result.id_list[i];
        let distance = if result.is_distance_included {
            Some(result.distance_list[i])
        } else {
            None
        };
        array.push(
            &serde_wasm_bindgen::to_value(&SearchResultJs { id, distance })
                .map_err(|err| Error::new(&format!("{err}")))?,
        );
    }
    Ok(array)
}
