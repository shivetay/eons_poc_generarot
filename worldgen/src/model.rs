//! Model domenowy: osady, NPC z życiem społecznym, wydarzenia.

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct PersonId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct VillageId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Role {
    Soltys,
    Rolnik,
    Kowal,
    Mysliwy,
    Zielarka,
    Straznik,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Soltys => "sołtys",
            Role::Rolnik => "rolnik",
            Role::Kowal => "kowal",
            Role::Mysliwy => "myśliwy",
            Role::Zielarka => "zielarka",
            Role::Straznik => "strażnik",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SocialStatus {
    Soltys,
    Rzemieslnik,
    Chlop,
    Przybysz,
}

impl SocialStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            SocialStatus::Soltys => "sołtys",
            SocialStatus::Rzemieslnik => "rzemieślnik",
            SocialStatus::Chlop => "chłop",
            SocialStatus::Przybysz => "przybysz",
        }
    }

    pub fn from_role(role: &Role) -> Self {
        match role {
            Role::Soltys => SocialStatus::Soltys,
            Role::Kowal | Role::Zielarka => SocialStatus::Rzemieslnik,
            _ => SocialStatus::Chlop,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SettlementSize {
    Mala,
    Srednia,
    Duza,
}

impl SettlementSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            SettlementSize::Mala => "mała",
            SettlementSize::Srednia => "średnia",
            SettlementSize::Duza => "duża",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Livelihood {
    Rolnictwo,
    Rzemioslo,
    Myslistwo,
    HandelLokalny,
}

impl Livelihood {
    pub fn as_str(&self) -> &'static str {
        match self {
            Livelihood::Rolnictwo => "rolnictwo",
            Livelihood::Rzemioslo => "rzemiosło",
            Livelihood::Myslistwo => "myślistwo",
            Livelihood::HandelLokalny => "handel lokalny",
        }
    }
}

/// Potrzeby NPC w skali 0–100.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Needs {
    pub food: u8,
    pub shelter: u8,
    pub safety: u8,
}

impl Needs {
    pub fn balanced(offset: u8) -> Self {
        Self {
            food: 55u8.saturating_add(offset % 20),
            shelter: 60u8.saturating_add(offset % 15),
            safety: 50u8.saturating_add(offset % 25),
        }
    }

    pub fn is_strained(&self) -> bool {
        self.food < 35 || self.shelter < 35 || self.safety < 30
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum RelationKind {
    Spouse,
    ParentChild,
    Ally,
    Rival,
}

impl RelationKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            RelationKind::Spouse => "małżeństwo",
            RelationKind::ParentChild => "pokrewieństwo",
            RelationKind::Ally => "sojusz",
            RelationKind::Rival => "rywalizacja",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Relation {
    pub a: PersonId,
    pub b: PersonId,
    pub kind: RelationKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Person {
    pub id: PersonId,
    /// Wieś, w której NPC obecnie przebywa.
    pub village_id: VillageId,
    /// Dom / stałe miejsce zamieszkania.
    pub home_village_id: VillageId,
    /// Miejsce pracy (budynek lub zajęcie).
    pub workplace: String,
    pub name: String,
    pub role: Role,
    pub social_status: SocialStatus,
    pub needs: Needs,
    pub alive: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum EventKind {
    HarvestGood,
    HarvestBad,
    Death,
    Succession,
    Conflict,
    Birth,
    Work,
    NeedCrisis,
    Migration,
    Marriage,
    SocialChange,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Event {
    pub year: u32,
    pub village_id: VillageId,
    pub kind: EventKind,
    pub cause: String,
    pub participant_ids: Vec<PersonId>,
    pub effect: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Village {
    pub id: VillageId,
    pub name: String,
    pub size: SettlementSize,
    pub livelihood: Livelihood,
    pub population: u32,
    pub districts: Vec<String>,
    pub buildings: Vec<String>,
    pub food: i32,
    pub people: Vec<Person>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct World {
    pub year: u32,
    pub villages: Vec<Village>,
    pub events: Vec<Event>,
    pub relations: Vec<Relation>,
    pub seed: u64,
    pub rng_state: u64,
}

impl World {
    pub fn village(&self, id: VillageId) -> Option<&Village> {
        self.villages.iter().find(|v| v.id == id)
    }

    pub fn village_mut(&mut self, id: VillageId) -> Option<&mut Village> {
        self.villages.iter_mut().find(|v| v.id == id)
    }

    pub fn person(&self, id: PersonId) -> Option<&Person> {
        self.villages
            .iter()
            .flat_map(|v| v.people.iter())
            .find(|p| p.id == id)
    }

    pub fn person_mut(&mut self, id: PersonId) -> Option<&mut Person> {
        for village in &mut self.villages {
            if let Some(p) = village.people.iter_mut().find(|p| p.id == id) {
                return Some(p);
            }
        }
        None
    }

    pub fn living_in(&self, village_id: VillageId) -> impl Iterator<Item = &Person> {
        self.village(village_id)
            .into_iter()
            .flat_map(|v| v.people.iter())
            .filter(|p| p.alive)
    }

    pub fn any_living(&self) -> bool {
        self.villages
            .iter()
            .flat_map(|v| v.people.iter())
            .any(|p| p.alive)
    }

    pub fn has_relation(&self, a: PersonId, b: PersonId, kind: RelationKind) -> bool {
        self.relations.iter().any(|r| {
            if r.kind != kind {
                return false;
            }
            match kind {
                RelationKind::ParentChild => r.a == a && r.b == b,
                _ => (r.a == a && r.b == b) || (r.a == b && r.b == a),
            }
        })
    }

    pub fn allies_of(&self, id: PersonId) -> Vec<PersonId> {
        self.relations
            .iter()
            .filter(|r| r.kind == RelationKind::Ally)
            .filter_map(|r| {
                if r.a == id {
                    Some(r.b)
                } else if r.b == id {
                    Some(r.a)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn add_relation(&mut self, a: PersonId, b: PersonId, kind: RelationKind) {
        if a == b || self.has_relation(a, b, kind.clone()) {
            return;
        }
        self.relations.push(Relation { a, b, kind });
    }

    pub fn settlement_views(&self) -> Vec<SettlementView> {
        self.villages
            .iter()
            .map(|v| SettlementView {
                name: v.name.clone(),
                population: v.population,
                size: v.size.as_str().to_string(),
                livelihood: v.livelihood.as_str().to_string(),
                districts: v.districts.clone(),
                buildings: v.buildings.clone(),
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SettlementView {
    pub name: String,
    pub population: u32,
    pub size: String,
    pub livelihood: String,
    pub districts: Vec<String>,
    pub buildings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GenerationView {
    pub chronicle: String,
    pub settlements: Vec<SettlementView>,
}
