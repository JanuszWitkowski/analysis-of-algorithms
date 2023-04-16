use std::fs::File;
use std::io::Write;
use super::hyperloglog;
use super::myhash;
use super::multiset;

// fn run_for_ns(hash: fn(usize, usize) -> usize, ns: &[usize], b: usize) -> Vec<(f64, f64)> {
//     ns 
//     .iter()
//     .map(|&n| {
//         let m = multiset::MultiSet::new(n);
//         let n_est = hyperloglog::hyperloglog(m, hash, b);
//         let n = n as f64;
//         (n, n_est as f64 / n)
//     })
//     .collect::<Vec<(f64, f64)>>()
// }

pub fn test_estimator_for_hash(ns: &[usize], hash: fn(usize, usize) -> usize, hashname: &'static str) {
    let filename = format!("results/hyperloglog_{}.csv", hashname);
    let bits = 5;
    let mut f = File::create(filename).unwrap();
    for &n in ns {
        let m = multiset::MultiSet::new(n);
        let n_est = hyperloglog::hyperloglog(m, hash, bits);
        let n = n as f64;
        writeln!(f, "{};{};{}", n, n_est, n_est / n).unwrap();
    }
}
