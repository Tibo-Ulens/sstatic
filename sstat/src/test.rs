#[cfg(test)]
mod test {
    use std::assert_matches::assert_matches;

    use crate::parse::*;

    #[test]
	#[rustfmt::skip]
	fn parse_page() {
		let input = "
			[title test]
			[author test]

			(doc [id main])


			rest
		";

		let result = Parser::parse_page(input);
		assert_matches!(result, Ok(_));

		let (rest, (page, span)) = result.unwrap();

		assert_eq!(rest, "\n\n\n\t\t\trest\n\t\t");
		assert_eq!(
			page,
			Page {
				attributes: vec![
					Attribute {
						lbracket: LBracket { span: Span { start: 4, end: 5 } },
						attribute_name: Identifier { name: "title", span: Span { start: 5, end: 10 } },
						attribute_value: Text { text: "test", span: Span { start: 11, end: 15 } },
						rbracket: RBracket { span: Span { start: 15, end: 16 } },
						span: Span { start: 4, end: 16 }
					},
					Attribute {
						lbracket: LBracket { span: Span { start: 20, end: 21 } },
						attribute_name: Identifier { name: "author", span: Span { start: 21, end: 27 } },
						attribute_value: Text { text: "test", span: Span { start: 28, end: 32 } },
						rbracket: RBracket { span: Span { start: 32, end: 33 } },
						span: Span { start: 16, end: 33 }
					}
				],
				doc: DocNode {
					lparen: LParen { span: Span { start: 38, end: 39 } },
					doc: Doc { span: Span { start: 39, end: 42 } },
					attributes: vec![
						Attribute {
							lbracket: LBracket { span: Span { start: 43, end: 44 } },
							attribute_name: Identifier { name: "id", span: Span { start: 44, end: 46 } },
							attribute_value: Text { text: "main", span: Span { start: 47, end: 51 } },
							rbracket: RBracket { span: Span { start: 51, end: 52 } },
							span: Span { start: 43, end: 52 }
						}
					],
					nodes: vec![],
					rparen: RParen { span: Span { start: 52, end: 53 } },
					span: Span { start: 33, end: 53 }
				},
				span: Span { start: 0, end: 53 },
			}
		);
		assert_eq!(span, Span { start: 0, end: 53 });
	}

    #[test]
	#[rustfmt::skip]
	fn parse_attribute() {
		let input = "[example_name (lots of example values)] rest";

		let result = Parser::parse_attribute()(input, 0);
		assert_matches!(result, Ok(_));

		let (rest, (attr, span)) = result.unwrap();

		assert_eq!(rest, " rest");
		assert_eq!(
			attr,
			Attribute {
				lbracket: LBracket { span: Span { start: 0, end: 1 } },
				attribute_name: Identifier { name: "example_name", span: Span { start: 1, end: 13 } },
				attribute_value: Text { text: "(lots of example values)", span: Span { start: 14, end: 38 } },
				rbracket: RBracket { span: Span { start: 38, end: 39 } },
				span: Span { start: 0, end: 39 },
			}
		);
		assert_eq!(span, Span { start: 0, end: 39 });
	}

    #[test]
	#[rustfmt::skip]
	fn parse_attribute_no_value() {
		let input = "[example_name] rest";

		let result = Parser::parse_attribute()(input, 0);
		assert_matches!(result, Ok(_));

		let (rest, (attr, span)) = result.unwrap();

		assert_eq!(rest, " rest");
		assert_eq!(
			attr,
			Attribute {
				lbracket: LBracket { span: Span { start: 0, end: 1 } },
				attribute_name: Identifier { name: "example_name", span: Span { start: 1, end: 13 } },
				attribute_value: Text { text: "", span: Span { start: 13, end: 13 } },
				rbracket: RBracket { span: Span { start: 13, end: 14 } },
				span: Span { start: 0, end: 14 },
			}
		);
		assert_eq!(span, Span { start: 0, end: 14 });
	}

    #[test]
    #[rustfmt::skip]
    fn parse_doc() {
        let input = "  ;; comment
		(doc) rest";

        let result = Parser::parse_doc_node()(input, 0);
        assert_matches!(result, Ok(_));

        let (rest, (doc, span)) = result.unwrap();

        assert_eq!(rest, " rest");
        assert_eq!(
            doc,
            DocNode {
                lparen: LParen { span: Span { start: 15, end: 16 } },
                doc: Doc { span: Span { start: 16, end: 19 } },
                attributes: vec![],
				nodes: vec![],
				rparen: RParen { span: Span { start: 19, end: 20 } },
                span: Span { start: 0, end: 20 },
            }
        );
        assert_eq!(span, Span { start: 0, end: 20 });
    }
}
