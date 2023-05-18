// mod processor;

use std::env;
use std::collections::HashSet;

const DEFAULT_RING_SIZE:    usize = 5;

fn is_illegal(config: &Vec<u8>) -> bool {
    let ring_size = config.len();
    let mut count = 0;
    // First processor
    if config[0] == config[ring_size - 1] {
        count += 1;
    }
    // The rest of the ring
    for i in 1..ring_size {
        if config[i] != config[i - 1] {
            count += 1;
        }
        if count == 2 {
            return true;
        }
    }
    false
}

fn get_configs(set: HashSet<Vec<u8>>, i: usize, ring_size: usize) -> HashSet<Vec<u8>> {
    // println!("i={}", i);
    if i == ring_size {
        // println!("get_configs end!");
        return set;
    }
    let mut new_set = HashSet::new();
    for j in 0..=ring_size {
        for config in &set {
            let mut new_config = config.clone();
            new_config[i] = j as u8;
            new_set.insert(new_config);
        }
    }
    // for config in &new_set {
    //     println!("{:?}", config);
    // }
    // println!("---");
    get_configs(new_set, i + 1, ring_size)
}

fn get_all_configs(ring_size: usize) -> HashSet<Vec<u8>> {
    let mut set = HashSet::new();
    set.insert(vec![0; ring_size]);
    get_configs(set, 0, ring_size)
}

fn filter_configs(configs: HashSet<Vec<u8>>) -> HashSet<Vec<u8>> {
    let mut new_set = HashSet::new();
    for config in configs {
        if is_illegal(&config) {
            new_set.insert(config);
        }
    }
    new_set
}

fn max_steps(ring_size: usize) -> usize {
    let mut configs = filter_configs(get_all_configs(ring_size));
    println!("Number of illegal configs: {}", configs.len());
    let mut steps = 0;
    println!("Starting...");
    while !configs.is_empty() {
        let mut new_configs = HashSet::new();
        for config in &configs {
            if config[0] == config[ring_size - 1] {
                let mut new_one = config.clone();
                new_one[0] = (new_one[0] + 1) % (ring_size as u8 + 1);
                new_configs.insert(new_one);
            }
            for i in 1..ring_size {
                if config[i] != config[i-1] {
                    let mut new_one = config.clone();
                    new_one[i] = new_one[i-1];
                    new_configs.insert(new_one);
                }
            }
        }
        configs = HashSet::new();
        for config in new_configs {
            if is_illegal(&config) {
                configs.insert(config);
            }
        }
        steps += 1;
        println!("Finished step {}", steps);
    }
    steps
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let ring_size: usize = match args.len() {
        2 => {
            match args[1].parse() {
                Ok(n) => n,
                _ => {
                    println!("Argument doesn't have a correct type. Defaulting to {}.", DEFAULT_RING_SIZE);
                    DEFAULT_RING_SIZE
                }
            }
        },
        _ => {
            println!("Supply with ONE argument of type unsigned integer. Defaulting to {}", DEFAULT_RING_SIZE);
            DEFAULT_RING_SIZE
        }
    };
    println!("Ring size: {}", ring_size);
    let max_steps = max_steps(ring_size);
    println!("Max steps: {}", max_steps);
}
