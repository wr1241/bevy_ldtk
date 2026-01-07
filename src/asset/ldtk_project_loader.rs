use bevy::asset::{
    AssetLoader,
    {LoadContext, io::Reader},
};

use crate::asset::{ldtk_project::LDtkProject, ldtk_project_loader_error::LDtkProjectLoaderError};
use crate::ldtk::prelude::LdtkJson;

#[derive(Default)]
pub(crate) struct LDtkProjectLoader;

impl AssetLoader for LDtkProjectLoader {
    type Asset = LDtkProject;

    type Settings = ();

    type Error = LDtkProjectLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let json_root = serde_json::from_slice::<LdtkJson>(&bytes)?;
        Ok(LDtkProject::new(load_context.asset_path(), json_root))
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
