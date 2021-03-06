mod latex_import;
mod latex_include;

use self::{latex_import::LatexImportLinkProvider, latex_include::LatexIncludeLinkProvider};
use crate::{
    feature::{ConcatProvider, FeatureProvider, FeatureRequest},
    protocol::{DocumentLink, DocumentLinkParams},
};
use async_trait::async_trait;

pub struct LinkProvider {
    provider: ConcatProvider<DocumentLinkParams, DocumentLink>,
}

impl LinkProvider {
    pub fn new() -> Self {
        Self {
            provider: ConcatProvider::new(vec![
                Box::new(LatexImportLinkProvider),
                Box::new(LatexIncludeLinkProvider),
            ]),
        }
    }
}

impl Default for LinkProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl FeatureProvider for LinkProvider {
    type Params = DocumentLinkParams;
    type Output = Vec<DocumentLink>;

    async fn execute<'a>(&'a self, req: &'a FeatureRequest<Self::Params>) -> Self::Output {
        self.provider.execute(req).await
    }
}
