use crate::parser::Node;
use crate::types::Literal;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Add, Div, Mul};

#[derive(Debug)]
enum Cell {
    Lit(Literal),
    Quotation,
}

impl Add<Cell> for Cell {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Cell::Lit(l1), Cell::Lit(l2)) => Cell::Lit(l1 + l2),
            _ => panic!("Type error!"),
        }
    }
}

impl Mul<Cell> for Cell {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Cell::Lit(l1), Cell::Lit(l2)) => Cell::Lit(l1 * l2),
            _ => panic!("Type error!"),
        }
    }
}

impl Div<Cell> for Cell {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Cell::Lit(l1), Cell::Lit(l2)) => Cell::Lit(l1 / l2),
            _ => panic!("Type error!"),
        }
    }
}

type Stack = Vec<Cell>;

enum Operation {
    Plus,
    Div,
    Mul,
    Swap,
    Println,
}

impl From<String> for Operation {
    fn from(s: String) -> Self {
        match s.as_str() {
            "+" => Self::Plus,
            "/" => Self::Div,
            "*" => Self::Mul,
            "swap" => Self::Swap,
            "." => Self::Println,
            _ => panic!("[{}] is not a known operation.", s)
        }
    }
}

pub(crate) struct Interpreter {
    active_stack: usize,
    stacks: Vec<Stack>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            active_stack: 0,
            stacks: vec![vec![]],
        }
    }
    pub fn apply(&mut self, node: Node) {
        match node {
            Node::Lit(l) => {
                println!("adding {:?} to stack", l);
                self.stacks[self.active_stack].push(Cell::Lit(l));
                println!("stack: {:?}", self.stacks[self.active_stack]);
            },
            Node::Word(name) => {
                println!("stack before op: {:?}", self.stacks[self.active_stack]);
                self.execute(name.into());
                println!("stack after op: {:?}", self.stacks[self.active_stack]);
            }
        }
    }

    fn execute(&mut self, op: Operation) {
        match op {
            Operation::Swap => {
                let mut stack = self.stacks.get_mut(self.active_stack).unwrap();
                let stack_len = stack.len();
                stack.swap(stack_len - 2, stack_len - 1);
            }
            Operation::Println => {
                let mut stack = self.stacks.get_mut(self.active_stack).unwrap();
                println!("{:?}", stack.pop().unwrap());
            }
            Operation::Plus => {
                let mut stack = self.stacks.get_mut(self.active_stack).unwrap();
                let l1 = stack.pop().unwrap();
                let l2 = stack.pop().unwrap();
                stack.push(l1 + l2);
            }
            Operation::Mul => {
                let mut stack = self.stacks.get_mut(self.active_stack).unwrap();
                let l1 = stack.pop().unwrap();
                let l2 = stack.pop().unwrap();
                stack.push(l1 * l2);
            }
            Operation::Div => {
                let mut stack = self.stacks.get_mut(self.active_stack).unwrap();
                let l1 = stack.pop().unwrap();
                let l2 = stack.pop().unwrap();
                stack.push(l2 / l1);
            }
        }
    }
}