//! Warstwa prezentacji: tekst kroniki wyłącznie z zapisanych faktów.

use crate::model::{EventKind, RelationKind, Role, World};

pub fn render(world: &World) -> String {
    let mut out = String::new();
    out.push_str("=== Kronika osad ===\n");
    out.push_str(&format!("Ziarno: {}\n", world.seed));
    out.push_str(&format!("Lata symulacji: {}\n", world.year));
    out.push_str(&format!("Liczba osad: {}\n", world.villages.len()));
    out.push_str(&format!("Relacje społeczne: {}\n", world.relations.len()));
    out.push('\n');

    out.push_str("--- Osady (stan końcowy) ---\n");
    for village in &world.villages {
        out.push_str(&format!(
            "\n{} — wielkość: {}, zajęcie: {}, populacja: {}, zapasy: {}\n",
            village.name,
            village.size.as_str(),
            village.livelihood.as_str(),
            village.population,
            village.food
        ));
        out.push_str(&format!("  Dzielnice: {}\n", village.districts.join(", ")));
        out.push_str(&format!("  Budynki: {}\n", village.buildings.join(", ")));
        out.push_str("  Ważni mieszkańcy:\n");
        for person in &village.people {
            let status = if person.alive { "żyje" } else { "zmarł(a)" };
            out.push_str(&format!(
                "    - {} ({}, status: {}, praca: {}, dom-wieś: {}), potrzeby ż/s/b={}/{}/{}, {}\n",
                person.name,
                person.role.as_str(),
                person.social_status.as_str(),
                person.workplace,
                person.home_village_id.0,
                person.needs.food,
                person.needs.shelter,
                person.needs.safety,
                status
            ));
        }
        let has_soltys = village
            .people
            .iter()
            .any(|p| p.alive && p.role == Role::Soltys);
        if !has_soltys && village.people.iter().any(|p| p.alive) {
            out.push_str("  (Uwaga: wśród żywych brak sołtysa.)\n");
        }
    }

    out.push_str("\n--- Więzi (skrót) ---\n");
    if world.relations.is_empty() {
        out.push_str("(brak zapisanych relacji)\n");
    } else {
        for rel in world.relations.iter().take(40) {
            let na = world.person(rel.a).map(|p| p.name.as_str()).unwrap_or("?");
            let nb = world.person(rel.b).map(|p| p.name.as_str()).unwrap_or("?");
            let label = match rel.kind {
                RelationKind::ParentChild => format!("{na} → dziecko {nb}"),
                _ => format!("{na} — {} — {nb}", rel.kind.as_str()),
            };
            out.push_str(&format!("- {label}\n"));
        }
        if world.relations.len() > 40 {
            out.push_str(&format!(
                "... i {} kolejnych więzi\n",
                world.relations.len() - 40
            ));
        }
    }

    out.push_str("\n--- Przebieg historii ---\n");
    if world.events.is_empty() {
        out.push_str("(brak zapisanych wydarzeń)\n");
        return out;
    }

    let mut current_year = u32::MAX;
    for event in &world.events {
        if event.year != current_year {
            current_year = event.year;
            out.push_str(&format!("\nRok {current_year}\n"));
        }
        let place = world
            .village(event.village_id)
            .map(|v| v.name.as_str())
            .unwrap_or("?");
        let kind = match event.kind {
            EventKind::HarvestGood => "Dobry urodzaj",
            EventKind::HarvestBad => "Słabe zbiory",
            EventKind::Death => "Śmierć",
            EventKind::Succession => "Sukcesja",
            EventKind::Conflict => "Konflikt",
            EventKind::Birth => "Narodziny",
            EventKind::Work => "Praca",
            EventKind::NeedCrisis => "Niedostatek",
            EventKind::Migration => "Migracja",
            EventKind::Marriage => "Małżeństwo",
            EventKind::SocialChange => "Zmiana społeczna",
        };
        let who = event
            .participant_ids
            .iter()
            .filter_map(|id| world.person(*id))
            .map(|p| p.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!(
            "  [{kind} @ {place}] przyczyna: {}; uczestnicy: {}; skutek: {}\n",
            event.cause,
            if who.is_empty() { "—" } else { &who },
            event.effect
        ));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::{SimConfig, generate};

    #[test]
    fn chronicle_mentions_social_life() {
        let world = generate(SimConfig {
            years: 10,
            seed: Some(3),
        });
        let text = render(&world);
        assert!(text.contains("Kronika"));
        assert!(text.contains("Więzi") || text.contains("Relacje"));
        for v in &world.villages {
            assert!(text.contains(&v.name));
        }
    }
}
