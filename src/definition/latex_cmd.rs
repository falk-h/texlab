use crate::{
    feature::{FeatureProvider, FeatureRequest},
    protocol::{LocationLink, TextDocumentPositionParams},
    syntax::SyntaxNode,
    workspace::DocumentContent,
};
use async_trait::async_trait;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct LatexCommandDefinitionProvider;

#[async_trait]
impl FeatureProvider for LatexCommandDefinitionProvider {
    type Params = TextDocumentPositionParams;
    type Output = Vec<LocationLink>;

    async fn execute<'a>(&'a self, req: &'a FeatureRequest<Self::Params>) -> Self::Output {
        let mut links = Vec::new();
        if let DocumentContent::Latex(table) = &req.current().content {
            if let Some(cmd) = table
                .find(req.params.position)
                .last()
                .and_then(|node| table.as_command(*node))
            {
                for doc in req.related() {
                    if let DocumentContent::Latex(table) = &doc.content {
                        table
                            .command_definitions
                            .iter()
                            .filter(|def| def.definition_name(&table) == cmd.name.text())
                            .map(|def| {
                                let def_range = table[def.parent].range();
                                LocationLink {
                                    origin_selection_range: Some(cmd.range()),
                                    target_uri: doc.uri.clone().into(),
                                    target_range: def_range,
                                    target_selection_range: def_range,
                                }
                            })
                            .for_each(|link| links.push(link));

                        table
                            .math_operators
                            .iter()
                            .filter(|op| op.definition_name(&table) == cmd.name.text())
                            .map(|op| {
                                let def_range = table[op.parent].range();
                                LocationLink {
                                    origin_selection_range: Some(cmd.range()),
                                    target_uri: doc.uri.clone().into(),
                                    target_range: def_range,
                                    target_selection_range: def_range,
                                }
                            })
                            .for_each(|link| links.push(link));
                    }
                }
            }
        }
        links
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        feature::FeatureTester,
        protocol::{Range, RangeExt},
    };
    use indoc::indoc;

    #[tokio::test]
    async fn empty_latex_document() {
        let actual_links = FeatureTester::new()
            .file("main.tex", "")
            .main("main.tex")
            .position(0, 0)
            .test_position(LatexCommandDefinitionProvider)
            .await;

        assert!(actual_links.is_empty());
    }

    #[tokio::test]
    async fn empty_bibtex_document() {
        let actual_links = FeatureTester::new()
            .file("main.bib", "")
            .main("main.bib")
            .position(0, 0)
            .test_position(LatexCommandDefinitionProvider)
            .await;

        assert!(actual_links.is_empty());
    }

    #[tokio::test]
    async fn command_definition() {
        let actual_links = FeatureTester::new()
            .file(
                "foo.tex",
                indoc!(
                    r#"
                        \include{bar}
                        \foo
                    "#
                ),
            )
            .file("bar.tex", r#"\newcommand{\foo}{bar}"#)
            .file("baz.tex", r#"\newcommand{\foo}{baz}"#)
            .main("foo.tex")
            .position(1, 3)
            .test_position(LatexCommandDefinitionProvider)
            .await;

        let expected_links = vec![LocationLink {
            origin_selection_range: Some(Range::new_simple(1, 0, 1, 4)),
            target_uri: FeatureTester::uri("bar.tex").into(),
            target_range: Range::new_simple(0, 0, 0, 22),
            target_selection_range: Range::new_simple(0, 0, 0, 22),
        }];

        assert_eq!(actual_links, expected_links);
    }

    #[tokio::test]
    async fn math_operator() {
        let actual_links = FeatureTester::new()
            .file(
                "foo.tex",
                indoc!(
                    r#"
                        \include{bar}
                        \foo
                    "#
                ),
            )
            .file("bar.tex", r#"\DeclareMathOperator{\foo}{bar}"#)
            .file("baz.tex", r#"\DeclareMathOperator{\foo}{baz}"#)
            .main("foo.tex")
            .position(1, 3)
            .test_position(LatexCommandDefinitionProvider)
            .await;

        let expected_links = vec![LocationLink {
            origin_selection_range: Some(Range::new_simple(1, 0, 1, 4)),
            target_uri: FeatureTester::uri("bar.tex").into(),
            target_range: Range::new_simple(0, 0, 0, 31),
            target_selection_range: Range::new_simple(0, 0, 0, 31),
        }];

        assert_eq!(actual_links, expected_links);
    }
}
