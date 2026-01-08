use std::collections::HashMap;

use bevy::{asset::UntypedAssetId, math::I64Vec2, prelude::*};

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
#[require(Transform, Visibility)]
pub struct LDtkTile;

#[derive(Component, Deref, Eq, PartialEq, Reflect)]
pub struct LDtkGridCoord(pub IVec2);

#[derive(Component, Deref, Eq, PartialEq, Reflect)]
pub struct BevyGridCoord(pub IVec2);

#[derive(Component)]
pub(crate) struct Spawned;
