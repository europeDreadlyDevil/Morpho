use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Clone)]
pub struct Prog(pub Vec<Stmt>);

#[derive(Debug, Clone, PartialOrd)]
pub enum Expr {
    Ident(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    StringLit(String),
    Ref(Box<Expr>),
    Array(Vec<Expr>),
    Dictionary(Vec<(Expr, Expr)>),
    Call(CallExpr),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    NotEq(Box<Expr>, Box<Expr>),
    Func(FuncPtr),
    Counter((String, (i64, i64))),
    Range((i64, i64)),
    AnonFunc(AnonymousFunc),
    Gt(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Ge(Box<Expr>, Box<Expr>),
    Le(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Neg(Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::Ident(a), Expr::Ident(b)) => a == b,
            (Expr::Integer(a), Expr::Integer(b)) => a == b,
            (Expr::Float(a), Expr::Float(b)) => {
                const EPSILON: f64 = 1e-10;
                (a - b).abs() < EPSILON
            }
            (Expr::Bool(a), Expr::Bool(b)) => a == b,
            (Expr::StringLit(a), Expr::StringLit(b)) => a == b,
            (Expr::Array(a), Expr::Array(b)) => a == b,
            (Expr::Dictionary(a), Expr::Dictionary(b)) => a == b,
            (Expr::Call(a), Expr::Call(b)) => a == b,
            (Expr::Add(a, b), Expr::Add(c, d)) => a == c && b == d,
            (Expr::Sub(a, b), Expr::Sub(c, d)) => a == c && b == d,
            (Expr::Mul(a, b), Expr::Mul(c, d)) => a == c && b == d,
            (Expr::Div(a, b), Expr::Div(c, d)) => a == c && b == d,
            (Expr::Eq(a, b), Expr::Eq(c, d)) => a == c && b == d,
            (Expr::NotEq(a, b), Expr::NotEq(c, d)) => a == c && b == d,
            (Expr::Func(a), Expr::Func(b)) => a == b,
            (Expr::Counter((s1, (low1, high1))), Expr::Counter((s2, (low2, high2)))) => {
                s1 == s2 && low1 == low2 && high1 == high2
            }
            (Expr::Range((start1, end1)), Expr::Range((start2, end2))) => {
                start1 == start2 && end1 == end2
            }
            (Expr::AnonFunc(a), Expr::AnonFunc(b)) => a == b,
            (Expr::Gt(a1, b1), Expr::Gt(a2, b2)) => a1 == a2 && b1 == b2,
            (Expr::Lt(a1, b1), Expr::Lt(a2, b2)) => a1 == a2 && b1 == b2,
            (Expr::Ge(a1, b1), Expr::Ge(a2, b2)) => a1 == a2 && b1 == b2,
            (Expr::Le(a1, b1), Expr::Le(a2, b2)) => a1 == a2 && b1 == b2,
            (Expr::Or(a1, b1), Expr::Or(a2, b2)) => a1 == a2 && b1 == b2,
            (Expr::And(a1, b1), Expr::And(a2, b2)) => a1 == a2 && b1 == b2,
            (Expr::Not(a), Expr::Not(b)) => a == b,
            (Expr::Neg(a), Expr::Neg(b)) => a == b,
            (Expr::Xor(a1,b1 ), Expr::Xor(a2, b2)) => a1 == a2 && b1 == b2,
            (Expr::Mod(a, b), Expr::Mod(c, d)) => a == c && b == d,
            _ => false,
        }
    }
}

// Implementing the Eq trait
impl Eq for Expr {}

impl Hash for Expr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Expr::Ident(ref s) => {
                state.write_u8(0);
                s.hash(state);
            }
            Expr::Integer(ref i) => {
                state.write_u8(1);
                i.hash(state);
            }
            Expr::Float(ref f) => {
                state.write_u8(2);
                f.to_bits().hash(state);
            }
            Expr::Bool(ref b) => {
                state.write_u8(3);
                b.hash(state);
            }
            Expr::StringLit(ref s) => {
                state.write_u8(4);
                s.hash(state);
            }
            Expr::Ref(ref expr) => {
                state.write_u8(5);
                expr.hash(state);
            }
            Expr::Array(ref arr) => {
                state.write_u8(6);
                arr.hash(state);
            }
            Expr::Dictionary(ref dict) => {
                state.write_u8(7);
                dict.hash(state);
            }
            Expr::Call(ref call) => {
                state.write_u8(8);
                call.hash(state);
            }
            Expr::Add(ref lhs, ref rhs) => {
                state.write_u8(9);
                lhs.hash(state);
                rhs.hash(state);
            }
            Expr::Sub(ref lhs, ref rhs) => {
                state.write_u8(10);
                lhs.hash(state);
                rhs.hash(state);
            }
            Expr::Mul(ref lhs, ref rhs) => {
                state.write_u8(11);
                lhs.hash(state);
                rhs.hash(state);
            }
            Expr::Div(ref lhs, ref rhs) => {
                state.write_u8(12);
                lhs.hash(state);
                rhs.hash(state);
            }
            Expr::Eq(ref lhs, ref rhs) => {
                state.write_u8(13);
                lhs.hash(state);
                rhs.hash(state);
            }
            Expr::NotEq(ref lhs, ref rhs) => {
                state.write_u8(14);
                lhs.hash(state);
                rhs.hash(state);
            }
            Expr::Func(ref func) => {
                state.write_u8(15);
                func.hash(state);
            }
            Expr::Counter((ref s, (ref low, ref high))) => {
                state.write_u8(16);
                s.hash(state);
                low.hash(state);
                high.hash(state);
            }
            Expr::Range((ref start, ref end)) => {
                state.write_u8(17);
                start.hash(state);
                end.hash(state);
            }
            Expr::AnonFunc(ref func) => {
                state.write_u8(18);
                func.hash(state);
            }
            Expr::Gt(ref lhs, ref rhs) => {
                state.write_u8(19);
                lhs.hash(state);
                rhs.hash(state);
            }
            Expr::Lt(ref lhs, ref rhs) => {
                state.write_u8(20);
                lhs.hash(state);
                rhs.hash(state);
            }
            Expr::Ge(ref lhs, ref rhs) => {
                state.write_u8(21);
                lhs.hash(state);
                rhs.hash(state);
            }
            Expr::Le(ref lhs, ref rhs) => {
                state.write_u8(22);
                lhs.hash(state);
                rhs.hash(state);
            }
            Expr::Or(ref lhs, ref rhs) => {
                state.write_u8(23);
                lhs.hash(state);
                rhs.hash(state);
            }
            Expr::And(ref lhs, ref rhs) => {
                state.write_u8(24);
                lhs.hash(state);
                rhs.hash(state);
            }
            Expr::Not(ref expr) => {
                state.write_u8(25);
                expr.hash(state);
            }
            Expr::Neg(ref expr) => {
                state.write_u8(26);
                expr.hash(state);
            }
            Expr::Xor(ref lhs, ref rhs) => {
                state.write_u8(27);
                lhs.hash(state);
                rhs.hash(state);
            }
            Expr::Mod(ref lhs, ref rhs) => {
                state.write_u8(28);
                lhs.hash(state);
                rhs.hash(state);
            }
        }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Hash)]
pub struct FuncPtr {
    pub ident: String,
    pub args: Option<Vec<Expr>>,
}

impl FuncPtr {
    pub fn new(ident: &str, args: Option<Vec<Expr>>) -> Self {
        Self {
            ident: ident.into(),
            args,
        }
    }
}
#[derive(PartialEq, Debug, Clone, PartialOrd, Hash)]
pub struct CallExpr {
    func_name: String,
    args: Vec<Expr>,
}

impl CallExpr {
    pub fn new(func_name: String, args: Vec<Expr>) -> Self {
        CallExpr { func_name, args }
    }
    pub fn get_name(&self) -> String {
        self.func_name.clone()
    }
    pub fn get_args(&self) -> Vec<Expr> {
        self.args.clone()
    }
}

#[derive(PartialEq, Debug, Clone, PartialOrd, Hash, Eq)]
pub enum Stmt {
    FuncIdent(FuncIdent),
    FuncBody(FuncBody),
    VarIdent(VarIdent),
    VarAssign(VarAssign),
    ReturnValue(Box<Expr>),
    Expr(Box<Expr>),
}

#[derive(PartialEq, Debug, Clone, PartialOrd, Hash, Eq)]
pub struct VarAssign {
    pub ident: String,
    pub expr: Expr,
}

impl VarAssign {
    pub fn new(ident: String, expr: Expr) -> Self {
        Self { ident, expr }
    }
}

#[derive(PartialEq, Debug, Clone, PartialOrd, Hash, Eq)]
pub struct AnonymousFunc {
    pub args: Vec<(String, Expr)>,
    pub rty: String,
    pub stmt: Option<FuncBody>,
}

impl AnonymousFunc {
    pub fn new_w_rty(args: Vec<(String, Expr)>, rty: String, stmt: Option<FuncBody>) -> Self {
        Self { args, stmt, rty }
    }

    pub fn new_wo_rty(args: Vec<(String, Expr)>, stmt: Option<FuncBody>) -> Self {
        Self {
            args,
            stmt,
            rty: "void".into(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, PartialOrd, Hash, Eq)]
pub struct FuncIdent {
    pub ident: String,
    pub args: Vec<(String, String)>,
    pub rty: String,
    pub stmt: Option<FuncBody>,
}

impl FuncIdent {
    pub fn new_w_rty(
        ident: &str,
        args: Vec<(String, String)>,
        rty: String,
        stmt: Option<FuncBody>,
    ) -> Self {
        Self {
            ident: ident.into(),
            args,
            stmt,
            rty,
        }
    }

    pub fn new_wo_rty(ident: &str, args: Vec<(String, String)>, stmt: Option<FuncBody>) -> Self {
        Self {
            ident: ident.into(),
            args,
            stmt,
            rty: "void".into(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, PartialOrd, Hash, Eq)]
pub struct FuncBody {
    pub stmt: Vec<Stmt>,
}

impl FuncBody {
    pub fn new(stmt: Vec<Stmt>) -> Self {
        Self { stmt }
    }
}

#[derive(PartialEq, Debug, Clone, PartialOrd, Hash, Eq)]
pub struct VarIdent {
    pub ident: String,
    pub expr: Expr,
}

impl VarIdent {
    pub fn new(ident: &str, expr: Expr) -> Self {
        Self {
            ident: ident.into(),
            expr,
        }
    }
}
