use annoy_rs::*;
use std::env;
use std::time;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let dim = args[1].parse().unwrap();
    let size = args[2].parse::<u64>().unwrap();
    let n_result = args[3].parse().unwrap();
    let n_loop = args[4].parse().unwrap();

    let metrics = vec![IndexType::Angular, IndexType::Euclidean];
    for &metric in metrics.iter() {
        let path = format!("../index.{}.{}d.ann", metric, dim);
        let index = AnnoyIndex::load(dim, path.as_str(), metric).unwrap();
        let t_start = time::Instant::now();
        for i in 0..n_loop {
            let id = i % size;
            let v = index.get_item_vector(id);
            index.get_nearest(v.as_slice(), n_result, -1, true);
        }
        let t_end = time::Instant::now();
        let diff = t_end - t_start;
        println!("[Rust] annoy-rs");
        println!("[{}] Total time elapsed: {}s", metric, diff.as_secs_f32());
        println!(
            "[{}] Avg time elapsed: {}ms",
            metric,
            diff.as_secs_f32() * 1000.0 / n_loop as f32
        );
        println!()
    }
}
