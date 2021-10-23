pub mod strgen {
    use crate::RNGWheel;
    pub trait StringGenerator {
        fn get(&mut self, l: usize) -> String;
    }
    pub struct AlphaBetStringGenerator {
        alphabet: Vec<char>,
        held_string: String,
    }
    impl AlphaBetStringGenerator {
        pub fn new(alpha: Vec<char>) -> AlphaBetStringGenerator {
            let x = String::new();
            return AlphaBetStringGenerator {
                held_string: x,
                alphabet: alpha,
            };
        }
    }
    impl StringGenerator for AlphaBetStringGenerator {
        fn get(&mut self, l: usize) -> String {
            let rng = RNGWheel::new(l);
            let len = self.alphabet.len();
            self.held_string = String::new();
            for num in rng {
                let index = num as usize % len;
                self.held_string.push(self.alphabet[index]);
            }
            return self.held_string.clone();
        }
    }
}
