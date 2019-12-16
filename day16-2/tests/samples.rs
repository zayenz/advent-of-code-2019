use assert_cli;

#[test]
fn sample0() {
    assert_cli::Assert::main_binary()
        .stdin(
            "100
03036732577212944063491565474664",
        )
        .stdout()
        .is("84462026")
        .unwrap();
}

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin(
            "100
02935109699940807407585447034323",
        )
        .stdout()
        .is("78725270")
        .unwrap();
}

#[test]
fn sample2() {
    assert_cli::Assert::main_binary()
        .stdin(
            "100
03081770884921959731165446850517",
        )
        .stdout()
        .is("53553731")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("82994322")
        .unwrap();
}
