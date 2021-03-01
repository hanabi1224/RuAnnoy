use crate::internals::mmap_ext::*;
use memmap2::Mmap;

pub const INT32_SIZE: usize = 4;
pub const FLOAT32_SIZE: usize = 4;

// pub fn is_zero_vec(v: &Vec<f32>) -> bool {
//     for item in v {
//         if *item != 0.0 {
//             return false;
//         }
//     }
//     return true;
// }

pub fn minkowski_margin(u: &[f32], v: &[f32], bias: f32) -> f32 {
    return bias + dot_product(u, v);
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

pub fn dot_product(u: &[f32], v: &[f32]) -> f32 {
    let mut d: f32 = 0.0;
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
    let mut sum: f32 = 0.0;
    for i in 0..u.len() {
        sum += (u[i] - v[i]).powi(2);
    }
    return sum;
}

pub fn manhattan_distance(u: &[f32], v: &[f32]) -> f32 {
    let mut sum: f32 = 0.0;
    for i in 0..u.len() {
        sum += (u[i] - v[i]).abs();
    }
    return sum;
}

pub fn get_nth_descendant_id(
    mmap: &Mmap,
    node_offset: i64,
    index_type_offset: i32,
    n: usize,
) -> i64 {
    let child_offset = node_offset as usize + index_type_offset as usize + n * INT32_SIZE;
    let child_id = mmap.read_i32(child_offset) as i64;
    return child_id;
}

#[cfg(test)]
mod tests {
    use crate::types::utils::*;

    #[test]
    fn test_cosine_distance() {
        let r = cosine_distance(
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
    fn test_euclidean_distance() {
        let r = euclidean_distance(
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
    fn test_dot_distance() {
        let r = -dot_product(
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
        assert_eq!(-r, 3.5382423400878906);
    }
}
