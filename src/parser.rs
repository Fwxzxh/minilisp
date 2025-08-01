use crate::data::Expr;

/// Splits the input string into a vector of tokens.
fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '(' | ')' => {
                tokens.push(c.to_string());
                chars.next();
            }

            '"' => {
                chars.next(); // consume opening quote
                let mut s = String::new();
                while let Some(&next_c) = chars.peek() {
                    if next_c == '"' {
                        break;
                    }
                    s.push(chars.next().unwrap());
                }
                if chars.next().is_none() { /* Unterminated string */ }
                tokens.push(format!("\"{}\"", s));
            }

            _ if c.is_whitespace() => {
                chars.next(); // consume whitespace
            }

            _ => {
                let mut s = String::new();
                while let Some(&next_c) = chars.peek() {
                    if next_c.is_whitespace() || next_c == '(' || next_c == ')' {
                        break;
                    }
                    s.push(chars.next().unwrap());
                }
                tokens.push(s);
            }
        }
    }
    tokens
}

/// Converts a single token into an `Expr`.
fn atom(token: &str) -> Expr {
    if token.starts_with('"') && token.ends_with('"') {
        return Expr::String(token[1..token.len() - 1].to_string());
    }

    match token {
        "true" => Expr::Bool(true),
        "false" => Expr::Bool(false),
        _ => token
            .parse::<f64>()
            .map(Expr::Number)
            .unwrap_or_else(|_| Expr::Symbol(token.to_string())),
    }
}

/// Recursively reads tokens to build an expression tree.
fn read_from_tokens(tokens: &mut &[String]) -> Result<Expr, String> {
    if tokens.is_empty() {
        return Err("Unexpected EOF".to_string());
    }

    let token = tokens[0].clone();
    *tokens = &tokens[1..];

    match token.as_str() {
        "(" => {
            let mut list = Vec::new();
            while !tokens.is_empty() && tokens[0] != ")" {
                list.push(read_from_tokens(tokens)?);
            }
            if tokens.is_empty() {
                return Err("Missing closing parenthesis.".to_string());
            }
            *tokens = &tokens[1..]; // consume ')'
            Ok(Expr::List(list))
        }
        ")" => Err("Unexpected closing parenthesis.".to_string()),
        _ => Ok(atom(&token)),
    }
}

/// Parses a string into a Lisp expression.
///
/// This function takes a string slice representing Lisp code, tokenizes it,
/// and then builds an abstract syntax tree (`Expr`).
///
/// # Arguments
///
/// * `input` - A string slice containing the Lisp code.
///
/// # Returns
///
/// A `Result` which is:
/// - `Ok(Expr)`: If parsing is successful, containing the root expression.
/// - `Err(String)`: If parsing fails, containing an error message.
pub fn parse(input: &str) -> Result<Expr, String> {
    let tokens = tokenize(input);
    let mut tokens_slice = tokens.as_slice();
    let result = read_from_tokens(&mut tokens_slice)?;

    if !tokens_slice.is_empty() {
        Err("Unexpected tokens after main expression.".to_string())
    } else {
        Ok(result)
    }
}
