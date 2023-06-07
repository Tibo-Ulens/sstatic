//! AST types for document nodes

use super::*;

mod node_inner;
pub(crate) use node_inner::*;

/// A generic identifier
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub(crate) struct Identifier<'s> {
    pub(crate) name: &'s str,

    pub(crate) span: Span,
}

impl Parser {
    /// Returns the longest input slice that matches the requirements for an
    /// identifier
    fn take_identifier() -> impl Fn(&str, usize) -> ParseResult<&str> {
        move |input: &str, start: usize| {
            if input.is_empty() {
                return Err(ErrorKind::Failure(
                    ParseError::ExpectedIdentifier {
                        found: "end-of-file".to_string(),
                    }
                    .into(),
                ));
            }

            let id_start = input.chars().next().unwrap();
            if !UnicodeXID::is_xid_start(id_start) {
                return Err(ErrorKind::Error(
                    ParseError::ExpectedIdentifier {
                        found: id_start.to_string(),
                    }
                    .into(),
                ));
            }

            Self::take_while(UnicodeXID::is_xid_continue)(input, start)
        }
    }
}

/// A slice of text
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub(crate) struct Text<'s> {
    pub(crate) text: &'s str,

    pub(crate) span: Span,
}

impl Parser {
    /// Keep taking text until an unescaped `(` is found
    fn take_text() -> impl Fn(&str, usize) -> ParseResult<&str> {
        move |input: &str, start: usize| {
            if input.is_empty() {
                return Err(ErrorKind::Failure(
                    ParseError::UnexpectedToken {
                        expected: "TEXT".to_string(),
                        found: "end-of-file".to_string(),
                    }
                    .into(),
                ));
            }

            let mut idx = start;
            let mut chars = input.chars();
            let mut prev = chars.next().unwrap();

            for curr in chars {
                if curr == '(' && prev != '\\' {
                    break;
                }

                prev = curr;
                idx += 1;
            }

            let rest = unsafe { input.get_unchecked(idx..) };
            let text = unsafe { input.get_unchecked(..idx) };
            let span = Span::new(start, idx);

            Ok((rest, (text, span)))
        }
    }
}

/// The root node of the AST, represents a single page
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Page<'s> {
    /// The global attributes of this page
    pub(crate) attributes: Vec<Attribute<'s>>,
    /// The document node of this page
    pub(crate) doc: DocNode<'s>,

    pub(crate) span: Span,
}

impl Parser {
    /// Parse a single page
    pub(crate) fn parse_page<'p, 'i: 'p>(input: &'i str) -> ParseResult<Page<'i>> {
        let start = 0;

        let (rest, (_, span)) = Self::take_non_parseable()(input, start)?;
        let mut global_span = span;

        let (rest, (attributes, span)) = Self::many(Self::parse_attribute())(rest, span.end)?;
        let attributes = attributes.into_iter().map(|(a, _)| a).collect();

        let (rest, (doc, span)) = Self::parse_doc_node()(rest, span.end)?;

        global_span.end = span.end;

        let page = Page {
            attributes,
            doc,
            span: global_span,
        };

        Ok((rest, (page, global_span)))
    }
}

/// A list of attributes for a given node
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Attribute<'s> {
    pub(crate) lbracket: LBracket,
    pub(crate) attribute_name: Identifier<'s>,
    pub(crate) attribute_value: Text<'s>,
    pub(crate) rbracket: RBracket,

    pub(crate) span: Span,
}

impl Parser {
    /// Parse an [`Attribute`]
    ///
    /// ```ebnf
    /// attribute = "[", attribute_name, attribute_value "]";
    /// ```
    pub(crate) fn parse_attribute<'i>() -> impl Fn(&'i str, usize) -> ParseResult<Attribute<'i>> {
        move |input: &str, start: usize| {
            let (rest, (_, span)) = Self::take_non_parseable()(input, start)?;
            let mut global_span = span;

            // "["
            let (rest, (_tag, span)) = Self::tag("[")(rest, span.end)?;
            let lbracket = LBracket { span };

            let (rest, (_, span)) = Self::take_non_parseable()(rest, span.end)?;

            // attribute_name
            let (rest, (name, span)) = Self::take_identifier()(rest, span.end)?;
            let attribute_name = Identifier { name, span };

            let (rest, (_, span)) = Self::take_non_parseable()(rest, span.end)?;

            // attribute_value
            let (rest, (text, span)) = Self::take_while(|c| c != ']')(rest, span.end)?;
            let attribute_value = Text { text, span };

            let (rest, (_, span)) = Self::take_non_parseable()(rest, span.end)?;

            // "]"
            let (rest, (_tag, span)) = Self::tag("]")(rest, span.end)?;
            let rbracket = RBracket { span };

            global_span.end = span.end;

            let attribute = Attribute {
                lbracket,
                attribute_name,
                attribute_value,
                rbracket,
                span: global_span,
            };

            Ok((rest, (attribute, global_span)))
        }
    }
}

/// A document node, contains all the markup of a single page
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DocNode<'s> {
    pub(crate) lparen: LParen,
    pub(crate) doc: Doc,
    pub(crate) attributes: Vec<Attribute<'s>>,
    pub(crate) nodes: Vec<Node<'s>>,
    pub(crate) rparen: RParen,

    pub(crate) span: Span,
}

impl Parser {
    /// Parse a doc node
    ///
    /// ```ebnf
    /// doc = "(", "doc", { attribute }, { node }, ")"
    /// ```
    pub(crate) fn parse_doc_node<'i>() -> impl Fn(&'i str, usize) -> ParseResult<DocNode<'i>> {
        move |input: &str, start: usize| {
            let (rest, (_, span)) = Self::take_non_parseable()(input, start)?;
            let mut global_span = span;

            // "("
            let (rest, (_tag, span)) = Self::tag("(")(rest, span.end)?;
            let lparen = LParen { span };

            let (rest, (_, span)) = Self::take_non_parseable()(rest, span.end)?;

            // "doc"
            let (rest, (_tag, span)) = Self::tag("doc")(rest, span.end)?;
            let doc = Doc { span };

            let (rest, (_, span)) = Self::take_non_parseable()(rest, span.end)?;

            // { attribute }
            let (rest, (attributes, span)) = Self::many(Self::parse_attribute())(rest, span.end)?;
            let attributes = attributes.into_iter().map(|(a, _)| a).collect();

            let (rest, (_, span)) = Self::take_non_parseable()(rest, span.end)?;

            // { node }
            let (rest, (nodes, span)) = Self::many(Self::parse_node())(rest, span.end)?;
            let nodes = nodes.into_iter().map(|(n, _)| n).collect();

            let (rest, (_, span)) = Self::take_non_parseable()(rest, span.end)?;

            // ")"
            let (rest, (_tag, span)) = Self::tag(")")(rest, span.end)?;
            let rparen = RParen { span };

            global_span.end = span.end;

            let doc = DocNode {
                lparen,
                doc,
                attributes,
                nodes,
                rparen,
                span: global_span,
            };

            Ok((rest, (doc, global_span)))
        }
    }
}
