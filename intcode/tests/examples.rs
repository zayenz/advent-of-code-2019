#![allow(dead_code, unused_imports)]

///! This file includes all the example tests given for the IntCode architecture.
use joinery::{Joinable, JoinableIterator};

use intcode;
use intcode::SingleInput;

#[test]
fn test_full_memory() {
    let cases = [
        (vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]),
        (vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]),
        (vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]),
        (
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        ),
        //        (vec![], vec![]),
    ];

    for (initial, result) in &cases {
        let mut ic = intcode::IntCode::new(initial);
        let mut input = ();
        ic.run(&mut input).unwrap();
        assert_eq!(ic.store(), &result[..]);
    }
}

#[test]
fn test_input_to_first_output() {
    let cases = [
        (
            vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
            vec![(7, 0), (8, 1), (9, 0)],
        ),
        (
            vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
            vec![(7, 1), (8, 0), (9, 0)],
        ),
        (
            vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],
            vec![(7, 0), (8, 1), (9, 0)],
        ),
        (
            vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],
            vec![(7, 1), (8, 0), (9, 0)],
        ),
        (
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            vec![(-1, 1), (0, 0), (1, 1)],
        ),
        (
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            vec![(-1, 1), (0, 0), (1, 1)],
        ),
        (
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            vec![(7, 999), (8, 1000), (9, 1001)],
        ),
        //        (vec![], vec![
        //            ( ,),
        //        ]),
    ];

    for (initial, io) in &cases {
        for (input, output) in io {
            //            println!("{}, {}, [{}]", input, output, initial.iter().join_with(", "));
            let mut ic = intcode::IntCode::new(initial);
            let mut input = SingleInput::new(*input);
            ic.run(&mut input).unwrap();
            assert_eq!(ic.pop_output().ok().unwrap(), *output);
        }
    }
}

#[test]
fn test_no_input_to_full_output() {
    let cases = [
        (
            vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ],
            vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ],
        ),
        (
            vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0],
            vec![1219070632396864],
        ),
        (vec![104, 1125899906842624, 99], vec![1125899906842624]),
        //        (vec![], vec![]),
    ];

    for (initial, output) in &cases {
        //            println!("{}, {}, [{}]", input, output, initial.iter().join_with(", "));
        let mut ic = intcode::IntCode::new(initial);
        ic.run(&mut ()).unwrap();
        assert_eq!(ic.output_copy(), *output);
    }
}
