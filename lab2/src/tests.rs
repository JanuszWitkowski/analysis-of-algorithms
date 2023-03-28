use std::fs::File;
use std::io::Write;
use super::mincount;
use super::myhash;
use super::multiset;

fn check_k(k: usize, hash: fn(usize, usize) -> f64, ns: &[usize], b: usize) -> Vec<(f64, f64)> {
    ns 
    .iter()
    .map(|&n| {
        let m = multiset::MultiSet::new(n);
        let n_est = mincount::mincount(m, hash, k, b);
        let n = n as f64;
        (n, n_est as f64 / n)
    })
    .collect::<Vec<(f64, f64)>>()
}

fn avg_dist(k: usize, hash: fn(usize, usize) -> f64, ns: &[usize], b: usize) -> f64 {
    let results = check_k(k, hash, ns, b);
    let sum = results.iter().map(|(_, diff)| (diff - 1.0).abs()).sum::<f64>();
    sum / (ns.len() as f64)
}

// task 5b
pub fn test_estimator(ns: &[usize]) {
    let ks = [2, 3, 10, 100, 400];
    for k in ks {
        let filename = format!("results/5b_k{}.csv", k);
        let mut f = File::create(filename).unwrap();
        for &n in ns {
            let m = multiset::MultiSet::new(n);
            let n_est = mincount::mincount(m, myhash::hash_blake2, k, 6);
            let n = n as f64;
            writeln!(f, "{};{}", n, n_est as f64 / n).unwrap();
        }
    }
}

// task 5c
pub fn test_best_k(ns: &[usize], min_count: usize) {
    let mut left = 2;
    let mut right = 400;
    while left < right {
        let mid = (left + right) / 2;
        println!("running experiment for k = {}... ", mid);
        let arr = check_k(mid, myhash::hash_blake2, ns, 6);
        let good: usize = arr.iter().map(|&(_, x)| if x > 0.9 && x < 1.1 { 1 } else { 0 }).sum();
        if good >= min_count {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    let mut f = File::create(filename).unwrap();
    println!("k = {}", left);
    writeln!(f, "{}", left).unwrap();
}

// task 6
pub fn test_myhash(ns: &[usize]) {
    let bytes_arr = [1, 2, 3, 4, 5, 6];
    let filename = format!("results/6.csv");
    let mut f = File::create(filename).unwrap();
    writeln!(f, "b;blake2;md4;sha1;sha2;sha3").unwrap();
    for byte in bytes_arr {
        let blake2_avg_diff = avg_dist(400, myhash::hash_blake2, ns, byte);
        let md4_avg_diff = avg_dist(400, myhash::hash_md4, ns, byte);
        let sha1_avg_diff = avg_dist(400, myhash::hash_sha1, ns, byte);
        let sha2_avg_diff = avg_dist(400, myhash::hash_sha2, ns, byte);
        let sha3_avg_diff = avg_dist(400, myhash::hash_sha3, ns, byte);
        writeln!(f, "{};{};{};{};{};{}", byte*8, blake2_avg_diff,
                                            md4_avg_diff, sha1_avg_diff,
                                            sha2_avg_diff, sha3_avg_diff).unwrap();
    }
}

// task 6 BAD hash
pub fn test_bad_hash(ns: &[usize]) {
    let bytes_arr = [1, 2, 3, 4, 5, 6];
    let filename = format!("results/bad6.csv");
    let mut f = File::create(filename).unwrap();
    writeln!(f, "b;mod256").unwrap();
    for byte in bytes_arr {
        let bad_avg_diff = avg_dist(400, myhash::hash_bad_modulo, ns, byte);
        writeln!(f, "{};{}", byte*8, bad_avg_diff).unwrap();
    }
}
