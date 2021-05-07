pub struct Lifetime {}

#[cfg(feature = "printing")]
mod printing {
    use super::*;
    use quote::{Tokens, ToTokens};
    use proc_macro2::{TokenTree, TokenNode};

    impl ToTokens for Lifetime {
        fn to_tokens(&self, tokens: &mut Tokens) {
        }
    }
}

fn dummy() -> proc_macro2::TokenTree {
    panic!()
}
