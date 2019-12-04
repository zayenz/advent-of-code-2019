use assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin("14")
        .stdout()
        .is("2")
        .unwrap();
}
#[test]
fn sample2() {
    assert_cli::Assert::main_binary()
        .stdin("1969")
        .stdout()
        .is("966")
        .unwrap();
}
#[test]
fn sample3() {
    assert_cli::Assert::main_binary()
        .stdin("100756")
        .stdout()
        .is("50346")
        .unwrap();
}
#[test]
fn sample_all() {
    assert_cli::Assert::main_binary()
        .stdin(
            "14
            1969
            100756",
        )
        .stdout()
        .is("51314")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("5244112")
        .unwrap();
}
