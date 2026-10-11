//! Minimalny model domenowy iteracji 1: jedna wieś, postacie, wydarzenia.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PersonId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    pub id: PersonId,
    pub name: String,
    pub role: Role,
    pub alive: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventKind {
    HarvestGood,
    HarvestBad,
    Death,
    Succession,
    Conflict,
    Birth,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub year: u32,
    pub kind: EventKind,
    /// Krótki opis przyczyny (fakt, nie ozdobnik).
    pub cause: String,
    pub participant_ids: Vec<PersonId>,
    /// Opis skutku odzwierciedlonego w stanie świata.
    pub effect: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Village {
    pub name: String,
    /// Zapasy żywności w umownych jednostkach.
    pub food: i32,
    pub people: Vec<Person>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct World {
    pub year: u32,
    pub village: Village,
    pub events: Vec<Event>,
    /// Ziarno użyte do tej generacji (informacyjnie; nie jest jeszcze API użytkownika).
    pub seed: u64,
    /// Stan generatora losowego do kontynuacji symulacji.
    pub rng_state: u64,
}

impl World {
    pub fn person(&self, id: PersonId) -> Option<&Person> {
        self.village.people.iter().find(|p| p.id == id)
    }

    pub fn person_mut(&mut self, id: PersonId) -> Option<&mut Person> {
        self.village.people.iter_mut().find(|p| p.id == id)
    }

    pub fn living(&self) -> impl Iterator<Item = &Person> {
        self.village.people.iter().filter(|p| p.alive)
    }

    pub fn living_mut(&mut self) -> impl Iterator<Item = &mut Person> {
        self.village.people.iter_mut().filter(|p| p.alive)
    }
}
