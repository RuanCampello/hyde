#[macro_use]
mod common;

use hyde::*;

test_operation!(
    compound_one,
    "p∧(¬p∨q)", [{ 'p': true, 'q': false } => false, "it must return false"]
);

test_operation!(
    compound_two,
    "¬p ∧ q", [{ 'p': false, 'q': true } => true, "it must return true"]
);

test_operation!(
    compound_three,
    "¬p ∨ ¬q", [{ 'p': true, 'q': false } => true, "it must return true"]
);
