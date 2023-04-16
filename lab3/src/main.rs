mod myhash;
mod multiset;
mod hyperloglog;

fn main() {
    println!("Hello, world!");
    println!("{:?}", myhash::hash_blake2(123, 4*8));
    println!("{:?}", hyperloglog::split_hash(36, 6, 2));
    println!("{:?}", hyperloglog::rho(36, 6));
    println!("{:?}", hyperloglog::rho(36, 7));
    println!("{:?}", hyperloglog::rho(0, 6));
    println!("{:?}", hyperloglog::alpha_m(16));
    println!("{:?}", hyperloglog::alpha_m(32));
    println!("{:?}", hyperloglog::alpha_m(64));
    println!("{:?}", hyperloglog::alpha_m(128));
}
