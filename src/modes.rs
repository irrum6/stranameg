pub mod modes {
    #[derive(Clone)]
    pub enum Modes {
        RandomLetters,
        RandomLettersFromCustomAlphabet,
        RandomLettersFromAlphabetFile,
        RandomWord,
        RandomWordFromListFile,
        CoupledWordsNouns,
        CoupledWordsNames,
        CoupledWordsListFiles,
    }
    impl Modes {
        pub fn from(s: &str) -> Modes {
            return match s {
                "rls" | "10" => Modes::RandomLetters,
                "rla" | "11" => Modes::RandomLettersFromCustomAlphabet,
                "rlaf" | "12" => Modes::RandomLettersFromAlphabetFile,
                "raw" | "21" => Modes::RandomWord,
                "rawl" | "22" => Modes::RandomWordFromListFile,
                "cow" | "31" => Modes::CoupledWordsNouns,
                "cowe" | "32" => Modes::CoupledWordsNames,
                "cowf" | "33" => Modes::CoupledWordsListFiles,
                _ => Modes::RandomLetters,
            };
        }
    }
}
