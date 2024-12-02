pub fn hello_world() -> String {
    let greet = std::env::var("GREET").expect("GREET env var must be set");
    format!("hello {greet}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("hello foobar")]
    #[test]
    fn foobar(#[case] expected: &str) {
        let actual = hello_world();
        assert_eq!(&actual, expected)
    }
}
