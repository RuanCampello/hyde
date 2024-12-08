use hyde::*;

#[allow(unused_macros)]
macro_rules! test_operation {
    ($name:ident, $expression:expr, [ $( { $($var:tt : $val:expr),* } => $expected:expr, $comment:expr ),* $(,)? ]) => {
        #[test]
        fn $name() {
            let tokenizer = Tokenizer::new($expression);

            $(
                let mut prep = std::collections::HashMap::new();
                $(
                    prep.insert($var, $val);
                )*
                let result = tokenizer.parse_and_evaluate(&prep).unwrap();
                assert_eq!(result, $expected, "Failed on input: {:?}. Comment: {}", prep, $comment);
            )*
        }
    };
}

test_operation!(
    conjunction_operation,
    "p∧q",
    [
        { 'p': true, 'q': false } => false, "if either p or q is false, it must return false",
        { 'p': true, 'q': true } => true, "if both p and q are true, it must return true"
    ]
);

test_operation!(
    disjunction_operation,
    "p∨q",
    [
        { 'p': true, 'q': false } => true, "if either of p or q is true, it must return true",
        { 'p': false, 'q': false } => false, "if both p and q are false, it must return false"
    ]
);

test_operation!(
    condition_operation,
    "p→q",
    [
        { 'p': true, 'q': false } => false, "if p is true, q must also be true. TRUE → FALSE must return false",
        { 'p': false, 'q': false } => true, "if p is false, the condition does not matter. This must return true"
    ]
);
