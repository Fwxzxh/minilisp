use crate::data::{Env, Expr};

/// Evaluates a Lisp expression within a given environment.
///
/// This function recursively evaluates an expression, handling symbols,
/// special forms (`define`, `lambda`, `if`), and function applications.
///
/// # Arguments
///
/// * `expr` - A reference to the expression to be evaluated.
/// * `env` - A mutable reference to the environment where variables and functions are stored.
///
/// # Returns
///
/// A `Result` which is:
/// - `Ok(Expr)`: If the evaluation is successful, containing the resulting expression.
/// - `Err(String)`: If an error occurs during evaluation, containing an error message.
pub fn eval(expr: &Expr, env: &mut Env) -> Result<Expr, String> {
    match expr {
        Expr::Symbol(s) => env
            .get(s)
            .cloned()
            .ok_or_else(|| format!("Variable '{}' not found.", s)),
        Expr::Number(_) | Expr::Bool(_) | Expr::String(_) | Expr::Func { .. } => Ok(expr.clone()),
        Expr::List(list) => {
            if list.is_empty() {
                return Ok(Expr::List(Vec::new()));
            }
            let first = &list[0];
            let args = &list[1..];

            if let Expr::Symbol(s) = first {
                match s.as_str() {
                    "define" => eval_define(args, env),
                    "lambda" => eval_lambda(args),
                    "if" => eval_if(args, env),
                    _ => apply_procedure(first, args, env),
                }
            } else {
                apply_procedure(first, args, env)
            }
        }
    }
}

fn eval_define(args: &[Expr], env: &mut Env) -> Result<Expr, String> {
    if args.len() != 2 {
        return Err("'define' requires a symbol and a value.".to_string());
    }
    if let Expr::Symbol(name) = &args[0] {
        let value = eval(&args[1], env)?;
        env.insert(name.clone(), value);
        Ok(Expr::Symbol(name.clone()))
    } else {
        Err("The first argument to 'define' must be a symbol.".to_string())
    }
}

fn eval_lambda(args: &[Expr]) -> Result<Expr, String> {
    if args.len() != 2 {
        return Err("'lambda' requires a list of parameters and a body.".to_string());
    }
    let params_list = match &args[0] {
        Expr::List(p) => p,
        _ => return Err("The first argument to 'lambda' must be a list of symbols.".to_string()),
    };
    let params = params_list
        .iter()
        .map(|p| match p {
            Expr::Symbol(s) => Ok(s.clone()),
            _ => Err("Lambda parameters must be symbols.".to_string()),
        })
        .collect::<Result<Vec<String>, String>>()?;
    let body = Box::new(args[1].clone());
    Ok(Expr::Func { params, body })
}

fn eval_if(args: &[Expr], env: &mut Env) -> Result<Expr, String> {
    if args.len() != 3 {
        return Err("'if' requires a condition, a then branch, and an else branch.".to_string());
    }
    let cond = eval(&args[0], env)?;
    match cond {
        Expr::Bool(b) => eval(if b { &args[1] } else { &args[2] }, env),
        _ => Err("The condition for 'if' must evaluate to a boolean.".to_string()),
    }
}

fn apply_procedure(op_expr: &Expr, args: &[Expr], env: &mut Env) -> Result<Expr, String> {
    let evaluated_args = args
        .iter()
        .map(|arg| eval(arg, env))
        .collect::<Result<Vec<Expr>, String>>()?;

    if let Expr::Symbol(s) = op_expr {
        match apply_builtin_op(s, &evaluated_args) {
            Ok(result) => return Ok(result),
            Err(e) => {
                if e != "Not a built-in operator" {
                    return Err(e);
                }
                // If it's "Not a built-in operator", fall through to evaluate it as a function
            }
        }
    }

    let evaluated_op = eval(op_expr, env)?;
    if let Expr::Func { params, body } = evaluated_op {
        if params.len() != evaluated_args.len() {
            return Err(format!(
                "Function expects {} arguments, but received {}.",
                params.len(),
                evaluated_args.len()
            ));
        }
        let mut func_env = env.clone();
        for (param_name, arg_value) in params.iter().zip(evaluated_args) {
            func_env.insert(param_name.clone(), arg_value);
        }
        eval(&body, &mut func_env)
    } else {
        Err(format!("Not a function: {}", op_expr))
    }
}

fn apply_builtin_op(op: &str, args: &[Expr]) -> Result<Expr, String> {
    let numeric_op = |f: fn(f64, f64) -> f64, initial: f64| -> Result<Expr, String> {
        let nums = args
            .iter()
            .map(|arg| match arg {
                Expr::Number(n) => Ok(*n),
                _ => Err(format!("Operator '{}' requires number arguments.", op)),
            })
            .collect::<Result<Vec<f64>, String>>()?;
        if op != "+" && op != "*" && nums.is_empty() {
            return Err(format!("Operator '{}' requires at least one argument.", op));
        }
        let result = nums.iter().fold(initial, |acc, &x| f(acc, x));
        Ok(Expr::Number(result))
    };

    match op {
        "+" => numeric_op(|a, b| a + b, 0.0),
        "*" => numeric_op(|a, b| a * b, 1.0),
        "-" => {
            let nums = args
                .iter()
                .map(|arg| match arg {
                    Expr::Number(n) => Ok(*n),
                    _ => Err(format!("Operator '{}' requires number arguments.", op)),
                })
                .collect::<Result<Vec<f64>, String>>()?;
            if nums.is_empty() {
                return Err("Operator '-' requires at least one argument.".to_string());
            }
            let first = nums[0];
            if nums.len() == 1 {
                Ok(Expr::Number(-first))
            } else {
                Ok(Expr::Number(
                    nums[1..].iter().fold(first, |acc, &x| acc - x),
                ))
            }
        }
        "/" => {
            if args.iter().skip(1).any(|arg| {
                if let Expr::Number(n) = arg {
                    *n == 0.0
                } else {
                    false
                }
            }) {
                return Err("Division by zero.".to_string());
            }
            let nums = args
                .iter()
                .map(|arg| match arg {
                    Expr::Number(n) => Ok(*n),
                    _ => Err(format!("Operator '{}' requires number arguments.", op)),
                })
                .collect::<Result<Vec<f64>, String>>()?;
            if nums.is_empty() {
                return Err("Operator '/' requires at least one argument.".to_string());
            }
            let first = nums[0];
            Ok(Expr::Number(
                nums[1..].iter().fold(first, |acc, &x| acc / x),
            ))
        }
        ">" => {
            if args.len() != 2 {
                return Err("'>' requires two arguments.".to_string());
            }
            if let (Expr::Number(n1), Expr::Number(n2)) = (&args[0], &args[1]) {
                Ok(Expr::Bool(n1 > n2))
            } else {
                Err("'>' requires number arguments.".to_string())
            }
        }
        "concat" => {
            let strings = args
                .iter()
                .map(|arg| match arg {
                    Expr::String(s) => Ok(s.as_str()),
                    _ => Err("'concat' requires string arguments.".to_string()),
                })
                .collect::<Result<Vec<&str>, String>>()?;
            Ok(Expr::String(strings.concat()))
        }
        _ => Err("Not a built-in operator".to_string()),
    }
}
