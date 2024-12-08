use crate::Tokenizer;
use std::collections::HashMap;

/// Interface that operate over a truth table.
/// Read more [here](https://en.wikipedia.org/wiki/Truth_table).
pub struct TruthTable<'a> {
    expression: &'a str,
    variables: Vec<char>,
}

impl<'a> TruthTable<'a> {
    pub fn new(expression: &'a str) -> Self {
        let variables = Self::extract_variables_from_expr(expression);

        Self {
            expression,
            variables,
        }
    }

    pub fn generate(&self) -> Result<(), String> {
        let combinations = 1 << self.variables.len();
        let col_title = self
            .variables
            .iter()
            .map(|var| var.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        println!("{col_title} | {expr}", expr = self.expression);

        for n in 0..combinations {
            let mut prep: HashMap<char, bool> = HashMap::new();

            for (idx, &var) in self.variables.iter().enumerate() {
                let res = (n & (1 << idx)) != 0;
                prep.insert(var, res);
            }

            let tokenizer = Tokenizer::new(self.expression);
            let result = tokenizer.parse_and_evaluate(&prep)?;

            let row = self
                .variables
                .iter()
                .map(|&var| if prep[&var] { "T" } else { "F" })
                .collect::<Vec<&str>>()
                .join(" ");

            println!("{} | {}", row, if result { "T" } else { "F" });
        }

        Ok(())
    }

    /// Only extracts the uniques variables for a given expression.
    fn extract_variables_from_expr(expression: &str) -> Vec<char> {
        let mut variables = expression
            .chars()
            .filter(|var| var.is_alphabetic())
            .collect::<Vec<char>>();

        variables.sort_unstable();
        variables.dedup();

        variables
    }
}
