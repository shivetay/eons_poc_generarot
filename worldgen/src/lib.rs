//! Generator historii świata — osady, państwa, życie NPC, kronika.

mod chronicle;
mod model;
mod rng;
mod sim;

pub use chronicle::render;
pub use model::{
    Event, EventKind, GenerationView, Livelihood, Needs, Person, PersonId, Polity, Relation,
    RelationKind, Role, SettlementSize, SettlementView, SocialStatus, State, StateId, Village,
    VillageId, World,
};
pub use sim::{DEFAULT_YEARS, SETTLEMENT_COUNT, STATE_COUNT, SimConfig, advance, generate};

pub fn generate_chronicle(config: SimConfig) -> String {
    let world = generate(config);
    render(&world)
}

pub fn generation_view(world: &World) -> GenerationView {
    GenerationView {
        chronicle: render(world),
        settlements: world.settlement_views(),
    }
}
