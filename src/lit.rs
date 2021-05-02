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

#[cfg(feature = "parsing")]
pub mod parsing {
    use super::*;
    use synom::{Synom, PResult, Cursor, parse_error};

    impl Synom for Lit {
        fn parse(input: Cursor) -> PResult<Self> {
            parse_error()
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
