use std::fs::{File, create_dir_all};
use std::io::Write;
use rand::Rng; // 0.8.5
use crate::theory;

const NAME_RESULTS: &str = "results/";
const NAME_FORMAT:  &str = ".csv";
const NAME_QS:      &str = "qs";
const NAME_THEORY:  &str = "theory_";
const NAME_N:       &str = "n";
const NAME_P:       &str = "p";
const NAME_SIM:     &str = "sim_";

const MSG_DONE:     &str = "Done!";
const MSG_FIXED_N:  &str = "Done with fixed n = ";
const MSG_FIXED_P:  &str = "Done with fixed P(q, n) = ";
const MSG_SIM:      &str = "Done with double-spending simulation for n = ";

const N_FIXED_P_LEFT:           usize = 1;
const N_FIXED_P_RIGHT:          usize = 220;
const NUMBER_OF_SIMULATIONS:    usize = 100_000;
const THRESHOLD:                usize = 128;


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
    let (mut result_nakamoto, mut result_grunspan);
    for n in ns {
        filename = format!("{}{}{}{}{}", NAME_RESULTS, NAME_THEORY, NAME_N, n, NAME_FORMAT);
        output = File::create(filename).unwrap();
        for q in &qs {
            result_nakamoto = theory::nakamoto(*q, n);
            result_grunspan = theory::grunspan(*q, n);
            writeln!(output, "{};{};{}", q, result_nakamoto, result_grunspan).unwrap();
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
    let (mut filename, mut output);
    let (mut n_nakamoto, mut n_grunspan);
    for p in ps {
        filename = format!("{}{}{}{}{}", NAME_RESULTS, NAME_THEORY, NAME_P, p, NAME_FORMAT);
        output = File::create(filename).unwrap();
        for q in &qs {
            (n_nakamoto, n_grunspan) = binary_searches(p, *q, N_FIXED_P_LEFT, N_FIXED_P_RIGHT);
            writeln!(output, "{};{};{}", q, n_nakamoto, n_grunspan).unwrap();
        }
        println!("{}{}", MSG_FIXED_P, p);
    }
    println!("{}", MSG_DONE);
}


pub fn double_spending_simulator() {
    println!("Starting Double-Spending Simulations...");
    create_dir_all(NAME_RESULTS).unwrap();
    let ns = [1, 3, 6, 12, 24, 48];
    let qs = calculate_qs();
    let (mut filename, mut output);
    let mut result;
    for n in ns {
        filename = format!("{}{}{}{}{}", NAME_RESULTS, NAME_SIM, NAME_N, n, NAME_FORMAT);
        output = File::create(filename).unwrap();
        for q in &qs {
            // MONTE CARLO
            result = 0.0;
            for _ in 0..NUMBER_OF_SIMULATIONS {
                if double_spending(*q, n, THRESHOLD) {
                // if double_spending(*q, n, THRESHOLD * n) {
                    result += 1.0;
                }
            }
            result /= NUMBER_OF_SIMULATIONS as f64;
            writeln!(output, "{};{}", q, result).unwrap();
        }
        println!("{}{}", MSG_SIM, n);
    }
    println!("{}", MSG_DONE);
}



fn calculate_qs() -> Vec<f64> {
    (1..32).map(|x| (x as f64)/64.0).collect::<Vec<f64>>()
}

fn binary_searches(p: f64, q: f64, l: usize, r: usize) -> (usize, usize) {
    let (mut left, mut right) = (l, r);
    let mut mid = 0;
    let mut result;
    // Nakamoto
    while left < right {
        mid = (left + right) / 2;
        result = theory::nakamoto(q, mid);
        // println!("[N](p={} | q={} | mid={}) result={}", p, q, mid, result);
        if result <= p {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    let n_nakamoto = mid;
    // Grunspan
    (left, right) = (l, r);
    while left < right {
        mid = (left + right) / 2;
        result = theory::grunspan(q, mid);
        // println!("[G](p={} | q={} | mid={}) result={}", p, q, mid, result);
        if result <= p {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    let n_grunspan = mid;
    (n_nakamoto, n_grunspan)
}

// DOUBLE SPENDING - wersja z dwoma zmiennymi
fn double_spending(q: f64, n: usize, threshold: usize) -> bool {
    // Gałąź uczciwych użytkowników oraz gałąź adwersarza
    let (mut chain_real, mut chain_fake) = (0, 0);
    let mut prob: f64;
    let attack_successful: bool;
    loop {      // Powtarzaj dopóki adwersarzowi się nie uda lub się nie podda.
        prob = rand::thread_rng().gen();
        if prob <= q {          // Adwersarz wybudował blok przed uczciwymi użytkownikami.
            chain_fake += 1;
        } else {                // Uczciwi użytkownicy byli pierwszym w wybudowaniu bloku.
            chain_real += 1;
        }
        if chain_real < n {     // Czekamy aż uczciwa gałąź zatwierdzi prawdziwą tranzakcję.
            continue;
        }
        if chain_fake >= chain_real {       // Adwersarz wybudował dłuższy ciąg bloków i publikuje go.
            attack_successful = true;
            break;
        }
        if chain_real - chain_fake >= threshold {    // Adwersarz się poddaje - uznaje że nie nadgoni uczciwej gałęzi.
        // if chain_fake <= n + threshold {
        // if chain_real >= n + threshold {
            attack_successful = false;
            break;
        }
    }
    attack_successful
}

