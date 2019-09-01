//!
//! The expression parser.
//!

mod operator;

pub use self::operator::AddSubOperandParser as AddSubOperatorOperandParser;
pub use self::operator::AndOperandParser as AndOperatorOperandParser;
pub use self::operator::AssignmentOperandParser as AssignmentOperatorOperandParser;
pub use self::operator::CastingOperandParser as CastingOperatorOperandParser;
pub use self::operator::ComparisonOperandParser as ComparisonOperatorOperandParser;
pub use self::operator::MulDivRemOperandParser as MulDivRemOperatorOperandParser;
pub use self::operator::OrOperandParser as OrOperatorOperandParser;
pub use self::operator::Parser as OperatorExpressionParser;
pub use self::operator::XorOperandParser as XorOperatorOperandParser;

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::TokenStream;
use crate::syntax::Expression;
use crate::Error;

#[derive(Default)]
pub struct Parser {}

impl Parser {
    pub fn parse(self, stream: Rc<RefCell<TokenStream>>) -> Result<Expression, Error> {
        OperatorExpressionParser::default().parse(stream)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::lexical::BooleanLiteral;
    use crate::lexical::Lexeme;
    use crate::lexical::Literal;
    use crate::lexical::Location;
    use crate::lexical::Symbol;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::ExpressionOperator;

    #[test]
    fn ok() {
        let code = br#"true || false"#;

        let expected = Expression::new(vec![
            ExpressionElement::new(
                ExpressionObject::Operand(ExpressionOperand::Literal(Literal::Boolean(
                    BooleanLiteral::True,
                ))),
                Token::new(
                    Lexeme::Literal(Literal::Boolean(BooleanLiteral::True)),
                    Location::new(1, 1),
                ),
            ),
            ExpressionElement::new(
                ExpressionObject::Operand(ExpressionOperand::Literal(Literal::Boolean(
                    BooleanLiteral::False,
                ))),
                Token::new(
                    Lexeme::Literal(Literal::Boolean(BooleanLiteral::False)),
                    Location::new(1, 9),
                ),
            ),
            ExpressionElement::new(
                ExpressionObject::Operator(ExpressionOperator::Or),
                Token::new(
                    Lexeme::Symbol(Symbol::DoubleVerticalBar),
                    Location::new(1, 6),
                ),
            ),
        ]);

        let result = Parser::default()
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_vec()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
