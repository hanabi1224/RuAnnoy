#[cfg(test)]
mod tests {
    #[cfg(feature = "cffi")]
    use libc::c_char;
    use ru_annoy::*;
    #[cfg(feature = "cffi")]
    use std::alloc::{alloc, Layout};
    #[cfg(feature = "cffi")]
    use std::ffi::CString;
    #[cfg(feature = "cffi")]
    use std::ptr;
    #[cfg(feature = "cffi")]
    use std::slice;

    const TEST_INDEX_DIM: usize = 5;
    const TEST_NODE_COUNT: usize = 100;

    #[test]
    fn sanity_tests_angular() {
        sanity_tests_inner(
            IndexType::Angular,
            &[
                -0.38846132159233093,
                0.8791206479072571,
                0.05800916627049446,
                0.8664266467094421,
                0.40251824259757996,
            ],
            &[0, 4, 37, 61, 29],
            &[0.0, 0.41608825, 0.5517523, 0.7342095, 0.7592962],
        );
    }

    #[test]
    fn sanity_tests_euclidean() {
        sanity_tests_inner(
            IndexType::Euclidean,
            &[
                1.5223065614700317,
                -1.5206894874572754,
                0.22699929773807526,
                0.40814927220344543,
                0.6402528285980225,
            ],
            &[0, 84, 16, 20, 49],
            &[
                0.0,
                0.9348742961883545,
                1.047611,
                1.1051676273345947,
                1.1057792901992798,
            ],
        );
    }

    #[test]
    fn sanity_tests_manhattan() {
        sanity_tests_inner(
            IndexType::Manhattan,
            &[
                -0.794453501701355,
                0.9076822996139526,
                1.8164416551589966,
                -0.7839958071708679,
                -0.655002236366272,
            ],
            &[0, 34, 89, 83, 41],
            &[
                0.0,
                1.6835994720458984,
                1.7976360321044922,
                2.139925003051758,
                2.144656181335449,
            ],
        );
    }

    #[test]
    fn sanity_tests_dot() {
        sanity_tests_inner(
            IndexType::Dot,
            &[
                -1.2958463430404663,
                0.26883116364479065,
                0.4247128665447235,
                0.47918426990509033,
                0.5626800656318665,
            ],
            &[42, 89, 0, 40, 67],
            &[
                3.553952693939209,
                3.5382423400878906,
                3.151576042175293,
                3.045288324356079,
                2.7035549,
            ],
        );
    }

    fn sanity_tests_inner(
        index_type: IndexType,
        expected_item3_vec: &[f32],
        expected_id_list: &[u64],
        expected_distance_list: &[f32],
    ) {
        let filepath = format!("tests/index.{}.{}d.ann", index_type, TEST_INDEX_DIM);
        let index = AnnoyIndex::load(TEST_INDEX_DIM, &filepath, index_type).unwrap();
        assert_eq!(index.get_item_vector(3), expected_item3_vec);

        let v0 = index.get_item_vector(0);
        let nearest = index.get_nearest(v0.as_ref(), 5, -1, true);
        let id_list = nearest.id_list;
        let distance_list = nearest.distance_list;
        assert_eq!(index.size, TEST_NODE_COUNT);
        assert_eq!(id_list, expected_id_list);
        assert_eq!(distance_list, expected_distance_list);
        assert_eq!(distance_list.len(), expected_distance_list.len());
        for i in 0..distance_list.len() {
            let a = distance_list[i];
            let b = expected_distance_list[i];
            assert!((a - b).abs() < 1e-5);
        }

        #[cfg(feature = "cffi")]
        sanity_tests_inner_ffi(
            index_type,
            expected_item3_vec,
            expected_id_list,
            expected_distance_list,
        );
    }

    #[cfg(feature = "cffi")]
    fn sanity_tests_inner_ffi(
        index_type: IndexType,
        expected_item3_vec: &[f32],
        expected_id_list: &[u64],
        expected_distance_list: &[f32],
    ) {
        let filepath = format!("tests/index.{}.{}d.ann", index_type, TEST_INDEX_DIM);
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
                let result_count = get_result_count(nearest_raw);
                let id_list_raw = get_id_list(nearest_raw);
                let id_list = slice::from_raw_parts(id_list_raw as *mut u64, result_count).to_vec();
                assert_eq!(id_list, expected_id_list);
                let distance_list_raw = get_distance_list(nearest_raw);
                let distance_list =
                    slice::from_raw_parts(distance_list_raw as *mut f32, result_count).to_vec();
                assert_eq!(distance_list, expected_distance_list);
                free_search_result(nearest_raw);
            }
            {
                let nearest_raw = get_nearest_to_item(index, 0, 5, -1, true);
                let result_count = get_result_count(nearest_raw);
                let id_list_raw = get_id_list(nearest_raw);
                let id_list = slice::from_raw_parts(id_list_raw as *mut u64, result_count).to_vec();
                assert_eq!(id_list, expected_id_list);
                let distance_list_raw = get_distance_list(nearest_raw);
                let distance_list =
                    slice::from_raw_parts(distance_list_raw as *mut f32, result_count).to_vec();
                assert_eq!(distance_list, expected_distance_list);
                free_search_result(nearest_raw);
            }
            {
                let nearest_raw = get_nearest(index, v0_raw, 5, -1, false);
                let result_count = get_result_count(nearest_raw);
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
                let result_count = get_result_count(nearest_raw);
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
    #[cfg(feature = "cffi")]
    fn invalid_index_cffi() {
        let index_ptr = load_annoy_index(
            CString::new("invalid_index.ann").unwrap().into_raw() as *const c_char,
            TEST_INDEX_DIM as i32,
            IndexType::Angular as u8,
        );
        assert_eq!(index_ptr, ptr::null());
    }

    #[test]
    fn hole_tests() {
        let filepath = "tests/hole.10d.ann";
        let index = AnnoyIndex::load(10, filepath, IndexType::Angular).unwrap();
        let v1 = vec![
            0.10471842,
            0.55223828,
            0.44094049,
            0.98384884,
            0.22485616,
            -0.79840456,
            -1.78999692,
            -1.11747558,
            0.05733591,
            1.35356555,
        ];
        let nearest = index.get_nearest(v1.as_ref(), 100, -1, true);
        assert_eq!(nearest.count, 1);
        assert_eq!(nearest.id_list[0], 1000);
    }
}
