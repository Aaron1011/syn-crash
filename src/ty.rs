use delimited::Delimited;
use super::*;

ast_enum_of_structs! {
    /// The different kinds of types recognized by the compiler
    pub enum Ty {
        /// A bare function (e.g. `fn(usize) -> bool`)
        pub BareFn(TyBareFn {
            pub ty: Box<BareFnTy>,
        }),
        /// A tuple (`(A, B, C, D, ...)`)
        pub Tup(TyTup {
            pub paren_token: tokens::Paren,
            pub tys: Delimited<Ty, tokens::Comma>,
            pub lone_comma: Option<tokens::Comma>,
        }),
        /// A path (`module::module::...::Type`), optionally
        /// "qualified", e.g. `<Vec<T> as SomeTrait>::SomeType`.
        ///
        /// Type parameters are stored in the Path itself
        pub Path(TyPath {
            pub qself: Option<QSelf>,
            pub path: Path,
        }),
        /// A trait object type `Bound1 + Bound2 + Bound3`
        /// where `Bound` is a trait or a lifetime.
        pub TraitObject(TyTraitObject {
            pub bounds: Delimited<TyParamBound, tokens::Add>,
        }),
        /// No-op; kept solely so that we can pretty-print faithfully
        pub Paren(TyParen {
            pub paren_token: tokens::Paren,
            pub ty: Box<Ty>,
        }),
        /// No-op: kept solely so that we can pretty-print faithfully
        pub Group(TyGroup {
            pub group_token: tokens::Group,
            pub ty: Box<Ty>,
        }),
        /// TyKind::Infer means the type should be inferred instead of it having been
        /// specified. This can appear anywhere in a type.
        pub Infer(TyInfer {
            pub underscore_token: tokens::Underscore
        }),
    }
}

ast_struct! {
    pub struct MutTy {
        pub ty: Ty,
        pub mutability: Mutability,
    }
}

ast_enum! {
    #[cfg_attr(feature = "clone-impls", derive(Copy))]
    pub enum Mutability {
        Mutable(tokens::Mut),
        Immutable,
    }
}

ast_struct! {
    /// A "Path" is essentially Rust's notion of a name.
    ///
    /// It's represented as a sequence of identifiers,
    /// along with a bunch of supporting information.
    ///
    /// E.g. `std::cmp::PartialEq`
    pub struct Path {
        /// A `::foo` path, is relative to the crate root rather than current
        /// module (like paths in an import).
        pub leading_colon: Option<tokens::Colon2>,
        /// The segments in the path: the things separated by `::`.
        pub segments: Delimited<PathSegment, tokens::Colon2>,
    }
}

impl Path {
    pub fn global(&self) -> bool {
        self.leading_colon.is_some()
    }
}


impl<T> From<T> for Path
    where T: Into<PathSegment>
{
    fn from(segment: T) -> Self {
        Path {
            leading_colon: None,
            segments: vec![(segment.into(), None)].into(),
        }
    }
}

ast_struct! {
    /// A segment of a path: an identifier, an optional lifetime, and a set of types.
    ///
    /// E.g. `std`, `String` or `Box<T>`
    pub struct PathSegment {
        /// The identifier portion of this path segment.
        pub ident: Ident,
        /// Type/lifetime parameters attached to this path. They come in
        /// two flavors: `Path<A,B,C>` and `Path(A,B) -> C`. Note that
        /// this is more than just simple syntactic sugar; the use of
        /// parens affects the region binding rules, so we preserve the
        /// distinction.
        pub parameters: PathParameters,
    }
}

impl<T> From<T> for PathSegment
    where T: Into<Ident>
{
    fn from(ident: T) -> Self {
        PathSegment {
            ident: ident.into(),
            parameters: PathParameters::None,
        }
    }
}

ast_enum! {
    /// Parameters of a path segment.
    ///
    /// E.g. `<A, B>` as in `Foo<A, B>` or `(A, B)` as in `Foo(A, B)`
    pub enum PathParameters {
        None,
        /// The `<'a, A, B, C>` in `foo::bar::baz::<'a, A, B, C>`
        AngleBracketed(AngleBracketedParameterData),
        /// The `(A, B)` and `C` in `Foo(A, B) -> C`
        Parenthesized(ParenthesizedParameterData),
    }
}

impl Default for PathParameters {
    fn default() -> Self {
        PathParameters::None
    }
}

impl PathParameters {
    pub fn is_empty(&self) -> bool {
        match *self {
            PathParameters::None => true,
            PathParameters::AngleBracketed(ref bracketed) => {
                bracketed.lifetimes.is_empty() && bracketed.types.is_empty() &&
                bracketed.bindings.is_empty()
            }
            PathParameters::Parenthesized(_) => false,
        }
    }
}

ast_struct! {
    /// A path like `Foo<'a, T>`
    pub struct AngleBracketedParameterData {
        pub turbofish: Option<tokens::Colon2>,
        pub lt_token: tokens::Lt,
        /// The lifetime parameters for this path segment.
        pub lifetimes: Delimited<Lifetime, tokens::Comma>,
        /// The type parameters for this path segment, if present.
        pub types: Delimited<Ty, tokens::Comma>,
        /// Bindings (equality constraints) on associated types, if present.
        ///
        /// E.g., `Foo<A=Bar>`.
        pub bindings: Delimited<TypeBinding, tokens::Comma>,
        pub gt_token: tokens::Gt,
    }
}

ast_struct! {
    /// Bind a type to an associated type: `A=Foo`.
    pub struct TypeBinding {
        pub ident: Ident,
        pub eq_token: tokens::Eq,
        pub ty: Ty,
    }
}


ast_struct! {
    /// A path like `Foo(A,B) -> C`
    pub struct ParenthesizedParameterData {
        pub paren_token: tokens::Paren,
        /// `(A, B)`
        pub inputs: Delimited<Ty, tokens::Comma>,
        /// `C`
        pub output: FunctionRetTy,
    }
}

ast_struct! {
    pub struct PolyTraitRef {
        /// The `for<'a>` in `for<'a> Foo<&'a T>`
        pub bound_lifetimes: Option<BoundLifetimes>,
        /// The `Foo<&'a T>` in `<'a> Foo<&'a T>`
        pub trait_ref: Path,
    }
}

ast_struct! {
    /// The explicit Self type in a "qualified path". The actual
    /// path, including the trait and the associated item, is stored
    /// separately. `position` represents the index of the associated
    /// item qualified with this Self type.
    ///
    /// ```rust,ignore
    /// <Vec<T> as a::b::Trait>::AssociatedItem
    ///  ^~~~~     ~~~~~~~~~~~~~~^
    ///  ty        position = 3
    ///
    /// <Vec<T>>::AssociatedItem
    ///  ^~~~~    ^
    ///  ty       position = 0
    /// ```
    pub struct QSelf {
        pub lt_token: tokens::Lt,
        pub ty: Box<Ty>,
        pub position: usize,
        pub as_token: Option<tokens::As>,
        pub gt_token: tokens::Gt,
    }
}

ast_struct! {
    pub struct BareFnTy {
        pub lifetimes: Option<BoundLifetimes>,
        pub unsafety: Unsafety,
        pub fn_token: tokens::Fn_,
        pub paren_token: tokens::Paren,
        pub inputs: Delimited<BareFnArg, tokens::Comma>,
        pub variadic: Option<tokens::Dot3>,
        pub output: FunctionRetTy,
    }
}

ast_enum! {
    #[cfg_attr(feature = "clone-impls", derive(Copy))]
    pub enum Unsafety {
        Unsafe(tokens::Unsafe),
        Normal,
    }
}

ast_struct! {
    /// An argument in a function type.
    ///
    /// E.g. `bar: usize` as in `fn foo(bar: usize)`
    pub struct BareFnArg {
        pub name: Option<(BareFnArgName, tokens::Colon)>,
        pub ty: Ty,
    }
}

ast_enum! {
    /// Names of arguments in the `BareFnArg` structure
    pub enum BareFnArgName {
        /// Argument with the provided name
        Named(Ident),
        /// Argument matched with `_`
        Wild(tokens::Underscore),
    }
}

ast_enum! {
    pub enum FunctionRetTy {
        /// Return type is not specified.
        ///
        /// Functions default to `()` and
        /// closures default to inference. Span points to where return
        /// type would be inserted.
        Default,
        /// Everything else
        Ty(Ty, tokens::RArrow),
    }
}

#[cfg(feature = "parsing")]
pub mod parsing {
    use super::*;
    use synom::Synom;
    use synom::tokens::*;

    impl Synom for Ty {
        named!(parse -> Self, call!(ambig_ty, true));

        fn description() -> Option<&'static str> {
            Some("type")
        }
    }

    impl Ty {
        /// In some positions, types may not contain the `+` character, to
        /// disambiguate them. For example in the expression `1 as T`, T may not
        /// contain a `+` character.
        ///
        /// This parser does not allow a `+`, while the default parser does.
        named!(pub without_plus -> Self, call!(ambig_ty, false));
    }

    named!(ambig_ty(allow_plus: bool) -> Ty, alt!(
        syn!(TyGroup) => { Ty::Group }
    ));

    impl Synom for TyInfer {
        named!(parse -> Self, map!(
            syn!(Underscore),
            |u| TyInfer { underscore_token: u }
        ));
    }

    impl Synom for TyTup {
        named!(parse -> Self, do_parse!(
            data: parens!(call!(Delimited::parse_terminated)) >>
            (TyTup {
                tys: data.0,
                paren_token: data.1,
                lone_comma: None, // TODO: does this just not parse?
            })
        ));
    }

    impl Synom for ParenthesizedParameterData {
        named!(parse -> Self, do_parse!(
            data: parens!(call!(Delimited::parse_terminated)) >>
            output: syn!(FunctionRetTy) >>
            (ParenthesizedParameterData {
                paren_token: data.1,
                inputs: data.0,
                output: output,
            })
        ));
    }

    impl Synom for FunctionRetTy {
        named!(parse -> Self, alt!(
            do_parse!(
                arrow: syn!(RArrow) >>
                ty: syn!(Ty) >>
                (FunctionRetTy::Ty(ty, arrow))
            )
            |
            epsilon!() => { |_| FunctionRetTy::Default }
        ));
    }

    // Only allow multiple trait references if allow_plus is true.
    named!(ty_poly_trait_ref(allow_plus: bool) -> Ty, alt!(
        cond_reduce!(allow_plus, call!(Delimited::parse_separated_nonempty)) => {
            |x| TyTraitObject { bounds: x }.into()
        }
        |
        syn!(TyParamBound) => {
            |x| TyTraitObject { bounds: vec![x].into() }.into()
        }
    ));

    impl Synom for TyGroup {
        named!(parse -> Self, do_parse!(
            data: grouped!(syn!(Ty)) >>
            (TyGroup {
                group_token: data.1,
                ty: Box::new(data.0),
            })
        ));
    }

    impl Synom for TyParen {
        named!(parse -> Self, do_parse!(
            data: parens!(syn!(Ty)) >>
            (TyParen {
                paren_token: data.1,
                ty: Box::new(data.0),
            })
        ));
    }

    named!(ty_no_eq_after -> Ty, terminated!(syn!(Ty), not!(syn!(Eq))));

    impl Path {
        named!(pub parse_mod_style -> Self, do_parse!(
            colon: option!(syn!(Colon2)) >>
            segments: call!(Delimited::parse_separated_nonempty_with,
                            mod_style_path_segment) >>
            (Path {
                leading_colon: colon,
                segments: segments,
            })
        ));
    }

    named!(mod_style_path_segment -> PathSegment, alt!(
        map!(syn!(Ident), Into::into)
        |
        alt!(
            syn!(Super) => { Into::into }
            |
            syn!(Self_) => { Into::into }
            |
            syn!(CapSelf) => { Into::into }
        )
    ));
}

