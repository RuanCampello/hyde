use hyde::*;
use std::collections::HashMap;

#[test]
fn compound_one() {
    let tokenizer = Tokenizer::new("p∧(¬p∨q)");
    let mut prep: HashMap<char, bool> = HashMap::new();
    prep.insert('p', true);
    prep.insert('q', false);

    let result = tokenizer.parse_and_evaluate(&prep).unwrap();
    assert!(!result);
}
