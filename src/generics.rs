use super::*;
use delimited::Delimited;

ast_struct! {
    /// Represents lifetimes and type parameters attached to a declaration
    /// of a function, enum, trait, etc.
    #[derive(Default)]
    pub struct Generics {
        pub lt_token: Option<tokens::Lt>,
        pub gt_token: Option<tokens::Gt>,
        pub lifetimes: Delimited<LifetimeDef, tokens::Comma>,
        pub ty_params: Delimited<(), tokens::Comma>,
    }
}

#[cfg(feature = "printing")]
#[cfg_attr(feature = "extra-traits", derive(Debug, Eq, PartialEq, Hash))]
#[cfg_attr(feature = "clone-impls", derive(Clone))]
/// Returned by `Generics::split_for_impl`.
pub struct ImplGenerics<'a>(&'a Generics);

ast_struct! {
    /// A set of bound lifetimes, e.g. `for<'a, 'b, 'c>`
    #[derive(Default)]
    pub struct BoundLifetimes {
        pub for_token: tokens::For,
        pub lt_token: tokens::Lt,
        pub lifetimes: Delimited<LifetimeDef, tokens::Comma>,
        pub gt_token: tokens::Gt,
    }
}

ast_struct! {
    /// A lifetime definition, e.g. `'a: 'b+'c+'d`
    pub struct LifetimeDef {
        pub attrs: Vec<Attribute>,
        pub bounds: Delimited<Lifetime, tokens::Add>,
    }
}

impl LifetimeDef {
    pub fn new(lifetime: Lifetime) -> Self {
        LifetimeDef {
            attrs: Vec::new(),
            bounds: Delimited::new(),
        }
    }
}

ast_enum! {
    /// The AST represents all type param bounds as types.
    /// `typeck::collect::compute_bounds` matches these against
    /// the "special" built-in traits (see `middle::lang_items`) and
    /// detects Copy, Send and Sync.
    pub enum TyParamBound {
        Trait(PolyTraitRef),
        Region(Lifetime),
    }
}
