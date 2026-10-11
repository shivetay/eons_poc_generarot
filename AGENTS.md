# Wskazówki dla agentów

- **Zatwierdzenie planu jest wymagane przed każdą zmianą.** Najpierw zbadaj potrzebny kontekst bez edycji, a następnie pokaż użytkownikowi zakres, pliki do zmiany/dodania/usunięcia i plan działania. Nie modyfikuj plików, dopóki użytkownik jednoznacznie nie zatwierdzi tego planu. Sama prośba o funkcję lub problem nie jest zatwierdzeniem planu. Jeśli zakres albo pliki istotnie się zmienią, przedstaw zaktualizowany plan i ponownie poproś o zgodę.
- Najpierw sprawdź `git status` i istniejące zmiany. Nie nadpisuj pracy użytkownika ani innych agentów.
- Zacznij od [mapy projektu](docs/PROJECT_OVERVIEW.md); czytaj tylko pliki związane z zadaniem, zamiast przeglądać całe repozytorium.
- Zadania dotyczące generatora: otwórz [plan generatora](generato.md) i — gdy potrzebujesz szczegółowych wskazówek domenowych — umiejętność [worldgen](.cursor/skills/worldgen/SKILL.md).
- Frontend Leptos znajduje się w `src/`; aplikacja Tauri/Rust w `src-tauri/src/`. Model domenowy i symulacja należą do skrzyni `worldgen/` i nie zależą od UI ani komend Tauri.
- Generator powinien być deterministyczny dla tego samego ziarna. Wydarzenia zapisuj jako dane z przyczynami i skutkami; tekst kroniki twórz z tych danych.
- Wersja 0.1 nie renderuje mapy. Geografia może być modelowana jako regiony połączone grafem sąsiedztwa.
- Pracuj w ograniczonym zakresie zadania, unikaj zmian w plikach generowanych (`dist/`, `target/`) i nie dodawaj zależności bez uzasadnienia.
- Uruchom adekwatne sprawdzenia: `cargo fmt --all -- --check`, `cargo check --workspace`, `cargo test --workspace`; przy zmianach frontendu także `trunk build`.
- Po pracy podsumuj zmienione pliki, wykonane testy i ewentualne blokery.
