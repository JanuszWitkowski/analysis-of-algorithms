use std::fs::File;
use std::io::Write;
use super::hyperloglog;
use super::myhash;
use super::multiset;

fn run_for_ns(hash: fn(usize, usize) -> usize, ns: &[usize], b: usize) -> Vec<(f64, f64)> {
    ns 
    .iter()
    .map(|&n| {
        let m = multiset::MultiSet::new(n);
        let n_est = hyperloglog::hyperloglog(m, hash, b);
        let n = n as f64;
        (n, n_est as f64)
    })
    .collect::<Vec<(f64, f64)>>()
}

pub fn test_estimator_for_hash(ns: &[usize], bits: usize, hash: fn(usize, usize) -> usize, hashname: &'static str) {
    let filename = format!("results/hyperloglog_{}_b{}.csv", hashname, bits);
    let mut f = File::create(filename).unwrap();
    for &n in ns {
        let m = multiset::MultiSet::new(n);
        let n_est = hyperloglog::hyperloglog(m, hash, bits);
        let n = n as f64;
        writeln!(f, "{};{};{}", n, n_est, n_est / n).unwrap();
    }
    println!("Done for {} b={}.", hashname, bits);
}

pub fn test_best_b(ns: &[usize], min_count: usize) {
    let mut left = 4;
    let mut right = 16;
    while left < right {
        let mid = (left + right) / 2;
        println!("Running experiment for b={}... ", mid);
        let arr = run_for_ns(myhash::hash_blake2, ns, mid);
        let good: usize = arr.iter().map(|&(_, x)| if x > 0.9 && x < 1.1 { 1 } else { 0 }).sum();
        if good >= min_count {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    let filename = format!("results/best_b.txt");
    let mut f = File::create(filename).unwrap();
    println!("b = {}", left);
    writeln!(f, "{}", left).unwrap();
}
