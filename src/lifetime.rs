use std::cmp::Ordering;
use std::fmt::{self, Display};
use std::hash::{Hash, Hasher};

use proc_macro2::Term;
use unicode_xid::UnicodeXID;

use Span;

pub struct Lifetime {
    pub sym: Term,
    pub span: Span,
}

#[cfg(feature = "printing")]
mod printing {
    use super::*;
    use quote::{Tokens, ToTokens};
    use proc_macro2::{TokenTree, TokenNode};

    impl ToTokens for Lifetime {
        fn to_tokens(&self, tokens: &mut Tokens) {
            tokens.append(dummy())
        }
    }
}

fn dummy() -> proc_macro2::TokenTree {
    panic!()
}
