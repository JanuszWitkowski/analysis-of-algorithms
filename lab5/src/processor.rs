pub struct Processor {
    id:     usize,
    reg:    usize,      // TODO: Change to Register.
    pred:   Option<&Processor>,
}

impl Processor {
    fn new(id: usize, reg: usize) -> Self where Self: Sized {
        Processor { id, reg, pred: None }
    }

    pub fn new_vector(length: usize, registers: Vec<usize>) -> Vec<Processor> {
        //
    }
}
