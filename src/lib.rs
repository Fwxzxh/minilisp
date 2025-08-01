//! A simple Lisp interpreter written in Rust.
//!
//! This library provides a parser and an evaluator for a small subset of the Lisp language.
//! It supports basic arithmetic, variables, functions (lambdas), and conditional logic.

pub mod data;
pub mod eval;
pub mod parser;

pub use data::{Env, Expr};
pub use eval::eval;
pub use parser::parse;
