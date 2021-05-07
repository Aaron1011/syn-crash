extern crate proc_macro2;

#[cfg(any(feature = "printing", feature = "parsing"))]
extern crate quote;

#[cfg_attr(feature = "parsing", macro_use)]
extern crate synom;

mod ty;

pub use synom::tokens;
pub use synom::delimited;

////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "parsing")]
use synom::{Synom};
