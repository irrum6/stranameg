pub mod strgen {
    use crate::RNGWheel;
    pub struct StringGenerator {
        alphabet: Vec<char>,
        heldString: String,
    }
    impl StringGenerator {
        pub fn new(alpha: Vec<char>) -> StringGenerator {
            let x = String::new();
            return StringGenerator {
                heldString: x,
                alphabet: alpha,
            };
        }
        pub fn get(&mut self, l: usize) -> String {
            let rng = RNGWheel::new(l);
            let len = self.alphabet.len();
            self.heldString = String::new();
            for num in rng {
                let index = num as usize % len;
                self.heldString.push(self.alphabet[index]);
            }
            return self.heldString.clone();
        }
    }
}
