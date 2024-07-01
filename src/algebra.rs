use std::ops::{Add, Div, Mul, Sub};

pub enum Expression {
    Const(i64),
    Symbol(String),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Pow(Box<Expression>, Box<Expression>),
    Neg(Box<Expression>),
    Exp(Box<Expression>),
}

impl Expression {
    pub fn Pow(self, index: Self) -> Self {
        Expression::Pow(Box::new(self), Box::new(index))
    }
}

impl Add for Expression {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Expression::Add(Box::new(self), Box::new(rhs))
    }
}

impl Sub for Expression {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Expression::Sub(Box::new(self), Box::new(rhs))
    }
}

impl Mul for Expression {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Expression::Mul(Box::new(self), Box::new(rhs))
    }
}

impl Div for Expression {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Expression::Div(Box::new(self), Box::new(rhs))
    }
}
