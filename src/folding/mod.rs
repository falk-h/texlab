mod bibtex_decl;
mod latex_env;
mod latex_section;

use self::{
    bibtex_decl::BibtexDeclarationFoldingProvider, latex_env::LatexEnvironmentFoldingProvider,
    latex_section::LatexSectionFoldingProvider,
};
use crate::{
    feature::{ConcatProvider, FeatureProvider, FeatureRequest},
    protocol::{FoldingRange, FoldingRangeParams},
};
use async_trait::async_trait;

pub struct FoldingProvider {
    provider: ConcatProvider<FoldingRangeParams, FoldingRange>,
}

impl FoldingProvider {
    pub fn new() -> Self {
        Self {
            provider: ConcatProvider::new(vec![
                Box::new(BibtexDeclarationFoldingProvider),
                Box::new(LatexEnvironmentFoldingProvider),
                Box::new(LatexSectionFoldingProvider),
            ]),
        }
    }
}

impl Default for FoldingProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl FeatureProvider for FoldingProvider {
    type Params = FoldingRangeParams;
    type Output = Vec<FoldingRange>;

    async fn execute<'a>(&'a self, req: &'a FeatureRequest<Self::Params>) -> Self::Output {
        self.provider.execute(req).await
    }
}
