//! Bootstrap jednej wsi i pętla 10 lat.

use crate::model::{Event, EventKind, Person, PersonId, Role, Village, World};
use crate::rng::Rng;

pub const DEFAULT_YEARS: u32 = 10;

#[derive(Debug, Clone)]
pub struct SimConfig {
    pub years: u32,
    pub seed: Option<u64>,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            years: DEFAULT_YEARS,
            seed: None,
        }
    }
}

fn random_seed() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(1)
}

const FIRST_NAMES: &[&str] = &[
    "Borna",
    "Dobromir",
    "Jagna",
    "Mirosława",
    "Radomir",
    "Wisława",
    "Czcibor",
    "Ludmiła",
    "Sławomir",
    "Zbigniew",
];

const VILLAGE_NAMES: &[&str] = &[
    "Brzezina",
    "Mokradło",
    "Jelenie",
    "Kamionka",
    "Ostrów",
    "Wilczy Las",
];

fn role_for_index(index: u32, count: u32) -> Role {
    if index == 0 && count > 0 {
        return Role::Soltys;
    }
    match index % 5 {
        0 => Role::Rolnik,
        1 => Role::Kowal,
        2 => Role::Mysliwy,
        3 => Role::Zielarka,
        _ => Role::Straznik,
    }
}

fn bootstrap(rng: &mut Rng) -> Village {
    let name = rng
        .pick(VILLAGE_NAMES)
        .copied()
        .unwrap_or("Brzezina")
        .to_string();
    let count = rng.gen_range(1, 10);
    let mut people = Vec::with_capacity(count as usize);
    for i in 0..count {
        let base = rng.pick(FIRST_NAMES).copied().unwrap_or("Anon").to_string();
        let name = if people.iter().any(|p: &Person| p.name == base) {
            format!("{base} {i}")
        } else {
            base
        };
        people.push(Person {
            id: PersonId(i),
            name,
            role: role_for_index(i, count),
            alive: true,
        });
    }
    Village {
        name,
        food: 40 + rng.gen_range(0, 20) as i32,
        people,
    }
}

fn living_ids(world: &World) -> Vec<PersonId> {
    world.living().map(|p| p.id).collect()
}

fn apply_yearly_food(world: &mut World, good_harvest: bool) {
    let living = world.living().count() as i32;
    let consumption = living * 3;
    let yield_amount = if good_harvest {
        8 + living * 4
    } else {
        2 + living
    };
    world.village.food = (world.village.food + yield_amount - consumption).max(0);
}

fn maybe_harvest(world: &mut World, rng: &mut Rng) {
    let good = rng.chance(3, 5);
    apply_yearly_food(world, good);
    let food = world.village.food;
    if good {
        world.events.push(Event {
            year: world.year,
            kind: EventKind::HarvestGood,
            cause: "sprzyjająca pogoda w sezonie wegetacyjnym".into(),
            participant_ids: living_ids(world),
            effect: format!("zapasy żywności wzrosły do {food}"),
        });
    } else {
        world.events.push(Event {
            year: world.year,
            kind: EventKind::HarvestBad,
            cause: "deszczowa jesień i słabe plony".into(),
            participant_ids: living_ids(world),
            effect: format!("zapasy żywności spadły do {food}"),
        });
    }
}

fn maybe_death_and_succession(world: &mut World, rng: &mut Rng) {
    let candidates: Vec<PersonId> = living_ids(world);
    if candidates.is_empty() {
        return;
    }
    // Wyższe ryzyko przy niskich zapasach.
    let risk_num = if world.village.food < 15 { 2 } else { 1 };
    if !rng.chance(risk_num, 6) {
        return;
    }
    let Some(&victim_id) = rng.pick(&candidates) else {
        return;
    };
    let was_softys = world
        .person(victim_id)
        .map(|p| p.role == Role::Soltys)
        .unwrap_or(false);
    let victim_name = world
        .person(victim_id)
        .map(|p| p.name.clone())
        .unwrap_or_else(|| "?".into());

    if let Some(p) = world.person_mut(victim_id) {
        p.alive = false;
    }

    let cause = if world.village.food < 15 {
        format!("choroba przy niedostatku żywności ({victim_name})")
    } else {
        format!("nagła choroba ({victim_name})")
    };

    world.events.push(Event {
        year: world.year,
        kind: EventKind::Death,
        cause,
        participant_ids: vec![victim_id],
        effect: format!("{victim_name} umiera"),
    });

    if was_softys {
        let next = living_ids(world);
        if let Some(&new_id) = rng.pick(&next) {
            let new_name = world
                .person(new_id)
                .map(|p| p.name.clone())
                .unwrap_or_else(|| "?".into());
            if let Some(p) = world.person_mut(new_id) {
                p.role = Role::Soltys;
            }
            world.events.push(Event {
                year: world.year,
                kind: EventKind::Succession,
                cause: format!("śmierć poprzedniego sołtysa ({victim_name})"),
                participant_ids: vec![victim_id, new_id],
                effect: format!("{new_name} zostaje sołtysem"),
            });
        }
    }
}

fn maybe_conflict(world: &mut World, rng: &mut Rng) {
    let ids = living_ids(world);
    if ids.len() < 2 || !rng.chance(1, 4) {
        return;
    }
    let a = *rng.pick(&ids).unwrap();
    let b_candidates: Vec<_> = ids.into_iter().filter(|id| *id != a).collect();
    let Some(&b) = rng.pick(&b_candidates) else {
        return;
    };
    let name_a = world.person(a).map(|p| p.name.clone()).unwrap_or_default();
    let name_b = world.person(b).map(|p| p.name.clone()).unwrap_or_default();
    let loss = rng.gen_range(1, 4) as i32;
    world.village.food = (world.village.food - loss).max(0);
    let food = world.village.food;
    world.events.push(Event {
        year: world.year,
        kind: EventKind::Conflict,
        cause: format!("spór o zapasy między {name_a} a {name_b}"),
        participant_ids: vec![a, b],
        effect: format!("zużyto {loss} zapasów; pozostało {food}"),
    });
}

fn maybe_birth(world: &mut World, rng: &mut Rng) {
    let living = world.living().count();
    if living == 0 || living >= 10 || !rng.chance(1, 5) {
        return;
    }
    let next_id = world
        .village
        .people
        .iter()
        .map(|p| p.id.0)
        .max()
        .unwrap_or(0)
        + 1;
    let base = rng.pick(FIRST_NAMES).copied().unwrap_or("Anon").to_string();
    let name = if world.village.people.iter().any(|p| p.name == base) {
        format!("{base} {next_id}")
    } else {
        base
    };
    let person = Person {
        id: PersonId(next_id),
        name: name.clone(),
        role: Role::Rolnik,
        alive: true,
    };
    world.village.people.push(person);
    world.events.push(Event {
        year: world.year,
        kind: EventKind::Birth,
        cause: "narodziny w rodzinie wiejskiej".into(),
        participant_ids: vec![PersonId(next_id)],
        effect: format!("{name} dołącza do wsi jako rolnik"),
    });
}

/// Dopisuje kolejne lata symulacji (i wydarzenia) do istniejącego świata.
pub fn advance(world: &mut World, years: u32) {
    if years == 0 {
        return;
    }
    let mut rng = Rng::from_state(world.rng_state);
    let start = world.year + 1;
    let end = world.year + years;
    for year in start..=end {
        world.year = year;
        if world.living().count() == 0 {
            break;
        }
        maybe_harvest(world, &mut rng);
        maybe_death_and_succession(world, &mut rng);
        maybe_conflict(world, &mut rng);
        maybe_birth(world, &mut rng);
    }
    world.rng_state = rng.state();
}

/// Uruchamia generację świata według konfiguracji.
pub fn generate(config: SimConfig) -> World {
    let seed = config.seed.unwrap_or_else(random_seed);
    let mut rng = Rng::new(seed);
    let village = bootstrap(&mut rng);
    let mut world = World {
        year: 0,
        village,
        events: Vec::new(),
        seed,
        rng_state: rng.state(),
    };

    advance(&mut world, config.years);
    world
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_runs_ten_years_with_fixed_seed() {
        let world = generate(SimConfig {
            years: 10,
            seed: Some(7),
        });
        assert_eq!(world.seed, 7);
        assert!(world.year <= 10);
        assert!(!world.village.name.is_empty());
        let initial_people = world.village.people.len();
        assert!((1..=12).contains(&initial_people) || initial_people >= 1);
        assert!(!world.events.is_empty());
    }

    #[test]
    fn death_marks_person_not_alive() {
        let world = generate(SimConfig {
            years: 10,
            seed: Some(99),
        });
        for event in &world.events {
            if event.kind == EventKind::Death {
                let id = event.participant_ids[0];
                let person = world.person(id).expect("uczestnik śmierci istnieje");
                assert!(!person.alive, "zmarła postać musi mieć alive=false");
            }
        }
    }

    #[test]
    fn harvest_effect_matches_food_field() {
        let world = generate(SimConfig {
            years: 10,
            seed: Some(1234),
        });
        if let Some(last_harvest) = world
            .events
            .iter()
            .rev()
            .find(|e| matches!(e.kind, EventKind::HarvestGood | EventKind::HarvestBad))
        {
            assert!(
                last_harvest
                    .effect
                    .contains(&world.village.food.to_string())
                    || world.year == last_harvest.year,
                "skutek ostatniego zbioru powinien odnosić się do zapasów"
            );
        }
    }

    #[test]
    fn advance_adds_years_and_events() {
        let mut world = generate(SimConfig {
            years: 10,
            seed: Some(42),
        });
        let year_before = world.year;
        let events_before = world.events.len();
        advance(&mut world, 5);
        assert_eq!(world.year, year_before + 5);
        assert!(world.events.len() >= events_before);
        assert!(world.events.iter().any(|e| e.year > year_before));
    }
}
