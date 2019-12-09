use assert_cli;

#[test]
fn sample0() {
    assert_cli::Assert::main_binary()
        .stdin(
            "3 3
123456789012",
        )
        .stdout()
        .is("1")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("2904")
        .unwrap();
}
