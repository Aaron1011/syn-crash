pub struct TokenTree {
    pub field: Vec<TokenTree>
}

pub trait Synom {
    fn parse();
}

pub struct Tokens {
    tts: Vec<TokenTree>,
}

pub struct Delimited<T> {
    inner: Vec<(T, Option<()>)>
}

impl<T> Delimited<T> {
    pub fn push_first(&mut self, token: T) {
        self.inner.push((token, None));
    }
}



fn make_ty() -> Ty {
    panic!()
}

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


impl Synom for Ty {
    fn parse() {
        <TyGroup as Synom>::parse();
    }
}

pub fn parse_test() {
    let mut res: Delimited<Ty> =         Delimited {
            inner: Vec::new(),
        };

    Ty::parse();
    res.push_first(make_ty());
}

impl Synom for TyGroup {
    fn parse() {
        <Ty as Synom>::parse();
        panic!()
    }
}

pub fn to_tokens(tokens: &mut Tokens) {}
