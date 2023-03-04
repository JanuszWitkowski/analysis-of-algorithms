use std::fs::File;
use std::io::Write;
use std::thread;
mod leader;
mod logger;

const _FILEPATH: &str = "data/";
const _HISTPATH: &str = "hist/";
const _PROBPATH: &str = "prob/";

fn hist_experiment_known(n: u32, iter: usize) {
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

fn hist_experiment_unknown(u: u32, iter: usize) {
    let n1 = 2_u32;
    let n2 = u / 2;
    let n3 = u;
    let mut l;

    let filename = format!("data_unknown_u{}_2_n{}.txt", u, n1);
    let output1 = File::create(filename);
    let mut output1 = match output1 {
        Ok(file) => file,
        Err(error) => panic!("Problem creating file : {:?}", error),
    };
    for _ in 1..=iter {
        (_, _, _, _, l, _) = leader::election_unknown(u, n1, false, false);
        write!(output1, "{}\n", l).expect("Error while writing data.");
    }

    let filename = format!("data_unknown_u{}_half_n{}.txt", u, n2);
    let output2 = File::create(filename);
    let mut output2 = match output2 {
        Ok(file) => file,
        Err(error) => panic!("Problem creating file : {:?}", error),
    };
    for _ in 1..=iter {
        (_, _, _, _, l, _) = leader::election_unknown(u, n2, false, false);
        write!(output2, "{}\n", l).expect("Error while writing data.");
    }

    let filename = format!("data_unknown_u{}_u_n{}.txt", u, n3);
    let output3 = File::create(filename);
    let mut output3 = match output3 {
        Ok(file) => file,
        Err(error) => panic!("Problem creating file : {:?}", error),
    };
    for _ in 1..=iter {
        (_, _, _, _, l, _) = leader::election_unknown(u, n3, false, false);
        write!(output3, "{}\n", l).expect("Error while writing data.");
    }
}

fn ev_var (ns: Vec<u32>, iter: usize) {
    let mut results = vec![0_u32; iter];
    let mut sum: u32 = 0;
    let mut ev: f64;
    let mut var: f64;

    let output_ev = File::create("ev.txt");
    let mut output_ev = match output_ev {
        Ok(file) => file,
        Err(error) => panic!("Problem creating file : {:?}", error),
    };
    let output_var = File::create("var.txt");
    let mut output_var = match output_var {
        Ok(file) => file,
        Err(error) => panic!("Problem creating file : {:?}", error),
    };

    let mut ctr: usize = 0;
    let total_size = ns.len();

    for n in ns {
        for i in 0..iter {
            (_, results[i], _) = leader::election(n, 1.0 / (n as f64), false);
        }
        sum = results.iter().sum();
        ev = (sum as f64) / (iter as f64);
        write!(output_ev, "{} {}\n", n, ev).expect("Error while writing data.");
        var = results.iter().map(|&x| ((x as f64) - ev).powf(2.0_f64)).sum();
        var = var / (iter as f64);
        write!(output_var, "{} {}\n", n, var).expect("Error while writing data.");
        ctr += 1;
        println!("EV/Var Progress: {}/{}", ctr, total_size);
    }
}

fn lambda (us: Vec<u32>, iter: usize) {
    let us_size = us.len();
    let mut results: Vec<[u32; 7]> = vec![[0_u32; 7]; us_size];    // 2, 10%, 25%, 50%, 75%, 90%, 100%
    let mut leader: u32;
    let mut l: u32 = 0;
    let mut u;
    let check_election = |leader: u32| -> u32 {
        if leader == 0 {
            0
        } else {
            1
        }
    };

    let output = File::create("lambda.txt");
    let mut output = match output {
        Ok(file) => file,
        Err(error) => panic!("Problem creating file : {:?}", error),
    };
    write!(output, "(u; l; n = 2, 10%, 25%, 50%, 75%, 90%, 100%)\n").expect("Fail 1.");

    for j in 0..us_size {
        u = us[j];
        for i in 0..iter {
            // n == 2
            (leader, l, _, _, _, _) = leader::election_unknown(u, 2, true, false);
            results[j][0] += check_election (leader);
            // n == 10% u
            (leader, _, _, _, _, _) = leader::election_unknown(u, u/10, true, false);
            results[j][1] += check_election (leader);
            // n == 25% u
            (leader, _, _, _, _, _) = leader::election_unknown(u, (u * 25)/100, true, false);
            results[j][2] += check_election (leader);
            // n == 50% u
            (leader, _, _, _, _, _) = leader::election_unknown(u, u/2, true, false);
            results[j][3] += check_election (leader);
            // n == 75% u
            (leader, _, _, _, _, _) = leader::election_unknown(u, (u * 75)/100, true, false);
            results[j][4] += check_election (leader);
            // n == 90% u
            (leader, _, _, _, _, _) = leader::election_unknown(u, (u * 9)/10, true, false);
            results[j][5] += check_election (leader);
            // n == u
            (leader, _, _, _, _, _) = leader::election_unknown(u, u, true, false);
            results[j][6] += check_election (leader);
        }
        write!(output, "{}; {};", u, l).expect("Fail 2.");
        for i in 0..results[0].len() {
            write!(output, " {}", (results[j][i] as f64)/(iter as f64)).expect("Fail 3.");
        }
        write!(output, "\n").expect("Fail 4.");

        println!("Lambda Progress: {}/{}", j+1, us_size);
    }
}

fn main() {
    // let n: u32 = 100;
    // let u: u32 = 512;
    // const ITER: usize = 1000000;
    const ITER: usize = 1000;
    // hist_experiment_known(n, iter);
    // hist_experiment_unknown(u, iter);
    // let ns: Vec<u32> = vec![10, 20, 30, 40, 50, 60, 70, 80, 90,
    //                         100, 200, 300, 400, 500, 600, 700, 800, 900,
    //                         1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000,
    //                         10000];
    let xs1: Vec<u32> = vec![16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384];
    let xs2: Vec<u32> = vec![16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384];
    // ev_var(xs1, ITER);
    // lambda(xs2, ITER);
    let thread_ev_var = thread::spawn(|| {ev_var(xs1, ITER)});
    let thread_lambda = thread::spawn(|| {lambda(xs2, ITER)});
    thread_ev_var.join().unwrap();
    thread_lambda.join().unwrap();
}
