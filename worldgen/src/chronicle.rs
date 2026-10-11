//! Warstwa prezentacji: tekst kroniki wyłącznie z zapisanych faktów.

use crate::model::{EventKind, Role, World};

pub fn render(world: &World) -> String {
    let mut out = String::new();
    out.push_str("=== Kronika wsi ===\n");
    out.push_str(&format!("Wieś: {}\n", world.village.name));
    out.push_str(&format!("Ziarno: {}\n", world.seed));
    out.push_str(&format!("Lata symulacji: {}\n", world.year));
    out.push_str(&format!("Zapasy żywności: {}\n", world.village.food));
    out.push('\n');
    out.push_str("--- Mieszkańcy (stan końcowy) ---\n");
    for person in &world.village.people {
        let status = if person.alive { "żyje" } else { "zmarł(a)" };
        out.push_str(&format!(
            "- {} ({}), {}\n",
            person.name,
            person.role.as_str(),
            status
        ));
    }
    out.push('\n');
    out.push_str("--- Przebieg historii ---\n");
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
        let kind = match event.kind {
            EventKind::HarvestGood => "Dobry urodzaj",
            EventKind::HarvestBad => "Słabe zbiory",
            EventKind::Death => "Śmierć",
            EventKind::Succession => "Sukcesja",
            EventKind::Conflict => "Konflikt",
            EventKind::Birth => "Narodziny",
        };
        let who = event
            .participant_ids
            .iter()
            .filter_map(|id| world.person(*id))
            .map(|p| p.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!(
            "  [{kind}] przyczyna: {}; uczestnicy: {}; skutek: {}\n",
            event.cause,
            if who.is_empty() { "—" } else { &who },
            event.effect
        ));
    }

    if !world.living().any(|p| p.role == Role::Soltys) && world.living().count() > 0 {
        out.push_str("\n(Uwaga: wśród żywych brak sołtysa.)\n");
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::{SimConfig, generate};

    #[test]
    fn chronicle_mentions_village_and_events() {
        let world = generate(SimConfig {
            years: 10,
            seed: Some(3),
        });
        let text = render(&world);
        assert!(text.contains(&world.village.name));
        assert!(text.contains("Kronika"));
        if let Some(first) = world.events.first() {
            assert!(text.contains(&first.cause) || text.contains("Rok"));
        }
    }
}
