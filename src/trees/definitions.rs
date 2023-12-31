use crate::vm::definitions::{BinaryOperation, Reg, Source, TurtleOperation, UnaryOperation};

#[derive(Debug, Clone)]
pub enum Node {
    Null,
    Val(Source),
    Unary(UnaryOperation, Box<Node>),
    Binary(BinaryOperation, Box<Node>, Box<Node>),
    Then(Box<Node>, Box<Node>),
    Print(Box<Node>),
    Store(Reg, Box<Node>),
    /// condition, if_not_zero, if_zero
    If(Box<Node>, Box<Node>, Box<Node>),
    /// condition, block; repeat block until condition equals zero
    While(Box<Node>, Box<Node>),
    /// count, block; repeat block count times
    Repeat(Box<Node>, Box<Node>),
    Compare(Box<Node>, Box<Node>),
    Turtle(TurtleOperation),
}

// usable registers up until STACK_START, then stack after
pub const STACK_START: u8 = 100;
