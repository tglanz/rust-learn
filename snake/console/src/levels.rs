use engine::{terrain, level};

pub fn gen_level(rows: &i16, columns: &i16) -> level::Level {
    let terrain_size_ref = &terrain::TerrainSize::new(rows, columns);
    level::Level::new(terrain_size_ref, &level::create_bound_indices(terrain_size_ref))
}