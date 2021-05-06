use super::*;
use delimited::Delimited;

ast_struct! {
    /// Represents lifetimes and type parameters attached to a declaration
    /// of a function, enum, trait, etc.
    #[derive(Default)]
    pub struct Generics {
        pub lifetimes: Delimited<LifetimeDef, tokens::Comma>,
    }
}
ast_struct! {
    /// A lifetime definition, e.g. `'a: 'b+'c+'d`
    pub struct LifetimeDef {
        pub attrs: Vec<TokenTree>,
        pub bounds: Delimited<(), tokens::Add>,
    }
}
