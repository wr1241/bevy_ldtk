use std::collections::HashMap;

use bevy::{asset::UntypedAssetId, prelude::*};

use crate::{
    asset::prelude::LDtkProject,
    ldtk::prelude::{Level, World},
};

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct LDtkProjectHandle(pub Handle<LDtkProject>);

impl From<&LDtkProjectHandle> for AssetId<LDtkProject> {
    fn from(handle: &LDtkProjectHandle) -> Self {
        handle.0.id()
    }
}

impl From<&LDtkProjectHandle> for UntypedAssetId {
    fn from(handle: &LDtkProjectHandle) -> Self {
        handle.0.id().into()
    }
}

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct LDtkWorld {
    pub world: World,
    pub level: Level,
    pub tileset_handles: HashMap<i64, (usize, Handle<Image>, Handle<TextureAtlasLayout>)>,
}

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct LDtkLevel(pub String);

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct LDtkLayer(pub String);

#[derive(Component)]
pub struct LDtkTile;

#[derive(Clone, Component, Hash, Eq, PartialEq)]
pub struct LDtkCoord {
    pub x: i64,
    pub y: i64,
}

impl LDtkCoord {
    pub fn from_ldtk_pixel(tile_size: usize, x: i64, y: i64) -> Self {
        Self {
            x: x / tile_size as i64,
            y: -y / tile_size as i64,
        }
    }
}

#[derive(Component)]
pub(crate) struct Spawned;
