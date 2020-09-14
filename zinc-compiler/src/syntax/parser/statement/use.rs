//!
//! The `use` statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::keyword::Keyword;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::Token;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::parser::expression::path::Parser as PathOperandParser;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::statement::r#use::builder::Builder as UseStatementBuilder;
use crate::syntax::tree::statement::r#use::Statement as UseStatement;

/// The missing alias identifier error hint.
pub static HINT_EXPECTED_ALIAS_IDENTIFIER: &str =
    "specify the alias identifier after the `as` keyword, e.g. `use crate::Data as GlobalData;`";

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    KeywordUse,
    /// The `use` has been parsed so far.
    Path,
    /// The `use {path}` has been parsed so far.
    AsOrNext,
    /// The `use {path} as` has been parsed so far.
    AliasIdentifier,
    /// The `use {path} as {identifier}` has been parsed so far.
    Semicolon,
}

impl Default for State {
    fn default() -> Self {
        Self::KeywordUse
    }
}

///
/// The `use` statement parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The builder of the parsed value.
    builder: UseStatementBuilder,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses a 'use' statement.
    ///
    /// 'use jabberwocky::gone;'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(UseStatement, Option<Token>), Error> {
        loop {
            match self.state {
                State::KeywordUse => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Use),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Path;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["use"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::Path => {
                    let (expression, next) =
                        PathOperandParser::default().parse(stream.clone(), None)?;
                    self.builder.set_path(expression);
                    self.next = next;
                    self.state = State::AsOrNext;
                }
                State::AsOrNext => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::As),
                            ..
                        } => {
                            self.state = State::AliasIdentifier;
                        }
                        token => {
                            self.next = Some(token);
                            self.state = State::Semicolon;
                        }
                    }
                }
                State::AliasIdentifier => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let identifier = Identifier::new(location, identifier.inner);
                            self.builder.set_alias_identifier(identifier);
                            self.state = State::Semicolon;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_identifier(
                                location,
                                lexeme,
                                Some(HINT_EXPECTED_ALIAS_IDENTIFIER),
                            )));
                        }
                    }
                }
                State::Semicolon => {
                    return match crate::syntax::parser::take_or_next(self.next.take(), stream)? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        } => Ok((self.builder.finish(), None)),
                        Token { lexeme, location } => Err(Error::Syntax(
                            SyntaxError::expected_one_of(location, vec![";"], lexeme, None),
                        )),
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::error::Error;
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::lexeme::Lexeme;
    use crate::lexical::token::location::Location;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::statement::r#use::Statement as UseStatement;

    #[test]
    fn ok() {
        let input = r#"use mega::ultra::namespace;"#;

        let expected = Ok((
            UseStatement::new(
                Location::new(1, 1),
                ExpressionTree::new_with_leaves(
                    Location::new(1, 16),
                    ExpressionTreeNode::operator(ExpressionOperator::Path),
                    Some(ExpressionTree::new_with_leaves(
                        Location::new(1, 9),
                        ExpressionTreeNode::operator(ExpressionOperator::Path),
                        Some(ExpressionTree::new(
                            Location::new(1, 5),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::new(1, 5), "mega".to_owned()),
                            )),
                        )),
                        Some(ExpressionTree::new(
                            Location::new(1, 11),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::new(1, 11), "ultra".to_owned()),
                            )),
                        )),
                    )),
                    Some(ExpressionTree::new(
                        Location::new(1, 18),
                        ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                            Identifier::new(Location::new(1, 18), "namespace".to_owned()),
                        )),
                    )),
                ),
                None,
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_with_alias() {
        let input = r#"use mega::ultra::namespace as MegaUltraNamespace;"#;

        let expected = Ok((
            UseStatement::new(
                Location::new(1, 1),
                ExpressionTree::new_with_leaves(
                    Location::new(1, 16),
                    ExpressionTreeNode::operator(ExpressionOperator::Path),
                    Some(ExpressionTree::new_with_leaves(
                        Location::new(1, 9),
                        ExpressionTreeNode::operator(ExpressionOperator::Path),
                        Some(ExpressionTree::new(
                            Location::new(1, 5),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::new(1, 5), "mega".to_owned()),
                            )),
                        )),
                        Some(ExpressionTree::new(
                            Location::new(1, 11),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::new(1, 11), "ultra".to_owned()),
                            )),
                        )),
                    )),
                    Some(ExpressionTree::new(
                        Location::new(1, 18),
                        ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                            Identifier::new(Location::new(1, 18), "namespace".to_owned()),
                        )),
                    )),
                ),
                Some(Identifier::new(
                    Location::new(1, 31),
                    "MegaUltraNamespace".to_owned(),
                )),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_semicolon() {
        let input = r#"use jabberwocky"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::new(1, 16),
            vec![";"],
            Lexeme::Eof,
            None,
        )));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
