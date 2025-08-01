use minilisp_rust::{Env, Expr, eval, parse};
use std::collections::HashMap;

// --- Helper functions for tests ---

fn run_eval_test(input: &str, expected: Result<Expr, String>) {
    let mut env: Env = HashMap::new();
    let parsed = parse(input);
    assert!(
        parsed.is_ok(),
        "Parse failed for '{}': {:?}",
        input,
        parsed.err().unwrap()
    );
    let result = eval(&parsed.unwrap(), &mut env);
    assert_eq!(result, expected);
}

/// Helper to test for specific evaluation errors.
fn run_eval_error_test(input: &str, expected_error: &str) {
    let mut env: Env = HashMap::new();
    let parsed = parse(input).unwrap();
    let result = eval(&parsed, &mut env);
    assert!(
        result.is_err(),
        "Expected an error for '{}', but got Ok.",
        input
    );
    assert_eq!(result.err().unwrap(), expected_error);
}

/// Helper to test for specific parsing errors.
fn run_parse_error_test(input: &str, expected_error: &str) {
    let result = parse(input);
    assert!(
        result.is_err(),
        "Expected a parse error for '{}', but got Ok.",
        input
    );
    assert_eq!(result.err().unwrap(), expected_error);
}

// --- Success Tests ---

#[test]
fn test_eval_simple_addition() {
    run_eval_test("(+ 1 2 3)", Ok(Expr::Number(6.0)));
}

#[test]
fn test_eval_nested_expression() {
    run_eval_test("(* (+ 1 2) (- 5 2))", Ok(Expr::Number(9.0)));
}

#[test]
fn test_define_variable() {
    let mut env: Env = HashMap::new();
    eval(&parse("(define x 3)").unwrap(), &mut env).unwrap();
    assert_eq!(env.get("x"), Some(&Expr::Number(3.0)));
}

#[test]
fn test_use_defined_variable() {
    let mut env: Env = HashMap::new();
    eval(&parse("(define x 5)").unwrap(), &mut env).unwrap();
    let result = eval(&parse("(+ x 2)").unwrap(), &mut env);
    assert_eq!(result, Ok(Expr::Number(7.0)));
}

#[test]
fn test_lambda_application() {
    run_eval_test("((lambda (x) (* x x)) 5)", Ok(Expr::Number(25.0)));
}

#[test]
fn test_define_lambda_and_apply() {
    let mut env: Env = HashMap::new();
    eval(
        &parse("(define square (lambda (x) (* x x)))").unwrap(),
        &mut env,
    )
    .unwrap();
    let result = eval(&parse("(square 3)").unwrap(), &mut env);
    assert_eq!(result, Ok(Expr::Number(9.0)));
}

#[test]
fn test_if_true() {
    run_eval_test(
        "(if (> 5 2) \"yes\" \"no\")",
        Ok(Expr::String("yes".to_string())),
    );
}

#[test]
fn test_if_false() {
    run_eval_test(
        "(if (> 2 5) \"yes\" \"no\")",
        Ok(Expr::String("no".to_string())),
    );
}

#[test]
fn test_parse_string() {
    assert_eq!(
        parse("\"hello world\"").unwrap(),
        Expr::String("hello world".to_string())
    );
}

#[test]
fn test_parse_empty_string() {
    assert_eq!(parse("\"\"").unwrap(), Expr::String("".to_string()));
}

#[test]
fn test_eval_concat() {
    run_eval_test(
        "(concat \"hello\" \" \" \"world\")",
        Ok(Expr::String("hello world".to_string())),
    );
}

#[test]
fn test_string_in_if() {
    run_eval_test(
        "(if (> 1 0) \"greater\" \"smaller\")",
        Ok(Expr::String("greater".to_string())),
    );
}

// --- Error Condition Tests ---

#[test]
fn test_eval_division_by_zero() {
    run_eval_error_test("(/ 10 0)", "Division by zero.");
}

#[test]
fn test_eval_undefined_variable() {
    run_eval_error_test("(+ x 5)", "Variable 'x' not found.");
}

#[test]
fn test_eval_invalid_define_syntax() {
    run_eval_error_test("(define x)", "'define' requires a symbol and a value.");
}

#[test]
fn test_parse_unclosed_parenthesis() {
    run_parse_error_test("(+ 1 2", "Missing closing parenthesis.");
}

#[test]
fn test_parse_unexpected_closing_parenthesis() {
    run_parse_error_test(")", "Unexpected closing parenthesis.");
}

#[test]
fn test_parse_extra_tokens() {
    run_parse_error_test("(+ 1 2) 3", "Unexpected tokens after main expression.");
}
