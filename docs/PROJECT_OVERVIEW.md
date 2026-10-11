# Mapa projektu

## Podsumowanie

Repozytorium zawiera aplikację desktopową Tauri 2 z frontendem Leptos 0.8 i workspace Rust 2024. Iteracja 1 generatora: skrzynia `worldgen/` symuluje jedną wieś przez 10 lat; okno aplikacji pokazuje kronikę tekstową (komenda Tauri `generate_world_chronicle`). Droga do pełnej wersji 0.1: [ROADMAP_0.1.md](ROADMAP_0.1.md).

## Gdzie szukać

| Obszar | Lokalizacja | Obecna rola |
| --- | --- | --- |
| Interfejs Leptos | `src/app.rs` | Konsola kroniki w głównym oknie |
| Wejście frontendu | `src/main.rs` | Montuje komponent aplikacji |
| Komendy Tauri | `src-tauri/src/lib.rs` | `generate_world_chronicle` → `worldgen` |
| Wejście aplikacji Tauri | `src-tauri/src/main.rs` | Uruchamia bibliotekę Tauri |
| Konfiguracja frontendu | `Cargo.toml`, `Trunk.toml`, `index.html`, `styles.css` | Zależności i budowanie frontendu |
| Konfiguracja Tauri | `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json` | Zależności i konfiguracja aplikacji desktopowej |
| Skrzynia domeny generatora | `worldgen/` | Model wsi, symulacja, kronika |
| Plan domeny generatora | `generato.md` | Zakres, model i etapy generatora 0.1 |
| Droga do v0.1 | `docs/ROADMAP_0.1.md` | Iteracje od wąskiego slice do pełnego 0.1 |
| Globalne zasady dla agentów | `AGENTS.md` | Krótkie reguły pracy w repozytorium |
| Agent implementujący | `.cursor/agents/worldgen-implementer.md` | Wykonywanie ograniczonych zadań generatora |
| Agent przeglądu | `.cursor/agents/worldgen-reviewer.md` | Odczytowy przegląd zmian generatora |
| Wiedza domenowa na żądanie | `.cursor/skills/worldgen/SKILL.md` | Procedura i zasady prac nad generatorem |

## Ograniczanie zużycia kontekstu

- `AGENTS.md` zawiera wyłącznie reguły przydatne przy większości zadań; szczegóły są w dokumentach wskazanych linkami.
- Przed zmianami obowiązuje zatwierdzenie planu opisane w `AGENTS.md`; przegląd lub propozycja naprawy nie upoważnia do edycji.
- `generato.md` czytaj przy pracach nad planem lub zakresem funkcjonalnym generatora.
- `SKILL.md` wczytuj przy implementacji, projektowaniu lub debugowaniu generatora; nie jest potrzebny do niezwiązanych z nim zmian.
- Otwieraj tylko potrzebne moduły z `src/`, `src-tauri/src/` lub `worldgen/`; nie czytaj całego repozytorium ani wyników kompilacji.
- `dist/` i `target/` są artefaktami budowania, nie źródłem kontekstu do prac.

## Przydatne polecenia

- `cargo tauri dev` — uruchomienie aplikacji (frontend: `trunk build` → `dist/`, bez `trunk serve` / portu 1420).
- `powershell ./scripts/dev.ps1` — to samo przez skrypt.
- `cargo fmt --all -- --check` — sprawdzenie formatowania Rust.
- `cargo check --workspace` — sprawdzenie kompilacji workspace.
- `cargo test --workspace` — uruchomienie testów workspace.
- `trunk build` — ręczne budowanie frontendu do `dist/`.

Nie otwieraj `http://127.0.0.1:1420` w przeglądarce — aplikacja działa w oknie Tauri.
