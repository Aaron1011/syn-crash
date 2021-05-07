use delimited::Delimited;
use super::*;

pub enum Ty {
	/// A tuple (`(A, B, C, D, ...)`)
	Tup(TyTup),
	/// A trait object type `Bound1 + Bound2 + Bound3`
	/// where `Bound` is a trait or a lifetime.
	TraitObject(TyTraitObject),
	/// No-op: kept solely so that we can pretty-print faithfully
	Group(TyGroup),
}
/// A tuple (`(A, B, C, D, ...)`)
pub struct TyTup {
	pub tys: Vec<Ty>,
}
/// A trait object type `Bound1 + Bound2 + Bound3`
/// where `Bound` is a trait or a lifetime.
pub struct TyTraitObject {
	pub bounds: TokenTree,
}
/// No-op: kept solely so that we can pretty-print faithfully
pub struct TyGroup {
	pub ty: Box<Ty>,
}


pub struct ParenthesizedParameterData {
	pub inputs: Delimited<Ty, tokens::Comma>,
}

#[cfg(feature = "parsing")]
pub mod parsing {
    use super::*;
    use synom::Synom;
    use synom::tokens::*;

    impl Synom for Ty {
        fn parse(i: ::synom::Cursor) -> ::synom::PResult<Self> {
            <TyGroup as ::synom::Synom>::parse(i);
            panic!()
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
			<Ty as ::synom::Synom>::parse(i);
            panic!()
		}
	}
}


