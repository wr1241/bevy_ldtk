mod ldtk_project;
mod ldtk_project_loader;
mod ldtk_project_loader_error;

pub mod prelude {
    use crate::asset::{ldtk_project, ldtk_project_loader, ldtk_project_loader_error};
    use bevy::prelude::*;

    pub(crate) use ldtk_project::LDtkProject;
    pub(crate) use ldtk_project_loader::LDtkProjectLoader;
    pub use ldtk_project_loader_error::LDtkProjectLoaderError;

    pub(crate) fn plugin(app: &mut App) {
        app.init_asset::<LDtkProject>()
            .init_asset_loader::<LDtkProjectLoader>();
    }
}
