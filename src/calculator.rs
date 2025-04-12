use std::fmt;

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
    InvalidDecimal,
    ZeroDivision,
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
            CalculatorError::InvalidDecimal => {
                write!(
                    f,
                    "Error: The expression contains an invalid decimal number. There are more than 1 '.' within a number."
                )
            }
            CalculatorError::ZeroDivision => {
                write!(f, "Error: Trying to divide by 0 is mathematically undefeined.")
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
        let precedence = |op: char| {
            return match op {
                '+' | '-' => 1u8,
                '*' | '/' => 2u8,
                _ => 0u8,
            };
        };

        let mut i = 0;
        self.tokens.clear();
        let mut operations = Vec::new();
        let chars: Vec<char> = self.expr.chars().collect();

        while i < chars.len() {
            let c = chars[i];

            if c.is_whitespace() {
                i += 1;
                continue;
            }

            if c.is_ascii_digit() || c == '.' {
                let mut num_str = String::new();
                let mut dotted = false;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    if chars[i] == '.' {
                        if !dotted {
                            dotted = true;
                        } else {
                            return Err(CalculatorError::InvalidDecimal);
                        }
                    }
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
                        if precedence(top) >= precedence(c) {
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
                        '/' => {
                            if b == 0.0 {
                                return Err(CalculatorError::ZeroDivision);
                            }
                            a / b
                        }
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
