pub mod languages {
    #[derive(Clone)]
    pub enum Languages {
        English,
        Georgian,
        German,
    }
    impl Languages {
        pub fn abbr(&self) -> String {
            let result = match *self {
                Languages::English => "en",
                Languages::Georgian => "ka",
                Languages::German => "de",
            };
            return String::from(result);
        }
        pub fn is_german(&self) -> bool {
            return matches!(*self, Languages::German);
        }
        pub fn from(s: &str) -> Languages {
            return match s {
                "en" | "En" | "eN" | "EN" => Languages::English,
                "ka" | "KA" | "Ka" | "kA" => Languages::Georgian,
                "de" | "dE" | "De" | "DE" => Languages::German,
                _ => Languages::English,
            };
        }
        pub fn get_alphabet(&self) -> String {
            let result = match *self {
                Languages::Georgian => "აბგდევზთიკლმნოპჟრსტუფქღყშჩცძწჭხჯჰააააეეეიიიოოოუუ",
                Languages::English => "abcdefghijklmnopqrstuvwxyzaaaaeeeiiiooouuy",
                _ => "abcdefghijklmnopqrstuvwxyzaaaaeeeiiiooouuy",
            };
            return String::from(result);
        }
    }
}
