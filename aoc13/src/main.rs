use std::{cmp::Ordering, error::Error, fmt::Display};

use parser::Expression;

mod lexer;

mod parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComparisonResult {
    InOrder,
    OutOfOrder,
    Undetermined,
}

impl ComparisonResult {
    pub fn is_determined(&self) -> bool {
        matches!(self, ComparisonResult::InOrder) || matches!(self, ComparisonResult::OutOfOrder)
    }
}

impl Display for ComparisonResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match self {
                ComparisonResult::InOrder => "packets are in the right order",
                ComparisonResult::OutOfOrder => "packets are not in the right order",
                ComparisonResult::Undetermined => "comparison result is undetermined",
            }
        ))
    }
}

fn are_in_order(left: &Expression, right: &Expression) -> ComparisonResult {
    match (left, right) {
        (Expression::Integer(left), Expression::Integer(right)) => {
            if left < right {
                return ComparisonResult::InOrder;
            }
            if left > right {
                return ComparisonResult::OutOfOrder;
            }
            ComparisonResult::Undetermined
        }
        (Expression::Integer(left), Expression::List(_)) => {
            let left = Expression::List(vec![Expression::Integer(*left)]);
            are_in_order(&left, right)
        }
        (Expression::List(_), Expression::Integer(right)) => {
            let right = Expression::List(vec![Expression::Integer(*right)]);
            are_in_order(left, &right)
        }
        (Expression::List(left), Expression::List(right)) => {
            let min_length = usize::min(left.len(), right.len());
            for i in 0..min_length {
                let result = are_in_order(&left[i], &right[i]);
                if result.is_determined() {
                    return result;
                }
            }
            if left.len() < right.len() {
                return ComparisonResult::InOrder;
            }
            if left.len() > right.len() {
                return ComparisonResult::OutOfOrder;
            }
            ComparisonResult::Undetermined
        }
    }
}

fn solve_part_1(expressions: Vec<Expression>) {
    assert!(expressions.len() % 2 == 0);
    assert!(expressions
        .iter()
        .all(|expression| matches!(expression, Expression::List(_))));
    let expressions: &[Expression] = &expressions;

    let mut sum = 0;
    for (i, [left, right]) in expressions
        .chunks(2)
        .map(|chunk| TryInto::<&[Expression; 2]>::try_into(chunk).unwrap())
        .enumerate()
    {
        println!("{left}");
        println!("{right}");
        let result = are_in_order(left, right);
        assert!(result.is_determined());
        if result == ComparisonResult::InOrder {
            sum += i + 1;
        }
        println!("{result}\n");
    }
    println!("sum of indices: {sum}");
}

fn is_divier_packet(expression: &Expression) -> bool {
    // warning! this is the most beautiful code you will ever see!
    if let Expression::List(sub_expressions) = expression {
        if sub_expressions.len() == 1 {
            if let Expression::List(sub_expressions) = &sub_expressions[0] {
                if sub_expressions.len() == 1 {
                    if let Expression::Integer(value) = sub_expressions[0] {
                        return value == 2 || value == 6;
                    }
                }
            }
        }
    }
    false
}

fn solve_part_2(mut expressions: Vec<Expression>) {
    expressions.push(Expression::List(vec![Expression::List(vec![
        Expression::Integer(2),
    ])]));
    expressions.push(Expression::List(vec![Expression::List(vec![
        Expression::Integer(6),
    ])]));
    expressions.sort_by(|left, right| {
        let result = are_in_order(left, right);
        match result {
            ComparisonResult::InOrder => Ordering::Less,
            ComparisonResult::OutOfOrder | ComparisonResult::Undetermined => Ordering::Greater,
        }
    });
    let mut product = 1;
    for (i, expression) in expressions.iter().enumerate() {
        if is_divier_packet(expression) {
            product *= i + 1;
        }
        println!("{expression}");
    }
    println!("product: {product}");
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("real_input.txt").unwrap();
    let tokens = lexer::tokenize(&input)?;
    let expressions = parser::parse(tokens)?;
    solve_part_1(expressions.clone());
    solve_part_2(expressions);
    Ok(())
}
