use anyhow::Result;
use turbo_tasks::{ValueToString, Vc};
use turbopack_binding::turbopack::core::{
    asset::Asset,
    chunk::{ChunkableModuleReference, ChunkingType, ChunkingTypeOption},
    reference::AssetReference,
    resolve::ResolveResult,
};

#[turbo_tasks::value]
pub struct NextServerComponentModuleReference {
    asset: Vc<Box<dyn Asset>>,
}

#[turbo_tasks::value_impl]
impl NextServerComponentModuleReference {
    #[turbo_tasks::function]
    pub fn new(asset: Vc<Box<dyn Asset>>) -> Vc<Self> {
        NextServerComponentModuleReference { asset }.cell()
    }
}

#[turbo_tasks::value_impl]
impl ValueToString for NextServerComponentModuleReference {
    #[turbo_tasks::function]
    async fn to_string(&self) -> Result<Vc<String>> {
        Ok(Vc::cell(format!(
            "Next.js server component {}",
            self.asset.ident().to_string().await?
        )))
    }
}

#[turbo_tasks::value_impl]
impl AssetReference for NextServerComponentModuleReference {
    #[turbo_tasks::function]
    fn resolve_reference(&self) -> Vc<ResolveResult> {
        ResolveResult::asset(self.asset).cell()
    }
}

#[turbo_tasks::value_impl]
impl ChunkableModuleReference for NextServerComponentModuleReference {
    #[turbo_tasks::function]
    fn chunking_type(&self) -> Vc<ChunkingTypeOption> {
        // TODO(alexkirsz) Instead of isolated parallel, have the server component
        // reference create a new chunk group entirely?
        Vc::cell(Some(ChunkingType::IsolatedParallel))
    }
}
