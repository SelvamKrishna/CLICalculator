use std::fmt;

fn precedence(op: char) -> Option<u8> {
    return match op {
        '+' | '-' => { Some(1) }
        '*' | '/' => { Some(2) }
        _ => { None }
    };
}

enum RPNToken {
    Operation(char),
    Number(f32),
}

pub struct Calculator {
    tokens: Vec<RPNToken>,
    expr: String,
}

pub enum CalculatorError {
    UnsupportedToken,
    MismatchedParantheses,
    InvalidExpression,
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            CalculatorError::UnsupportedToken => {
                write!(f, "Error: The expression contains an unsupported token")
            }
            CalculatorError::MismatchedParantheses => {
                write!(f, "Error: Mismatched parentheses — make sure every '(' has a matching ')'.")
            }
            CalculatorError::InvalidExpression => {
                write!(
                    f,
                    "Error: The expression is invalid — it may be incomplete, malformed, or missing operands."
                )
            }
        };
    }
}

impl Calculator {
    pub fn new(expr: String) -> Self {
        return Calculator {
            tokens: Vec::new(),
            expr,
        };
    }

    fn tokenizer(&mut self) -> Result<(), CalculatorError> {
        self.tokens.clear();
        let mut operations = Vec::new();
        let chars: Vec<char> = self.expr.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];

            if c.is_whitespace() {
                i += 1;
                continue;
            }

            if c.is_ascii_digit() || c == '.' {
                let mut num_str = String::new();
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    num_str.push(chars[i]);
                    i += 1;
                }

                match num_str.parse::<f32>() {
                    Ok(n) => self.tokens.push(RPNToken::Number(n)),
                    Err(_) => {
                        return Err(CalculatorError::InvalidExpression);
                    }
                }
                continue;
            }

            match c {
                '+' | '-' | '*' | '/' => {
                    while let Some(&top) = operations.last() {
                        if precedence(top).unwrap_or(0) >= precedence(c).unwrap_or(0) {
                            self.tokens.push(RPNToken::Operation(operations.pop().unwrap()));
                        } else {
                            break;
                        }
                    }
                    operations.push(c);
                }
                '(' => operations.push(c),
                ')' => {
                    while let Some(top) = operations.pop() {
                        if top == '(' {
                            break;
                        }
                        self.tokens.push(RPNToken::Operation(top));
                    }
                }
                _ => {
                    return Err(CalculatorError::UnsupportedToken);
                }
            }

            i += 1;
        }

        while let Some(op) = operations.pop() {
            if op == '(' || op == ')' {
                return Err(CalculatorError::MismatchedParantheses);
            }
            self.tokens.push(RPNToken::Operation(op));
        }

        return Ok(());
    }

    pub fn eval(&mut self) -> Result<f32, CalculatorError> {
        let mut result: Vec<f32> = Vec::new();
        self.tokenizer()?;

        for token in &self.tokens {
            match token {
                RPNToken::Operation(op) => {
                    if result.len() < 2 {
                        return Err(CalculatorError::InvalidExpression);
                    }
                    let b = result.pop().unwrap();
                    let a = result.pop().unwrap();
                    let out = match op {
                        '+' => a + b,
                        '-' => a - b,
                        '*' => a * b,
                        '/' => a / b,
                        _ => unreachable!(),
                    };
                    result.push(out);
                }
                RPNToken::Number(n) => result.push(*n),
            }
        }

        return if let Some(value) = result.pop() {
            Ok(value)
        } else {
            Err(CalculatorError::InvalidExpression)
        };
    }
}
