#[cfg(feature = "extra-traits")]
use std::fmt;

use super::*;

use proc_macro2::{TokenNode, Delimiter};

ast_struct! {
    /// Represents a macro invocation. The Path indicates which macro
    /// is being invoked, and the vector of token-trees contains the source
    /// of the macro invocation.
    pub struct Mac {
        pub path: Path,
        pub bang_token: tokens::Bang,
        /// The `example` in `macro_rules! example { ... }`.
        pub ident: Option<Ident>,
        pub tokens: Vec<TokenTree>,
    }
}

pub struct TokenTree(pub proc_macro2::TokenTree);

#[cfg(feature = "parsing")]
pub mod parsing {
    use super::*;

    use proc_macro2::{TokenNode, TokenTree};
    use synom::tokens::*;
    use synom::{Synom, PResult, Cursor, parse_error};

    impl Synom for Mac {
        named!(parse -> Self, do_parse!(
            what: syn!(Path) >>
            bang: syn!(Bang) >>
            body: call!(::TokenTree::parse_delimited) >>
            (Mac {
                path: what,
                bang_token: bang,
                ident: None,
                tokens: vec![body],
            })
        ));
    }

    impl ::TokenTree {
        pub fn parse_list(input: Cursor) -> PResult<Vec<Self>> {
            parse_error()
        }

        pub fn parse_delimited(input: Cursor) -> PResult<Self> {
            parse_error()
        }
    }
}
