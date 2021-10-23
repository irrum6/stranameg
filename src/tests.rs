#[cfg(test)]
pub mod tests {
    use crate::StringGenerator;
    use crate::ABCGenerator;
    #[test]
    pub fn check_len() {
        let charski: Vec<char> = "abc".chars().collect();
        let len: usize = 16;
        let mut sg = ABCGenerator::new(charski);
        let strong = sg.get(len);
        assert_eq!(strong.len(), len);
    }
}
