use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

use crate::{components::prelude::*, ldtk::prelude::*};

pub(crate) fn spawn_ldtk_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ldtk_world_query: Query<(Entity, &LDtkWorld), Without<Spawned>>,
    mut clear_color: ResMut<ClearColor>,
) {
    ldtk_world_query
        .iter()
        .filter(|(_, ldtk_world)| {
            ldtk_world
                .tileset_handles
                .values()
                .all(|(_, image_handle, _)| {
                    asset_server
                        .get_dependency_load_state(image_handle.id())
                        .is_some_and(|load_state| load_state.is_loaded())
                })
        })
        .for_each(|(world_entity, ldtk_world)| {
            let level_entity = spawn_ldtk_level(
                &mut commands,
                &ldtk_world.level,
                &ldtk_world.tileset_handles,
            );
            commands
                .entity(world_entity)
                .add_child(level_entity)
                .insert(Spawned);

            if let Ok(color) = Srgba::hex(&ldtk_world.level.bg_color) {
                clear_color.0 = Color::Srgba(color);
            }
        });
}

fn spawn_ldtk_level(
    commands: &mut Commands,
    level: &Level,
    tileset_handle_map: &HashMap<i64, (usize, Handle<Image>, Handle<TextureAtlasLayout>)>,
) -> Entity {
    let mut translation = Vec3::new(0., 0., level.world_depth as f32);
    if level.world_x != -1 {
        translation.x = level.world_x as f32;
    }
    if level.world_y != -1 {
        translation.y = -level.world_y as f32;
    }
    let transform = Transform::from_translation(translation);

    let level_entity = commands
        .spawn((
            Name::new(level.identifier.clone()),
            LDtkLevel(level.iid.clone()),
            transform,
        ))
        .id();

    if let Some(layers) = &level.layer_instances {
        let mut layer_z = 0;

        layers
            .iter()
            .rev()
            .for_each(|layer| match layer.layer_instance_type {
                LayerInstanceType::Entities => {
                    // TODO:
                }
                LayerInstanceType::IntGrid => {
                    let Some(tileset_def_uid) = layer.tileset_def_uid else {
                        return;
                    };

                    let Some((
                        tile_size,
                        tileset_image_handle,
                        tileset_texture_atlas_layout_handle,
                    )) = tileset_handle_map.get(&tileset_def_uid)
                    else {
                        return;
                    };

                    let (layer_entity, new_layer_z) = spawn_ldtk_int_grid_layer(
                        commands,
                        layer,
                        layer_z,
                        *tile_size,
                        &tileset_image_handle,
                        &tileset_texture_atlas_layout_handle,
                    );
                    commands.entity(level_entity).add_child(layer_entity);
                    layer_z = new_layer_z;
                }
                LayerInstanceType::Tiles => {
                    // TODO:
                }
                LayerInstanceType::AutoLayer => {
                    // TODO:
                }
            });
    };

    level_entity
}

fn spawn_ldtk_int_grid_layer(
    commands: &mut Commands,
    layer: &LayerInstance,
    layer_z: i64,
    tile_size: usize,
    tileset_image_handle: &Handle<Image>,
    tileset_texture_atlas_layout_handle: &Handle<TextureAtlasLayout>,
) -> (Entity, i64) {
    let layer_entity = commands
        .spawn((
            Name::new(layer.identifier.clone()),
            LDtkLayer(layer.iid.clone()),
        ))
        .insert_if(Visibility::Hidden, || !layer.visible)
        .id();

    let entended_layers = extend_ldtk_layer(
        layer
            .grid_tiles
            .iter()
            .chain(layer.auto_layer_tiles.iter())
            .filter(|tile| {
                tile.px.x as i64 / (tile_size as i64) < layer.c_wid
                    && tile.px.y as i64 / (tile_size as i64) < layer.c_hei
            }),
    );
    let extended_layer_count = entended_layers.len();

    let tile_entities = entended_layers
        .into_iter()
        .enumerate()
        .flat_map(|(layer_index, tiles)| std::iter::repeat(layer_index).zip(tiles.into_iter()))
        .map(|(layer_index, tile)| {
            let x = (tile.px.x as i64 + layer.px_total_offset_x) as f32;
            let y = (-tile.px.y as i64 + layer.px_total_offset_y) as f32;
            let z = (layer_z + layer_index as i64) as f32;
            let transform = Transform::from_xyz(x, y, z);

            let mut sprite = Sprite::from_atlas_image(
                tileset_image_handle.clone(),
                TextureAtlas {
                    layout: tileset_texture_atlas_layout_handle.clone(),
                    index: tile.t as usize,
                },
            );

            sprite.color.set_alpha((tile.a * layer.opacity) as f32);
            sprite.flip_x = tile.f & 0b01 != 0;
            sprite.flip_y = tile.f & 0b10 != 0;

            let ldtk_grid_coord = LDtkGridCoord(IVec2::new(
                tile.px.x / layer.grid_size as i32,
                tile.px.y / layer.grid_size as i32,
            ));

            let bevy_grid_coord = BevyGridCoord(IVec2::new(
                tile.px.x / layer.grid_size as i32,
                layer.c_hei as i32 - tile.px.y / layer.grid_size as i32 - 1,
            ));

            commands
                .spawn((
                    transform,
                    sprite,
                    LDtkTile,
                    ldtk_grid_coord,
                    bevy_grid_coord,
                ))
                .id()
        })
        .collect::<Vec<_>>();

    commands.entity(layer_entity).add_children(&tile_entities);

    (layer_entity, layer_z + extended_layer_count as i64)
}

fn extend_ldtk_layer<'a, I: Iterator<Item = &'a TileInstance>>(
    tiles: I,
) -> Vec<Vec<&'a TileInstance>> {
    let mut coord_set: HashSet<IVec2> = HashSet::new();
    let mut this_layer_tiles = Vec::new();
    let mut above_layer_tiles = Vec::new();

    tiles.for_each(|tile| {
        if coord_set.contains(&tile.px) {
            above_layer_tiles.push(tile);
        } else {
            coord_set.insert(tile.px);
            this_layer_tiles.push(tile);
        }
    });

    let mut layered_tiles = vec![this_layer_tiles];
    if !above_layer_tiles.is_empty() {
        layered_tiles.extend(extend_ldtk_layer(above_layer_tiles.into_iter()));
    }

    layered_tiles
}

pub fn despawn_all_ldtk_worlds(
    mut commands: Commands,
    ldtk_world_query: Query<Entity, With<LDtkWorld>>,
) {
    ldtk_world_query
        .iter()
        .for_each(|world_entity| commands.entity(world_entity).despawn());
}
