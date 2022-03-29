use crate::internals::mmap_ext::*;
// #[cfg(feature = "simd")]
// use core::intrinsics::{fadd_fast, fdiv_fast, fmul_fast, fsub_fast, sqrtf32};
use memmap2::Mmap;
use std::mem;
#[cfg(feature = "simd")]
use std::simd::*;

// cfg_if! {
//     if #[cfg(feature = "simd")] {
//         lazy_static! {
//             static ref SIMD_INDICES:SimdUsize<SIMD_LANES> = SimdUsize::<SIMD_LANES>::from_array([0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]);
//         }
//     }
// }
#[cfg(feature = "simd")]
const SIMD_LANES: usize = 8;
#[cfg(feature = "simd")]
type SimdType = Simd<f32, SIMD_LANES>;
pub const INT32_SIZE: usize = mem::size_of::<i32>();
pub const FLOAT32_SIZE: usize = mem::size_of::<f32>();

pub fn minkowski_margin(u: &[f32], v: &[f32], bias: f32) -> f32 {
    bias + dot_product(u, v)
}

/*
template<typename S, typename T, typename Distance, typename Random, class ThreadedBuildPolicy>
  class AnnoyIndex : public AnnoyIndexInterface<S, T,
  AnnoyIndex<int32_t, uint64_t, Hamming, Kiss64Random, AnnoyIndexThreadedBuildPolicy> _index;
  static inline bool margin(const Node<S, T>* n, const T* y, int f) {
    static const size_t n_bits = sizeof(T) * 8;
    T chunk = n->v[0] / n_bits;
    return (y[chunk] & (static_cast<T>(1) << (n_bits - 1 - (n->v[0] % n_bits)))) != 0;
  }
*/
// pub fn hamming_margin(u: &[f32], v: &[f32], bias: f32) -> f32 {
//     return bias + dot_product(u, v);
// }

// #[inline(never)]
pub fn dot_product(u: &[f32], v: &[f32]) -> f32 {
    cfg_if! {
        if #[cfg(feature = "simd")] {
            dot_product_simd(u, v)
        } else {
            dot_product_no_simd(u, v)
        }
    }
}

pub fn dot_product_no_simd(u: &[f32], v: &[f32]) -> f32 {
    u.iter().zip(v.iter()).map(|(x, y)| x * y).sum()
}

#[cfg(feature = "simd")]
pub fn dot_product_simd(u: &[f32], v: &[f32]) -> f32 {
    let length = u.len();
    let mut dp = SimdType::splat(0.0);
    for i in (0..length).step_by(SIMD_LANES) {
        let u_chunk = extract_simd_type_from_slice(u, i, length);
        let v_chunk = extract_simd_type_from_slice(v, i, length);
        dp += u_chunk * v_chunk;
    }
    dp.reduce_sum()
}

// #[inline(never)]
pub fn cosine_distance(u: &[f32], v: &[f32]) -> f32 {
    cfg_if! {
        if #[cfg(feature = "simd")] {
            cosine_distance_simd(u, v)
        } else {
            cosine_distance_no_simd(u, v)
        }
    }
}

pub fn cosine_distance_no_simd(u: &[f32], v: &[f32]) -> f32 {
    // want to calculate (a/|a| - b/|b|)^2
    // = a^2 / a^2 + b^2 / b^2 - 2ab/|a||b|
    // = 2 - 2cos
    let mut pp: f32 = 0.0;
    let mut qq: f32 = 0.0;
    let mut pq: f32 = 0.0;
    for (_u, _v) in u.iter().zip(v.iter()) {
        pp += _u * _u;
        qq += _v * _v;
        pq += _u * _v;
    }
    let ppqq = pp * qq;
    if ppqq.is_sign_positive() {
        2.0 - 2.0 * pq / ppqq.sqrt()
    } else {
        2.0
    }
}

#[cfg(feature = "simd")]
pub fn cosine_distance_simd(u: &[f32], v: &[f32]) -> f32 {
    let length = u.len();
    let mut ppm = SimdType::default();
    let mut qqm = SimdType::default();
    let mut pqm = SimdType::default();
    for i in (0..length).step_by(SIMD_LANES) {
        let u_chunk = extract_simd_type_from_slice(u, i, length);
        let v_chunk = extract_simd_type_from_slice(v, i, length);
        ppm += u_chunk * u_chunk;
        qqm += v_chunk * v_chunk;
        pqm += u_chunk * v_chunk;
    }
    let pp = ppm.reduce_sum();
    let qq = qqm.reduce_sum();
    let pq = pqm.reduce_sum();
    let ppqq = pp * qq;
    if ppqq.is_sign_positive() {
        2.0 - 2.0 * pq / ppqq.sqrt()
    } else {
        2.0
    }
    // let ppqq = unsafe { fmul_fast(pp, qq) };
    // if ppqq.is_sign_positive() {
    //     unsafe { fsub_fast(2.0, fmul_fast(2.0, fdiv_fast(pq, sqrtf32(ppqq)))) }
    // } else {
    //     2.0
    // }
}

// #[inline(never)]
pub fn euclidean_distance(u: &[f32], v: &[f32]) -> f32 {
    cfg_if! {
        if #[cfg(feature = "simd")] {
            euclidean_distance_simd(u, v)
        } else {
            euclidean_distance_no_simd(u, v)
        }
    }
}

pub fn euclidean_distance_no_simd(u: &[f32], v: &[f32]) -> f32 {
    u.iter().zip(v.iter()).map(|(x, y)| power(x - y)).sum()
}

#[cfg(feature = "simd")]
pub fn euclidean_distance_simd(u: &[f32], v: &[f32]) -> f32 {
    let length = u.len();
    let mut sum = SimdType::default();
    for i in (0..length).step_by(SIMD_LANES) {
        let u_chunk = extract_simd_type_from_slice(u, i, length);
        let v_chunk = extract_simd_type_from_slice(v, i, length);
        sum += power_simd_type(u_chunk - v_chunk);
    }
    sum.reduce_sum()
}

// #[inline(never)]
pub fn manhattan_distance(u: &[f32], v: &[f32]) -> f32 {
    cfg_if! {
        if #[cfg(feature = "simd")] {
            manhattan_distance_simd(u, v)
        } else {
            manhattan_distance_no_simd(u, v)
        }
    }
}

pub fn manhattan_distance_no_simd(u: &[f32], v: &[f32]) -> f32 {
    u.iter().zip(v.iter()).map(|(x, y)| (x - y).abs()).sum()
}

#[cfg(feature = "simd")]
pub fn manhattan_distance_simd(u: &[f32], v: &[f32]) -> f32 {
    let length = u.len();
    let mut sum = SimdType::default();
    for i in (0..length).step_by(SIMD_LANES) {
        let u_chunk = extract_simd_type_from_slice(u, i, length);
        let v_chunk = extract_simd_type_from_slice(v, i, length);
        let diff = u_chunk - v_chunk;
        sum += diff.abs();
    }
    sum.reduce_sum()
}

pub fn get_nth_descendant_id(
    mmap: &Mmap,
    node_offset: usize,
    offset_before_children: usize,
    n: usize,
) -> usize {
    let child_offset = node_offset + offset_before_children + n * INT32_SIZE;
    mmap.read_i32(child_offset) as usize
}

#[cfg(feature = "simd")]
fn extract_simd_type_from_slice(array: &[f32], start: usize, length: usize) -> SimdType {
    let end = start + SIMD_LANES;
    if end > length {
        let mut simd_array = SimdType::default();
        // let part = &array[start..length];
        // for i in 0..part.len() {
        //     simd_array[i] = part[i];
        // }
        for i in start..length {
            simd_array[i - start] = array[i];
        }
        simd_array
        // SimdType::gather_or_default(&array[start..length], *SIMD_INDICES)
    } else {
        let array_fixed =
            unsafe { *(&array[start..end] as *const [f32] as *const [f32; SIMD_LANES]) };
        SimdType::from_array(array_fixed)
    }
}

#[inline(always)]
fn power(f: f32) -> f32 {
    f * f
}

#[cfg(feature = "simd")]
#[inline(always)]
fn power_simd_type(f: SimdType) -> SimdType {
    f * f
}

#[cfg(test)]
mod tests {
    use crate::tests::*;
    use crate::types::utils::*;
    use rand::prelude::*;
    use std::ops::Range;

    #[cfg(feature = "simd")]
    const SIMD_PARITY_PRECISION: usize = 2;

    lazy_static::lazy_static! {
        static ref BENCH_ARRAY_1: Vec<f32> = gen_range_array(0..255);
        static ref BENCH_ARRAY_2: Vec<f32> = gen_range_array(0..255);
    }

    fn gen_range_array(range: Range<usize>) -> Vec<f32> {
        let mut v = Vec::with_capacity(range.end - range.start);
        let mut rng = rand::thread_rng();
        for _i in range {
            v.push(rng.gen());
        }
        v
    }

    #[test]
    fn test_cosine_distance_no_simd() {
        let r = cosine_distance_no_simd(
            &[
                1.0689810514450073,
                0.5634735226631165,
                0.24886439740657806,
                0.7266523241996765,
                -0.646281898021698,
            ],
            &[
                1.0810768604278564,
                0.27467215061187744,
                0.09680532664060593,
                0.8381305932998657,
                -0.10710930079221725,
            ],
        );
        assert_eq!(r.sqrt(), 0.41608825);
    }

    #[test]
    #[cfg(feature = "simd")]
    fn test_cosine_distance_simd() {
        let r = cosine_distance_simd(
            &[
                1.0689810514450073,
                0.5634735226631165,
                0.24886439740657806,
                0.7266523241996765,
                -0.646281898021698,
            ],
            &[
                1.0810768604278564,
                0.27467215061187744,
                0.09680532664060593,
                0.8381305932998657,
                -0.10710930079221725,
            ],
        );
        assert_eq!(r.sqrt(), 0.41608825);
    }

    #[test]
    fn test_manhattan_distance() {
        let r = manhattan_distance(
            &[
                0.38532835245132446,
                -0.7025920152664185,
                -0.36306384205818176,
                0.6611577868461609,
                0.7517421245574951,
            ],
            &[
                -0.1129668727517128,
                -1.1781376600265503,
                -0.4161655306816101,
                0.6437731385231018,
                0.11246972531080246,
            ],
        );
        assert_eq!(r, 1.6835994720458984);
    }

    #[test]
    fn test_euclidean_distance_no_simd() {
        let r = euclidean_distance_no_simd(
            &[
                0.17124426364898682,
                -0.2053004503250122,
                -0.05337066948413849,
                0.45046135783195496,
                0.8933278918266296,
            ],
            &[
                -0.1711193025112152,
                -0.05677059665322304,
                -0.6459991931915283,
                0.7939537167549133,
                0.3780410885810852,
            ],
        );
        assert_eq!(r.sqrt(), 0.9348742961883545);
    }

    #[test]
    #[cfg(feature = "simd")]
    fn test_euclidean_distance_simd() {
        let r = euclidean_distance_simd(
            &[
                0.17124426364898682,
                -0.2053004503250122,
                -0.05337066948413849,
                0.45046135783195496,
                0.8933278918266296,
            ],
            &[
                -0.1711193025112152,
                -0.05677059665322304,
                -0.6459991931915283,
                0.7939537167549133,
                0.3780410885810852,
            ],
        );
        assert_eq!(r.sqrt(), 0.9348742961883545);
    }

    #[test]
    fn test_dot_product_no_simd() {
        let r = dot_product_no_simd(
            &[
                -0.04954042658209801,
                -1.297113060951233,
                -1.1471800804138184,
                -0.04162896052002907,
                0.3858293294906616,
            ],
            &[
                -0.4422854483127594,
                -1.4724655151367188,
                -1.4223746061325073,
                -1.7370465993881226,
                -0.25310218334198,
            ],
        );
        assert_eq!(r, 3.5382423400878906);
    }

    #[test]
    #[cfg(feature = "simd")]
    fn test_dot_product_simd() {
        let r = dot_product_simd(
            &[
                -0.04954042658209801,
                -1.297113060951233,
                -1.1471800804138184,
                -0.04162896052002907,
                0.3858293294906616,
            ],
            &[
                -0.4422854483127594,
                -1.4724655151367188,
                -1.4223746061325073,
                -1.7370465993881226,
                -0.25310218334198,
            ],
        );
        assert_eq!(r, 3.5382423400878906);
    }

    #[test]
    #[cfg(feature = "simd")]
    fn test_cosine_distance_simd_parity() {
        let a = &BENCH_ARRAY_1;
        let b = &BENCH_ARRAY_2;
        assert_eq!(
            cosine_distance_no_simd(a, b).round_to(SIMD_PARITY_PRECISION),
            cosine_distance_simd(a, b).round_to(SIMD_PARITY_PRECISION)
        );
    }

    #[test]
    #[cfg(feature = "simd")]
    fn test_euclidean_distance_simd_parity() {
        let a = &BENCH_ARRAY_1;
        let b = &BENCH_ARRAY_2;
        assert_eq!(
            euclidean_distance_no_simd(a, b).round_to(SIMD_PARITY_PRECISION),
            euclidean_distance_simd(a, b).round_to(SIMD_PARITY_PRECISION)
        );
    }

    #[test]
    #[cfg(feature = "simd")]
    fn test_dot_product_simd_parity() {
        let a = &BENCH_ARRAY_1;
        let b = &BENCH_ARRAY_2;
        assert_eq!(
            dot_product_no_simd(a, b).round_to(SIMD_PARITY_PRECISION),
            dot_product_simd(a, b).round_to(SIMD_PARITY_PRECISION)
        );
    }

    #[test]
    #[cfg(feature = "simd")]
    fn test_manhattan_distance_simd_parity() {
        let a = &BENCH_ARRAY_1;
        let b = &BENCH_ARRAY_2;
        assert_eq!(
            manhattan_distance_no_simd(a, b).round_to(SIMD_PARITY_PRECISION),
            manhattan_distance_simd(a, b).round_to(SIMD_PARITY_PRECISION)
        );
    }

    #[cfg(feature = "bench")]
    mod bench {
        use super::*;
        extern crate test;
        use test::Bencher;

        #[bench]
        fn bench_euclidean_distance_no_simd(bencher: &mut Bencher) {
            let a = &BENCH_ARRAY_1;
            let b = &BENCH_ARRAY_2;
            bencher.iter(|| euclidean_distance_no_simd(a, b));
        }

        #[bench]
        #[cfg(feature = "simd")]
        fn bench_euclidean_distance_simd(bencher: &mut Bencher) {
            let a = &BENCH_ARRAY_1;
            let b = &BENCH_ARRAY_2;
            bencher.iter(|| euclidean_distance_simd(a, b));
        }

        #[bench]
        fn bench_dot_product_no_simd(bencher: &mut Bencher) {
            let a = &BENCH_ARRAY_1;
            let b = &BENCH_ARRAY_2;
            bencher.iter(|| dot_product_no_simd(a, b));
        }

        #[bench]
        #[cfg(feature = "simd")]
        fn bench_dot_product_simd(bencher: &mut Bencher) {
            let a = &BENCH_ARRAY_1;
            let b = &BENCH_ARRAY_2;
            bencher.iter(|| dot_product_simd(a, b));
        }

        #[bench]
        fn bench_cosine_distance_no_simd(bencher: &mut Bencher) {
            let a = &BENCH_ARRAY_1;
            let b = &BENCH_ARRAY_2;
            bencher.iter(|| cosine_distance_no_simd(a, b));
        }

        #[bench]
        #[cfg(feature = "simd")]
        fn bench_cosine_distance_simd(bencher: &mut Bencher) {
            let a = &BENCH_ARRAY_1;
            let b = &BENCH_ARRAY_2;
            bencher.iter(|| cosine_distance_simd(a, b));
        }

        #[bench]
        fn bench_manhattan_distance_no_simd(bencher: &mut Bencher) {
            let a = &BENCH_ARRAY_1;
            let b = &BENCH_ARRAY_2;
            bencher.iter(|| manhattan_distance_no_simd(a, b));
        }

        #[bench]
        #[cfg(feature = "simd")]
        fn bench_manhattan_distance_simd(bencher: &mut Bencher) {
            let a = &BENCH_ARRAY_1;
            let b = &BENCH_ARRAY_2;
            bencher.iter(|| manhattan_distance_simd(a, b));
        }
    }
}
