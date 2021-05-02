#[cfg(feature = "extra-traits")]
use std::fmt;

use super::*;

use proc_macro2::{TokenNode, Delimiter};

ast_struct! {
    /// Represents a macro invocation. The Path indicates which macro
    /// is being invoked, and the vector of token-trees contains the source
    /// of the macro invocation.
    pub struct Mac {
    }
}

pub struct TokenTree(pub proc_macro2::TokenTree);

#[cfg(feature = "parsing")]
pub mod parsing {
    use super::*;

    use proc_macro2::{TokenNode, TokenTree};
    use synom::tokens::*;
    use synom::{Synom, PResult, Cursor, parse_error};


    impl ::TokenTree {
        pub fn parse_list(input: Cursor) -> PResult<Vec<Self>> {
            parse_error()
        }

        pub fn parse_delimited(input: Cursor) -> PResult<Self> {
            parse_error()
        }
    }
}
