#[cfg(test)]
pub mod tests {
    // use strgen::gela;
    #[test]
    fn unnamed() {
        assert_eq!(1, 1);
    }

    #[test]
    fn unnamed2() {
        use stranameg::stringer::print_help;
        print_help();
        assert_eq!(1, 1);
    }
}
