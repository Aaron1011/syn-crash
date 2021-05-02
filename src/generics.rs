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
        pub ty_params: Delimited<TyParam, tokens::Comma>,
        pub where_clause: WhereClause,
    }
}

#[cfg(feature = "printing")]
#[cfg_attr(feature = "extra-traits", derive(Debug, Eq, PartialEq, Hash))]
#[cfg_attr(feature = "clone-impls", derive(Clone))]
/// Returned by `Generics::split_for_impl`.
pub struct ImplGenerics<'a>(&'a Generics);

#[cfg(feature = "printing")]
#[cfg_attr(feature = "extra-traits", derive(Debug, Eq, PartialEq, Hash))]
#[cfg_attr(feature = "clone-impls", derive(Clone))]
/// Returned by `Generics::split_for_impl`.
pub struct TyGenerics<'a>(&'a Generics);

#[cfg(feature = "printing")]
impl Generics {
    /// Split a type's generics into the pieces required for impl'ing a trait
    /// for that type.
    ///
    /// ```
    /// # extern crate syn;
    /// # #[macro_use]
    /// # extern crate quote;
    /// # fn main() {
    /// # let generics: syn::Generics = Default::default();
    /// # let name = syn::Ident::from("MyType");
    /// let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    /// quote! {
    ///     impl #impl_generics MyTrait for #name #ty_generics #where_clause {
    ///         // ...
    ///     }
    /// }
    /// # ;
    /// # }
    /// ```
    pub fn split_for_impl(&self) -> (ImplGenerics, TyGenerics, &WhereClause) {
        (ImplGenerics(self), TyGenerics(self), &self.where_clause)
    }
}

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
        pub lifetime: Lifetime,
        pub colon_token: Option<tokens::Colon>,
        pub bounds: Delimited<Lifetime, tokens::Add>,
    }
}

impl LifetimeDef {
    pub fn new(lifetime: Lifetime) -> Self {
        LifetimeDef {
            attrs: Vec::new(),
            lifetime: lifetime,
            colon_token: None,
            bounds: Delimited::new(),
        }
    }
}

ast_struct! {
    /// A generic type parameter, e.g. `T: Into<String>`.
    pub struct TyParam {
        pub attrs: Vec<Attribute>,
        pub ident: Ident,
        pub colon_token: Option<tokens::Colon>,
        pub bounds: Delimited<TyParamBound, tokens::Add>,
        pub eq_token: Option<tokens::Eq>,
        pub default: Option<Ty>,
    }
}

impl From<Ident> for TyParam {
    fn from(ident: Ident) -> Self {
        TyParam {
            attrs: vec![],
            ident: ident,
            colon_token: None,
            bounds: Delimited::new(),
            eq_token: None,
            default: None,
        }
    }
}

ast_enum! {
    /// The AST represents all type param bounds as types.
    /// `typeck::collect::compute_bounds` matches these against
    /// the "special" built-in traits (see `middle::lang_items`) and
    /// detects Copy, Send and Sync.
    pub enum TyParamBound {
        Trait(PolyTraitRef, TraitBoundModifier),
        Region(Lifetime),
    }
}

ast_enum! {
    /// A modifier on a bound, currently this is only used for `?Sized`, where the
    /// modifier is `Maybe`. Negative bounds should also be handled here.
    #[cfg_attr(feature = "clone-impls", derive(Copy))]
    pub enum TraitBoundModifier {
        None,
        Maybe(tokens::Question),
    }
}

ast_struct! {
    /// A `where` clause in a definition
    #[derive(Default)]
    pub struct WhereClause {
        pub where_token: Option<tokens::Where>,
    }
}

impl WhereClause {
    pub fn none() -> Self {
        WhereClause::default()
    }
}

ast_enum_of_structs! {
    /// A single predicate in a `where` clause
    pub enum WherePredicate {
        /// A type binding, e.g. `for<'c> Foo: Send+Clone+'c`
        pub BoundPredicate(WhereBoundPredicate {
            /// Any lifetimes from a `for` binding
            pub bound_lifetimes: Option<BoundLifetimes>,
            /// The type being bounded
            pub bounded_ty: Ty,
            pub colon_token: tokens::Colon,
            /// Trait and lifetime bounds (`Clone+Send+'static`)
            pub bounds: Delimited<TyParamBound, tokens::Add>,
        }),

        /// A lifetime predicate, e.g. `'a: 'b+'c`
        pub RegionPredicate(WhereRegionPredicate {
            pub lifetime: Lifetime,
            pub colon_token: Option<tokens::Colon>,
            pub bounds: Delimited<Lifetime, tokens::Add>,
        }),

        /// An equality predicate (unsupported)
        pub EqPredicate(WhereEqPredicate {
            pub lhs_ty: Ty,
            pub eq_token: tokens::Eq,
            pub rhs_ty: Ty,
        }),
    }
}
