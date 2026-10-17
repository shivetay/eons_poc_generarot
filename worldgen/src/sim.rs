//! Bootstrap czterech osad i pętla życia społecznego NPC.

use crate::model::{
    Event, EventKind, Livelihood, Needs, Person, PersonId, Polity, RelationKind, Role,
    SettlementSize, SocialStatus, State, StateId, Village, VillageId, World,
};
use crate::rng::Rng;

pub const DEFAULT_YEARS: u32 = 10;
pub const SETTLEMENT_COUNT: usize = 4;
pub const STATE_COUNT: usize = 2;

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
    "Brodnica",
    "Gaj",
];

const DISTRICT_POOL: &[&str] = &[
    "Centrum",
    "Zagrody",
    "Przedmieście",
    "Nad potokiem",
    "Przy lesie",
    "Targowisko",
];

const BUILDING_POOL: &[&str] = &[
    "karczma",
    "kuźnia",
    "kaplica",
    "spichlerz",
    "młyn",
    "studnia",
    "dom sołtysa",
    "warsztat",
    "stajnia",
    "łaźnia",
];

const STATE_NAMES: &[&str] = &[
    "Księstwo Borów",
    "Królestwo Nadbrzeża",
    "Księstwo Kamienia",
    "Królestwo Łąk",
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

fn workplace_for(role: &Role, buildings: &[String], livelihood: Livelihood) -> String {
    let prefer = match role {
        Role::Kowal => "kuźnia",
        Role::Soltys => "dom sołtysa",
        Role::Zielarka => "kaplica",
        Role::Rolnik => "spichlerz",
        Role::Mysliwy => "Przy lesie",
        Role::Straznik => "Centrum",
    };
    if let Some(b) = buildings.iter().find(|b| b.as_str() == prefer) {
        return b.clone();
    }
    buildings
        .first()
        .cloned()
        .unwrap_or_else(|| livelihood.as_str().to_string())
}

fn pick_size(rng: &mut Rng) -> SettlementSize {
    match rng.gen_range(0, 2) {
        0 => SettlementSize::Mala,
        1 => SettlementSize::Srednia,
        _ => SettlementSize::Duza,
    }
}

fn pick_livelihood(rng: &mut Rng) -> Livelihood {
    match rng.gen_range(0, 3) {
        0 => Livelihood::Rolnictwo,
        1 => Livelihood::Rzemioslo,
        2 => Livelihood::Myslistwo,
        _ => Livelihood::HandelLokalny,
    }
}

fn population_for_size(size: SettlementSize, rng: &mut Rng) -> u32 {
    match size {
        SettlementSize::Mala => rng.gen_range(20, 45),
        SettlementSize::Srednia => rng.gen_range(46, 90),
        SettlementSize::Duza => rng.gen_range(91, 160),
    }
}

fn important_npc_count(size: SettlementSize, rng: &mut Rng) -> u32 {
    match size {
        SettlementSize::Mala => rng.gen_range(2, 4),
        SettlementSize::Srednia => rng.gen_range(3, 6),
        SettlementSize::Duza => rng.gen_range(4, 8),
    }
}

fn pick_unique_name(rng: &mut Rng, used: &[String], pool: &[&str]) -> String {
    for _ in 0..32 {
        let base = rng.pick(pool).copied().unwrap_or("Osada").to_string();
        if !used.iter().any(|n| n == &base) {
            return base;
        }
    }
    format!("Osada {}", used.len() + 1)
}

fn pick_list(rng: &mut Rng, pool: &[&str], min: u32, max: u32) -> Vec<String> {
    let count = rng.gen_range(min, max) as usize;
    let mut out = Vec::new();
    let mut guard = 0;
    while out.len() < count && guard < 64 {
        guard += 1;
        let item = rng.pick(pool).copied().unwrap_or("—").to_string();
        if !out.contains(&item) {
            out.push(item);
        }
    }
    if out.is_empty() {
        out.push(pool.first().copied().unwrap_or("—").to_string());
    }
    out
}

fn bootstrap_village(
    rng: &mut Rng,
    id: VillageId,
    state_id: StateId,
    used_names: &mut Vec<String>,
    next_person_id: &mut u32,
) -> Village {
    let name = pick_unique_name(rng, used_names, VILLAGE_NAMES);
    used_names.push(name.clone());
    let size = pick_size(rng);
    let livelihood = pick_livelihood(rng);
    let population = population_for_size(size, rng);
    let districts = pick_list(rng, DISTRICT_POOL, 1, 3);
    let buildings = pick_list(rng, BUILDING_POOL, 3, 6);
    let npc_count = important_npc_count(size, rng);

    let mut people = Vec::with_capacity(npc_count as usize);
    for i in 0..npc_count {
        let base = rng.pick(FIRST_NAMES).copied().unwrap_or("Anon").to_string();
        let person_name = if people.iter().any(|p: &Person| p.name == base) {
            format!("{base} {}", *next_person_id)
        } else {
            base
        };
        let role = role_for_index(i, npc_count);
        let workplace = workplace_for(&role, &buildings, livelihood);
        people.push(Person {
            id: PersonId(*next_person_id),
            village_id: id,
            home_village_id: id,
            workplace,
            name: person_name,
            social_status: SocialStatus::from_role(&role),
            role,
            needs: Needs::balanced((*next_person_id % 20) as u8),
            alive: true,
        });
        *next_person_id += 1;
    }

    Village {
        id,
        state_id,
        name,
        size,
        livelihood,
        population,
        districts,
        buildings,
        food: 40 + rng.gen_range(0, 25) as i32,
        people,
    }
}

fn pick_ruler(village: &Village) -> PersonId {
    village
        .people
        .iter()
        .find(|p| p.alive && p.role == Role::Soltys)
        .or_else(|| village.people.iter().find(|p| p.alive))
        .map(|p| p.id)
        .unwrap_or(PersonId(0))
}

fn bootstrap_states(world: &mut World, rng: &mut Rng) {
    let mut used_state_names = Vec::new();
    let mut states = Vec::with_capacity(STATE_COUNT);
    for i in 0..STATE_COUNT {
        let state_id = StateId(i as u32);
        let village_ids: Vec<VillageId> = world
            .villages
            .iter()
            .filter(|v| v.state_id == state_id)
            .map(|v| v.id)
            .collect();
        let capital_id = village_ids[0];
        let capital_name = world
            .village(capital_id)
            .map(|v| v.name.clone())
            .unwrap_or_default();
        let ruler_id = world
            .village(capital_id)
            .map(pick_ruler)
            .unwrap_or(PersonId(0));
        let name = pick_unique_name(rng, &used_state_names, STATE_NAMES);
        used_state_names.push(name.clone());
        let polity = if i == 0 {
            Polity::Ksiestwo
        } else {
            Polity::Krolestwo
        };
        states.push(State {
            id: state_id,
            name: name.clone(),
            polity,
            capital_id,
            ruler_id,
            village_ids: village_ids.clone(),
        });
        let ruler_name = world
            .person(ruler_id)
            .map(|p| p.name.clone())
            .unwrap_or_else(|| "?".into());
        world.events.push(Event {
            year: 0,
            village_id: capital_id,
            kind: EventKind::SocialChange,
            cause: format!("utworzenie {} {}", polity.as_str(), name),
            participant_ids: vec![ruler_id],
            effect: format!("{name} obejmuje osady; stolica {capital_name}, władca {ruler_name}"),
        });
    }
    world.states = states;
}

fn seed_social_ties(world: &mut World, rng: &mut Rng) {
    let village_ids: Vec<VillageId> = world.villages.iter().map(|v| v.id).collect();
    for vid in village_ids {
        let ids: Vec<PersonId> = world.living_in(vid).map(|p| p.id).collect();
        if ids.len() >= 2 {
            let a = ids[0];
            let b = ids[1];
            world.add_relation(a, b, RelationKind::Ally);
            let name_a = world.person(a).map(|p| p.name.clone()).unwrap_or_default();
            let name_b = world.person(b).map(|p| p.name.clone()).unwrap_or_default();
            let vname = world
                .village(vid)
                .map(|v| v.name.clone())
                .unwrap_or_default();
            world.events.push(Event {
                year: 0,
                village_id: vid,
                kind: EventKind::SocialChange,
                cause: format!("wspólne życie w {vname}"),
                participant_ids: vec![a, b],
                effect: format!("{name_a} i {name_b} zawierają sojusz sąsiedzki"),
            });
        }
        if ids.len() >= 3 && rng.chance(1, 2) {
            world.add_relation(ids[0], ids[2], RelationKind::Rival);
        }
        if ids.len() >= 2 && rng.chance(1, 2) {
            let a = ids[ids.len() - 2];
            let b = ids[ids.len() - 1];
            if !world.has_relation(a, b, RelationKind::Spouse) {
                world.add_relation(a, b, RelationKind::Spouse);
                let name_a = world.person(a).map(|p| p.name.clone()).unwrap_or_default();
                let name_b = world.person(b).map(|p| p.name.clone()).unwrap_or_default();
                world.events.push(Event {
                    year: 0,
                    village_id: vid,
                    kind: EventKind::Marriage,
                    cause: format!("związek zawarty w osadzie"),
                    participant_ids: vec![a, b],
                    effect: format!("{name_a} i {name_b} tworzą rodzinę"),
                });
            }
        }
    }
}

fn living_ids(village: &Village) -> Vec<PersonId> {
    village
        .people
        .iter()
        .filter(|p| p.alive)
        .map(|p| p.id)
        .collect()
}

fn apply_yearly_food(village: &mut Village, good_harvest: bool) {
    let living = village.people.iter().filter(|p| p.alive).count() as i32;
    let consumption = living * 3 + (village.population / 20) as i32;
    let yield_amount = if good_harvest {
        10 + living * 4 + (village.population / 15) as i32
    } else {
        2 + living
    };
    village.food = (village.food + yield_amount - consumption).max(0);
}

fn update_needs_after_harvest(world: &mut World, village_id: VillageId, good: bool) {
    let food_stock = world.village(village_id).map(|v| v.food).unwrap_or(0);
    if let Some(village) = world.village_mut(village_id) {
        for person in village.people.iter_mut().filter(|p| p.alive) {
            if good && food_stock > 20 {
                person.needs.food = person.needs.food.saturating_add(12).min(100);
                person.needs.shelter = person.needs.shelter.saturating_add(4).min(100);
            } else {
                person.needs.food = person.needs.food.saturating_sub(18);
                person.needs.safety = person.needs.safety.saturating_sub(8);
            }
        }
    }
}

fn maybe_harvest(world: &mut World, village_id: VillageId, rng: &mut Rng) {
    let good = rng.chance(3, 5);
    let Some(village) = world.village_mut(village_id) else {
        return;
    };
    apply_yearly_food(village, good);
    let food = village.food;
    let name = village.name.clone();
    let participants = living_ids(village);
    if good {
        world.events.push(Event {
            year: world.year,
            village_id,
            kind: EventKind::HarvestGood,
            cause: format!("sprzyjająca pogoda w {name}"),
            participant_ids: participants,
            effect: format!("zapasy żywności w {name} wzrosły do {food}"),
        });
    } else {
        world.events.push(Event {
            year: world.year,
            village_id,
            kind: EventKind::HarvestBad,
            cause: format!("słabe plony w {name}"),
            participant_ids: participants,
            effect: format!("zapasy żywności w {name} spadły do {food}"),
        });
    }
    update_needs_after_harvest(world, village_id, good);
}

fn maybe_work(world: &mut World, village_id: VillageId, rng: &mut Rng) {
    let workers: Vec<(PersonId, String, String)> = world
        .living_in(village_id)
        .map(|p| (p.id, p.name.clone(), p.workplace.clone()))
        .collect();
    if workers.is_empty() || !rng.chance(2, 3) {
        return;
    }
    let Some(&(id, _, _)) = rng.pick(&workers) else {
        return;
    };
    let (name, workplace) = workers
        .iter()
        .find(|(pid, _, _)| *pid == id)
        .map(|(_, n, w)| (n.clone(), w.clone()))
        .unwrap_or_default();
    let village_name = world
        .village(village_id)
        .map(|v| v.name.clone())
        .unwrap_or_default();

    if let Some(p) = world.person_mut(id) {
        p.needs.food = p.needs.food.saturating_add(6).min(100);
        p.needs.shelter = p.needs.shelter.saturating_add(3).min(100);
        p.needs.safety = p.needs.safety.saturating_add(2).min(100);
    }

    world.events.push(Event {
        year: world.year,
        village_id,
        kind: EventKind::Work,
        cause: format!("{name} wykonuje pracę przy {workplace}"),
        participant_ids: vec![id],
        effect: format!("potrzeby {name} w {village_name} poprawiają się dzięki pracy"),
    });
}

fn maybe_need_crisis(world: &mut World, village_id: VillageId, rng: &mut Rng) {
    let strained: Vec<(PersonId, String)> = world
        .living_in(village_id)
        .filter(|p| p.needs.is_strained())
        .map(|p| (p.id, p.name.clone()))
        .collect();
    if strained.is_empty() || !rng.chance(1, 2) {
        return;
    }
    let Some(&(id, _)) = rng.pick(&strained) else {
        return;
    };
    let name = strained
        .iter()
        .find(|(pid, _)| *pid == id)
        .map(|(_, n)| n.clone())
        .unwrap_or_default();
    let needs = world.person(id).map(|p| p.needs.clone());
    let Some(needs) = needs else {
        return;
    };
    let village_name = world
        .village(village_id)
        .map(|v| v.name.clone())
        .unwrap_or_default();
    world.events.push(Event {
        year: world.year,
        village_id,
        kind: EventKind::NeedCrisis,
        cause: format!(
            "niezaspokojone potrzeby {name} (żywność {}, schronienie {}, bezpieczeństwo {})",
            needs.food, needs.shelter, needs.safety
        ),
        participant_ids: vec![id],
        effect: format!("{name} cierpi niedostatek w {village_name}"),
    });
}

fn maybe_death_and_succession(world: &mut World, village_id: VillageId, rng: &mut Rng) {
    let food = world.village(village_id).map(|v| v.food).unwrap_or(0);
    let candidates = world
        .village(village_id)
        .map(living_ids)
        .unwrap_or_default();
    if candidates.is_empty() {
        return;
    }
    let risk_num = if food < 15 { 2 } else { 1 };
    if !rng.chance(risk_num, 6) {
        return;
    }
    let Some(&victim_id) = rng.pick(&candidates) else {
        return;
    };
    let was_soltys = world
        .person(victim_id)
        .map(|p| p.role == Role::Soltys)
        .unwrap_or(false);
    let victim_name = world
        .person(victim_id)
        .map(|p| p.name.clone())
        .unwrap_or_else(|| "?".into());
    let village_name = world
        .village(village_id)
        .map(|v| v.name.clone())
        .unwrap_or_default();

    let was_ruler = world.states.iter().any(|s| s.ruler_id == victim_id);

    if let Some(p) = world.person_mut(victim_id) {
        p.alive = false;
    }
    if let Some(v) = world.village_mut(village_id) {
        v.population = v.population.saturating_sub(1).max(1);
    }

    let cause = if food < 15 {
        format!("choroba przy niedostatku żywności w {village_name} ({victim_name})")
    } else {
        format!("nagła choroba w {village_name} ({victim_name})")
    };

    world.events.push(Event {
        year: world.year,
        village_id,
        kind: EventKind::Death,
        cause,
        participant_ids: vec![victim_id],
        effect: format!("{victim_name} umiera"),
    });

    if was_ruler {
        succeed_state_ruler(world, victim_id, village_id, &victim_name, rng);
    }

    if was_soltys {
        let living = world
            .village(village_id)
            .map(living_ids)
            .unwrap_or_default();
        let allies = world.allies_of(victim_id);
        let preferred: Vec<PersonId> = living
            .iter()
            .copied()
            .filter(|id| allies.contains(id))
            .collect();
        let pool = if preferred.is_empty() {
            living
        } else {
            preferred
        };
        if let Some(&new_id) = rng.pick(&pool) {
            let new_name = world
                .person(new_id)
                .map(|p| p.name.clone())
                .unwrap_or_else(|| "?".into());
            if let Some(p) = world.person_mut(new_id) {
                p.role = Role::Soltys;
                p.social_status = SocialStatus::Soltys;
            }
            let cause = if allies.contains(&new_id) {
                format!("sojusznicy popierają sukcesję po {victim_name}")
            } else {
                format!("śmierć sołtysa w {village_name} ({victim_name})")
            };
            world.events.push(Event {
                year: world.year,
                village_id,
                kind: EventKind::Succession,
                cause,
                participant_ids: vec![victim_id, new_id],
                effect: format!("{new_name} zostaje sołtysem w {village_name}"),
            });
        }
    }
}

fn maybe_conflict(world: &mut World, village_id: VillageId, rng: &mut Rng) {
    let ids = world
        .village(village_id)
        .map(living_ids)
        .unwrap_or_default();
    if ids.len() < 2 || !rng.chance(1, 4) {
        return;
    }
    let a = *rng.pick(&ids).unwrap();
    let rivals: Vec<PersonId> = ids
        .iter()
        .copied()
        .filter(|id| *id != a && world.has_relation(a, *id, RelationKind::Rival))
        .collect();
    let b = if !rivals.is_empty() && rng.chance(2, 3) {
        *rng.pick(&rivals).unwrap()
    } else {
        let b_candidates: Vec<_> = ids.into_iter().filter(|id| *id != a).collect();
        let Some(&b) = rng.pick(&b_candidates) else {
            return;
        };
        b
    };
    let name_a = world.person(a).map(|p| p.name.clone()).unwrap_or_default();
    let name_b = world.person(b).map(|p| p.name.clone()).unwrap_or_default();
    let village_name = world
        .village(village_id)
        .map(|v| v.name.clone())
        .unwrap_or_default();
    let loss = rng.gen_range(1, 4) as i32;
    if let Some(v) = world.village_mut(village_id) {
        v.food = (v.food - loss).max(0);
    }
    for id in [a, b] {
        if let Some(p) = world.person_mut(id) {
            p.needs.safety = p.needs.safety.saturating_sub(15);
        }
    }
    world.add_relation(a, b, RelationKind::Rival);
    let food = world.village(village_id).map(|v| v.food).unwrap_or(0);
    world.events.push(Event {
        year: world.year,
        village_id,
        kind: EventKind::Conflict,
        cause: format!("spór społeczny w {village_name} między {name_a} a {name_b}"),
        participant_ids: vec![a, b],
        effect: format!("rywalizacja pogłębia się; zużyto {loss} zapasów (pozostało {food})"),
    });
}

fn maybe_marriage(world: &mut World, village_id: VillageId, rng: &mut Rng) {
    let ids: Vec<PersonId> = world.living_in(village_id).map(|p| p.id).collect();
    if ids.len() < 2 || !rng.chance(1, 5) {
        return;
    }
    let a = *rng.pick(&ids).unwrap();
    let b_candidates: Vec<_> = ids
        .into_iter()
        .filter(|id| {
            *id != a
                && !world.has_relation(a, *id, RelationKind::Spouse)
                && !world.has_relation(a, *id, RelationKind::ParentChild)
                && !world.has_relation(*id, a, RelationKind::ParentChild)
        })
        .collect();
    let Some(&b) = rng.pick(&b_candidates) else {
        return;
    };
    let name_a = world.person(a).map(|p| p.name.clone()).unwrap_or_default();
    let name_b = world.person(b).map(|p| p.name.clone()).unwrap_or_default();
    let village_name = world
        .village(village_id)
        .map(|v| v.name.clone())
        .unwrap_or_default();
    world.add_relation(a, b, RelationKind::Spouse);
    world.add_relation(a, b, RelationKind::Ally);
    world.events.push(Event {
        year: world.year,
        village_id,
        kind: EventKind::Marriage,
        cause: format!("związek zawarty w {village_name}"),
        participant_ids: vec![a, b],
        effect: format!("{name_a} i {name_b} tworzą rodzinę"),
    });
}

fn maybe_birth(world: &mut World, village_id: VillageId, rng: &mut Rng) {
    let living = world.living_in(village_id).count();
    if living == 0 || living >= 10 || !rng.chance(1, 5) {
        return;
    }
    let spouses: Vec<(PersonId, PersonId)> = world
        .relations
        .iter()
        .filter(|r| r.kind == RelationKind::Spouse)
        .filter(|r| {
            world
                .person(r.a)
                .is_some_and(|p| p.alive && p.village_id == village_id)
                && world
                    .person(r.b)
                    .is_some_and(|p| p.alive && p.village_id == village_id)
        })
        .map(|r| (r.a, r.b))
        .collect();
    let parents = if let Some(&(a, b)) = rng.pick(&spouses) {
        Some((a, b))
    } else {
        None
    };

    let next_id = world
        .villages
        .iter()
        .flat_map(|v| v.people.iter())
        .map(|p| p.id.0)
        .max()
        .unwrap_or(0)
        + 1;
    let base = rng.pick(FIRST_NAMES).copied().unwrap_or("Anon").to_string();
    let village_name = world
        .village(village_id)
        .map(|v| v.name.clone())
        .unwrap_or_default();
    let buildings = world
        .village(village_id)
        .map(|v| v.buildings.clone())
        .unwrap_or_default();
    let livelihood = world
        .village(village_id)
        .map(|v| v.livelihood)
        .unwrap_or(Livelihood::Rolnictwo);
    let name = {
        let taken = world
            .villages
            .iter()
            .flat_map(|v| v.people.iter())
            .any(|p| p.name == base);
        if taken {
            format!("{base} {next_id}")
        } else {
            base
        }
    };
    let role = Role::Rolnik;
    let person = Person {
        id: PersonId(next_id),
        village_id,
        home_village_id: village_id,
        workplace: workplace_for(&role, &buildings, livelihood),
        name: name.clone(),
        role,
        social_status: SocialStatus::Chlop,
        needs: Needs::balanced(10),
        alive: true,
    };
    if let Some(v) = world.village_mut(village_id) {
        v.people.push(person);
        v.population = v.population.saturating_add(1);
    }
    let mut participants = vec![PersonId(next_id)];
    let mut cause = format!("narodziny w {village_name}");
    if let Some((pa, pb)) = parents {
        world.add_relation(pa, PersonId(next_id), RelationKind::ParentChild);
        world.add_relation(pb, PersonId(next_id), RelationKind::ParentChild);
        participants.extend([pa, pb]);
        let n1 = world.person(pa).map(|p| p.name.clone()).unwrap_or_default();
        let n2 = world.person(pb).map(|p| p.name.clone()).unwrap_or_default();
        cause = format!("narodziny w rodzinie {n1} i {n2} w {village_name}");
    }
    world.events.push(Event {
        year: world.year,
        village_id,
        kind: EventKind::Birth,
        cause,
        participant_ids: participants,
        effect: format!("{name} dołącza do {village_name} jako rolnik"),
    });
}

fn maybe_migration(world: &mut World, rng: &mut Rng) {
    let migrants: Vec<(PersonId, VillageId, String)> = world
        .villages
        .iter()
        .flat_map(|v| v.people.iter())
        .filter(|p| p.alive && p.needs.is_strained() && p.role != Role::Soltys)
        .map(|p| (p.id, p.village_id, p.name.clone()))
        .collect();
    if migrants.is_empty() || !rng.chance(1, 3) {
        return;
    }
    let Some(&(person_id, from_id, _)) = rng.pick(&migrants) else {
        return;
    };
    let name = migrants
        .iter()
        .find(|(id, _, _)| *id == person_id)
        .map(|(_, _, n)| n.clone())
        .unwrap_or_default();
    let targets: Vec<VillageId> = world
        .villages
        .iter()
        .map(|v| v.id)
        .filter(|id| *id != from_id)
        .collect();
    let Some(&to_id) = rng.pick(&targets) else {
        return;
    };
    let from_name = world
        .village(from_id)
        .map(|v| v.name.clone())
        .unwrap_or_default();
    let to_name = world
        .village(to_id)
        .map(|v| v.name.clone())
        .unwrap_or_default();
    let to_buildings = world
        .village(to_id)
        .map(|v| v.buildings.clone())
        .unwrap_or_default();
    let to_livelihood = world
        .village(to_id)
        .map(|v| v.livelihood)
        .unwrap_or(Livelihood::Rolnictwo);

    let Some(mut person) = (|| {
        let village = world.village_mut(from_id)?;
        let idx = village.people.iter().position(|p| p.id == person_id)?;
        Some(village.people.remove(idx))
    })() else {
        return;
    };

    if let Some(v) = world.village_mut(from_id) {
        v.population = v.population.saturating_sub(1).max(1);
    }

    person.village_id = to_id;
    person.home_village_id = to_id;
    person.social_status = SocialStatus::Przybysz;
    person.workplace = workplace_for(&person.role, &to_buildings, to_livelihood);
    person.needs.shelter = person.needs.shelter.saturating_add(10).min(100);
    person.needs.safety = 40;

    if let Some(v) = world.village_mut(to_id) {
        v.people.push(person);
        v.population = v.population.saturating_add(1);
    }

    world.events.push(Event {
        year: world.year,
        village_id: to_id,
        kind: EventKind::Migration,
        cause: format!("niedostatek i napięcia w {from_name}"),
        participant_ids: vec![person_id],
        effect: format!("{name} przenosi się z {from_name} do {to_name} jako przybysz"),
    });
}

fn succeed_state_ruler(
    world: &mut World,
    dead_ruler: PersonId,
    village_id: VillageId,
    victim_name: &str,
    rng: &mut Rng,
) {
    let Some(state_id) = world
        .states
        .iter()
        .find(|s| s.ruler_id == dead_ruler)
        .map(|s| s.id)
    else {
        return;
    };
    let capital_id = world
        .state(state_id)
        .map(|s| s.capital_id)
        .unwrap_or(village_id);
    let state_name = world
        .state(state_id)
        .map(|s| s.name.clone())
        .unwrap_or_default();
    let village_ids = world
        .state(state_id)
        .map(|s| s.village_ids.clone())
        .unwrap_or_default();

    let mut candidates: Vec<PersonId> = village_ids
        .iter()
        .flat_map(|vid| world.living_in(*vid).map(|p| p.id))
        .collect();
    let allies = world.allies_of(dead_ruler);
    let preferred: Vec<PersonId> = candidates
        .iter()
        .copied()
        .filter(|id| allies.contains(id))
        .collect();
    if !preferred.is_empty() {
        candidates = preferred;
    }
    // Prefer living soltys in capital.
    let capital_soltys: Vec<PersonId> = world
        .living_in(capital_id)
        .filter(|p| p.role == Role::Soltys)
        .map(|p| p.id)
        .collect();
    let pool = if !capital_soltys.is_empty() {
        capital_soltys
    } else {
        candidates
    };
    let Some(&new_id) = rng.pick(&pool) else {
        return;
    };
    let new_name = world
        .person(new_id)
        .map(|p| p.name.clone())
        .unwrap_or_else(|| "?".into());
    if let Some(state) = world.state_mut(state_id) {
        state.ruler_id = new_id;
    }
    world.events.push(Event {
        year: world.year,
        village_id: capital_id,
        kind: EventKind::RulerSuccession,
        cause: format!("śmierć władcy {victim_name} w {state_name}"),
        participant_ids: vec![dead_ruler, new_id],
        effect: format!("{new_name} obejmuje władzę w {state_name}"),
    });
}

fn maybe_interstate_tension(world: &mut World, rng: &mut Rng) {
    if world.states.len() < 2 || !rng.chance(1, 4) {
        return;
    }
    let a = world.states[0].id;
    let b = world.states[1].id;
    let name_a = world.state(a).map(|s| s.name.clone()).unwrap_or_default();
    let name_b = world.state(b).map(|s| s.name.clone()).unwrap_or_default();
    let capital_a = world.state(a).map(|s| s.capital_id).unwrap_or(VillageId(0));
    let ruler_a = world.state(a).map(|s| s.ruler_id);
    let ruler_b = world.state(b).map(|s| s.ruler_id);
    let mut participants = Vec::new();
    if let Some(id) = ruler_a {
        participants.push(id);
    }
    if let Some(id) = ruler_b {
        participants.push(id);
    }
    let border_food_drop = rng.gen_range(1, 3) as i32;
    // Tension drains a bit of food at both capitals (levy / unrest).
    for capital in [
        capital_a,
        world.state(b).map(|s| s.capital_id).unwrap_or(VillageId(0)),
    ] {
        if let Some(v) = world.village_mut(capital) {
            v.food = (v.food - border_food_drop).max(0);
        }
    }
    world.events.push(Event {
        year: world.year,
        village_id: capital_a,
        kind: EventKind::InterstateTension,
        cause: format!("spór graniczny i pretensje między {name_a} a {name_b}"),
        participant_ids: participants,
        effect: format!(
            "napięcie między państwami; stolice tracą po {border_food_drop} zapasów na zbrojenia"
        ),
    });
}

fn maybe_new_alliance(world: &mut World, village_id: VillageId, rng: &mut Rng) {
    let ids: Vec<PersonId> = world.living_in(village_id).map(|p| p.id).collect();
    if ids.len() < 2 || !rng.chance(1, 6) {
        return;
    }
    let a = *rng.pick(&ids).unwrap();
    let b_candidates: Vec<_> = ids
        .into_iter()
        .filter(|id| *id != a && !world.has_relation(a, *id, RelationKind::Ally))
        .collect();
    let Some(&b) = rng.pick(&b_candidates) else {
        return;
    };
    let name_a = world.person(a).map(|p| p.name.clone()).unwrap_or_default();
    let name_b = world.person(b).map(|p| p.name.clone()).unwrap_or_default();
    world.add_relation(a, b, RelationKind::Ally);
    world.events.push(Event {
        year: world.year,
        village_id,
        kind: EventKind::SocialChange,
        cause: "wspólna praca i sąsiedztwo".into(),
        participant_ids: vec![a, b],
        effect: format!("{name_a} i {name_b} zawierają sojusz"),
    });
}

fn simulate_year_for_all(world: &mut World, rng: &mut Rng) {
    let ids: Vec<VillageId> = world.villages.iter().map(|v| v.id).collect();
    for village_id in ids {
        if world.living_in(village_id).count() == 0 {
            continue;
        }
        maybe_harvest(world, village_id, rng);
        maybe_work(world, village_id, rng);
        maybe_need_crisis(world, village_id, rng);
        maybe_death_and_succession(world, village_id, rng);
        maybe_conflict(world, village_id, rng);
        maybe_marriage(world, village_id, rng);
        maybe_birth(world, village_id, rng);
        maybe_new_alliance(world, village_id, rng);
    }
    maybe_migration(world, rng);
    maybe_interstate_tension(world, rng);
}

pub fn advance(world: &mut World, years: u32) {
    if years == 0 {
        return;
    }
    let mut rng = Rng::from_state(world.rng_state);
    let start = world.year + 1;
    let end = world.year + years;
    for year in start..=end {
        world.year = year;
        if !world.any_living() {
            break;
        }
        simulate_year_for_all(world, &mut rng);
    }
    world.rng_state = rng.state();
}

pub fn generate(config: SimConfig) -> World {
    let seed = config.seed.unwrap_or_else(random_seed);
    let mut rng = Rng::new(seed);
    let mut used_names = Vec::new();
    let mut next_person_id = 0u32;
    let mut villages = Vec::with_capacity(SETTLEMENT_COUNT);
    // 2 państwa × 2 osady: wsie 0–1 → state 0, wsie 2–3 → state 1.
    for i in 0..SETTLEMENT_COUNT {
        let state_id = StateId((i / 2) as u32);
        villages.push(bootstrap_village(
            &mut rng,
            VillageId(i as u32),
            state_id,
            &mut used_names,
            &mut next_person_id,
        ));
    }
    let mut world = World {
        year: 0,
        villages,
        states: Vec::new(),
        events: Vec::new(),
        relations: Vec::new(),
        seed,
        rng_state: rng.state(),
    };
    bootstrap_states(&mut world, &mut rng);
    seed_social_ties(&mut world, &mut rng);
    world.rng_state = rng.state();
    advance(&mut world, config.years);
    world
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_creates_four_settlements_with_social_life() {
        let world = generate(SimConfig {
            years: 10,
            seed: Some(7),
        });
        assert_eq!(world.villages.len(), SETTLEMENT_COUNT);
        assert_eq!(world.states.len(), STATE_COUNT);
        for state in &world.states {
            assert_eq!(state.village_ids.len(), 2);
            assert!(world.person(state.ruler_id).is_some());
        }
        assert!(!world.relations.is_empty());
        for v in &world.villages {
            assert!(!v.people.is_empty());
            for p in &v.people {
                assert!(!p.workplace.is_empty());
            }
        }
        let kinds: Vec<_> = world.events.iter().map(|e| &e.kind).collect();
        assert!(
            kinds.iter().any(|k| matches!(
                k,
                EventKind::Work
                    | EventKind::Migration
                    | EventKind::Marriage
                    | EventKind::SocialChange
                    | EventKind::NeedCrisis
                    | EventKind::InterstateTension
            )),
            "oczekiwano wydarzeń życia społecznego"
        );
    }

    #[test]
    fn two_states_partition_villages() {
        let world = generate(SimConfig {
            years: 5,
            seed: Some(21),
        });
        assert_eq!(world.states.len(), 2);
        let owned: Vec<_> = world
            .states
            .iter()
            .flat_map(|s| s.village_ids.iter())
            .copied()
            .collect();
        assert_eq!(owned.len(), 4);
        for v in &world.villages {
            assert!(owned.contains(&v.id));
            assert_eq!(
                world.state_for_village(v.id).map(|s| s.id),
                Some(v.state_id)
            );
        }
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
                assert!(!person.alive);
            }
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
    }

    #[test]
    fn migration_moves_person_between_villages() {
        let world = generate(SimConfig {
            years: 40,
            seed: Some(123),
        });
        let migrations: Vec<_> = world
            .events
            .iter()
            .filter(|e| e.kind == EventKind::Migration)
            .collect();
        assert!(
            !migrations.is_empty(),
            "oczekiwano co najmniej jednej migracji"
        );
        let ev = *migrations.last().unwrap();
        let id = ev.participant_ids[0];
        let last_for_person = migrations
            .iter()
            .rev()
            .find(|e| e.participant_ids.first() == Some(&id))
            .unwrap();
        let person = world.person(id).expect("migrant istnieje");
        assert_eq!(person.social_status, SocialStatus::Przybysz);
        assert_eq!(person.village_id, last_for_person.village_id);
    }
}
