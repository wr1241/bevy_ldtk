mod ldtk;
mod level_selection;

pub(crate) mod prelude {
    use super::ldtk;
    use super::level_selection;

    pub(crate) use ldtk::spawn_ldtk_world;
    pub(crate) use level_selection::apply_level_selection;

    pub use ldtk::despawn_all_ldtk_worlds;
}
