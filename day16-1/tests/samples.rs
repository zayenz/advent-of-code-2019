use assert_cli;

#[test]
fn sample0() {
    assert_cli::Assert::main_binary()
        .stdin(
            "100
80871224585914546619083218645595",
        )
        .stdout()
        .is("24176176")
        .unwrap();
}

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin(
            "100
19617804207202209144916044189917",
        )
        .stdout()
        .is("73745418")
        .unwrap();
}

#[test]
fn sample2() {
    assert_cli::Assert::main_binary()
        .stdin(
            "100
69317163492948606335995924319873",
        )
        .stdout()
        .is("52432133")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("90744714")
        .unwrap();
}
