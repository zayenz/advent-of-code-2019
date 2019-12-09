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

use crate::ArgMode::{Immediate, Position, Relative};
use crate::InputError::NoInputAvailable;
use aoc2019::input::get_numbers;
use failure::_core::fmt::Formatter;
use itertools::Itertools;
use std::fmt::Display;

/// When true, the int-code routines will output diagnostic
const DEBUG: bool = false;

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
struct OpArgs1 {
    arg: Word,
}

impl OpArgs1 {
    fn from(store: &[Word], pc: usize) -> OpArgs1 {
        OpArgs1 { arg: store[pc] }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
struct OpArgs2 {
    arg1: Word,
    arg2: Word,
}

impl OpArgs2 {
    fn from(store: &[Word], pc: usize) -> OpArgs2 {
        OpArgs2 {
            arg1: store[pc],
            arg2: store[pc + 1],
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
struct OpArgs3 {
    arg1: Word,
    arg2: Word,
    arg3: Word,
}

impl OpArgs3 {
    fn from(store: &[Word], pc: usize) -> OpArgs3 {
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
    RelativeBaseOffset(OpArgs1, ArgModes),
    Halt,
}

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
enum ArgMode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<Word> for ArgMode {
    type Error = Error;

    fn try_from(value: Word) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Position),
            1 => Ok(Immediate),
            2 => Ok(Relative),
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

pub type Word = i128;

#[derive(Clone, Debug)]
pub struct IntCode {
    store: Vec<Word>,
    pc: usize,
    relative_base: Word,
    instruction_counter: u64,
    output: VecDeque<Word>,
}

impl IntCode {
    pub fn new(store: &[Word]) -> IntCode {
        IntCode {
            store: Vec::from(store),
            pc: 0,
            relative_base: 0,
            instruction_counter: 0,
            output: VecDeque::new(),
        }
    }

    pub fn store(&self) -> &[Word] {
        &self.store
    }

    pub fn output_copy(&self) -> Vec<Word> {
        self.output.iter().cloned().collect()
    }

    pub fn has_output(&self) -> bool {
        !self.output.is_empty()
    }

    pub fn pop_output(&mut self) -> Result<Word, Error> {
        self.output.pop_front().ok_or_else(|| err_msg("No output"))
    }

    fn get(&mut self, arg: Word, arg_mode: ArgMode) -> Word {
        match arg_mode {
            Immediate => arg,
            Position | Relative => {
                let position = match arg_mode {
                    Position => arg,
                    Relative => self.relative_base + arg,
                    Immediate => unreachable!("Already handled above"),
                } as usize;
                if position >= self.store.len() {
                    self.store.resize(position + 1, 0);
                };
                self.store[position]
            }
        }
    }

    fn get_store_position(&mut self, arg: Word, arg_mode: ArgMode) -> usize {
        (match arg_mode {
            Position => arg,
            Immediate => unreachable!("No store argument should have immediate as value"),
            Relative => (self.relative_base + arg),
        }) as usize
    }

    fn set(&mut self, position: usize, value: Word) {
        if position >= self.store.len() {
            self.store.resize(position + 1, 0);
        };
        self.store[position] = value;
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
            9 => Ok(Op::RelativeBaseOffset(
                OpArgs1::from(&self.store, self.pc + 1),
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
            Add(OpArgs3 { arg1, arg2, arg3 }, (am1, am2, am3)) => {
                let arg1 = self.get(arg1, am1);
                let arg2 = self.get(arg2, am2);
                let result = arg1 + arg2;
                let position = self.get_store_position(arg3, am3);
                self.set(position, result);
                self.instruction_counter += 1;
                Continue(self.pc + 4)
            }
            Multiply(OpArgs3 { arg1, arg2, arg3 }, (am1, am2, am3)) => {
                let arg1 = self.get(arg1, am1);
                let arg2 = self.get(arg2, am2);
                let result = arg1 * arg2;
                let position = self.get_store_position(arg3, am3);
                self.set(position, result);
                self.instruction_counter += 1;
                Continue(self.pc + 4)
            }
            Input(OpArgs1 { arg }, (am1, _, _)) => match input.read() {
                Ok(read_input) => {
                    let position = self.get_store_position(arg, am1);
                    self.set(position, read_input);
                    self.instruction_counter += 1;
                    Continue(self.pc + 2)
                }
                Err(NoInputAvailable) => NeedsInput,
            },
            Output(OpArgs1 { arg }, (am1, _, _)) => {
                let output = self.get(arg, am1);
                self.output.push_back(output);
                self.instruction_counter += 1;
                Continue(self.pc + 2)
            }
            JumpIfTrue(OpArgs2 { arg1, arg2 }, (am1, am2, _)) => {
                let arg1 = self.get(arg1, am1);
                let arg2 = self.get(arg2, am2);
                self.instruction_counter += 1;
                if arg1 != 0 {
                    Continue(arg2 as usize)
                } else {
                    Continue(self.pc + 3)
                }
            }
            JumpIfFalse(OpArgs2 { arg1, arg2 }, (am1, am2, _)) => {
                let arg1 = self.get(arg1, am1);
                let arg2 = self.get(arg2, am2);
                self.instruction_counter += 1;
                if arg1 == 0 {
                    Continue(arg2 as usize)
                } else {
                    Continue(self.pc + 3)
                }
            }
            LessThan(OpArgs3 { arg1, arg2, arg3 }, (am1, am2, am3)) => {
                let arg1 = self.get(arg1, am1);
                let arg2 = self.get(arg2, am2);
                let result = if arg1 < arg2 { 1 } else { 0 };
                let position = self.get_store_position(arg3, am3);
                self.set(position, result);
                self.instruction_counter += 1;
                Continue(self.pc + 4)
            }
            Equal(OpArgs3 { arg1, arg2, arg3 }, (am1, am2, am3)) => {
                let arg1 = self.get(arg1, am1);
                let arg2 = self.get(arg2, am2);
                let result = if arg1 == arg2 { 1 } else { 0 };
                let position = self.get_store_position(arg3, am3);
                self.set(position, result);
                self.instruction_counter += 1;
                Continue(self.pc + 4)
            }
            RelativeBaseOffset(OpArgs1 { arg }, (am1, _am2, _am3)) => {
                self.relative_base += self.get(arg, am1);
                self.instruction_counter += 1;
                Continue(self.pc + 2)
            }
            Op::Halt => {
                self.instruction_counter += 1;
                AluStatus::Halt
            }
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

    pub fn executed_instructions_count(&self) -> u64 {
        self.instruction_counter
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
    fn read(&mut self) -> Result<Word, InputError> {
        self.input.take().ok_or(NoInputAvailable)
    }
}

impl Input for () {
    fn read(&mut self) -> Result<Word, InputError> {
        Err(NoInputAvailable)
    }
}

impl Input for VecDeque<Word> {
    fn read(&mut self) -> Result<Word, InputError> {
        self.pop_front().ok_or(NoInputAvailable)
    }
}
