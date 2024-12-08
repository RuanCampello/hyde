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

macro_rules! test_table {
    ($name:ident, $expression:expr) => {
        #[test]
        fn $name() {
            let truth_table = TruthTable::new($expression);
            let result = truth_table.generate();

            assert!(result.is_ok());
        }
    };
}
