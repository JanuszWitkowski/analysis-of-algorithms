mod myhash;
mod multiset;
mod hyperloglog;
mod tests;

fn main() {
    // println!("Hello, world!");
    // println!("{:?}", myhash::hash_blake2(123, 4*8));
    // println!("{:?}", hyperloglog::split_hash(36, 6, 2));
    // println!("{:?}", hyperloglog::rho(36, 6));
    // println!("{:?}", hyperloglog::rho(36, 7));
    // println!("{:?}", hyperloglog::rho(0, 6));
    // println!("{:?}", hyperloglog::alpha_m(16));
    // println!("{:?}", hyperloglog::alpha_m(32));
    // println!("{:?}", hyperloglog::alpha_m(64));
    // println!("{:?}", hyperloglog::alpha_m(128));
    let ns: [usize; 10_000] = (1..=10_000).collect::<Vec<usize>>().try_into().unwrap();
    let bits = 5;
    tests::test_estimator_for_hash(&ns, bits, myhash::hash_blake2, "blake2");
    tests::test_estimator_for_hash(&ns, bits, myhash::hash_md4, "md4");
    tests::test_estimator_for_hash(&ns, bits, myhash::hash_sha1, "sha1");
    tests::test_estimator_for_hash(&ns, bits, myhash::hash_sha2, "sha2");
    tests::test_estimator_for_hash(&ns, bits, myhash::hash_sha3, "sha3");

    let bits = 16;
    tests::test_estimator_for_hash(&ns, bits, myhash::hash_blake2, "blake2");
    tests::test_estimator_for_hash(&ns, bits, myhash::hash_md4, "md4");
    tests::test_estimator_for_hash(&ns, bits, myhash::hash_sha1, "sha1");
    tests::test_estimator_for_hash(&ns, bits, myhash::hash_sha2, "sha2");
    tests::test_estimator_for_hash(&ns, bits, myhash::hash_sha3, "sha3");

    tests::test_best_b(&ns, 9500);

    for b in 4..=16 {
        tests::test_estimator_for_hash(&ns, b, myhash::hash_blake2, "blake2");
    }
}
