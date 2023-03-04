use rand::Rng;  // 0.8.5
use crate::logger;

const NULL: u32 = 0;
const SINGLE: u32 = 1;

pub fn election(n: u32, p: f64, gather_logs: bool) -> (u32, u32, logger::Logger) {
    let mut leader: u32 = 0;
    let mut slots: u32 = 0;
    let mut logger = logger::Logger::new(gather_logs);
    let mut slot: u32 = 0;
    let mut q: f64;

    while slot != SINGLE {
        slot = 0;
        slots += 1;
        logger = logger::log_slot(logger, slots);
        for node in 1..=n {
            q = rand::thread_rng().gen_range(0.0..1.0);
            if q < p {
                slot += 1;
                leader = node;
                logger = logger::log_signal(logger, node);
            }
        }
        if slot == NULL {
            logger = logger::log_null(logger, slots);
        } else if slot > SINGLE {
            logger = logger::log_collision(logger, slots);
        }
    }

    logger = logger::log(logger, "SINGLE".to_string());
    logger = logger::log_leader(logger, leader, slots);
    return (leader, slots, logger);
}

pub fn election_unknown(u: u32, n: u32, stop_at_1: bool, gather_logs: bool) -> (u32, u32, u32, u32, u32, logger::Logger) {
    let mut leader: u32 = 0;
    let mut rounds: u32 = 0;
    let mut slots: u32 = 0;
    let mut total: u32 = 0;
    let mut logger = logger::Logger::new(gather_logs);

    let l = (u as f32).log2().ceil() as u32;
    let mut p: f64;
    let mut q: f64;
    let mut slot: u32 = 0;

    while slot != SINGLE {
        rounds += 1;
        logger = logger::log_round(logger, rounds);
        for i in 1..=l {
            p = 0.5_f64.powf(i as f64);
            total += 1;
            slot = 0;
            logger = logger::log_slot_and_chance(logger, i, p);
            for node in 1..=n {
                q = rand::thread_rng().gen_range(0.0..1.0);
                if q < p {
                    slot += 1;
                    leader = node;
                    logger = logger::log_signal(logger, node);
                }
            }
            if slot == SINGLE {
                slots = i;
                logger = logger::log_single(logger, i);
                break;
            } else if slot == NULL {
                logger = logger::log_null(logger, i);
            } else {
                logger = logger::log_collision(logger, i);
            }
        }
        if stop_at_1 && slot != SINGLE {
            leader = 0;
            break;
        }
    }

    if stop_at_1 && slot != SINGLE {
        logger = logger::log(logger, "Nie udało się wybrać lidera w jednej rundzie.".to_string());
    }
    else {
        logger = logger::log_leader_with_rounds(logger, leader, rounds, slots, total);
    }
    return (leader, l, rounds, slots, total, logger);
}
