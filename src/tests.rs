#[cfg(test)]
pub mod tests {
    use crate::{fill_list, ABCGenerator, Languages, ListGenerator, ListType, StringGenerator};
    #[test]
    pub fn check_len() {
        let charski: Vec<char> = "abc".chars().collect();
        let len: usize = 16;
        let mut sg = ABCGenerator::new(charski);
        let strong = sg.get(len);
        assert_eq!(strong.len(), len);
    }
    #[test]
    pub fn check_min_length() {
        let lst = ListType::Nouns;
        let lan = Languages::English;
        let mut lsg = ListGenerator::new(lst, lan);
        fill_list(&mut lsg);
        let len = 32;
        let strong = lsg.get(len as usize);
        let strong_lena = strong.len();
        assert_eq!(strong_lena >= len, true);
    }
}
