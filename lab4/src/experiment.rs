use std::fs::{File, create_dir_all};
use std::io::Write;
use crate::theory;

const NAME_RESULTS: &str = "results/";
const NAME_FORMAT:  &str = ".csv";
const NAME_QS:      &str = "qs";
const NAME_THEORY:  &str = "theory_";
const NAME_N:       &str = "n";
const NAME_P:       &str = "p";

const MSG_DONE:     &str = "Done!";
const MSG_FIXED_N:  &str = "Done with fixed n = ";
const MSG_FIXED_P:  &str = "Done with fixed P(q, n) = ";

const N_FIXED_P_LEFT:   usize = 1;
const N_FIXED_P_RIGHT:  usize = 220;


pub fn theory_fixed_n() {
    println!("Starting Theory Fixed n...");
    create_dir_all(NAME_RESULTS).unwrap();
    let ns = [1, 3, 6, 12, 24, 48];
    let qs = calculate_qs();
    let mut filename = format!("{}{}{}", NAME_RESULTS, NAME_QS, NAME_FORMAT);
    let mut output = File::create(filename).unwrap();
    for q in &qs {
        writeln!(output, "{}", q).unwrap();
    }
    // let (mut filename, mut output);
    let (mut result_nakamuto, mut result_grunspan);
    for n in ns {
        filename = format!("{}{}{}{}{}", NAME_RESULTS, NAME_THEORY, NAME_N, n, NAME_FORMAT);
        output = File::create(filename).unwrap();
        for q in &qs {
            result_nakamuto = theory::nakamuto(*q, n);
            result_grunspan = theory::grunspan(*q, n);
            writeln!(output, "{};{};{}", q, result_nakamuto, result_grunspan).unwrap();
        }
        println!("{}{}", MSG_FIXED_N, n);
    }
    println!("{}", MSG_DONE);
}


pub fn theory_fixed_p() {
    println!("Starting Theory Fixed P(q, n)...");
    create_dir_all(NAME_RESULTS).unwrap();
    let ps = [0.001_f64, 0.01, 0.1];
    let qs = calculate_qs();
    // let eps = f64::EPSILON;
    let (mut filename, mut output);
    let (mut n_nakamuto, mut n_grunspan);
    // let (n_left, n_right) = (1, 100);
    for p in ps {
        filename = format!("{}{}{}{}{}", NAME_RESULTS, NAME_THEORY, NAME_P, p, NAME_FORMAT);
        output = File::create(filename).unwrap();
        for q in &qs {
            (n_nakamuto, n_grunspan) = binary_searches(p, *q, N_FIXED_P_LEFT, N_FIXED_P_RIGHT);
            writeln!(output, "{};{};{}", q, n_nakamuto, n_grunspan).unwrap();
        }
        println!("{}{}", MSG_FIXED_P, p);
    }
    println!("{}", MSG_DONE);
}


// public static void Ex9b()
// {
//    const int numOfTrials = 10_000;
//    var qs = Enumerable.Range(1, 25).Select(i => 0.02 * i).ToList();
//    var ns = new[] {1, 3, 6, 12, 24, 48};
//    var res = ns.Select(n => qs.Select(q => Enumerable.Range(0, numOfTrials)
//                           .Select(_ =>
//                           {
//                              return SingleExperiment(q, n);
//                           })
//                           .Select(x => x ? 1.0 : 0.0)
//                           .Sum() / (double)numOfTrials)
//                       .ToList())
//                   .ToList();
   
   
//    foreach (var t in ns)
//    {
//       File.WriteAllText(path + "\\Ex9b" + t, string.Join("\n", res));
//    }
// }

// public static bool SingleExperiment(double q, int n)
// {
//    Random r = new Random();
//    var threshold = 30;
//    var legitimateBranch = 0;
//    var adversaryBranch = 0;

//    while (true)
//    {
//       if (r.NextDouble() <= q)
//       {
//          adversaryBranch++;
//       }
//       else
//       {
//          legitimateBranch++;
//       }

//       if (legitimateBranch < n)
//       {
//          continue;
//       }

//       if (adversaryBranch >= legitimateBranch)
//       {
//          return true;
//       }
      
//       if (legitimateBranch >= n + threshold)
//       {
//          return false;
//       }
//    }
// }


fn calculate_qs() -> Vec<f64> {
    (1..32).map(|x| (x as f64)/64.0).collect::<Vec<f64>>()
}

fn binary_searches(p: f64, q: f64, mut left: usize, mut right: usize) -> (usize, usize) {
    let mut mid = 0;
    let mut result;
    // Nakamuto
    while left < right {
        mid = (left + right) / 2;
        result = theory::nakamuto(q, mid);
        println!("[q={}; mid={}] (p={}; result={})", q, mid, p, result);
        if result <= p {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    let n_nakamuto = mid;
    // Grunspan
    while left < right {
        mid = (left + right) / 2;
        result = theory::grunspan(q, mid);
        if result >= p {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    let n_grunspan = mid;
    (n_nakamuto, n_grunspan)
}

