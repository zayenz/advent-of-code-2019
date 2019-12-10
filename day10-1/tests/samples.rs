use assert_cli;

#[test]
fn sample0() {
    assert_cli::Assert::main_binary()
        .stdin(
            ".#..#
.....
#####
....#
...##",
        )
        .stdout()
        .is("8")
        .unwrap();
}
#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin(
            "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####",
        )
        .stdout()
        .is("33")
        .unwrap();
}
#[test]
fn sample2() {
    assert_cli::Assert::main_binary()
        .stdin(
            "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
",
        )
        .stdout()
        .is("35")
        .unwrap();
}
#[test]
fn sample3() {
    assert_cli::Assert::main_binary()
        .stdin(
            ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..",
        )
        .stdout()
        .is("41")
        .unwrap();
}
#[test]
fn sample4() {
    assert_cli::Assert::main_binary()
        .stdin(
            ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
        )
        .stdout()
        .is("210")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("256")
        .unwrap();
}