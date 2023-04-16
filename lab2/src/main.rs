mod mincount;
mod multiset;
mod myhash;
mod tests;

fn main() {
    let ns: [usize; 10_000] = (1..=10_000).collect::<Vec<usize>>().try_into().unwrap();
    // tests::test_estimator(&ns);
    // tests::test_best_k(&ns, 9500); // Result = 331
    // tests::test_myhash(&ns);
    // tests::test_bad_hash(&ns);

    // Compare with HyperLogLog with the "same" amount of memory.
    let k27 = 27;
    let k5 = 5;

    tests::test_estimator_for_hash(&ns, k27, myhash::hash_blake2, "blake2");
    tests::test_estimator_for_hash(&ns, k27, myhash::hash_md4, "md4");
    tests::test_estimator_for_hash(&ns, k27, myhash::hash_sha1, "sha1");
    tests::test_estimator_for_hash(&ns, k27, myhash::hash_sha2, "sha2");
    tests::test_estimator_for_hash(&ns, k27, myhash::hash_sha3, "sha3");

    tests::test_estimator_for_hash(&ns, k5, myhash::hash_blake2, "blake2");
    tests::test_estimator_for_hash(&ns, k5, myhash::hash_md4, "md4");
    tests::test_estimator_for_hash(&ns, k5, myhash::hash_sha1, "sha1");
    tests::test_estimator_for_hash(&ns, k5, myhash::hash_sha2, "sha2");
    tests::test_estimator_for_hash(&ns, k5, myhash::hash_sha3, "sha3");
}
