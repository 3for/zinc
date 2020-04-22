//!
//! The constant array element tests.
//!

#![cfg(test)]

use num_bigint::BigInt;

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::constant::array::error::Error as ArrayConstantError;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_pushing_invalid_type() {
    let input = r#"
fn main() {
    const ARRAY: [u8; 2] = [1, false];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 32),
        ElementError::Constant(ConstantError::Array(
            ArrayConstantError::PushingInvalidType {
                expected: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
                found: Type::boolean().to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_index_out_of_range() {
    let input = r#"
fn main() {
    const VALUE: u8 = [1, 2, 3, 4, 5][5];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 38),
        ElementError::Constant(ConstantError::Array(ArrayConstantError::IndexOutOfRange {
            index: BigInt::from(5).to_string(),
            size: 5,
        })),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_slice_start_out_of_range() {
    let input = r#"
fn main() {
    const ARRAY: [u8; 2] = [1, 2, 3, 4, 5][-1 .. 1];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 43),
        ElementError::Constant(ConstantError::Array(
            ArrayConstantError::SliceStartOutOfRange {
                start: BigInt::from(-1).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_slice_end_out_of_range() {
    let input = r#"
fn main() {
    const ARRAY: [u8; 6] = [1, 2, 3, 4, 5][0 .. 6];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 43),
        ElementError::Constant(ConstantError::Array(
            ArrayConstantError::SliceEndOutOfRange {
                end: BigInt::from(6).to_string(),
                size: 5,
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_slice_end_lesser_than_start() {
    let input = r#"
fn main() {
    const ARRAY: [u8; 1] = [1, 2, 3, 4, 5][2 .. 1];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 43),
        ElementError::Constant(ConstantError::Array(
            ArrayConstantError::SliceEndLesserThanStart {
                start: BigInt::from(2).to_string(),
                end: BigInt::from(1).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
