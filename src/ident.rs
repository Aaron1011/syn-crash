use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt::{self, Display};
use std::hash::{Hash, Hasher};

use proc_macro2::Term;
use unicode_xid::UnicodeXID;

use Span;
use tokens;

#[derive(Copy, Clone, Debug)]
pub struct Ident {
    pub sym: Term,
    pub span: Span,
}
