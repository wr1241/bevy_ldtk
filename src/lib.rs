mod asset;
mod components;
mod ldtk;
mod resources;
mod systems;

pub mod prelude {
    use bevy::prelude::*;

    use crate::asset;
    use crate::components;
    use crate::resources;
    use crate::systems;

    pub use asset::prelude::*;
    pub use components::prelude::*;
    pub use resources::prelude::*;
    pub use systems::prelude::*;

    pub(crate) use systems::prelude::{apply_level_selection, spawn_ldtk_world};

    pub fn LDtkPlugin(app: &mut App) {
        app.add_plugins(asset::prelude::plugin);

        app.add_systems(
            PostUpdate,
            (apply_level_selection, spawn_ldtk_world).chain(),
        );
    }
}
