use std::fs::File;
use std::io::Write;
mod leader;
mod logger;

const FILEPATH: &str = "data/";
const HISTPATH: &str = "hist/";
const PROBPATH: &str = "prob/";

fn hist_experiment_known(n: u32, iter: u32) {
    // let filename = format!("{}{}hist_known_{}.txt", FILEPATH, HISTPATH, n);
    let filename = format!("data_known_{}.txt", n);
    let mut l;
    let output = File::create(filename);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => panic!("Problem creating file : {:?}", error),
    };
    for _ in 1..=iter {
        (_, l, _) = leader::election(n, 1.0 / (n as f64), false);
        write!(output, "{}\n", l).expect("Error while writing data.");
    }
}

fn hist_experiment_unknown(u: u32, iter: u32) {
    let n1 = 2_u32;
    let n2 = u / 2;
    let n3 = u;
    let mut l;
    // let filename = format!("{}{}hist_unknown_{}.txt", FILEPATH, HISTPATH, n1);

    let filename = format!("data_unknown_u{}_2_n{}.txt", u, n1);
    let output1 = File::create(filename);
    let mut output1 = match output1 {
        Ok(file) => file,
        Err(error) => panic!("Problem creating file : {:?}", error),
    };
    for _ in 1..=iter {
        (_, _, _, l, _) = leader::election_unknown(u, n1, false, false);
        write!(output1, "{}\n", l).expect("Error while writing data.");
    }

    let filename = format!("data_unknown_u{}_half_n{}.txt", u, n2);
    let output2 = File::create(filename);
    let mut output2 = match output2 {
        Ok(file) => file,
        Err(error) => panic!("Problem creating file : {:?}", error),
    };
    for _ in 1..=iter {
        (_, _, _, l, _) = leader::election_unknown(u, n2, false, false);
        write!(output2, "{}\n", l).expect("Error while writing data.");
    }

    let filename = format!("data_unknown_u{}_u_n{}.txt", u, n3);
    let output3 = File::create(filename);
    let mut output3 = match output3 {
        Ok(file) => file,
        Err(error) => panic!("Problem creating file : {:?}", error),
    };
    for _ in 1..=iter {
        (_, _, _, l, _) = leader::election_unknown(u, n3, false, false);
        write!(output3, "{}\n", l).expect("Error while writing data.");
    }
}

fn main() {
    let n: u32 = 100;
    let u: u32 = 512;
    let p: f64 = 1.0 / (n as f64);
    // let (_, _, logger) = leader::election(n, p, true);
    // let (_, _, _, _, logger) = leader::election_unknown(u, n, true, true);
    // logger::print(logger);
    let iter = 10000;
    hist_experiment_known(n, iter);
    hist_experiment_unknown(u, iter);
}
