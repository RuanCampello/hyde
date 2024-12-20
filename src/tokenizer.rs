use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Token {
    Exp(char),
    Conjunction,
    Disjunction,
    Conditional,
    Biconditional,
    ExclusiveDisjunction,
    Negation,
    LeftParen,
    RightParen,
}

/// Interface responsible to tokenize and operate over a given expression.
pub struct Tokenizer<'a> {
    expression: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn new(expression: &'a str) -> Self {
        Self { expression }
    }

    // TODO make prepositions optional
    pub fn parse_and_evaluate(&self, prepositions: &HashMap<char, bool>) -> Result<bool, String> {
        let tokens: Vec<Token> = self.tokenize();

        let mut operators: Vec<&Token> = Vec::new();
        let mut output: Vec<bool> = Vec::new();

        tokens.iter().try_for_each(|token| match token {
            Token::Exp(c) => {
                if let Some(&value) = prepositions.get(c) {
                    output.push(value);
                    Ok(())
                } else {
                    Err(format!("Variable {} is undefined", c))
                }
            }
            Token::LeftParen => {
                operators.push(&Token::LeftParen);
                Ok(())
            }
            Token::RightParen => {
                while let Some(top_op) = operators.pop() {
                    if top_op == &Token::LeftParen {
                        break;
                    }
                    Self::apply_operator(top_op, &mut output)?;
                }
                Ok(())
            }
            _ => {
                while operators.last().map_or(false, |&top_op| {
                    Self::precedence(top_op) <= Self::precedence(token)
                        && *top_op != Token::LeftParen
                }) {
                    let op = operators.pop().unwrap();
                    Self::apply_operator(op, &mut output)?;
                }
                operators.push(token);
                Ok(())
            }
        })?;

        while let Some(op) = operators.pop() {
            Self::apply_operator(op, &mut output)?;
        }

        output.pop().ok_or(String::from("Invalid expression"))
    }

    fn tokenize(&self) -> Vec<Token> {
        self.expression
            .chars()
            .filter_map(|c| match c {
                '∧' => Some(Token::Conjunction),
                '∨' => Some(Token::Disjunction),
                '¬' => Some(Token::Negation),
                '→' => Some(Token::Conditional),
                '↔' => Some(Token::Biconditional),
                '⊻' => Some(Token::ExclusiveDisjunction),
                '(' => Some(Token::LeftParen),
                ')' => Some(Token::RightParen),
                c if c.is_alphabetic() => Some(Token::Exp(c)),
                _ => None,
            })
            .collect()
    }

    /// Determines the precedence order of each operation/token.
    fn precedence(operation: &Token) -> usize {
        match operation {
            Token::Negation => 1,
            Token::Conjunction => 2,
            Token::Disjunction => 3,
            Token::Conditional => 4,
            Token::Biconditional => 5,
            Token::ExclusiveDisjunction => 6,
            _ => 0, // in case of parenthesis
        }
    }

    /// Applies the operation on the operands and remove them from the to-do operations stack.
    fn apply_operator(op: &Token, stack: &mut Vec<bool>) -> Result<(), String> {
        match op {
            Token::Negation => {
                if let Some(operand) = stack.pop() {
                    stack.push(!operand);
                }
            }
            _ => {
                let (left, right) = Self::get_operands(stack)?;
                match op {
                    Token::Conjunction => stack.push(left && right),
                    Token::Disjunction => stack.push(left || right),
                    Token::Conditional => stack.push(!left || right),
                    Token::Biconditional => stack.push(left == right),
                    Token::ExclusiveDisjunction => stack.push(left != right),

                    _ => return Err("Unexpected operator".into()),
                }
            }
        }
        Ok(())
    }

    /// Replaces the current operation to reuse the tokenizer for tests.
    fn replace_expr(&mut self, new_expression: &'a str) {
        self.expression = new_expression;
    }

    /// Returns the left and right operands, if present.
    fn get_operands(stack: &mut Vec<bool>) -> Result<(bool, bool), String> {
        let right = stack.pop().ok_or("Missing right operand")?;
        let left = stack.pop().ok_or("Missing left operand")?;
        Ok((left, right))
    }
}
