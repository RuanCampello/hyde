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
