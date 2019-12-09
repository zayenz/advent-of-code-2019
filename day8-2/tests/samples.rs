use assert_cli;

#[test]
fn sample0() {
    assert_cli::Assert::main_binary()
        .stdin(
            "2 2
0222112222120000",
        )
        .stdout()
        .is("·#
#·")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("\
#··#··##··###···##··####·
#··#·#··#·#··#·#··#·#····
####·#····###··#····###··
#··#·#·##·#··#·#····#····
#··#·#··#·#··#·#··#·#····
#··#··###·###···##··#····")
        .unwrap();
}
