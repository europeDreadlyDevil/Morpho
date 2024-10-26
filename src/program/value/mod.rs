use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};
use crate::ast::{CallExpr, Expr};
use crate::program::environment::LocalEnvironment;
use crate::program::function::Function;

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    String(String),
    Int(i64),
    Bool(bool),
    FuncPtr(fn(Vec<Value>, &mut LocalEnvironment) -> Value),
    Func(Function),
    CallFunc(CallExpr),
    Type(String),
    Range(i64, i64),
    Counter(String, i64, i64),
    Eq(Box<Expr>, Box<Expr>),
    None,
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
            _ => Value::None
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
            _ => Value::None
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
            _ => Value::None
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
            _ => Value::None
        }
    }
}

impl Value {
    pub(crate) fn into_type(self) -> Value {
        match self {
            Value::String(_) => Value::Type("string".into()),
            Value::Int(_) => Value::Type("int".into()),
            Value::FuncPtr(func) => Value::Type(format!("{:?}", func)),
            Value::Func(Function { rty, .. }) => Value::Type(rty),
            Value::Type(ty) => Value::Type(ty),
            Value::None => Value::Type("none".into()),
            Value::Bool(_) => Value::Type("bool".into()),
            Value::CallFunc { .. } => Value::Type("func".into()),
            Value::Range(s, e) => Value::Type(format!("range<{}, {}>", s, e)),
            Value::Counter(ident, s, e) => Value::Type(format!("counter<{}, {}, {}>", ident, s, e)),
            Value::Eq(_, _) => Value::Type("bool".into())
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "{s}"),
            Value::Int(i) => write!(f, "{i}"),
            Value::Func(func) => write!(f, "{}", func.get_ident()),
            Value::None => write!(f, "None"),
            Value::FuncPtr(func_ptr) => write!(f, "{:?}", func_ptr),
            Value::Type(ty) => write!(f, "{}", ty),
            Value::Bool(b) => write!(f, "{}", b),
            Value::CallFunc (call) => write!(f, "{}", call.get_name()),
            Value::Range(s, e) => write!(f, "range<{}, {}>", s, e),
            Value::Counter(ident, s, e) => write!(f, "counter<{}, {}, {}>", ident, s, e),
            Value::Eq(a, b) => write!(f, "{}", a==b )
        }
    }
}