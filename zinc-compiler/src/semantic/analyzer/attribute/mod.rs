//!
//! The semantic attribute.
//!

#[cfg(test)]
mod tests;

use std::convert::TryFrom;

use num::BigInt;

use zinc_syntax::Attribute as SyntaxAttribute;
use zinc_syntax::AttributeElementVariant as SyntaxAttributeElementVariant;
use zinc_syntax::Literal;

use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::error::Error;

///
/// The semantic attribute.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Attribute {
    /// The `#[test]` attribute.
    Test,
    /// The `#[should_panic]` attribute.
    ShouldPanic,
    /// The `#[ignore]` attribute.
    Ignore,
    /// The `#[zksync::msg(...)]` attribute.
    ZksyncMsg {
        /// The `zksync::msg.sender` field.
        sender: BigInt,
        /// The `zksync::msg.recipient` field.
        recipient: BigInt,
        /// The `zksync::msg.token_address` field.
        token_address: BigInt,
        /// The `zksync::msg.amount` field.
        amount: BigInt,
    },
}

impl Attribute {
    ///
    /// If the attribute is related to unit tests.
    ///
    pub fn is_test(&self) -> bool {
        match self {
            Self::Test => true,
            Self::ShouldPanic => true,
            Self::Ignore => true,
            Self::ZksyncMsg { .. } => true,
        }
    }
}

impl TryFrom<SyntaxAttribute> for Attribute {
    type Error = Error;

    fn try_from(mut value: SyntaxAttribute) -> Result<Self, Self::Error> {
        let element = value.elements.get_mut(0).ok_or(Error::AttributeEmpty {
            location: value.location,
        })?;
        let identifier = element.path.to_string();

        Ok(match identifier.as_str() {
            "test" => Self::Test,
            "should_panic" => Self::ShouldPanic,
            "ignore" => Self::Ignore,
            "zksync::msg" => match element.variant {
                Some(SyntaxAttributeElementVariant::Nested(ref mut nested)) => {
                    if nested.len() != zinc_const::contract::TRANSACTION_FIELDS_COUNT {
                        return Err(Error::AttributeElementsCount {
                            location: element.location,
                            name: identifier,
                            expected: zinc_const::contract::TRANSACTION_FIELDS_COUNT,
                            found: nested.len(),
                        });
                    }

                    let sender = nested.remove(0);
                    let name = sender.path.to_string();
                    if name.as_str() != "sender" {
                        return Err(Error::AttributeExpectedElement {
                            location: sender.location,
                            name: "zksync::msg".to_owned(),
                            position: 1,
                            expected: "sender".to_owned(),
                            found: name,
                        });
                    }
                    let sender = match sender.variant {
                        Some(SyntaxAttributeElementVariant::Value(Literal::Integer(
                            ref integer,
                        ))) => IntegerConstant::try_from(integer)?.value,
                        _ => {
                            return Err(Error::AttributeExpectedIntegerLiteral {
                                location: sender.location,
                                name: "sender".to_owned(),
                            })
                        }
                    };

                    let recipient = nested.remove(0);
                    let name = recipient.path.to_string();
                    if name.as_str() != "recipient" {
                        return Err(Error::AttributeExpectedElement {
                            location: recipient.location,
                            name: "zksync::msg".to_owned(),
                            position: 2,
                            expected: "recipient".to_owned(),
                            found: name,
                        });
                    }
                    let recipient = match recipient.variant {
                        Some(SyntaxAttributeElementVariant::Value(Literal::Integer(
                            ref integer,
                        ))) => IntegerConstant::try_from(integer)?.value,
                        _ => {
                            return Err(Error::AttributeExpectedIntegerLiteral {
                                location: recipient.location,
                                name: "recipient".to_owned(),
                            })
                        }
                    };

                    let token_address = nested.remove(0);
                    let name = token_address.path.to_string();
                    if name.as_str() != "token_address" {
                        return Err(Error::AttributeExpectedElement {
                            location: token_address.location,
                            name: "zksync::msg".to_owned(),
                            position: 3,
                            expected: "token_address".to_owned(),
                            found: name,
                        });
                    }
                    let token_address = match token_address.variant {
                        Some(SyntaxAttributeElementVariant::Value(Literal::Integer(
                            ref integer,
                        ))) => IntegerConstant::try_from(integer)?.value,
                        _ => {
                            return Err(Error::AttributeExpectedIntegerLiteral {
                                location: token_address.location,
                                name: "token_address".to_owned(),
                            })
                        }
                    };

                    let amount = nested.remove(0);
                    let name = amount.path.to_string();
                    if name.as_str() != "amount" {
                        return Err(Error::AttributeExpectedElement {
                            location: amount.location,
                            name: "zksync::msg".to_owned(),
                            position: 4,
                            expected: "amount".to_owned(),
                            found: name,
                        });
                    }
                    let amount = match amount.variant {
                        Some(SyntaxAttributeElementVariant::Value(Literal::Integer(
                            ref integer,
                        ))) => IntegerConstant::try_from(integer)?.value,
                        _ => {
                            return Err(Error::AttributeExpectedIntegerLiteral {
                                location: amount.location,
                                name: "amount".to_owned(),
                            })
                        }
                    };

                    Self::ZksyncMsg {
                        sender,
                        recipient,
                        token_address,
                        amount,
                    }
                }
                _ => {
                    return Err(Error::AttributeExpectedNested {
                        location: element.location,
                        name: "zksync::msg".to_owned(),
                    })
                }
            },
            _ => {
                return Err(Error::AttributeUnknown {
                    location: value.location,
                    found: identifier,
                })
            }
        })
    }
}
