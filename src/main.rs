extern crate lalrpop_util;

#[allow(unused)]
use std::io;

pub mod parser;
pub mod ast;

// TODO: Test for invalid input.

#[test]
fn parse_strings_and_chars() {
	assert_eq!(
		&format!("{:?}", parser::parse_Comparison("\"\"").unwrap()),
		"»«");
	assert_eq!(
		&format!("{:?}", parser::parse_Comparison("\"This is a string.\"").unwrap()),
		"»This is a string.«");
	assert_eq!(
		&format!("{:?}", parser::parse_Comparison("»חלךξκλолд«").unwrap()),
		"»חלךξκλолд«");
	assert_eq!(
		&format!("{:?}", parser::parse_Comparison("'a'").unwrap()),
		"›a‹");
	assert_eq!(
		&format!("{:?}", parser::parse_Comparison("›ξ‹").unwrap()),
		"›ξ‹");
}

#[test]
fn parse_mixed_expressions() {
	assert_eq!(
		&format!("{:?}", parser::parse_Comparison("1 + 4 == 2 * 3").unwrap()),
		"Eq[Add[1, 4], Mul[2, 3]]");
	assert_eq!(
		&format!("{:?}", parser::parse_BooleanExpr("1 ≤ 2 and True").unwrap()),
		"And[Leq[1, 2], True]");
	assert_eq!(
		&format!("{:?}", parser::parse_BooleanExpr("1 + 4 == 2 * 3 or not 4 * (1 + 3) ≠ 5").unwrap()),
		"Or[Eq[Add[1, 4], Mul[2, 3]], Not[Neq[Mul[4, Add[1, 3]], 5]]]");
	
}

#[test]
fn parse_comparison() {
	assert_eq!(
		&format!("{:?}", parser::parse_Comparison("1 == 2").unwrap()),
		"Eq[1, 2]");
	assert_eq!(
		&format!("{:?}", parser::parse_Comparison("1 != 2").unwrap()),
		"Neq[1, 2]");
	assert_eq!(
		&format!("{:?}", parser::parse_Comparison("1 ≠ 2").unwrap()),
		"Neq[1, 2]");
	assert_eq!(
		&format!("{:?}", parser::parse_Comparison("1 <= 2").unwrap()),
		"Leq[1, 2]");
	assert_eq!(
		&format!("{:?}", parser::parse_Comparison("1 ≤ 2").unwrap()),
		"Leq[1, 2]");
	assert_eq!(
		&format!("{:?}", parser::parse_Comparison("1 >= 2").unwrap()),
		"Geq[1, 2]");
	assert_eq!(
		&format!("{:?}", parser::parse_Comparison("1 ≥ 2").unwrap()),
		"Geq[1, 2]");
	assert_eq!(
		&format!("{:?}", parser::parse_Comparison("1 < 2").unwrap()),
		"Lesser[1, 2]");
	assert_eq!(
		&format!("{:?}", parser::parse_Comparison("1 > 2").unwrap()),
		"Greater[1, 2]");
}

#[test]
fn parse_logic_expression() {
	assert_eq!(
		&format!("{:?}", parser::parse_BooleanExpr("True").unwrap()),
		"True");
	assert_eq!(
		&format!("{:?}", parser::parse_BooleanExpr("False").unwrap()),
		"False");
	assert_eq!(
		&format!("{:?}", parser::parse_BooleanExpr("False and True").unwrap()),
		"And[False, True]");
	assert_eq!(
		&format!("{:?}", parser::parse_BooleanExpr("False or True").unwrap()),
		"Or[False, True]");
	assert_eq!(
		&format!("{:?}", parser::parse_BooleanExpr("False and True or True").unwrap()),
		"Or[And[False, True], True]");
	assert_eq!(
		&format!("{:?}", parser::parse_BooleanExpr("False and (True or False)").unwrap()),
		"And[False, Or[True, False]]");
	assert_eq!(
		&format!("{:?}", parser::parse_BooleanExpr("False and False and False").unwrap()),
		"And[And[False, False], False]");
	assert_eq!(
		&format!("{:?}", parser::parse_BooleanExpr("not False").unwrap()),
		"Not[False]");
	assert_eq!(
		&format!("{:?}", parser::parse_BooleanExpr("True and (False or not True)").unwrap()),
		"And[True, Or[False, Not[True]]]");
}

#[test]
fn parse_arithmetic_expr() {
	assert_eq!(
		&format!("{:?}", parser::parse_ArithmeticExpr("22").unwrap()),
		"22");
	assert_eq!(
		&format!("{:?}", parser::parse_ArithmeticExpr("22 * 44").unwrap()),
		"Mul[22, 44]");
	assert_eq!(
		&format!("{:?}", parser::parse_ArithmeticExpr("44 + 66").unwrap()),
		"Add[44, 66]");
	assert_eq!(
		&format!("{:?}", parser::parse_ArithmeticExpr("22 * 44 + 66").unwrap()),
		"Add[Mul[22, 44], 66]");
	assert_eq!(
		&format!("{:?}", parser::parse_ArithmeticExpr("22 * (44 + 66)").unwrap()),
		"Mul[22, Add[44, 66]]");
	assert_eq!(
		&format!("{:?}", parser::parse_ArithmeticExpr("22 * 44 * 66").unwrap()),
		"Mul[Mul[22, 44], 66]");
}

#[cfg(not(test))]
fn main() {
    println!("Welcome to moses version {}.", env!("CARGO_PKG_VERSION"));
	loop {
		print!("> ");
		let mut input = String::new();
		io::stdin().read_line(&mut input);
		let parsed = &format!("{:?}", parser::parse_Expr(&input).unwrap());
		let evaluated = ast_eval::parse_Expr(&*parsed).unwrap();
		println!("{}", evaluated);
	}
}