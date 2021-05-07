extern crate proc_macro;
extern crate proc_macro2;
extern crate unicode_xid;

#[cfg(any(feature = "printing", feature = "parsing"))]
extern crate quote;

#[cfg_attr(feature = "parsing", macro_use)]
extern crate synom;

mod ty;

pub use synom::span::Span;
pub use synom::tokens;
pub use synom::delimited;

////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "parsing")]
use synom::{Synom};
