mod leader;
mod logger;

fn main() {
    let n: u32 = 128;
    let u: u32 = 256;
    let p: f64 = 1.0 / (n as f64);
    // let (_, _, logger) = leader::election(n, p, true);
    let (_, _, _, _, logger) = leader::election_unknown(u, n, true, true);
    logger::print(logger);
    // println!("Hello world!");
}
