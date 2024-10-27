#[derive(Debug, PartialEq, Clone)]
pub struct Prog(pub Vec<Stmt>);

#[derive(Debug, Clone)]
pub enum Expr {
    Ident(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    StringLit(String),
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
    AnonFunc(AnonymousFunc)
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
            (Expr::Add(a, b), Expr::Add(c,d)) => a == c && b == d,
            (Expr::Sub(a, b), Expr::Sub(c,d)) => a == c && b == d,
            (Expr::Mul(a, b), Expr::Mul(c,d)) => a == c && b == d,
            (Expr::Div(a, b), Expr::Div(c,d)) => a == c && b == d,
            (Expr::Eq(a, b), Expr::Eq(c,d)) => a == c && b == d,
            (Expr::NotEq(a, b), Expr::NotEq(c,d)) => a == c && b == d,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FuncPtr {
    pub ident: String,
    pub args: Option<Vec<Expr>>
}

impl FuncPtr {
    pub fn new(ident: &str, args: Option<Vec<Expr>>) -> Self {
        Self {
            ident: ident.into(),
            args,
        }
    }
}
#[derive(PartialEq, Debug, Clone)]
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

#[derive(PartialEq, Debug, Clone)]
pub enum Stmt {
    FuncIdent(FuncIdent),
    FuncBody(FuncBody),
    VarIdent(VarIdent),
    Expr(Box<Expr>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct AnonymousFunc {
    pub args: Vec<(String, Expr)>,
    pub rty: String,
    pub stmt: Option<FuncBody>,
}

impl AnonymousFunc {
    pub fn new_w_rty(
        args: Vec<(String,Expr)>,
        rty: String,
        stmt: Option<FuncBody>,
    ) -> Self {
        Self {
            args,
            stmt,
            rty,
        }
    }

    pub fn new_wo_rty(args: Vec<(String,Expr)>, stmt: Option<FuncBody>) -> Self {
        Self {
            args,
            stmt,
            rty: "void".into(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
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

#[derive(PartialEq, Debug, Clone)]
pub struct FuncBody {
    pub stmt: Vec<Stmt>,
}

impl FuncBody {
    pub fn new(stmt: Vec<Stmt>) -> Self {
        Self { stmt }
    }
}

#[derive(PartialEq, Debug, Clone)]
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
