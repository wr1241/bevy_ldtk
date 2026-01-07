use std::collections::HashSet;

use bevy::{image::TextureAtlasLayout, math::UVec2};
use serde::{Deserialize, Serialize};

use crate::ldtk::{ldtk_json::TilesetDefinition, prelude::Level};

impl From<&TilesetDefinition> for TextureAtlasLayout {
    fn from(tileset_def: &TilesetDefinition) -> Self {
        let tile_size = UVec2::splat(tileset_def.tile_grid_size as u32);
        let columns = tileset_def.c_wid as u32;
        let rows = tileset_def.c_hei as u32;
        let padding = if tileset_def.spacing != 0 {
            Some(UVec2::splat(tileset_def.spacing as u32))
        } else {
            None
        };
        let offset = if tileset_def.padding != 0 {
            Some(UVec2::splat(tileset_def.padding as u32))
        } else {
            None
        };

        Self::from_grid(tile_size, columns, rows, padding, offset)
    }
}

impl Level {
    pub fn get_tileset_uid_set(&self) -> HashSet<i64> {
        self.layer_instances
            .iter()
            .flat_map(|layers| {
                layers
                    .iter()
                    .map(|layer| layer.override_tileset_uid.or(layer.tileset_def_uid))
            })
            .filter_map(|uid| uid)
            .collect::<HashSet<_>>()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayerInstanceType {
    #[serde(rename = "IntGrid")]
    IntGrid,

    #[serde(rename = "Entities")]
    Entities,

    #[serde(rename = "Tiles")]
    Tiles,

    #[serde(rename = "AutoLayer")]
    AutoLayer,
}
