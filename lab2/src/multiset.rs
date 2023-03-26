pub struct MultiSet {
    n       : usize,
    counter : usize,
    first   : usize,
    to_gen  : usize,
}

impl MultiSet {
    pub fn new(n: usize) -> MultiSet {
        MultiSet { 
            n,
            counter : 0,  
            first   : (n * (n - 1)) / 2,
            to_gen  : n,
        }
    }

    pub fn new_with_multiplier(n: usize, mult: usize) -> MultiSet {
        MultiSet { 
            n,
            counter : 0,  
            first   : (n * (n - 1)) / 2,
            to_gen  : n * mult,
        }
    }
}

impl Iterator for MultiSet {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.to_gen == 0 {
            None
        } else {
            self.to_gen -= 1;
            self.counter = (self.counter % self.n) + 1;
            Some(self.first + self.counter)
        }
    }
}
