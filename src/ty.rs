use delimited::Delimited;
use super::*;

ast_enum_of_structs! {
    /// The different kinds of types recognized by the compiler
    pub enum Ty {
        /// A tuple (`(A, B, C, D, ...)`)
        pub Tup(TyTup {
            pub tys: Delimited<Ty, tokens::Comma>,
        }),
        /// A trait object type `Bound1 + Bound2 + Bound3`
        /// where `Bound` is a trait or a lifetime.
        pub TraitObject(TyTraitObject {
            pub bounds: Delimited<PolyTraitRef, tokens::Add>,
        }),
        /// No-op: kept solely so that we can pretty-print faithfully
        pub Group(TyGroup {
            pub ty: Box<Ty>,
        }),
    }
}

ast_struct! {
    /// A path like `Foo(A,B) -> C`
    pub struct ParenthesizedParameterData {
        pub inputs: Delimited<Ty, tokens::Comma>,
    }
}

ast_struct! {
    pub struct PolyTraitRef {
        /// The `for<'a>` in `for<'a> Foo<&'a T>`
        pub bound_lifetimes: Delimited<Vec<TokenTree>, tokens::Comma>
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

    named!(ambig_ty(allow_plus: bool) -> Ty, alt!(
        syn!(TyGroup) => { Ty::Group }
    ));

    impl Synom for TyTup {
        named!(parse -> Self, do_parse!(
            data: parens!(call!(Delimited::parse_terminated)) >>
            (TyTup {
                tys: data.0,
            })
        ));
    }

    impl Synom for ParenthesizedParameterData {
        named!(parse -> Self, do_parse!(
            data: parens!(call!(Delimited::parse_terminated)) >>
            (ParenthesizedParameterData {
                inputs: data.0,
            })
        ));
    }

    impl Synom for TyGroup {
        named!(parse -> Self, do_parse!(
            data: grouped!(syn!(Ty)) >>
            (TyGroup {
                ty: Box::new(data.0),
            })
        ));
    }
}


