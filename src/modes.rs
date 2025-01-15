pub mod modes {
    #[derive(Clone)]
    pub enum Modes {
        Password,
        Password84,
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
                "pass2"=>Modes::Password84,
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
