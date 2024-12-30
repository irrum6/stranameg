pub mod modes {
    #[derive(Clone)]
    pub enum Modes {
        Password,
        RandomLetters,
        RandomLettersFromCustomAlphabet,
        RandomLettersFromAlphabetFile,
        RandomWord,
        RandomWordFromListFile,
        CoupledWordsNouns,
        CoupledWordsNames,
        CoupledWordsListFiles,
        SimpleSentences,
    }
    impl Modes {
        pub fn from(s: &str) -> Modes {
            return match s {
                "pass"=>Modes::Password,
                "rls" => Modes::RandomLetters,
                "rla" => Modes::RandomLettersFromCustomAlphabet,
                "rlaf" => Modes::RandomLettersFromAlphabetFile,
                "raw" => Modes::RandomWord,
                "rawl" => Modes::RandomWordFromListFile,
                "cow" | "cwo" => Modes::CoupledWordsNouns,
                "cowe" | "cwe" => Modes::CoupledWordsNames,
                "cowf" | "cwf" => Modes::CoupledWordsListFiles,
                "sen" => Modes::SimpleSentences,
                _ => Modes::RandomLetters,
            };
        }
    }
}
