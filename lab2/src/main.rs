mod mincount;
mod multiset;
mod hashes;
mod tests;

fn main() {
    let ns: [usize; 10_000] = (1..=10_000).collect::<Vec<usize>>().try_into().unwrap();
    // note: https://www.wolframalpha.com/input?i=%281-10000%2F2%5E%2848%29%29%5E10000
    tests::test_estimator(&ns);
    tests::test_best_k(&ns, 9500); // Result = ???
    tests::test_hashes(&ns);

    // for i in 1..5 {
    //     let m = multiset::MultiSet::new_with_multiplier(i, 1);
    //     println!("Start with {}", i);
    //     for x in m {
    //         print!("{} ", x);
    //     }
    //     println!("\nDone with {}", i);
    // }
}
