use assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0")
        .stdout()
        .is("43210")
        .unwrap();
}

#[test]
fn sample2() {
    assert_cli::Assert::main_binary()
        .stdin("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0")
        .stdout()
        .is("54321")
        .unwrap();
}

#[test]
fn sample3() {
    assert_cli::Assert::main_binary()
        .stdin(
            "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
        )
        .stdout()
        .is("65210")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("22012")
        .unwrap();
}
