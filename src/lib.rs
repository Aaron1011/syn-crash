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
pub use attr::{Attribute, AttrStyle, MetaItem, NestedMetaItem, MetaItemList,
               MetaNameValue};

mod data;
pub use data::{Field, Variant, VariantData, Visibility, VisRestricted, VisCrate,
               VisPublic, VisInherited};

mod generics;
pub use generics::{Generics, LifetimeDef, TraitBoundModifier, TyParam, TyParamBound,
                   WhereBoundPredicate, WhereClause, WhereEqPredicate, WherePredicate,
                   WhereRegionPredicate, BoundLifetimes};
#[cfg(feature = "printing")]
pub use generics::{ImplGenerics, Turbofish, TyGenerics};

mod ident;
pub use ident::Ident;

mod lifetime;
pub use lifetime::Lifetime;

mod lit;
pub use lit::{Lit, LitKind};

mod mac;
pub use mac::{Mac, TokenTree};

mod op;
pub use op::{BinOp, UnOp};

mod ty;
pub use ty::{Abi, AbiKind, AngleBracketedParameterData, BareFnArg, BareFnArgName, BareFnTy,
             FunctionRetTy, MutTy, Mutability, ParenthesizedParameterData, Path,
             PathParameters, PathSegment, PolyTraitRef, QSelf, Ty, TypeBinding, Unsafety,
             TySlice, TyArray, TyPtr, TyRptr, TyBareFn, TyNever, TyTup, TyPath,
             TyTraitObject, TyImplTrait, TyParen, TyInfer, TyGroup};
#[cfg(feature = "printing")]
pub use ty::PathTokens;

pub use synom::span::Span;
pub use synom::tokens;
pub use synom::delimited;

mod gen {
    #[cfg(feature = "visit")]
    pub mod visit;

    #[cfg(feature = "visit_mut")]
    pub mod visit_mut;

    #[cfg(feature = "fold")]
    pub mod fold;
}
pub use gen::*;

////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "parsing")]
pub use synom::ParseError;

#[cfg(feature = "parsing")]
use synom::{Synom, SynomBuffer};


#[cfg(feature = "printing")]
struct TokensOrDefault<'a, T: 'a>(&'a Option<T>);

#[cfg(feature = "printing")]
impl<'a, T> quote::ToTokens for TokensOrDefault<'a, T>
    where T: quote::ToTokens + Default,
{
    fn to_tokens(&self, tokens: &mut quote::Tokens) {
        match *self.0 {
            Some(ref t) => t.to_tokens(tokens),
            None => T::default().to_tokens(tokens),
        }
    }
}
