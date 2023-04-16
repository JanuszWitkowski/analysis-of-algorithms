use crate::multiset::MultiSet;

const N_OF_HASH_OUTPUT_BITS: usize = 32;
const N_OF_HASH_OUTPUT_BYTES: usize = N_OF_HASH_OUTPUT_BITS / 8;

fn split_hash(hash_value: usize, hash_length: usize, n_of_bits: usize) -> (usize, usize) {
    let tail_length = hash_length - n_of_bits;
    let left = hash_value >> tail_length;
    let right = hash_value % 2_usize.pow(tail_length as u32);
    (left, right)
}

fn rho(n: usize, length: usize) -> usize {
    let mut i = 1;
    while i <= length && (n >> (length - i)) & 1 == 1 {
        i += 1;
    }
    i
}

fn alpha_m(m: usize) -> f64 {
    match m {
        16 => 0.673,
        32 => 0.697,
        64 => 0.709,
        m_ => 0.7213/(1.0 + (1.079/(m_ as f64))),
    }
}

pub fn hyperloglog(multiset: MultiSet, h: fn(usize, usize) -> usize, bits: usize) -> f64 {
    let m: usize = 2_usize.pow(bits as u32);
    let mut m_arr = vec![0; m];
    for x in multiset {
        let hx = h(x, N_OF_HASH_OUTPUT_BYTES);
        let (j, w) = split_hash(hx, N_OF_HASH_OUTPUT_BITS, bits);
        // let j = 1 + jminus1;
        let r = rho(w, N_OF_HASH_OUTPUT_BITS);
        if m_arr[j] < r {
            m_arr[j] = r;
        }
    }
    let n_est: f64 = alpha_m(m) * (m as f64).powf(2.0) * (m_arr.iter().map(|x| 2.0_f64.powf(-(*x as f64))).sum::<f64>()).powf(-1.0);
    if n_est < 2.5 * (m as f64) {
        let v = m_arr.iter().filter(|&x| *x == 0).count();
        if v != 0 {
            return (m as f64) * ((m as f64) / (v as f64)).log2();
        }
    } else if 30.0 * n_est > 2.0_f64.powf(32.0) {
        let huge: f64 = 2.0_f64.powf(32.0);
        return (-huge) * (1.0 - (n_est / huge)).log2();
    }
    return n_est;
}

