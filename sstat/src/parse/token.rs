//! Tokens for keywords, delimiters, and punctuation

use super::Span;

macro_rules! define_delimiters {
	($($token:literal $name:ident #[$doc:meta])*) => {
		$(
			#[$doc]
			#[allow(missing_docs)]
			#[derive(Clone, Copy, Debug, PartialEq, Eq)]
			pub(crate) struct $name {
				pub(crate) span: Span
			}
		)*
	};
}

macro_rules! define_keywords {
	($($token:literal $name:ident #[$doc:meta])*) => {
		$(
			#[doc = concat!('`', $token, '`')]
			#[$doc]
			#[allow(missing_docs)]
			#[derive(Clone, Copy, Debug, PartialEq, Eq)]
			pub(crate) struct $name {
				pub(crate) span: Span
			}
		)*
	};
}

define_delimiters! {
    '(' LParen   /// `(`
    ')' RParen   /// ')'
    '[' LBracket /// '['
    ']' RBracket /// ']'
}

define_keywords! {
    "doc"   Doc   /// The top level document node
    "sec"   Sec   /// A section
    "title" Title /// A title
    "p"     P     /// A paragraph
}
