pub mod strgen {
    use crate::RNGWheel;
    pub struct StringGenerator {
        alphabet: Vec<char>,
        held_string: String,
    }
    impl StringGenerator {
        pub fn new(alpha: Vec<char>) -> StringGenerator {
            let x = String::new();
            return StringGenerator {
                held_string: x,
                alphabet: alpha,
            };
        }
        pub fn get(&mut self, l: usize) -> String {
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
