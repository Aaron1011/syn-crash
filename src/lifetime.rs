use std::cmp::Ordering;
use std::fmt::{self, Display};
use std::hash::{Hash, Hasher};

use proc_macro2::Term;
use unicode_xid::UnicodeXID;

use Span;

#[cfg_attr(feature = "extra-traits", derive(Debug))]
#[cfg_attr(feature = "clone-impls", derive(Clone))]
pub struct Lifetime {
    pub sym: Term,
    pub span: Span,
}

#[cfg(feature = "parsing")]
pub mod parsing {
    use super::*;
    use synom::{Synom, PResult, Cursor, parse_error};

    impl Synom for Lifetime {
        fn parse(input: Cursor) -> PResult<Self> {
            let (rest, span, sym) = match input.word() {
                Some(word) => word,
                _ => return parse_error(),
            };
            if !sym.as_str().starts_with('\'') {
                return parse_error();
            }

            Ok((rest, Lifetime {
                sym: sym,
                span: Span(span),
            }))
        }
    }
}

#[cfg(feature = "printing")]
mod printing {
    use super::*;
    use quote::{Tokens, ToTokens};
    use proc_macro2::{TokenTree, TokenNode};

    impl ToTokens for Lifetime {
        fn to_tokens(&self, tokens: &mut Tokens) {
            tokens.append(TokenTree {
                span: self.span.0,
                kind: TokenNode::Term(self.sym),
            })
        }
    }
}
