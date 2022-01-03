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
                "rls"  => Modes::RandomLetters,
                "rla"  => Modes::RandomLettersFromCustomAlphabet,
                "rlaf" => Modes::RandomLettersFromAlphabetFile,
                "raw"  => Modes::RandomWord,
                "rawl"  => Modes::RandomWordFromListFile,
                "cow" | "cw" => Modes::CoupledWordsNouns,
                "cowe" | "cwe" => Modes::CoupledWordsNames,
                "cowf" | "cwf" => Modes::CoupledWordsListFiles,
                _ => Modes::RandomLetters,
            };
        }
    }
}
