use assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin("1,0,0,0,99")
        .stdout()
        .is("2")
        .unwrap();
}
#[test]
fn sample2() {
    assert_cli::Assert::main_binary()
        .stdin("2,3,0,3,99")
        .stdout()
        .is("2")
        .unwrap();
}
#[test]
fn sample3() {
    assert_cli::Assert::main_binary()
        .stdin("2,4,4,5,99,0")
        .stdout()
        .is("2")
        .unwrap();
}
#[test]
fn sample4() {
    assert_cli::Assert::main_binary()
        .stdin("1,1,1,4,99,5,6,0,99")
        .stdout()
        .is("30")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("5290681")
        .unwrap();
}
