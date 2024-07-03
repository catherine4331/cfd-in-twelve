use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

const ZERO: Expression = Expression::Const(0);
const ONE: Expression = Expression::Const(1);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    Const(i64),
    Variable(String),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Pow(Box<Expression>, Box<Expression>),
    Neg(Box<Expression>),
    Exp(Box<Expression>),
}

impl Expression {
    pub fn pow(self, index: Self) -> Self {
        Expression::Pow(Box::new(self), Box::new(index))
    }

    pub fn diff(self, respect_to: &String) -> Self {
        match self {
            Expression::Const(_) => ZERO,
            Expression::Variable(var) => {
                if var == *respect_to {
                    ONE
                } else {
                    ZERO
                }
            }
            Expression::Add(lhs, rhs) => lhs.diff(respect_to) + rhs.diff(respect_to),
            Expression::Sub(lhs, rhs) => lhs.diff(respect_to) - rhs.diff(respect_to),
            Expression::Mul(f, g) => {
                f.clone().diff(respect_to) * *g.clone() + *f * g.diff(respect_to)
            }
            Expression::Div(f, g) => {
                (*g.clone() * f.clone().diff(respect_to) - *f * g.clone().diff(respect_to))
                    / (g.pow(Expression::Const(2)))
            }
            Expression::Pow(expr, index) => {
                expr.clone().diff(respect_to)
                    * *index.clone()
                    * Expression::Pow(expr, Box::new(*index - ONE))
            }
            Expression::Neg(expr) => Expression::Neg(Box::new(expr.diff(respect_to))),
            Expression::Exp(ref expr) => expr.clone().diff(respect_to) * self,
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Const(value) => write!(f, "{value}"),
            Expression::Variable(var) => write!(f, "{var}"),
            Expression::Add(lhs, rhs) => write!(f, "({} + {})", lhs, rhs),
            Expression::Sub(lhs, rhs) => write!(f, "({} - {})", lhs, rhs),
            Expression::Mul(lhs, rhs) => write!(f, "{}*{}", lhs, rhs),
            Expression::Div(numerator, denominator) => write!(f, "{}/{}", numerator, denominator),
            Expression::Pow(expr, index) => write!(f, "({})**{}", expr, index),
            Expression::Neg(expr) => write!(f, "-{}", expr),
            Expression::Exp(expr) => write!(f, "exp({})", expr),
        }
    }
}

impl Add for Expression {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Expression::Const(lhs) => {
                if lhs == 0 {
                    return rhs;
                } else if let Expression::Const(rhs) = rhs {
                    return Expression::Const(lhs + rhs);
                }
            }
            _ => {
                if let Expression::Const(rhs) = rhs {
                    if rhs == 0 {
                        return self;
                    }
                }
            }
        }

        Expression::Add(Box::new(self), Box::new(rhs))
    }
}

impl Sub for Expression {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Expression::Const(lhs) => {
                if lhs == 0 {
                    return rhs;
                } else if let Expression::Const(rhs) = rhs {
                    return Expression::Const(lhs - rhs);
                }
            }
            _ => {
                if let Expression::Const(rhs) = rhs {
                    if rhs == 0 {
                        return self;
                    }
                }
            }
        }

        Expression::Sub(Box::new(self), Box::new(rhs))
    }
}

impl Mul for Expression {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Expression::Const(lhs) => {
                if lhs == 0 {
                    return Expression::Const(0);
                } else if lhs == 1 {
                    return rhs;
                } else if let Expression::Const(rhs) = rhs {
                    return Expression::Const(lhs * rhs);
                }
            }
            _ => {
                if let Expression::Const(rhs) = rhs {
                    if rhs == 0 {
                        return Expression::Const(0);
                    } else if rhs == 1 {
                        return self;
                    }
                }
            }
        }

        Expression::Sub(Box::new(self), Box::new(rhs))
    }
}

impl Div for Expression {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Expression::Div(Box::new(self), Box::new(rhs))
    }
}

#[cfg(test)]
mod tests {
    use super::{Expression, ZERO};

    #[test]
    pub fn add_zero_lhs() {
        let expr = ZERO + Expression::Variable("x".to_string());

        assert_eq!(expr, Expression::Variable("x".to_string()));
    }

    #[test]
    pub fn add_zero_rhs() {
        let expr = Expression::Variable("x".to_string()) + ZERO;

        assert_eq!(expr, Expression::Variable("x".to_string()));
    }

    #[test]
    pub fn add_two_consts() {
        let expr = Expression::Const(15) + Expression::Const(5);

        assert_eq!(expr, Expression::Const(20));
    }

    #[test]
    pub fn sub_zero_lhs() {
        let expr = ZERO - Expression::Variable("x".to_string());

        assert_eq!(expr, Expression::Variable("x".to_string()));
    }

    #[test]
    pub fn sub_zero_rhs() {
        let expr = Expression::Variable("x".to_string()) - ZERO;

        assert_eq!(expr, Expression::Variable("x".to_string()));
    }

    #[test]
    pub fn sub_two_consts() {
        let expr = Expression::Const(15) - Expression::Const(5);

        assert_eq!(expr, Expression::Const(10));
    }

    #[test]
    pub fn basic_power() {
        let expr = Expression::Variable("x".to_string()).pow(Expression::Const(3));

        let diff_expr = expr.diff(&"x".to_string());

        assert_eq!(
            diff_expr,
            Expression::Const(3) * Expression::Variable("x".to_string()).pow(Expression::Const(2))
        );
    }

    #[test]
    pub fn composite_exp() {
        let expr = Expression::Exp(Box::new(
            Expression::Variable("x".to_string()) + Expression::Const(3),
        ));

        let diff_expr = expr.diff(&"x".to_string());

        assert_eq!(
            diff_expr,
            Expression::Exp(Box::new(
                Expression::Variable("x".to_string()) + Expression::Const(3)
            ))
        );
    }
}
