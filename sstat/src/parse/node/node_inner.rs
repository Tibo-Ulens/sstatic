//! AST node definitions

use super::*;

/// Generic container for all possible nodes
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Node<'s> {
    lparen: LParen,
    attributes: Vec<Attribute<'s>>,
    node_type: NodeType<'s>,
    rparen: RParen,
}

/// All possible types of nodes and their respective contents
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum NodeType<'s> {
    Sec(Vec<Node<'s>>),
}

impl Parser {
    /// Parse a nestable node
    ///
    /// ```ebnf
    /// node = "(", <name>, { attribute }, <inner>, ")"
    /// ```
    pub(crate) fn parse_node<'i>() -> impl Fn(&'i str, usize) -> ParseResult<Node<'i>> {
        move |input: &str, start: usize| {
            let (rest, (_, span)) = Self::take_non_parseable()(input, start)?;
            let mut global_span = span;

            // "("
            let (rest, (_tag, span)) = Self::tag("(")(rest, span.end)?;
            let lparen = LParen { span };

            let (rest, (_, span)) = Self::take_non_parseable()(rest, span.end)?;

            // <name>
            let (rest, (node_name, span)) = Self::take_identifier()(rest, span.end)?;

            let (rest, (_, span)) = Self::take_non_parseable()(rest, span.end)?;

            // { attribute }
            let (rest, (attributes, span)) = Self::many(Self::parse_attribute())(rest, span.end)?;
            let attributes = attributes.into_iter().map(|(a, _)| a).collect();

            let (rest, (_, span)) = Self::take_non_parseable()(rest, span.end)?;

            // <inner>
            let (rest, (node_type, span)) = match node_name {
                "sec" => Self::parse_sec_node()(rest, span.end)?,
                _ => unimplemented!(),
            };

            let (rest, (_, span)) = Self::take_non_parseable()(rest, span.end)?;

            // ")"
            let (rest, (_tag, span)) = Self::tag(")")(rest, span.end)?;
            let rparen = RParen { span };

            global_span.end = span.end;

            let node = Node {
                lparen,
                attributes,
                node_type,
                rparen,
            };

            Ok((rest, (node, global_span)))
        }
    }

    /// Parse a Sec node
    fn parse_sec_node<'i>() -> impl Fn(&'i str, usize) -> ParseResult<NodeType<'i>> {
        move |input: &str, start: usize| todo!()
    }
}
