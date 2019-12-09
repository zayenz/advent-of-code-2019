#![allow(dead_code, unused_imports)]

use failure::Error;
use failure::{bail, err_msg, Fail};
use joinery::{Joinable, JoinableIterator};
use rayon::prelude::*;
use strum_macros::EnumString;

use std::char;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::io::BufRead;
use std::ops::*;
use std::str;
use std::str::FromStr;
use std::{io, process};

use crate::ArgMode::{Immediate, Position};
use crate::InputError::NoInputAvailable;
use aoc2019::input::get_numbers;
use failure::_core::fmt::Formatter;
use std::fmt::Display;

/// When true, the int-code routines will output diagnostic
const DEBUG: bool = false;

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
struct OpArgs1 {
    arg: i32,
}

impl OpArgs1 {
    fn from(store: &[i32], pc: usize) -> OpArgs1 {
        OpArgs1 { arg: store[pc] }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
struct OpArgs2 {
    arg1: i32,
    arg2: i32,
}

impl OpArgs2 {
    fn from(store: &[i32], pc: usize) -> OpArgs2 {
        OpArgs2 {
            arg1: store[pc],
            arg2: store[pc + 1],
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
struct OpArgs3 {
    arg1: i32,
    arg2: i32,
    arg3: i32,
}

impl OpArgs3 {
    fn from(store: &[i32], pc: usize) -> OpArgs3 {
        OpArgs3 {
            arg1: store[pc],
            arg2: store[pc + 1],
            arg3: store[pc + 2],
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
enum Op {
    Add(OpArgs3, ArgModes),
    Multiply(OpArgs3, ArgModes),
    Input(OpArgs1, ArgModes),
    Output(OpArgs1, ArgModes),
    JumpIfTrue(OpArgs2, ArgModes),
    JumpIfFalse(OpArgs2, ArgModes),
    LessThan(OpArgs3, ArgModes),
    Equal(OpArgs3, ArgModes),
    Halt,
}

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
enum ArgMode {
    Position,
    Immediate,
}

impl ArgMode {
    fn get(self, store: &[i32], arg: i32) -> i32 {
        match self {
            Position => store[arg as usize],
            Immediate => arg,
        }
    }
}

impl TryFrom<i32> for ArgMode {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Position),
            1 => Ok(Immediate),
            _ => bail!("Value {} is not vaild for ArgMode", value),
        }
    }
}

type ArgModes = (ArgMode, ArgMode, ArgMode);

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
enum AluStatus {
    Continue(usize),
    NeedsInput,
    Halt,
}

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
pub enum Status {
    Done,
    NeedsInput,
}

pub type Word = i32;

#[derive(Clone, Debug)]
pub struct IntCode {
    store: Vec<Word>,
    pc: usize,
    output: VecDeque<Word>,
}

impl IntCode {
    pub fn new(store: &[Word]) -> IntCode {
        IntCode {
            store: Vec::from(store),
            pc: 0,
            output: VecDeque::new(),
        }
    }

    pub fn store(&self) -> &[Word] {
        &self.store
    }

    //    pub fn output(&self) -> &[Word] {
    //        &self.output
    //    }

    pub fn has_output(&self) -> bool {
        !self.output.is_empty()
    }

    pub fn pop_output(&mut self) -> Result<Word, Error> {
        self.output.pop_front().ok_or_else(|| err_msg("No output"))
    }

    fn decode(&self) -> Result<Op, Error> {
        let word = self.store[self.pc];
        let opcode = word % 100;
        let arg_mode1: ArgMode = ((word / 100) % 10).try_into()?;
        let arg_mode2: ArgMode = ((word / 1000) % 10).try_into()?;
        let arg_mode3: ArgMode = ((word / 10000) % 10).try_into()?;
        let arg_modes = (arg_mode1, arg_mode2, arg_mode3);
        match opcode {
            1 => Ok(Op::Add(OpArgs3::from(&self.store, self.pc + 1), arg_modes)),
            2 => Ok(Op::Multiply(
                OpArgs3::from(&self.store, self.pc + 1),
                arg_modes,
            )),
            3 => Ok(Op::Input(
                OpArgs1::from(&self.store, self.pc + 1),
                arg_modes,
            )),
            4 => Ok(Op::Output(
                OpArgs1::from(&self.store, self.pc + 1),
                arg_modes,
            )),
            5 => Ok(Op::JumpIfTrue(
                OpArgs2::from(&self.store, self.pc + 1),
                arg_modes,
            )),
            6 => Ok(Op::JumpIfFalse(
                OpArgs2::from(&self.store, self.pc + 1),
                arg_modes,
            )),
            7 => Ok(Op::LessThan(
                OpArgs3::from(&self.store, self.pc + 1),
                arg_modes,
            )),
            8 => Ok(Op::Equal(
                OpArgs3::from(&self.store, self.pc + 1),
                arg_modes,
            )),
            99 => Ok(Op::Halt),
            code => bail!("Unknown op code {} at {}", code, self.pc),
        }
    }

    fn step<I: Input>(&mut self, op: Op, input: &mut I) -> Result<AluStatus, Error> {
        use AluStatus::*;
        use Op::*;
        Ok(match op {
            Add(OpArgs3 { arg1, arg2, arg3 }, (am1, am2, _am3)) => {
                let arg1 = am1.get(&self.store, arg1);
                let arg2 = am2.get(&self.store, arg2);
                let result = arg1 + arg2;
                self.store[arg3 as usize] = result;
                Continue(self.pc + 4)
            }
            Multiply(OpArgs3 { arg1, arg2, arg3 }, (am1, am2, _am3)) => {
                let arg1 = am1.get(&self.store, arg1);
                let arg2 = am2.get(&self.store, arg2);
                let result = arg1 * arg2;
                self.store[arg3 as usize] = result;
                Continue(self.pc + 4)
            }
            Input(OpArgs1 { arg }, (_am1, _, _)) => match input.read() {
                Ok(read_input) => {
                    self.store[arg as usize] = read_input;
                    Continue(self.pc + 2)
                }
                Err(NoInputAvailable) => NeedsInput,
            },
            Output(OpArgs1 { arg }, (am1, _, _)) => {
                let output = am1.get(&self.store, arg);
                self.output.push_back(output);
                Continue(self.pc + 2)
            }
            JumpIfTrue(OpArgs2 { arg1, arg2 }, (am1, am2, _)) => {
                let arg1 = am1.get(&self.store, arg1);
                let arg2 = am2.get(&self.store, arg2);
                if arg1 != 0 {
                    Continue(arg2 as usize)
                } else {
                    Continue(self.pc + 3)
                }
            }
            JumpIfFalse(OpArgs2 { arg1, arg2 }, (am1, am2, _)) => {
                let arg1 = am1.get(&self.store, arg1);
                let arg2 = am2.get(&self.store, arg2);
                if arg1 == 0 {
                    Continue(arg2 as usize)
                } else {
                    Continue(self.pc + 3)
                }
            }
            LessThan(OpArgs3 { arg1, arg2, arg3 }, (am1, am2, _am3)) => {
                let arg1 = am1.get(&self.store, arg1);
                let arg2 = am2.get(&self.store, arg2);
                let result = if arg1 < arg2 { 1 } else { 0 };
                self.store[arg3 as usize] = result;
                Continue(self.pc + 4)
            }
            Equal(OpArgs3 { arg1, arg2, arg3 }, (am1, am2, _am3)) => {
                let arg1 = am1.get(&self.store, arg1);
                let arg2 = am2.get(&self.store, arg2);
                let result = if arg1 == arg2 { 1 } else { 0 };
                self.store[arg3 as usize] = result;
                Continue(self.pc + 4)
            }
            Op::Halt => AluStatus::Halt,
        })
    }

    pub fn run<I: Input>(&mut self, input: &mut I) -> Result<Status, Error> {
        if DEBUG {
            self.debug_print();
        }
        let status;
        loop {
            let op = self.decode()?;
            match self.step(op, input)? {
                AluStatus::Continue(next_pc) => {
                    self.pc = next_pc;
                }
                AluStatus::Halt => {
                    status = Some(Status::Done);
                    break;
                }
                AluStatus::NeedsInput => {
                    status = Some(Status::NeedsInput);
                    break;
                }
            }
            if DEBUG {
                self.debug_print();
            }
        }
        if DEBUG {
            self.debug_print();
        }
        Ok(status.expect("Must have a status"))
    }

    pub fn run_no_input(&mut self) -> Result<Status, Error> {
        self.run(&mut ())
    }

    pub fn debug_print(&self) {
        print!("Store: ");
        for (pos, value) in self.store.iter().enumerate() {
            if pos == self.pc {
                print!(">");
            }
            print!("{} ", value);
        }
        println!();
        print!("Output: [{}]", self.output.iter().join_with(", "));
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum InputError {
    NoInputAvailable,
}

impl Display for InputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "NoInputAvailable")
    }
}

impl Fail for InputError {}

pub trait Input {
    fn read(&mut self) -> Result<Word, InputError>;
}

pub struct SingleInput {
    input: Option<Word>,
}

impl SingleInput {
    pub fn new(word: Word) -> SingleInput {
        SingleInput { input: Some(word) }
    }
}

impl Input for SingleInput {
    fn read(&mut self) -> Result<i32, InputError> {
        self.input.take().ok_or(NoInputAvailable)
    }
}

impl Input for () {
    fn read(&mut self) -> Result<i32, InputError> {
        Err(NoInputAvailable)
    }
}

impl Input for VecDeque<Word> {
    fn read(&mut self) -> Result<i32, InputError> {
        self.pop_front().ok_or(NoInputAvailable)
    }
}
