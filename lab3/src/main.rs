mod myhash;
mod multiset;
mod hyperloglog;
mod tests;

fn main() {
    // let ns: [usize; 10_000] = (1..=10_000).collect::<Vec<usize>>().try_into().unwrap();
    let ns: [usize; 11] = (0..=10).map(|x| 1000*x + 1).collect::<Vec<usize>>().try_into().unwrap();
    let bits = 5;
    // tests::test_estimator_for_hash(&ns, 4, myhash::hash_blake2, "blake2");
    tests::test_estimator_for_hash(&ns, 4, myhash::hash_sha2, "sha256");

    // tests::test_estimator_for_hash(&ns, bits, myhash::hash_blake2, "blake2");
    // tests::test_estimator_for_hash(&ns, bits, myhash::hash_md4, "md4");
    // tests::test_estimator_for_hash(&ns, bits, myhash::hash_sha1, "sha1");
    // tests::test_estimator_for_hash(&ns, bits, myhash::hash_sha2, "sha2");
    // tests::test_estimator_for_hash(&ns, bits, myhash::hash_sha3, "sha3");

    // let bits = 16;
    // tests::test_estimator_for_hash(&ns, bits, myhash::hash_blake2, "blake2");
    // tests::test_estimator_for_hash(&ns, bits, myhash::hash_md4, "md4");
    // tests::test_estimator_for_hash(&ns, bits, myhash::hash_sha1, "sha1");
    // tests::test_estimator_for_hash(&ns, bits, myhash::hash_sha2, "sha2");
    // tests::test_estimator_for_hash(&ns, bits, myhash::hash_sha3, "sha3");

    // tests::test_best_b(&ns, 9500);

    // for b in 4..=16 {
    //     tests::test_estimator_for_hash(&ns, b, myhash::hash_blake2, "blake2");
    // }
}
