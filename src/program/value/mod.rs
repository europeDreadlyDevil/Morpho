use crate::ast::{CallExpr, Expr};
use crate::program::environment::LocalEnvironment;
use crate::program::evaluating_functions::eval_expr;
use crate::program::function::Function;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Add, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Sub};
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub enum CondType {
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
    Or,
    And,
}

impl CondType {
    pub(crate) fn eval_cond(
        &self,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        env: Arc<RwLock<LocalEnvironment>>,
    ) -> bool {
        match self {
            CondType::Eq => eval_expr(*lhs, env.clone()) == eval_expr(*rhs, env.clone()),
            CondType::Ne => eval_expr(*lhs, env.clone()) != eval_expr(*rhs, env.clone()),
            CondType::Gt => eval_expr(*lhs, env.clone()) > eval_expr(*rhs, env.clone()),
            CondType::Lt => eval_expr(*lhs, env.clone()) < eval_expr(*rhs, env.clone()),
            CondType::Ge => eval_expr(*lhs, env.clone()) >= eval_expr(*rhs, env.clone()),
            CondType::Le => eval_expr(*lhs, env.clone()) <= eval_expr(*rhs, env.clone()),
            CondType::Or => {
                if let Value::Bool(b) =
                    eval_expr(*lhs, env.clone()).logical_or(eval_expr(*rhs, env.clone()), env)
                {
                    b
                } else {
                    panic!("Expected bool type")
                }
            }
            CondType::And => {
                if let Value::Bool(b) =
                    eval_expr(*lhs, env.clone()).logical_and(eval_expr(*rhs, env.clone()))
                {
                    b
                } else {
                    panic!("Expected bool type")
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    String(String),
    Int(i64),
    Bool(bool),
    Float(f64),
    FuncPtr(fn(Vec<Value>, Arc<RwLock<LocalEnvironment>>) -> Value),
    RefValue(Arc<RwLock<Value>>),
    Func(Function),
    CallFunc(CallExpr),
    Type(String),
    Range(i64, i64),
    Counter(String, i64, i64),
    Cond(CondType, Box<Expr>, Box<Expr>),
    None,
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Value::Int(i) => Value::Int(i.neg()),
            Value::RefValue(r) => {
                let mut value = r.try_write().unwrap();
                *value = -value.clone();
                Value::None
            }
            _ => Value::None,
        }
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Value::Int(i) => Value::Int(!i),
            Value::Bool(b) => Value::Bool(!b),
            Value::RefValue(r) => {
                let mut value = r.try_write().unwrap();
                *value = !value.clone();
                Value::None
            }
            _ => Value::None,
        }
    }
}

macro_rules! impl_partial_ord {
    ($a: expr, $b: expr) => {
        if $a > $b {
            Some(Ordering::Greater)
        } else if $a < $b {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    };
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => impl_partial_ord!(a, b),
            (Value::Bool(a), Value::Bool(b)) => impl_partial_ord!(a, b),
            (_, _) => None,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::FuncPtr(a), Value::FuncPtr(b)) => a == b,
            (Value::Type(a), Value::Type(b)) => a == b,
            (Value::Func(a), Value::Func(b)) => a.rty == b.rty,
            (_, _) => false,
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::RefValue(a), Value::RefValue(b)) => {
                match (a.try_read().unwrap().clone(), b.try_read().unwrap().clone()) {
                    (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
                    _ => Value::None,
                }
            }
            (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
            (Value::RefValue(a), Value::Int(b)) => match a.try_read().unwrap().clone() {
                Value::Int(a) => Value::Int(a + b),
                _ => Value::None,
            },
            (Value::Int(a), Value::RefValue(b)) => match b.try_read().unwrap().clone() {
                Value::Int(b) => Value::Int(a + b),
                _ => Value::None,
            },
            (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
            (Value::RefValue(a), Value::Float(b)) => match a.try_read().unwrap().clone() {
                Value::Float(a) => Value::Float(a + b),
                _ => Value::None,
            },
            (Value::Float(a), Value::RefValue(b)) => match b.try_read().unwrap().clone() {
                Value::Float(b) => Value::Float(a + b),
                _ => Value::None,
            },
            _ => Value::None,
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::RefValue(a), Value::RefValue(b)) => {
                match (a.try_read().unwrap().clone(), b.try_read().unwrap().clone()) {
                    (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
                    _ => Value::None,
                }
            }
            (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
            (Value::RefValue(a), Value::Int(b)) => match a.try_read().unwrap().clone() {
                Value::Int(a) => Value::Int(a - b),
                _ => Value::None,
            },
            (Value::Int(a), Value::RefValue(b)) => match b.try_read().unwrap().clone() {
                Value::Int(b) => Value::Int(a - b),
                _ => Value::None,
            },
            (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
            (Value::RefValue(a), Value::Float(b)) => match a.try_read().unwrap().clone() {
                Value::Float(a) => Value::Float(a - b),
                _ => Value::None,
            },
            (Value::Float(a), Value::RefValue(b)) => match b.try_read().unwrap().clone() {
                Value::Float(b) => Value::Float(a - b),
                _ => Value::None,
            },
            _ => Value::None,
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
            (Value::RefValue(a), Value::RefValue(b)) => {
                match (a.try_read().unwrap().clone(), b.try_read().unwrap().clone()) {
                    (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
                    _ => Value::None,
                }
            }
            (Value::RefValue(a), Value::Int(b)) => match a.try_read().unwrap().clone() {
                Value::Int(a) => Value::Int(a * b),
                _ => Value::None,
            },
            (Value::Int(a), Value::RefValue(b)) => match b.try_read().unwrap().clone() {
                Value::Int(b) => Value::Int(a * b),
                _ => Value::None,
            },
            (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
            (Value::RefValue(a), Value::Float(b)) => match a.try_read().unwrap().clone() {
                Value::Float(a) => Value::Float(a * b),
                _ => Value::None,
            },
            (Value::Float(a), Value::RefValue(b)) => match b.try_read().unwrap().clone() {
                Value::Float(b) => Value::Float(a * b),
                _ => Value::None,
            },
            _ => Value::None,
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
            (Value::RefValue(a), Value::RefValue(b)) => {
                match (a.try_read().unwrap().clone(), b.try_read().unwrap().clone()) {
                    (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
                    _ => Value::None,
                }
            }
            (Value::RefValue(a), Value::Int(b)) => match a.try_read().unwrap().clone() {
                Value::Int(a) => Value::Int(a / b),
                _ => Value::None,
            },
            (Value::Int(a), Value::RefValue(b)) => match b.try_read().unwrap().clone() {
                Value::Int(b) => Value::Int(a / b),
                _ => Value::None,
            },
            (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
            (Value::RefValue(a), Value::Float(b)) => match a.try_read().unwrap().clone() {
                Value::Float(a) => Value::Float(a / b),
                _ => Value::None,
            },
            (Value::Float(a), Value::RefValue(b)) => match b.try_read().unwrap().clone() {
                Value::Float(b) => Value::Float(a / b),
                _ => Value::None,
            },
            _ => Value::None,
        }
    }
}

impl Rem for Value {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a % b),
            (Value::RefValue(a), Value::RefValue(b)) => {
                match (a.try_read().unwrap().clone(), b.try_read().unwrap().clone()) {
                    (Value::Int(a), Value::Int(b)) => Value::Int(a % b),
                    _ => Value::None,
                }
            }
            (Value::RefValue(a), Value::Int(b)) => match a.try_read().unwrap().clone() {
                Value::Int(a) => Value::Int(a % b),
                _ => Value::None,
            },
            (Value::Int(a), Value::RefValue(b)) => match b.try_read().unwrap().clone() {
                Value::Int(b) => Value::Int(a % b),
                _ => Value::None,
            },
            (Value::Float(a), Value::Float(b)) => Value::Float(a % b),
            (Value::RefValue(a), Value::Float(b)) => match a.try_read().unwrap().clone() {
                Value::Float(a) => Value::Float(a % b),
                _ => Value::None,
            },
            (Value::Float(a), Value::RefValue(b)) => match b.try_read().unwrap().clone() {
                Value::Float(b) => Value::Float(a % b),
                _ => Value::None,
            },
            _ => Value::None,
        }
    }
}

impl BitXor for Value {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a ^ b),
            (Value::RefValue(a), Value::RefValue(b)) => {
                match (a.try_read().unwrap().clone(), b.try_read().unwrap().clone()) {
                    (Value::Int(a), Value::Int(b)) => Value::Int(a ^ b),
                    _ => Value::None,
                }
            }
            (Value::RefValue(a), Value::Int(b)) => match a.try_read().unwrap().clone() {
                Value::Int(a) => Value::Int(a ^ b),
                _ => Value::None,
            },
            (Value::Int(a), Value::RefValue(b)) => match b.try_read().unwrap().clone() {
                Value::Int(b) => Value::Int(a ^ b),
                _ => Value::None,
            },
            _ => Value::None,
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
            Value::Cond(_, _, _) => Value::Type("bool".into()),
            Value::RefValue(r) => r.try_read().unwrap().clone().into_type(),
            Value::Float(_) => Value::Type("float".into()),
        }
    }

    pub fn logical_or(self, other: Value, env: Arc<RwLock<LocalEnvironment>>) -> Value {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a || b),
            (Value::RefValue(a), Value::Bool(b)) => match a.try_read().unwrap().clone() {
                Value::Bool(a) => Value::Bool(a || b),
                _ => Value::None,
            },
            (Value::Bool(a), Value::RefValue(b)) => match b.try_read().unwrap().clone() {
                Value::Bool(b) => Value::Bool(a || b),
                _ => Value::None,
            },
            (Value::RefValue(a), Value::RefValue(b)) => {
                match (a.try_read().unwrap().clone(), b.try_read().unwrap().clone()) {
                    (Value::Bool(a), Value::Bool(b)) => Value::Bool(a || b),
                    _ => Value::None,
                }
            }
            (Value::Cond(ty1, l1, r1), Value::Cond(ty2, l2, r2)) => Value::Bool(
                ty1.eval_cond(l1, r1, env.clone()) || ty2.eval_cond(l2, r2, env.clone()),
            ),
            (_, _) => panic!("Expected bool type"),
        }
    }
    pub fn logical_and(self, other: Value) -> Value {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a && b),
            (Value::RefValue(a), Value::Bool(b)) => match a.try_read().unwrap().clone() {
                Value::Bool(a) => Value::Bool(a && b),
                _ => Value::None,
            },
            (Value::Bool(a), Value::RefValue(b)) => match b.try_read().unwrap().clone() {
                Value::Bool(b) => Value::Bool(a && b),
                _ => Value::None,
            },
            (Value::RefValue(a), Value::RefValue(b)) => {
                match (a.try_read().unwrap().clone(), b.try_read().unwrap().clone()) {
                    (Value::Bool(a), Value::Bool(b)) => Value::Bool(a && b),
                    _ => Value::None,
                }
            }
            (_, _) => panic!("Expected bool type"),
        }
    }
    pub fn print(&self, env: Arc<RwLock<LocalEnvironment>>) {
        match self {
            Value::Cond(cond_ty, a, b) => match cond_ty {
                CondType::Eq => print!(
                    "{}",
                    eval_expr(*a.clone(), env.clone()) == eval_expr(*b.clone(), env)
                ),
                CondType::Ne => print!(
                    "{}",
                    eval_expr(*a.clone(), env.clone()) != eval_expr(*b.clone(), env)
                ),
                CondType::Gt => print!(
                    "{}",
                    eval_expr(*a.clone(), env.clone()) > eval_expr(*b.clone(), env)
                ),
                CondType::Lt => print!(
                    "{}",
                    eval_expr(*a.clone(), env.clone()) < eval_expr(*b.clone(), env)
                ),
                CondType::Ge => print!(
                    "{}",
                    eval_expr(*a.clone(), env.clone()) >= eval_expr(*b.clone(), env)
                ),
                CondType::Le => print!(
                    "{}",
                    eval_expr(*a.clone(), env.clone()) <= eval_expr(*b.clone(), env)
                ),
                CondType::Or => print!(
                    "{}",
                    eval_expr(*a.clone(), env.clone())
                        .logical_or(eval_expr(*b.clone(), env.clone()), env)
                ),
                CondType::And => print!(
                    "{}",
                    eval_expr(*a.clone(), env.clone()).logical_and(eval_expr(*b.clone(), env))
                ),
            },
            _ => print!("{self}"),
        }
    }
    pub fn println(&self, env: Arc<RwLock<LocalEnvironment>>) {
        match self {
            Value::Cond(cond_ty, a, b) => match cond_ty {
                CondType::Eq => println!(
                    "{}",
                    eval_expr(*a.clone(), env.clone()) == eval_expr(*b.clone(), env)
                ),
                CondType::Ne => println!(
                    "{}",
                    eval_expr(*a.clone(), env.clone()) != eval_expr(*b.clone(), env)
                ),
                CondType::Gt => println!(
                    "{}",
                    eval_expr(*a.clone(), env.clone()) > eval_expr(*b.clone(), env)
                ),
                CondType::Lt => println!(
                    "{}",
                    eval_expr(*a.clone(), env.clone()) < eval_expr(*b.clone(), env)
                ),
                CondType::Ge => println!(
                    "{}",
                    eval_expr(*a.clone(), env.clone()) >= eval_expr(*b.clone(), env)
                ),
                CondType::Le => println!(
                    "{}",
                    eval_expr(*a.clone(), env.clone()) <= eval_expr(*b.clone(), env)
                ),
                CondType::Or => println!(
                    "{}",
                    eval_expr(*a.clone(), env.clone())
                        .logical_or(eval_expr(*b.clone(), env.clone()), env)
                ),
                CondType::And => println!(
                    "{}",
                    eval_expr(*a.clone(), env.clone()).logical_and(eval_expr(*b.clone(), env))
                ),
            },
            _ => println!("{self}"),
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
            Value::CallFunc(call) => write!(f, "{}", call.get_name()),
            Value::Range(s, e) => write!(f, "range<{}, {}>", s, e),
            Value::Counter(ident, s, e) => write!(f, "counter<{}, {}, {}>", ident, s, e),
            Value::RefValue(r) => write!(f, "{:?}", r),
            Value::Float(f64) => write!(f, "{}", f64),
            _ => write!(f, "Not printable"),
        }
    }
}
