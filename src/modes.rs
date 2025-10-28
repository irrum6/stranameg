pub mod modes {
    #[derive(Clone)]
    pub enum Modes {
        Password,
        Password84,
        RandomLetters,
        RandomLettersFromCustomAlphabet,
        RandomLettersFromAlphabetFile,
        CoupledWordsNouns,
        CoupledWordsNames,
        CoupledWordsListFiles
    }
    impl Modes {
        pub fn from(s: &str) -> Modes {
            return match s {
                "pass"=>Modes::Password,
                "pass2"=>Modes::Password84,
                "rls" => Modes::RandomLetters,
                "rla" => Modes::RandomLettersFromCustomAlphabet,
                "rlaf" => Modes::RandomLettersFromAlphabetFile,
                "cow" | "cwo" => Modes::CoupledWordsNouns,
                "cowe" | "cwe" => Modes::CoupledWordsNames,
                "cowf" | "cwf" => Modes::CoupledWordsListFiles,
                _ => Modes::RandomLetters,
            };
        }
    }
}
