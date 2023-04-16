use digest::Digest;
use blake2::Blake2s256; // 256 bits
use md4::Md4; // 128 bits
use sha1::Sha1; // 160 bits 
use sha2::Sha256; // 256 bits
use sha3::Sha3_256; // 256 bits 

fn bytes_to_f64(bytes: &[u8], num_of_bytes: usize) -> f64 {
    // let mut sum = 0.0;
    // for &byte in bytes.iter().take(num_of_bytes) {
    //     sum += byte as f64;
    //     sum /= 256.0;
    // }
    // sum
    let mut sum = 0;
    for &byte in bytes.iter() {
        sum = (sum + (byte as usize)) % (num_of_bytes * 8);
    }
    sum as f64
}

pub fn hash_blake2(n: usize, num_of_bytes: usize) -> f64 {
    let mut hasher = Blake2s256::new();
    hasher.update(n.to_be_bytes());
    let res = hasher.finalize();
    let slice = res.as_slice();
    bytes_to_f64(slice, num_of_bytes)
}

pub fn hash_md4(n: usize, num_of_bytes: usize) -> f64 {
    let mut hasher = Md4::new();
    hasher.update(n.to_be_bytes());
    let res = hasher.finalize();
    let slice = res.as_slice();
    bytes_to_f64(slice, num_of_bytes)
}

pub fn hash_sha1(n: usize, num_of_bytes: usize) -> f64 {
    let mut hasher = Sha1::new();
    hasher.update(n.to_be_bytes());
    let res = hasher.finalize();
    let slice = res.as_slice();
    bytes_to_f64(slice, num_of_bytes)
}

pub fn hash_sha2(n: usize, num_of_bytes: usize) -> f64 {
    let mut hasher = Sha256::new();
    hasher.update(n.to_be_bytes());
    let res = hasher.finalize();
    let slice = res.as_slice();
    bytes_to_f64(slice, num_of_bytes)
}

pub fn hash_sha3(n: usize, num_of_bytes: usize) -> f64 {
    let mut hasher = Sha3_256::new();
    hasher.update(n.to_be_bytes());
    let res = hasher.finalize();
    let slice = res.as_slice();
    bytes_to_f64(slice, num_of_bytes)
}

pub fn hash_bad_modulo(n: usize, num_of_bytes: usize) -> f64 {
    (n % 256) as f64
}
