//! Fixed integration content, not a content-loading or campaign framework.
use bts_sim::{demo, ProgrammeId, ProgrammeKind, SimHour, World};

pub const SLOT_COUNT: usize = 14;
pub const PLAYER: bts_sim::StationId = demo::MAIN;
pub const RIVAL: bts_sim::StationId = demo::RIVAL;
pub const RIVAL_PROGRAMME: ProgrammeId = ProgrammeId(4);

pub fn hour(index: usize) -> SimHour {
    SimHour((index / 2) as u32 * 24 + if index.is_multiple_of(2) { 18 } else { 21 })
}

pub fn world() -> World {
    let mut world = demo::world();
    let mut movie = world.programmes[3].clone();
    movie.id = ProgrammeId(5);
    movie.kind = ProgrammeKind::Standalone;
    movie.name = "The Long Weekend (film)".into();
    world.programmes.push(movie);
    for sequence in [2, 3] {
        let mut episode = world.programmes[0].clone();
        episode.id = ProgrammeId(4 + sequence as u16);
        episode.kind = ProgrammeKind::Episode {
            series: bts_sim::SeriesId(1),
            sequence,
        };
        episode.name = format!("Neighbourhood Laughs: episode {sequence}");
        world.programmes.push(episode);
    }
    world
}
