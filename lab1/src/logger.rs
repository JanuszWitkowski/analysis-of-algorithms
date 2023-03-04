pub struct Logger {
    log_events: bool,
    logs: Vec<String>,
}

impl Logger {
    pub fn new(gather_logs: bool) -> Logger {
        Logger {
            log_events: gather_logs,
            logs: Vec::new(),
        }
    }
}

pub fn log(mut logger: Logger, str: String) -> Logger {
    if logger.log_events {
        logger.logs.push(str);
    }
    logger
}

pub fn log_round(mut logger: Logger, round: u32) -> Logger {
    if logger.log_events {
        logger.logs.push(format!("[RUNDA {}]", round));
    }
    logger
}

pub fn log_slot(mut logger: Logger, slot: u32) -> Logger {
    if logger.log_events {
        logger.logs.push(format!("SLOT {}", slot));
    }
    logger
}

pub fn log_slot_and_chance(mut logger: Logger, slot: u32, p: f64) -> Logger {
    if logger.log_events {
        logger.logs.push(format!("SLOT {} (p = {})", slot, p));
    }
    logger
}

pub fn log_signal(mut logger: Logger, signal: u32) -> Logger {
    if logger.log_events {
        logger.logs.push(format!("\t{} nadaje.", signal));
    }
    logger
}

pub fn log_null(mut logger: Logger, slot: u32) -> Logger {
    if logger.log_events {
        logger.logs.push(format!("NULL {}", slot));
    }
    logger
}

pub fn log_collision(mut logger: Logger, slot: u32) -> Logger {
    if logger.log_events {
        logger.logs.push(format!("COLLISION {}", slot));
    }
    logger
}

pub fn log_single(mut logger: Logger, slot: u32) -> Logger {
    if logger.log_events {
        logger.logs.push(format!("SINGLE {}", slot));
    }
    logger
}

pub fn log_leader(mut logger: Logger, leader: u32, slots: u32) -> Logger {
    if logger.log_events {
        logger.logs.push(format!("Po {} slotach wybrano lidera: {}", slots, leader));
    }
    logger
}

pub fn log_leader_with_rounds(mut logger: Logger, leader: u32, rounds: u32, slots: u32, total: u32) -> Logger {
    if logger.log_events {
        logger.logs.push(format!("W {} rundzie w {} slocie (po {} slotach łącznie) wybrano lidera: {}", rounds, slots, total, leader));
    }
    logger
}

pub fn print(logger: Logger) -> Logger {
    if !logger.log_events {
        println!("Nie ma logów.");
    }
    else {
        for log in logger.logs.iter() {
            println!("{}", log);
        }
    }
    logger
}
