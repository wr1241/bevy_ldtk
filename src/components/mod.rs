mod ldtk;

pub mod prelude {
    use crate::components::ldtk;

    pub(crate) use ldtk::Spawned;
    pub use ldtk::{LDtkLayer, LDtkLevel, LDtkProjectHandle, LDtkTile, LDtkWorld};
}
