use delimited::Delimited;
use super::*;

ast_enum_of_structs! {
    /// The different kinds of types recognized by the compiler
    pub enum Ty {
        /// A tuple (`(A, B, C, D, ...)`)
        pub Tup(TyTup {
            pub tys: Vec<Ty>,
        }),
        /// A trait object type `Bound1 + Bound2 + Bound3`
        /// where `Bound` is a trait or a lifetime.
        pub TraitObject(TyTraitObject {
            pub bounds: TokenTree
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

#[cfg(feature = "parsing")]
pub mod parsing {
    use super::*;
    use synom::Synom;
    use synom::tokens::*;

    impl Synom for Ty {
        named!(parse -> Self, call!(ambig_ty, true));
    }

    named!(ambig_ty(allow_plus: bool) -> Ty, alt!(
        syn!(TyGroup) => { Ty::Group }
    ));


    impl Synom for ParenthesizedParameterData {
        fn parse(i: ::synom::Cursor) -> ::synom::PResult<Self> {
            match ::synom::tokens::Paren::parse(i, |i| Delimited::parse_terminated(i)) {
                ::std::result::Result::Err(err) => panic!(),
                ::std::result::Result::Ok((i, o)) => {
                    let data = o;
                    ::std::result::Result::Ok((
                        i,
                        (ParenthesizedParameterData { inputs: data.0 }),
                    ))
                }
            }
        }
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


