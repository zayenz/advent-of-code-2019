use assert_cli;

#[test]
fn sample0() {
    assert_cli::Assert::main_binary()
        .stdin(
            "R8,U5,L5,D3
U7,R6,D4,L4",
        )
        .stdout()
        .is("30")
        .unwrap();
}

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83",
        )
        .stdout()
        .is("610")
        .unwrap();
}

#[test]
fn sample2() {
    assert_cli::Assert::main_binary()
        .stdin(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        )
        .stdout()
        .is("410")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("16524")
        .unwrap();
}
