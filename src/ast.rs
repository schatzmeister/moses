use std::fmt::{Debug, Formatter, Error};

pub enum Expr {
    Number(i32),
	True,
	False,
	Str(String),
	Char(char),
    Op(Opcode, Vec<Box<Expr>>),
}

#[derive(Copy, Clone, Debug)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
	And,
	Or,
	Not,
	Eq,
	Neq,
	Greater,
	Lesser,
	Geq,
	Leq,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
			True => write!(fmt, "True"),
			False => write!(fmt, "False"),
			Str(ref s) => write!(fmt, "»{:?}‹«", s),
			Char(c) => write!(fmt, "›{:?}‹", c),
            Op(op, ref operands) => write!(fmt, "{:?}{:?}", op, operands),
        }
    }
}