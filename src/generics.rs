use super::*;
use delimited::Delimited;


ast_struct! {
    /// A lifetime definition, e.g. `'a: 'b+'c+'d`
    pub struct LifetimeDef {
        pub attrs: Vec<TokenTree>,
        pub bounds: Delimited<(), tokens::Add>,
    }
}
