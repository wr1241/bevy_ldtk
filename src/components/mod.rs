mod ldtk;

pub mod prelude {
    use crate::components::ldtk;

    pub(crate) use ldtk::Spawned;
    pub use ldtk::{LDtkCoord, LDtkLayer, LDtkLevel, LDtkProjectHandle, LDtkTile, LDtkWorld};
}
