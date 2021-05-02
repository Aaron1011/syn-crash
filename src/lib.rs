#![doc(html_root_url = "https://dtolnay.github.io/syn")]
#![allow(warnings)]

#![cfg_attr(feature = "cargo-clippy", allow(large_enum_variant))]

extern crate proc_macro;
extern crate proc_macro2;
extern crate unicode_xid;

#[cfg(any(feature = "printing", feature = "parsing"))]
extern crate quote;

#[cfg_attr(feature = "parsing", macro_use)]
extern crate synom;

#[macro_use]
mod macros;

mod attr;
pub use attr::{Attribute, MetaItem, NestedMetaItem, MetaItemList,
               MetaNameValue};


mod generics;
pub use generics::{Generics, LifetimeDef, TraitBoundModifier, TyParam, TyParamBound,
                   BoundLifetimes};

mod ident;
pub use ident::Ident;

mod lifetime;
pub use lifetime::Lifetime;

mod mac;
pub use mac::{TokenTree};

mod ty;
pub use ty::{AngleBracketedParameterData, BareFnArg, BareFnArgName, BareFnTy,
             FunctionRetTy, ParenthesizedParameterData, Path,
             PathParameters, PathSegment, PolyTraitRef, QSelf, Ty, TypeBinding, Unsafety,
             TyBareFn, 
             TyTraitObject, TyGroup};

pub use synom::span::Span;
pub use synom::tokens;
pub use synom::delimited;

////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "parsing")]
pub use synom::ParseError;

#[cfg(feature = "parsing")]
use synom::{Synom, SynomBuffer};
