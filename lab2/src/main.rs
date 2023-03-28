mod mincount;
mod multiset;
mod myhash;
mod tests;

fn main() {
    let ns: [usize; 10_000] = (1..=10_000).collect::<Vec<usize>>().try_into().unwrap();
    // tests::test_estimator(&ns);
    // tests::test_best_k(&ns, 9500); // Result = 331
    // tests::test_myhash(&ns);
    tests::test_bad_hash(&ns);
}
