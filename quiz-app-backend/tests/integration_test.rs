#[cfg(test)]
mod tests {
    use dotenv::dotenv;

    fn setup() {
        dotenv().ok();
    }

    #[test]
    fn test_example() {
        setup();
        assert_eq!(2 + 2, 4);
    }
}
