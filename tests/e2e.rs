#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use annoy_rs::*;

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
        let filepath = format!("tests/index.{index_type}.{TEST_INDEX_DIM}d.ann");
        for index in [
            AnnoyIndex::load(TEST_INDEX_DIM, &filepath, index_type.clone()).unwrap(),
            AnnoyIndex::load_into_mem(TEST_INDEX_DIM, &filepath, index_type).unwrap(),
        ] {
            assert_eq!(index.get_item_vector(3), expected_item3_vec);

            let v0 = index.get_item_vector(0);
            let nearest = index.get_nearest(v0.as_ref(), 5, -1, true);
            let nearest2 = index.get_nearest_to_item(0, 5, -1, true);
            assert_eq!(format!("{nearest:?}"), format!("{nearest2:?}"));
            let id_list = nearest.id_list;
            let distance_list = nearest.distance_list;
            assert_eq!(index.size, TEST_NODE_COUNT);
            assert_eq!(id_list, expected_id_list);
            assert_eq!(
                distance_list.round_to(F32_PRECISION),
                expected_distance_list.round_to(F32_PRECISION)
            );
            assert_eq!(distance_list.len(), expected_distance_list.len());
            for i in 0..distance_list.len() {
                let a = distance_list[i];
                let b = expected_distance_list[i];
                assert!((a - b).abs() < 1e-5);
            }
        }
    }

    #[test]
    fn hole_tests() {
        static HOLE_INDEX_BYTES: &[u8] = include_bytes!("hole.10d.ann");
        let index =
            AnnoyIndex::load_from_buffer(HOLE_INDEX_BYTES.into(), 10, IndexType::Angular).unwrap();
        assert_eq!(index.dimension, 10);
        assert_eq!(index.size, 1001);
        let v1 = vec![
            0.10471842,
            0.552_238_3,
            0.440_940_5,
            0.98384884,
            0.22485616,
            -0.798_404_6,
            -1.789_996_9,
            -1.117_475_6,
            0.05733591,
            1.353_565_6,
        ];
        let nearest = index.get_nearest(v1.as_ref(), 100, -1, true);
        assert_eq!(nearest.count, 1);
        assert_eq!(nearest.id_list[0], 1000);
        assert_eq!(nearest.distance_list[0], 1.212572);
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
