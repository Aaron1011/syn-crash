use std::fmt;
use std::hash::{Hash, Hasher};

use proc_macro2::{self, Literal, TokenNode, Term};

use {Span, TokenTree};

#[derive(Clone)]
pub struct Lit {
    pub value: LitKind,
    pub span: Span,
}

#[derive(Clone)]
pub enum LitKind {
    Bool(bool),
    Other(Literal),
}

impl Lit {
    pub fn into_token_tree(self) -> TokenTree {
        let kind = match self.value {
            LitKind::Bool(true) => TokenNode::Term(Term::intern("true")),
            LitKind::Bool(false) => TokenNode::Term(Term::intern("false")),
            LitKind::Other(l) => TokenNode::Literal(l),
        };
        TokenTree(proc_macro2::TokenTree {
            span: self.span.0,
            kind: kind,
        })
    }
}

#[cfg(feature = "parsing")]
pub mod parsing {
    use super::*;
    use synom::{Synom, PResult, Cursor, parse_error};

    impl Synom for Lit {
        fn parse(input: Cursor) -> PResult<Self> {
            match input.literal() {
                Some((rest, span, lit)) => {
                    Ok((rest, Lit {
                        span: Span(span),
                        value: LitKind::Other(lit)
                    }))
                }
                _ => match input.word() {
                    Some((rest, span, sym)) => {
                        let kind = if sym.as_str() == "true" {
                            LitKind::Bool(true)
                        } else if sym.as_str() == "false" {
                            LitKind::Bool(false)
                        } else {
                            return parse_error();
                        };

                        Ok((rest, Lit {
                            span: Span(span),
                            value: kind
                        }))
                    }
                    _ => parse_error(),
                }
            }
        }
    }
}

#[cfg(feature = "printing")]
mod printing {
    use super::*;
    use quote::{Tokens, ToTokens};

    impl ToTokens for Lit {
        fn to_tokens(&self, tokens: &mut Tokens) {
        }
    }
}
