use std::collections::HashSet;

const RING_SIZE:        usize = 5;
// const MAX_HASHSET_SIZE: usize = 100_000_000;

fn is_illegal(config: &[u8]) -> bool {
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

fn get_configs(set: HashSet<[u8; RING_SIZE]>, i: usize) -> HashSet<[u8; RING_SIZE]> {
    println!("Finding config, i = {}", i);
    if i == RING_SIZE {
        return set;
    }
    let mut new_set = HashSet::new();
    for j in 0..=RING_SIZE {
        for config in &set {
            let mut new_config = *config;
            new_config[i] = j as u8;
            new_set.insert(new_config);
        }
    }
    get_configs(new_set, i + 1)
}

fn get_all_configs() -> HashSet<[u8; RING_SIZE]> {
    let mut set = HashSet::new();
    set.insert([0; RING_SIZE]);
    get_configs(set, 0)
}

fn filter_configs(configs: HashSet<[u8; RING_SIZE]>) -> HashSet<[u8; RING_SIZE]> {
    configs
        .into_iter()
        .filter(|config| is_illegal(config))
        // .filter(|config| is_illegal!(config))
        .collect()
}

fn max_steps(mut configs: HashSet<[u8; RING_SIZE]>) -> usize {
    let mut steps = 0;
    println!("Starting...");
    while !configs.is_empty() {
        let mut new_configs = HashSet::new();
        for config in &configs {
            if config[0] == config[RING_SIZE - 1] {
                let mut new_one = *config;
                new_one[0] = (new_one[0] + 1) % (RING_SIZE as u8 + 1);
                new_configs.insert(new_one);
            }
            for i in 1..RING_SIZE {
                if config[i] != config[i-1] {
                    let mut new_one = *config;
                    new_one[i] = new_one[i-1];
                    new_configs.insert(new_one);
                }
            }
        }

        configs =
            new_configs
            .into_iter()
            .filter(|config| is_illegal(config))
            // .filter(|config| is_illegal!(config))
            .collect();

        steps += 1;
        println!("{} steps finished, configs left: {}", steps, configs.len());
    }
    steps
}

pub fn me_experiment() {
    println!("ME EXPERIMENT");
    
    println!("Ring size: {}", RING_SIZE);

    let timer = std::time::Instant::now();
    let configs = filter_configs(get_all_configs());
    println!("Time spent on finding configs: {:?}", timer.elapsed());
    println!("Number of illegal configs: {}", configs.len());

    let timer = std::time::Instant::now();
    let max_steps = max_steps(configs);
    println!("Time spent on simulations: {:?}", timer.elapsed());
    println!("Max steps: {}", max_steps);

    println!();
}
