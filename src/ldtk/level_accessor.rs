use crate::{
    ldtk::ldtk_json::{LdtkJson, Level, World},
    resources::prelude::LevelSelection,
};

type IterWorldLevels<'a> = std::iter::FlatMap<
    std::slice::Iter<'a, World>,
    std::slice::Iter<'a, Level>,
    fn(&World) -> std::slice::Iter<'_, Level>,
>;

type ZipWorldIndexAndLevel<'a> =
    std::iter::Zip<std::iter::Repeat<usize>, std::iter::Enumerate<std::slice::Iter<'a, Level>>>;

type IterWorldLevelsWithIndices<'a> = std::iter::FlatMap<
    std::iter::Enumerate<std::slice::Iter<'a, World>>,
    std::iter::Map<
        ZipWorldIndexAndLevel<'a>,
        fn((usize, (usize, &Level))) -> (usize, usize, &Level),
    >,
    fn(
        (usize, &World),
    ) -> std::iter::Map<
        ZipWorldIndexAndLevel<'_>,
        fn((usize, (usize, &Level))) -> (usize, usize, &Level),
    >,
>;

pub(crate) trait LevelAccessor {
    fn worlds(&self) -> &[World];

    fn iter_world_levels(&self) -> IterWorldLevels<'_> {
        self.worlds().iter().flat_map(|world| world.levels.iter())
    }

    fn iter_world_levels_with_indices(&self) -> IterWorldLevelsWithIndices<'_> {
        self.worlds()
            .iter()
            .enumerate()
            .flat_map(|(world_index, world)| {
                std::iter::repeat(world_index)
                    .zip(world.levels.iter().enumerate())
                    .map(|(world_index, (level_index, level))| (world_index, level_index, level))
            })
    }

    fn find_world_level_by_iid(&self, level_iid: &str) -> Option<(&World, &Level)> {
        self.worlds()
            .iter()
            .flat_map(|world| std::iter::repeat(world).zip(world.levels.iter()))
            .find(|(_, level)| level_iid == &level.iid)
    }

    fn find_world_level_at_indices(
        &self,
        world_index: usize,
        level_index: usize,
    ) -> Option<(&World, &Level)> {
        self.worlds().get(world_index).and_then(|world| {
            world
                .levels
                .get(level_index)
                .and_then(|level| Some((world, level)))
        })
    }

    fn find_world_level(&self, level_selection: &LevelSelection) -> Option<(&World, &Level)> {
        match level_selection {
            LevelSelection::ByIndices(world_index, level_index) => {
                self.find_world_level_at_indices(*world_index, *level_index)
            }
            LevelSelection::ByIID(iid) => self.find_world_level_by_iid(iid),
        }
    }
}

impl LevelAccessor for LdtkJson {
    fn worlds(&self) -> &[World] {
        &self.worlds
    }
}
