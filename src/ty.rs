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
        fn parse(i: ::synom::Cursor) -> ::synom::PResult<Self> {
            ambig_ty(i, true)
        }
    }

    fn ambig_ty(i: ::synom::Cursor, allow_plus: bool) -> ::synom::PResult<Ty> {
        match <TyGroup as ::synom::Synom>::parse(i) {
            ::std::result::Result::Ok((i, o)) =>  panic!(),
            ::std::result::Result::Err(err) => ::std::result::Result::Err(err),
        }
    }


    impl Synom for ParenthesizedParameterData {
        fn parse(i: ::synom::Cursor) -> ::synom::PResult<Self> {
            match ::synom::tokens::Paren::parse(i, |i| Delimited::parse_terminated(i)) {
                ::std::result::Result::Err(err) => panic!(),
                ::std::result::Result::Ok((i, o)) => {
                    ParenthesizedParameterData { inputs: o.0 };
                    panic!()
                }
            }
        }
    }

	impl Synom for TyGroup {
		fn parse(i: ::synom::Cursor) -> ::synom::PResult<Self> {
			match ::synom::tokens::Group::parse(i, |i| <Ty as ::synom::Synom>::parse(i)) {
				::std::result::Result::Err(err) => ::std::result::Result::Err(err),
				::std::result::Result::Ok((i, o)) => {
					TyGroup {
							ty: Box::new(o.0),
					};
                    panic!()
				}
			}
		}
	}
}


