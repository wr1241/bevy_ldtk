use std::collections::HashSet;

use bevy::{asset::AssetPath, prelude::*};

use crate::ldtk::prelude::{LdtkJson, LevelAccessor, TilesetDefinition, World};

#[derive(Asset, TypePath)]
pub struct LDtkProject {
    asset_path: AssetPath<'static>,
    json_data: LdtkJson,
}

impl LDtkProject {
    pub fn new(asset_path: &AssetPath, json_data: LdtkJson) -> Self {
        let asset_path = asset_path.clone_owned();
        Self {
            asset_path,
            json_data,
        }
    }

    pub fn get_tileset_defs<'a>(
        &'a self,
        tileset_uid_set: &HashSet<i64>,
    ) -> Vec<&'a TilesetDefinition> {
        self.json_data
            .defs
            .tilesets
            .iter()
            .filter(|&tileset| tileset_uid_set.contains(&tileset.uid))
            .collect::<Vec<_>>()
    }

    pub fn get_asset_rel_path(&self, rel_path: &str) -> Option<String> {
        Some(
            self.asset_path
                .parent()?
                .resolve(rel_path)
                .ok()?
                .to_string(),
        )
    }
}

impl LevelAccessor for LDtkProject {
    fn worlds(&self) -> &[World] {
        &self.json_data.worlds
    }
}
