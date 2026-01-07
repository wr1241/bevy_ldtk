use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub enum LevelSelection {
    ByIndices(usize, usize),
    ByIID(String),
}

impl LevelSelection {
    pub fn by_indices(world_index: usize, level_index: usize) -> Self {
        Self::ByIndices(world_index, level_index)
    }

    pub fn by_iid(iid: impl Into<String>) -> Self {
        Self::ByIID(iid.into())
    }
}
