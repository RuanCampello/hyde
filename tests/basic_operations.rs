use hyde::*;

#[macro_use]
mod common;

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

test_operation!(
    bicondition_operation,
    "p↔q",
    [
        {'p': true, 'q': false } => false, "both values must be equal. it must return false",
        {'p': false, 'q': false } => true, "if they're equal, either true or false, it must return true"
    ]
);

test_operation!(
    exclusive_disjunction_operation,
    "p⊻q",
    [
        {'p': true, 'q': false } => true, "both values must be different. it must return true",
        {'p': true, 'q': true } => false, "if they're equal, either true or false, it must return false"
    ]
);
