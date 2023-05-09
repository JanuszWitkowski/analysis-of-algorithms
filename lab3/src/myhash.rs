use digest::Digest;
use blake2::Blake2s256; // 256 bits
use md4::Md4; // 128 bits
use sha1::Sha1; // 160 bits 
use sha2::Sha256; // 256 bits
use sha3::Sha3_256; // 256 bits 

fn shorten_hash(bytes: &[u8], num_of_bits: usize) -> usize {
    // let mut sum = 0;
    // let modulant = 2_u64.pow(num_of_bits as u64);
    // for &byte in bytes.iter() {
    //     sum = (sum + (byte as usize)) % modulant;
    // }
    // // println!("{:?}", sum);
    // sum

    // WEŹ KILKA PIERWSZYCH BAJTÓW
    // let mut sum = 0;
    // for &byte in bytes.iter().take(num_of_bits/8) {
    //     // sum *= 256;
    //     sum <<= 8;
    //     sum += byte as usize;
    // }
    // println!("HASH: {:b}", sum);
    // sum

    // SUMA MODULOWA
    // let mut sum: u64 = 0;
    // let modulant = 2_u64.pow(32);
    // let mut counter = 0;
    // for &byte in bytes.iter() {
    //     counter += 1;
    //     sum = (sum + (byte as u64)) % modulant;
    //     // println!("byte: {}; sum: {}", byte, sum);
    // }
    // // println!("({})HASH: {:b}", counter, sum);
    // sum as usize

    // JUST MODULO
    let mut sum: u128 = 0;
    // let modulant = 2_u128.pow(32);
    let modulant = 2_u128.pow((num_of_bits as u64).try_into().unwrap());
    for &byte in bytes.iter() {
        sum = (sum << 8) + (byte as u128);
        // println!("byte: {}; sum: {}", byte, sum);
    }
    let hash = (sum % modulant) as usize;
    // println!("HASH: {:b}", hash);
    (sum % modulant) as usize

    // let mut sum = 0;
    // let modulant = 2_usize.pow(num_of_bits as u32);
    // for &byte in bytes.iter().rev() {
    //     sum = (sum * 256 + (byte as usize)) % modulant;
    // }
    // // println!("{:?}", sum);
    // sum
}

pub fn hash_blake2(n: usize, num_of_bits: usize) -> usize {
    let mut hasher = Blake2s256::new();
    hasher.update(n.to_be_bytes());
    let res = hasher.finalize();
    let slice = res.as_slice();
    shorten_hash(slice, num_of_bits)
}

pub fn hash_md4(n: usize, num_of_bits: usize) -> usize {
    let mut hasher = Md4::new();
    hasher.update(n.to_be_bytes());
    let res = hasher.finalize();
    let slice = res.as_slice();
    shorten_hash(slice, num_of_bits)
}

pub fn hash_sha1(n: usize, num_of_bits: usize) -> usize {
    let mut hasher = Sha1::new();
    hasher.update(n.to_be_bytes());
    let res = hasher.finalize();
    let slice = res.as_slice();
    shorten_hash(slice, num_of_bits)
}

pub fn hash_sha2(n: usize, num_of_bits: usize) -> usize {
    let mut hasher = Sha256::new();
    hasher.update(n.to_be_bytes());
    let res = hasher.finalize();
    let slice = res.as_slice();
    shorten_hash(slice, num_of_bits)
}

pub fn hash_sha3(n: usize, num_of_bits: usize) -> usize {
    let mut hasher = Sha3_256::new();
    hasher.update(n.to_be_bytes());
    let res = hasher.finalize();
    let slice = res.as_slice();
    shorten_hash(slice, num_of_bits)
}

// pub fn hash_bad_modulo(n: usize, num_of_bits: usize) -> f64 {
//     (n % 256) as f64
// }
