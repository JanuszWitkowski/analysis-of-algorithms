mod myhash;
mod multiset;
mod hyperloglog;

fn main() {
    println!("Hello, world!");
    println!("{:?}", myhash::hash_blake2(123, 4));
}
