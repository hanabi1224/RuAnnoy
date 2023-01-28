#[cfg(test)]
mod tests {
    use annoy_rs_ffi::{annoy_rs::*, *};
    use libc::c_char;
    use std::alloc::{alloc, Layout};
    use std::ffi::CString;
    use std::ptr;
    use std::slice;

    const F32_PRECISION: usize = 2;
    const TEST_INDEX_DIM: usize = 5;
    const TEST_NODE_COUNT: usize = 100;

    #[test]
    fn sanity_tests_angular() {
        sanity_tests_inner(
            IndexType::Angular,
            &[
                -0.388_461_32,
                0.879_120_65,
                0.058_009_166,
                0.866_426_65,
                0.402_518_24,
            ],
            &[0, 4, 37, 61, 29],
            &[0.0, 0.416_088_22, 0.551_752_3, 0.734_209_54, 0.759_296_1],
        );
    }

    #[test]
    fn sanity_tests_euclidean() {
        sanity_tests_inner(
            IndexType::Euclidean,
            &[
                1.522_306_6,
                -1.520_689_5,
                0.226_999_3,
                0.408_149_27,
                0.640_252_8,
            ],
            &[0, 84, 20, 49, 94],
            &[0.0, 0.934_874_3, 1.105_167_6, 1.105_779_3, 1.129_980_7],
        );
    }

    #[test]
    fn sanity_tests_manhattan() {
        sanity_tests_inner(
            IndexType::Manhattan,
            &[
                -0.794_453_5,
                0.907_682_3,
                1.816_441_7,
                -0.783_995_8,
                -0.655_002_24,
            ],
            &[0, 34, 89, 83, 41],
            &[0.0, 1.683_599_5, 1.797_636, 2.139_925, 2.144_656_2],
        );
    }

    #[test]
    fn sanity_tests_dot() {
        sanity_tests_inner(
            IndexType::Dot,
            &[
                -1.295_846_3,
                0.268_831_16,
                0.424_712_87,
                0.479_184_27,
                0.562_680_07,
            ],
            &[42, 89, 0, 40, 61],
            &[3.553_952_7, 3.538_242_3, 3.151_576, 3.045_288_3, 2.615_417],
        );
    }

    fn sanity_tests_inner(
        index_type: IndexType,
        expected_item3_vec: &[f32],
        expected_id_list: &[u64],
        expected_distance_list: &[f32],
    ) {
        let filepath = format!("../tests/index.{index_type}.{TEST_INDEX_DIM}d.ann");
        let filepath_cstring = CString::new(filepath).unwrap();
        unsafe {
            let index = load_annoy_index(
                filepath_cstring.into_raw() as *const c_char,
                TEST_INDEX_DIM as i32,
                index_type as u8,
            );
            let dim = get_dimension(index);
            assert_eq!(dim, TEST_INDEX_DIM as i32);
            let v3_raw = alloc(Layout::array::<f32>(dim as usize).unwrap()) as *mut f32;
            get_item_vector(index, 3, v3_raw);
            // let v3_raw = get_item_vector(index, 3);
            let v3 = slice::from_raw_parts(v3_raw as *mut f32, dim as usize).to_vec();
            assert_eq!(v3, expected_item3_vec);

            let v0_raw = alloc(Layout::array::<f32>(dim as usize).unwrap()) as *mut f32;
            get_item_vector(index, 0, v0_raw);
            let _v0 = slice::from_raw_parts(v0_raw as *mut f32, dim as usize).to_vec();
            // let v0_raw = get_item_vector(index, 0);
            assert_eq!(TEST_NODE_COUNT, get_size(index) as usize);
            {
                let nearest_raw = get_nearest(index, v0_raw, 5, -1, true);
                let result_count = get_result_count(nearest_raw) as usize;
                let id_list_raw = get_id_list(nearest_raw);
                let id_list = slice::from_raw_parts(id_list_raw as *mut u64, result_count).to_vec();
                assert_eq!(id_list, expected_id_list);
                let distance_list_raw = get_distance_list(nearest_raw);
                let distance_list =
                    slice::from_raw_parts(distance_list_raw as *mut f32, result_count).to_vec();
                assert_eq!(
                    distance_list.round_to(F32_PRECISION),
                    expected_distance_list.round_to(F32_PRECISION)
                );
                free_search_result(nearest_raw);
            }
            {
                let nearest_raw = get_nearest_to_item(index, 0, 5, -1, true);
                let result_count = get_result_count(nearest_raw) as usize;
                let id_list_raw = get_id_list(nearest_raw);
                let id_list = slice::from_raw_parts(id_list_raw as *mut u64, result_count).to_vec();
                assert_eq!(id_list, expected_id_list);
                let distance_list_raw = get_distance_list(nearest_raw);
                let distance_list =
                    slice::from_raw_parts(distance_list_raw as *mut f32, result_count).to_vec();
                assert_eq!(
                    distance_list.round_to(F32_PRECISION),
                    expected_distance_list.round_to(F32_PRECISION)
                );
                free_search_result(nearest_raw);
            }
            {
                let nearest_raw = get_nearest(index, v0_raw, 5, -1, false);
                let result_count = get_result_count(nearest_raw) as usize;
                let id_list_raw = get_id_list(nearest_raw);
                let id_list = slice::from_raw_parts(id_list_raw as *mut u64, result_count).to_vec();
                assert_eq!(id_list, expected_id_list);
                let distance_list_raw = get_distance_list(nearest_raw);
                let distance_list =
                    slice::from_raw_parts(distance_list_raw as *mut f32, 0).to_vec();
                assert_eq!(0, distance_list.len());
                assert_eq!(0, distance_list.capacity());
                free_search_result(nearest_raw);
            }
            {
                let nearest_raw = get_nearest_to_item(index, 0, 5, -1, false);
                let result_count = get_result_count(nearest_raw) as usize;
                let id_list_raw = get_id_list(nearest_raw);
                let id_list = slice::from_raw_parts(id_list_raw as *mut u64, result_count).to_vec();
                assert_eq!(id_list, expected_id_list);
                let distance_list_raw = get_distance_list(nearest_raw);
                let distance_list =
                    slice::from_raw_parts(distance_list_raw as *mut f32, 0).to_vec();
                assert_eq!(0, distance_list.len());
                assert_eq!(0, distance_list.capacity());
                free_search_result(nearest_raw);
            }
            free_annoy_index(index);
        }
    }

    #[test]
    fn invalid_index_cffi() {
        let index_ptr = unsafe {
            load_annoy_index(
                CString::new("invalid_index.ann").unwrap().into_raw() as *const c_char,
                TEST_INDEX_DIM as i32,
                IndexType::Angular as u8,
            )
        };
        assert_eq!(index_ptr, ptr::null());
    }

    pub trait RoundToVec<T> {
        fn round_to(&self, n: usize) -> Vec<T>;
    }

    impl RoundToVec<f32> for [f32] {
        fn round_to(&self, n: usize) -> Vec<f32> {
            self.iter()
                .map(|v| {
                    let factor = 10.0_f32.powi(n as i32);
                    (v * factor).round() / factor
                })
                .collect()
        }
    }
}
