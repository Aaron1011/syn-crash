use super::*;
use delimited::Delimited;

use std::iter;

use proc_macro2::{self, Delimiter, TokenNode, Spacing};

ast_struct! {
    /// Doc-comments are promoted to attributes that have `is_sugared_doc` = true
    pub struct Attribute {
        pub style: AttrStyle,
        pub pound_token: tokens::Pound,
        pub bracket_token: tokens::Bracket,

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

        pub is_sugared_doc: bool,
    }
}

impl Attribute {
    /// Parses the tokens after the path as a [`MetaItem`](enum.MetaItem.html) if possible.
    pub fn meta_item(&self) -> Option<MetaItem> {
        None
    }
}

fn nested_meta_item_from_tokens(tts: &[proc_macro2::TokenTree])
    -> Option<(NestedMetaItem, &[proc_macro2::TokenTree])>
{
    None
}

fn list_of_nested_meta_items_from_tokens(mut tts: &[proc_macro2::TokenTree])
    -> Option<Delimited<NestedMetaItem, tokens::Comma>>
{
    let mut delimited = Delimited::new();
    let mut first = true;

    while !tts.is_empty() {
        let prev_comma = if first {
            first = false;
            None
        } else if let TokenNode::Op(',', Spacing::Alone) = tts[0].kind {
            let tok = tokens::Comma([Span(tts[0].span)]);
            tts = &tts[1..];
            if tts.is_empty() {
                break
            }
            Some(tok)
        } else {
            return None
        };
        let (nested, rest) = match nested_meta_item_from_tokens(tts) {
            Some(pair) => pair,
            None => return None,
        };
        match prev_comma {
            Some(comma) => delimited.push_next(nested, comma),
            None => delimited.push_first(nested),
        }
        tts = rest;
    }

    Some(delimited)
}


ast_enum! {
    /// Distinguishes between Attributes that decorate items and Attributes that
    /// are contained as statements within items. These two cases need to be
    /// distinguished for pretty-printing.
    #[cfg_attr(feature = "clone-impls", derive(Copy))]
    pub enum AttrStyle {
        /// Attribute of the form `#[...]`.
        Outer,

        /// Attribute of the form `#![...]`.
        Inner(tokens::Bang),
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

            /// Arguments to this attribute
            ///
            /// E.g. `"foo"` in `#[feature = "foo"]`
            pub lit: Lit,
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

        /// A Rust literal.
        ///
        /// E.g. `"name"` in `#[rename("name")]`.
        pub Literal(Lit),
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
                path_and_tts: brackets!(tuple!(
                    call!(::Path::parse_mod_style),
                    call!(::TokenTree::parse_list)
                )) >>
                ({
                    let ((path, tts), bracket) = path_and_tts;

                    Attribute {
                        style: AttrStyle::Outer,
                        path: path,
                        tts: tts,
                        is_sugared_doc: false,
                        pound_token: pound,
                        bracket_token: bracket,
                    }
                })
            )
            |
            map!(
                lit_doc_comment,
                |lit| Attribute {
                    style: AttrStyle::Outer,
                    path: "doc".into(),
                    tts: vec![
                        ::TokenTree(eq()),
                        ::TokenTree(lit),
                    ],
                    is_sugared_doc: true,
                    pound_token: tokens::Pound::default(),
                    bracket_token: tokens::Bracket::default(),
                }
            )
        ));
    }

    fn lit_doc_comment(input: Cursor) -> PResult<TokenTree> {
        parse_error()
    }
}
