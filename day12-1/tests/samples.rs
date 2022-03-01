use assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin(
            "10
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>",
        )
        .stdout()
        .is("179")
        .unwrap();
}

#[test]
fn sample2() {
    assert_cli::Assert::main_binary()
        .stdin(
            "100
<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>",
        )
        .stdout()
        .is("1940")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("12351")
        .unwrap();
}
