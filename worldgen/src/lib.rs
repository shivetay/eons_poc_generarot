//! Generator historii świata — iteracja 1 (jedna wieś, 10 lat).

mod chronicle;
mod model;
mod rng;
mod sim;

pub use chronicle::render;
pub use model::{Event, EventKind, Person, PersonId, Role, Village, World};
pub use sim::{DEFAULT_YEARS, SimConfig, advance, generate};

/// Wygodne API: wygeneruj świat i zwróć tekst kroniki.
pub fn generate_chronicle(config: SimConfig) -> String {
    let world = generate(config);
    render(&world)
}
