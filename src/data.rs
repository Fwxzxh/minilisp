use std::collections::HashMap;
use std::fmt;

/// Represents a Lisp expression.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// A symbol, which can represent a variable or a function name.
    Symbol(String),
    /// A floating-point number.
    Number(f64),
    /// A boolean value (`true` or `false`).
    Bool(bool),
    /// A string literal.
    String(String),
    /// A list of expressions.
    List(Vec<Expr>),
    /// A user-defined function (lambda).
    Func {
        /// The names of the function's parameters.
        params: Vec<String>,
        /// The body of the function, which is another expression.
        body: Box<Expr>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Expr::Symbol(s) => s.clone(),
            Expr::Number(n) => n.to_string(),
            Expr::Bool(b) => b.to_string(),
            Expr::String(s) => format!("\"{}\"", s),
            Expr::List(list) => {
                let xs: Vec<String> = list.iter().map(|x| x.to_string()).collect();
                format!("({})", xs.join(" "))
            }
            Expr::Func { .. } => "<function>".to_string(),
        };
        write!(f, "{}", s)
    }
}

/// Represents the evaluation environment, mapping variable names to expressions.
pub type Env = HashMap<String, Expr>;
