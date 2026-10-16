//! Generator historii świata — osady, życie NPC, kronika.

mod chronicle;
mod model;
mod rng;
mod sim;

pub use chronicle::render;
pub use model::{
    Event, EventKind, GenerationView, Livelihood, Needs, Person, PersonId, Relation, RelationKind,
    Role, SettlementSize, SettlementView, SocialStatus, Village, VillageId, World,
};
pub use sim::{DEFAULT_YEARS, SETTLEMENT_COUNT, SimConfig, advance, generate};

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
