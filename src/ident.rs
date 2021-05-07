use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt::{self, Display};
use std::hash::{Hash, Hasher};

use proc_macro2::Term;
use unicode_xid::UnicodeXID;

use Span;
use tokens;

/// A word of Rust code, such as a keyword or variable name.
///
/// An identifier consists of at least one Unicode code point, the first of
/// which has the XID_Start property and the rest of which have the XID_Continue
/// property. An underscore may be used as the first character as long as it is
/// not the only character.
///
/// - The empty string is not an identifier. Use `Option<Ident>`.
/// - An underscore by itself is not an identifier. Use
///   `syn::tokens::Underscore` instead.
/// - A lifetime is not an identifier. Use `syn::Lifetime` instead.
///
/// An identifier constructed with `Ident::new` is permitted to be a Rust
/// keyword, though parsing input with [`parse`], [`parse_str`] or
/// [`parse_tokens`] rejects Rust keywords.
///
/// [`parse`]: fn.parse.html
/// [`parse_str`]: fn.parse_str.html
/// [`parse_tokens`]: fn.parse_tokens.html
///
/// # Examples
///
/// A new ident can be created from a string using the `Ident::from` function.
///
/// ```rust
/// extern crate syn;
/// use syn::Ident;
/// #
/// # fn main() {
///
/// let ident = Ident::from("another_identifier");
///
/// # }
/// ```
///
/// When the ident is used in Macros 1.1 output, it needs to be turned into
/// a token stream. This is easy to do using the `quote!` macro from the `quote`
/// crate.
///
/// ```rust
/// # #[macro_use]
/// # extern crate quote;
/// # extern crate syn;
/// # use syn::Ident;
/// # fn main() {
/// # let ident = Ident::from("another_identifier");
/// #
/// // Create tokens using the ident.
/// let expanded = quote! { let #ident = 10; };
///
/// // Derive a new ident from the existing one.
/// let temp_ident = Ident::from(format!("new_{}", ident));
/// let expanded = quote! { let $temp_ident = 10; };
///
/// # }
/// ```
///
/// If `syn` is used to parse existing Rust source code, it is often useful to
/// convert the `Ident` to a more generic string data type at some point. The
/// methods `as_ref()` and `to_string()` achieve this.
/// 
/// ```rust
/// # use syn::Ident;
/// # let ident = Ident::from("another_identifier");
/// #
/// // Examine the ident as a &str.
/// let ident_str = ident.as_ref();
/// if ident_str.len() > 60 {
///     println!("Very long identifier: {}", ident_str)
/// }
///
/// // Create a String from the ident.
/// let ident_string = ident.to_string();
/// give_away(ident_string);
///
/// fn give_away(s: String) { /* ... */ }
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Ident {
    pub sym: Term,
    pub span: Span,
}
