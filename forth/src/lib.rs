use std::collections::HashMap;
use regex::Regex;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    custom_operators: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
    NotPrimitiveOperator,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            custom_operators: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    fn split_and_keep_delimiters(&mut self, input: &str) -> Vec<String> {
        let re = Regex::new(r"(:.*?;)").unwrap();
        let mut result = Vec::new();
        let mut last_end = 0;
    
        for mat in re.find_iter(input) {
            // Add the non-matching part before the current match
            if mat.start() > last_end {
                result.push(input[last_end..mat.start()].to_string());
            }
    
            // Add the current match
            result.push(mat.as_str().to_string());
    
            last_end = mat.end();
        }
    
        // Add any remaining part of the string after the last match
        if last_end < input.len() {
            result.push(input[last_end..].to_string());
        }
    
        result
    }

    fn is_valid_def(&mut self, input: &str) -> bool {
        return true;
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let DEF_NAME_REG: Regex = Regex::new(r"[A-Za-z-_$+\-\*\/][\w$]*(\.[\w$]+)?(\[\d+])?").unwrap();

        let chunks = self.split_and_keep_delimiters(input);
        let mut stack: Vec<i32> = Vec::new();

        for chunk in chunks {
            if chunk.len() == 0 {
                continue;
            }

            let tokens: Vec<&str> = chunk.split_whitespace().collect();
            if chunk.starts_with(":") {
                if !chunk.ends_with("; ") && !chunk.ends_with(";") {
                    return Err(Error::InvalidWord);
                }
                
                if tokens.len() < 3 {
                    return Err(Error::InvalidWord);
                }
            }

            let mut processed_input = String::new();
            let mut is_defining = false;
            let mut previous_token: &str = "";
            
            // replacing by primitive operator
            for token in tokens {
                if token == ":" {
                    is_defining = true;
                }

                let token_upper = token.to_uppercase();

                /* If token is already defined, we check if user want to redefine it */
                if let Some(definition) = self.custom_operators.get(&token_upper) {
                    if is_defining && previous_token == ":"  {
                        processed_input.push_str(&token_upper);
                        processed_input.push(' ');
                    } else {
                        processed_input.push_str(definition);
                        processed_input.push(' ');
                    }
                } else {
                    processed_input.push_str(&token_upper);
                    processed_input.push(' ');
                }

                previous_token = token;
            }

            let mut processed_tokens = processed_input.trim().split_whitespace();

            while let Some(token) = processed_tokens.next() {
                match token {
                    "+" | "-" | "*" | "/" => {
                        if stack.len() < 2 {
                            return Err(Error::StackUnderflow);
                        }
                        let a: i32 = stack.pop().unwrap();
                        let b: i32 = stack.pop().unwrap();
                        stack.push(match token {
                            "+" => b + a,
                            "-" => b - a,
                            "*" => b * a,
                            "/" => {
                                if a == 0 {
                                    return Err(Error::DivisionByZero);  
                                }
                                b / a
                            },
                            _ => unreachable!(),
                        });
                    }
                    "DUP" => {
                        if stack.len() < 1 {
                            return Err(Error::StackUnderflow);
                        }
                        let top = stack.last().unwrap();
                        stack.push(*top);
                    }
                    "DROP" => {
                        if stack.len() < 1 {
                            return Err(Error::StackUnderflow);
                        }
                        stack.pop();
                    }
                    "SWAP" => {
                        if stack.len() < 2 {
                            return Err(Error::StackUnderflow);
                        }
                        let top = stack.pop().unwrap();
                        let second_top = stack.pop().unwrap();
                        stack.push(top);
                        stack.push(second_top);
                    }
                    "OVER" => {
                        if stack.len() < 2 {
                            return Err(Error::StackUnderflow);
                        }
                        let second_top = *stack.get(stack.len() - 2).unwrap();
                        stack.push(second_top);
                    }
                    ":" => {
                        let operator = processed_tokens.next().unwrap();

                        if  !DEF_NAME_REG.is_match(operator) {
                            return Err(Error::InvalidWord);
                        }

                        let mut sub_operators: Vec<String> = Vec::new();
                        loop {
                            match processed_tokens.next() {
                                Some(token) => {
                                    if token == ";" {
                                        break;
                                    }
                                    sub_operators.push(token.to_uppercase())
                                }
                                None => break,
                            }
                        }
                        self.custom_operators
                            .insert(operator.to_uppercase(), sub_operators.join(" "));
                    }
                    _ => {
                        if let Ok(num) = token.parse::<i32>() {
                            stack.push(num);
                        } else {
                            return Err(Error::UnknownWord);
                        }
                    }
                }
            }
        }

        self.stack = stack;
        Ok(())
    }
}
