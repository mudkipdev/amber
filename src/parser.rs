use crate::utility;

pub const ADD: &str = "+";
pub const SUBTRACT: &str = "-";
pub const MULTIPLY: &str = "*";
pub const DIVIDE: &str = "/";

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub const ZERO: Number = Number(0);

    pub fn new(string: &str) -> Option<(&str, Self)> {
        let (string, number) = utility::extract_digits(string);

        match number.parse() {
            Ok(value) => Some((string, Self(value))),
            Err(_) => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    pub fn new(string: &str) -> Option<(&str, Self)> {
        match utility::extract_operator(string) {
            Ok((string, ADD)) => Some((string, Self::Add)),
            Ok((string, SUBTRACT)) => Some((string, Self::Subtract)),
            Ok((string, MULTIPLY)) => Some((string, Self::Multiply)),
            Ok((string, DIVIDE)) => Some((string, Self::Divide)),
            _ => None,
        }
    }

    pub fn compute(&self, left: i32, right: i32) -> Option<i32> {
        match self {
            Self::Add => left.checked_add(right),
            Self::Subtract => left.checked_sub(right),
            Self::Multiply => left.checked_mul(right),
            Self::Divide => left.checked_div(right),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub operator: Operator,
    pub left: Number,
    pub right: Number,
}

impl Expression {
    pub fn new(string: &str) -> Option<(&str, Self)> {
        Number::new(string).and_then(|(remainder, left)| {
            let (remainder, _) = utility::extract_whitespace(remainder);

            Operator::new(remainder).and_then(|(remainder, operator)| {
                let (remainder, _) = utility::extract_whitespace(remainder);

                Number::new(remainder).map(|(remainder, right)| {
                    (
                        remainder,
                        Self {
                            operator,
                            left,
                            right,
                        },
                    )
                })
            })
        })
    }

    pub fn compute(&self) -> Option<Number> {
        self.operator.compute(self.left.0, self.right.0).map(Number)
    }

    pub fn is_dividing_by_zero(&self) -> bool {
        self.operator == Operator::Divide && self.right == Number::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_with_whitespace() {
        assert_eq!(
            Expression::new("2 * 2").unwrap(),
            (
                "",
                Expression {
                    operator: Operator::Multiply,
                    left: Number(2),
                    right: Number(2),
                }
            ),
        );
    }

    fn assert_compute(string: &str, expected: Number) {
        let expression = Expression::new(string).unwrap().1;
        assert_eq!(expression.compute().unwrap(), expected);
    }

    #[test]
    fn compute_addition() {
        assert_compute("1+2", Number(3));
    }

    #[test]
    fn compute_subtraction() {
        assert_compute("1-5", Number(-4));
    }

    #[test]
    fn compute_multiplication() {
        assert_compute("3*3", Number(9));
    }

    #[test]
    fn compute_divide() {
        assert_compute("50/2", Number(25));
    }
}
