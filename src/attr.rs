use super::*;
use delimited::Delimited;

use std::iter;

use proc_macro2::{self, Delimiter, TokenNode, Spacing};

ast_struct! {
    /// Doc-comments are promoted to attributes that have `is_sugared_doc` = true
    pub struct Attribute {
        /// The path of the attribute.
        ///
        /// E.g. `derive` in `#[derive(Copy)]`
        /// E.g. `crate::precondition` in `#[crate::precondition x < 5]`
        pub path: Path,

        /// Any tokens after the path.
        ///
        /// E.g. `( Copy )` in `#[derive(Copy)]`
        /// E.g. `x < 5` in `#[crate::precondition x < 5]`
        pub tts: Vec<TokenTree>,
    }
}

ast_enum_of_structs! {
    /// A compile-time attribute item.
    ///
    /// E.g. `#[test]`, `#[derive(..)]` or `#[feature = "foo"]`
    pub enum MetaItem {
        /// Term meta item.
        ///
        /// E.g. `test` as in `#[test]`
        pub Term(Ident),

        /// List meta item.
        ///
        /// E.g. `derive(..)` as in `#[derive(..)]`
        pub List(MetaItemList {
            /// Name of this attribute.
            ///
            /// E.g. `derive` in `#[derive(..)]`
            pub ident: Ident,

            pub paren_token: tokens::Paren,

            /// Arguments to this attribute
            ///
            /// E.g. `..` in `#[derive(..)]`
            pub nested: Delimited<NestedMetaItem, tokens::Comma>,
        }),

        /// Name-value meta item.
        ///
        /// E.g. `feature = "foo"` as in `#[feature = "foo"]`
        pub NameValue(MetaNameValue {
            /// Name of this attribute.
            ///
            /// E.g. `feature` in `#[feature = "foo"]`
            pub ident: Ident,

            pub eq_token: tokens::Eq,
        }),
    }
}

impl MetaItem {
    /// Name of the item.
    ///
    /// E.g. `test` as in `#[test]`, `derive` as in `#[derive(..)]`, and
    /// `feature` as in `#[feature = "foo"]`.
    pub fn name(&self) -> &str {
        match *self {
            MetaItem::Term(ref name) => name.as_ref(),
            MetaItem::NameValue(ref pair) => pair.ident.as_ref(),
            MetaItem::List(ref list) => list.ident.as_ref(),
        }
    }
}

ast_enum_of_structs! {
    /// Possible values inside of compile-time attribute lists.
    ///
    /// E.g. the '..' in `#[name(..)]`.
    pub enum NestedMetaItem {
        /// A full `MetaItem`.
        ///
        /// E.g. `Copy` in `#[derive(Copy)]` would be a `MetaItem::Term(Ident::from("Copy"))`.
        pub MetaItem(MetaItem),
    }
}


#[cfg(feature = "parsing")]
pub mod parsing {
    use super::*;
    use synom::{PResult, Cursor, parse_error};
    use synom::tokens::*;
    use proc_macro2::{TokenNode, Spacing, TokenTree};

    fn eq() -> TokenTree {
        TokenTree {
            span: Default::default(),
            kind: TokenNode::Op('=', Spacing::Alone),
        }
    }

    impl Attribute {
        named!(pub parse_outer -> Self, alt!(
            do_parse!(
                pound: syn!(Pound) >>
                ({
                    let ((path, tts)) = panic!();

                    Attribute {
                        path: path,
                        tts: tts,
                    }
                })
            )
            |
            map!(
                lit_doc_comment,
                |lit| Attribute {
                    path: "doc".into(),
                    tts: vec![
                        ::TokenTree(eq()),
                        ::TokenTree(lit),
                    ],
                }
            )
        ));
    }

    fn lit_doc_comment(input: Cursor) -> PResult<TokenTree> {
        parse_error()
    }
}
