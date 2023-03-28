use std::fs::File;
use std::io::Write;
use super::mincount;
use super::hashes;
use super::multiset;

fn _k_experiment(k: usize, hash: fn(usize, usize) -> f64, ns: &[usize], b: usize) -> Vec<(f64, f64)> {
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
    let results = _k_experiment(k, hash, ns, b);
    let sum = results.iter().map(|(_, diff)| (diff - 1.0).abs()).sum::<f64>();
    sum / (ns.len() as f64)
}

// task 5b
pub fn test_estimator(ns: &[usize]) {
    println!("Running experiment 5b...");

    let ks = [2, 3, 10, 100, 400];

    for k in ks {
        let filename = format!("results/5b_k{}.csv", k);
        let mut f = File::create(filename).unwrap();
        for &n in ns {
            let m = multiset::MultiSet::new(n);
            let n_est = mincount::mincount(m, hashes::hash_blake2, k, 6);
            let n = n as f64;
            writeln!(f, "{};{}", n, n_est as f64 / n).unwrap();
        }
        println!("Done running experiment for k = {}.", k)
    }

    println!("Done running experiment 5b.")
}

// task 5c
pub fn test_best_k(ns: &[usize], min_count: usize) {
    println!("Running experiment 5c...");

    let mut left = 2;
    let mut right = 400;

    while left < right {
        let mid = (left + right) / 2;
        println!("running experiment for k = {}... ", mid);
        let arr = _k_experiment(mid, hashes::hash_blake2, ns, 6);
        let good: usize = arr.iter().map(|&(_, x)| if x > 0.9 && x < 1.1 { 1 } else { 0 }).sum();
        if good >= min_count {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    let filename = format!("results/5c.txt");
    let mut f = File::create(filename).unwrap();
    println!("k = {}", left);
    writeln!(f, "{}", left).unwrap();
    println!("Done running experiment 5c.")
}

// task 6
pub fn test_hashes(ns: &[usize]) {
    println!("Running experiment 6...");

    let bytes_arr = [1, 2, 3, 4, 5, 6];
    let filename = format!("results/6.csv");
    let mut f = File::create(filename).unwrap();
    writeln!(f, "b;blake2;md4;sha1;sha2;sha3").unwrap();
    for byte in bytes_arr {
        println!("Running experiment for b = {} bits...", 8 * byte);
        let blake2_avg_diff = avg_dist(400, hashes::hash_blake2, ns, byte);
        println!("blake2_avg_diff = {}", blake2_avg_diff);
        let md4_avg_diff = avg_dist(400, hashes::hash_md4, ns, byte);
        println!("md4_avg_diff = {}", md4_avg_diff);
        let sha1_avg_diff = avg_dist(400, hashes::hash_sha1, ns, byte);
        println!("sha1_avg_diff = {}", sha1_avg_diff);
        let sha2_avg_diff = avg_dist(400, hashes::hash_sha2, ns, byte);
        println!("sha2_avg_diff = {}", sha2_avg_diff);
        let sha3_avg_diff = avg_dist(400, hashes::hash_sha3, ns, byte);
        println!("sha3_avg_diff = {}", sha3_avg_diff);
        writeln!(f, "{};{};{};{};{};{}", byte*8, blake2_avg_diff,
                                            md4_avg_diff, sha1_avg_diff,
                                            sha2_avg_diff, sha3_avg_diff).unwrap();
    }

    println!("Done running experiment 6.")
}

// task 6 BAD hash
pub fn test_bad_hash(ns: &[usize]) {
    println!("Running experiment 6 with bad hashes...");

    let bytes_arr = [1, 2, 3, 4, 5, 6];
    let filename = format!("results/bad6.csv");
    let mut f = File::create(filename).unwrap();
    writeln!(f, "b;mod256").unwrap();
    for byte in bytes_arr {
        println!("Running experiment for b = {} bits...", 8 * byte);
        let bad_avg_diff = avg_dist(400, hashes::hash_bad_modulo, ns, byte);
        println!("bad_avg_diff = {}", bad_avg_diff);
        writeln!(f, "{};{}", byte*8, bad_avg_diff).unwrap();
    }

    println!("Done running experiment 6 with bad hashes.")
}
