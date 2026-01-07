use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

use crate::{components::prelude::*, ldtk::prelude::*, prelude::LevelSelection};

pub(crate) fn spawn_ldtk_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ldtk_world_query: Query<(Entity, &LDtkWorld), Without<Spawned>>,
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
        .id();

    let entended_layers = extend_ldtk_layer(
        tile_size,
        layer.grid_tiles.iter().chain(layer.auto_layer_tiles.iter()),
    );
    let extended_layer_count = entended_layers.len();

    let tile_entities = entended_layers
        .into_iter()
        .enumerate()
        .flat_map(|(layer_index, tiles)| std::iter::repeat(layer_index).zip(tiles.into_iter()))
        .map(|(layer_index, (coord, tile))| {
            let x = tile.px[0] as f32;
            let y = -tile.px[1] as f32;
            let z = (layer_z + layer_index as i64) as f32;
            let transform = Transform::from_xyz(x, y, z);

            let mut sprite = Sprite::from_atlas_image(
                tileset_image_handle.clone(),
                TextureAtlas {
                    layout: tileset_texture_atlas_layout_handle.clone(),
                    index: tile.t as usize,
                },
            );

            sprite.color.set_alpha(tile.a as f32);
            sprite.flip_x = tile.f & 0b01 != 0;
            sprite.flip_y = tile.f & 0b10 != 0;

            commands.spawn((coord, transform, sprite, LDtkTile)).id()
        })
        .collect::<Vec<_>>();

    commands.entity(layer_entity).add_children(&tile_entities);

    (layer_entity, layer_z + extended_layer_count as i64)
}

fn extend_ldtk_layer<'a, I: Iterator<Item = &'a TileInstance>>(
    tile_size: usize,
    tiles: I,
) -> Vec<Vec<(LDtkCoord, &'a TileInstance)>> {
    let mut coord_set: HashSet<LDtkCoord> = HashSet::new();
    let mut this_layer_tiles = Vec::new();
    let mut above_layer_tiles = Vec::new();

    tiles.for_each(|tile| {
        let coord = LDtkCoord::from_ldtk_pixel(tile_size, tile.px[0], tile.px[1]);
        if coord_set.contains(&coord) {
            above_layer_tiles.push(tile);
        } else {
            this_layer_tiles.push((coord.clone(), tile));
            coord_set.insert(coord);
        }
    });

    let mut layered_tiles = vec![this_layer_tiles];
    if !above_layer_tiles.is_empty() {
        layered_tiles.extend(extend_ldtk_layer(tile_size, above_layer_tiles.into_iter()));
    }

    layered_tiles
}

fn ldtk_pixel_coord_to_bevy(coord: (i64, i64)) -> (f32, f32) {
    (coord.0 as f32, -coord.1 as f32)
}

pub fn despawn_all_ldtk_worlds(
    mut commands: Commands,
    ldtk_world_query: Query<Entity, With<LDtkWorld>>,
) {
    ldtk_world_query
        .iter()
        .for_each(|world_entity| commands.entity(world_entity).despawn());
}
