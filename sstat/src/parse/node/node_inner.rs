//! AST node definitions

use super::*;

/// All possible types of nodes and their respective contents
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Node<'s> {
    Text {
        inner: Text<'s>,
    },

    Sec {
        lparen: LParen,
        sec: Sec,
        attributes: Vec<Attribute<'s>>,
        inner: Vec<Node<'s>>,
        rparen: RParen,

        span: Span,
    },
    Title {
        lparen: LParen,
        title: Title,
        attributes: Vec<Attribute<'s>>,
        inner: Vec<Node<'s>>,
        rparen: RParen,

        span: Span,
    },
    P {
        lparen: LParen,
        p: P,
        attributes: Vec<Attribute<'s>>,
        inner: Vec<Node<'s>>,
        rparen: RParen,

        span: Span,
    },
}

impl Parser {
    /// Parse a nestable node
    ///
    /// ```ebnf
    /// node = regular_node | ?TEXT?
    /// ```
    pub(crate) fn parse_node<'i>(&self) -> impl Fn(&'i str, usize) -> ParseResult<Node<'i>> + '_ {
        move |input: &str, start: usize| {
            let (rest, (_, span)) = self.take_non_parseable()(input, start)?;
            let mut global_span = span;

            // "("
            let (rest, (maybe_lparen_tag, span)) = self.optional(self.tag("("))(rest, span.end)?;

            if let Some(_) = maybe_lparen_tag {
                let lparen = LParen { span };

                self.parse_regular_node_start(lparen)(rest, span.end)
            } else {
                let (rest, (text, span)) = self.take_text()(rest, span.end)?;

                let text_node = Node::Text { inner: text };

                global_span.end = span.end;

                Ok((rest, (text_node, global_span)))
            }
        }
    }

    /// Parse the start of a regular, delimited node
    ///
    /// ```ebnf
    /// regular_node_start = "(", <name>, { attribute };
    /// ```
    pub(crate) fn parse_regular_node_start<'i>(
        &self,
        lparen: LParen,
    ) -> impl Fn(&'i str, usize) -> ParseResult<Node<'i>> + '_ {
        move |input: &str, start: usize| {
            let (rest, (_, span)) = self.take_non_parseable()(input, start)?;
            let mut global_span = span;

            // <name>
            let (rest, (node_name, name_span)) = self.take_identifier()(rest, span.end)?;
            let span = name_span;

            let (rest, (_, span)) = self.take_non_parseable()(rest, span.end)?;

            // { attribute }
            let (rest, (attributes, span)) = self.many(self.parse_attribute())(rest, span.end)?;
            let attributes = attributes.into_iter().map(|(a, _)| a).collect();

            let (rest, (_, span)) = self.take_non_parseable()(rest, span.end)?;

            let (rest, (node, span)) = match node_name {
                _ => unimplemented!(),
            };

            global_span.end = span.end;

            Ok((rest, (node, global_span)))
        }
    }
}
