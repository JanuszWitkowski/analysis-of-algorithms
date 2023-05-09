pub fn nakamoto(q: f64, n: usize) -> f64 {
    // let p = 1.0 - q;
    // let lambda = (n as f64) * (q / p);
    // let exp_minus_lambda = (- lambda).exp();
    // let mut curr_fact = 1.0;
    // let mut sum = 1.0;
    // let (mut temp1, mut temp3);
    // for k in 0..n {
    //     if k != 0 {     // TODO: poprawić bo to śmieszne jest xD
    //         curr_fact *= k as f64;
    //     }
    //     temp1 = lambda.powf(k as f64) * exp_minus_lambda / curr_fact;
    //     temp3 = (q / p).powf((n - k) as f64);
    //     sum -= temp1 * (1.0 - temp3);
    // }
    // sum
    let p = 1.0 - q;
    let lambda = (n as f64) * (q / p);
    let exp_minus_lambda = (- lambda).exp();
    let mut curr_fact = 1.0;
    let mut sum = 0.0;
    let (mut poisson, mut prob);
    for k in 0..n {
        poisson = lambda.powf(k as f64);
        prob = (q / p).powf((n - k) as f64);
        sum += poisson * (1.0 - prob) / curr_fact;
        curr_fact *= (k+1) as f64;
    }
    1.0 - exp_minus_lambda * sum
}

pub fn grunspan(q: f64, n: usize) -> f64 {
    // let p = 1.0 - q;
    // let mut sum = 1.0;
    // let pn = p.powf(n as f64);
    // let qn = q.powf(n as f64);
    // let mut pk = 1.0;
    // let mut qk = 1.0;
    // let mut newton = 1.0;
    // for k in 0..n {
    //     sum -= newton * (pn*qk - qn*pk);
    //     newton = newton * ((k + n) as f64) / ((k + 1) as f64);
    //     pk *= p;
    //     qk *= q;
    // }
    // sum
    let p = 1.0 - q;
    let mut sum = 0.0;
    let pn = p.powf(n as f64);
    let qn = q.powf(n as f64);
    let mut pk = 1.0;   // p^0 = 1
    let mut qk = 1.0;   // q^0 = 1
    let mut newton = 1.0;   // (n 0) = 1
    for k in 0..n {
        sum += newton * (pn*qk - qn*pk);
        newton = newton * ((k + n) as f64) / ((k + 1) as f64);  // Incremental formula for Newton
        pk *= p;
        qk *= q;
    }
    1.0 - sum
}

