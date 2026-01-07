use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    asset::prelude::LDtkProject,
    components::prelude::{LDtkProjectHandle, LDtkWorld},
    ldtk::prelude::LevelAccessor,
    resources::prelude::LevelSelection,
};

pub(crate) fn apply_level_selection(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_selection: Option<Res<LevelSelection>>,
    ldtk_project_assets: Res<Assets<LDtkProject>>,
    ldtk_project_handle_query: Query<(Entity, &LDtkProjectHandle)>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let Some(level_selection) = level_selection else {
        return;
    };

    if ldtk_project_handle_query
        .iter()
        .any(|(entity, ldtk_project_handle)| {
            let Some(ldtk_project) = ldtk_project_assets.get(ldtk_project_handle) else {
                return false;
            };

            let Some((world, level)) = ldtk_project.find_world_level(&level_selection) else {
                return false;
            };

            commands.entity(entity).despawn_children();

            let tileset_uid_set = level.get_tileset_uid_set();
            let tileset_defs = ldtk_project.get_tileset_defs(&tileset_uid_set);

            let tileset_handles = tileset_defs
                .iter()
                .map(|&tileset_def| {
                    let Some(rel_path) = &tileset_def.rel_path else {
                        return None;
                    };

                    let Some(tileset_path) = ldtk_project.get_asset_rel_path(rel_path) else {
                        return None;
                    };

                    let tileset_image_handle: Handle<Image> = asset_server.load(&tileset_path);
                    let tileset_texture_atlas_layout_handle =
                        texture_atlas_layouts.add(tileset_def);

                    Some((
                        tileset_def.uid,
                        (
                            tileset_def.tile_grid_size as usize,
                            tileset_image_handle,
                            tileset_texture_atlas_layout_handle,
                        ),
                    ))
                })
                .filter_map(|x| x)
                .collect::<HashMap<_, _>>();

            commands.entity(entity).insert(children![(
                Name::new(world.identifier.clone()),
                LDtkWorld {
                    world: world.clone(),
                    level: level.clone(),
                    tileset_handles,
                },
            ),]);

            true
        })
    {
        commands.remove_resource::<LevelSelection>();
    }
}
