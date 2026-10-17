# Mapa projektu

## Podsumowanie

Repozytorium zawiera aplikację desktopową Tauri 2 z frontendem Leptos 0.8 i workspace Rust 2024.
Skrzynia `worldgen/` generuje **2 państwa** (po 2 osady), symuluje historię (domyślnie 10 lat, kontynuacja +5),
z NPC (praca, potrzeby, status, rodziny, sojusze/rywalizacje, migracje). Okno aplikacji pokazuje
karty osad (z państwem) i kronikę. Życie społeczne i polityka widać jako **wpisy w kronice**.

Droga do pełnej v0.1: [ROADMAP_0.1.md](ROADMAP_0.1.md). Plan domenowy: [generato.md](../generato.md).
Ticket śledzący: [issue #10](https://github.com/shivetay/tauri_game_POC/issues/10)
(implementacja w tym repo `eons_poc_generarot`, branch `v1`).

## Gdzie szukać

| Obszar | Lokalizacja | Obecna rola |
| --- | --- | --- |
| Interfejs Leptos | `src/app.rs` | Karty osad + kronika; Generuj / +5 lat |
| Wejście frontendu | `src/main.rs` | Montuje komponent aplikacji |
| Komendy Tauri | `src-tauri/src/lib.rs` | `generate_world_chronicle`, `continue_world_chronicle` → `GenerationView` |
| Wejście aplikacji Tauri | `src-tauri/src/main.rs` | Uruchamia bibliotekę Tauri |
| Konfiguracja frontendu | `Cargo.toml`, `Trunk.toml`, `index.html`, `styles.css` | Budowanie WASM (`trunk build` → `dist/`) |
| Konfiguracja Tauri | `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json` | `beforeDevCommand`: `trunk build`, `frontendDist`: `../dist` |
| Skrzynia domeny | `worldgen/` | Model, RNG, symulacja, kronika |
| Skrypt dev | `scripts/dev.ps1` | `cargo tauri dev` |
| Plan domeny 0.1 | `generato.md` | Cel końcowy i kryteria akceptacji |
| Droga do v0.1 | `docs/ROADMAP_0.1.md` | Iteracje 1…6 |
| Zasady agentów | `AGENTS.md` | Plan → akceptacja → implementacja |
| Agent implementujący | `.cursor/agents/worldgen-implementer.md` | Ograniczone zadania generatora |
| Agent przeglądu | `.cursor/agents/worldgen-reviewer.md` | Przegląd zmian generatora |
| Skill domenowy | `.cursor/skills/worldgen/SKILL.md` | Procedura prac nad generatorem |

## Moduły `worldgen/`

| Moduł | Rola |
| --- | --- |
| `model.rs` | Osady, NPC, potrzeby, relacje, wydarzenia, `GenerationView` |
| `rng.rs` | LCG ze stanem do kontynuacji |
| `sim.rs` | Bootstrap 4 osad, pętla lat, życie społeczne, `advance` |
| `chronicle.rs` | Tekst kroniki wyłącznie z zapisanych faktów |

## Ograniczanie zużycia kontekstu

- `AGENTS.md` zawiera wyłącznie reguły przydatne przy większości zadań; szczegóły są w dokumentach wskazanych linkami.
- Przed zmianami obowiązuje zatwierdzenie planu opisane w `AGENTS.md`; przegląd lub propozycja naprawy nie upoważnia do edycji.
- `generato.md` czytaj przy pracach nad planem lub zakresem funkcjonalnym generatora.
- `SKILL.md` wczytuj przy implementacji, projektowaniu lub debugowaniu generatora.
- Otwieraj tylko potrzebne moduły z `src/`, `src-tauri/src/` lub `worldgen/`.
- `dist/` i `target/` są artefaktami budowania, nie źródłem kontekstu do prac.

## Przydatne polecenia

- `cargo tauri dev` — uruchomienie (frontend: `trunk build` → `dist/`, bez portu 1420).
- `powershell ./scripts/dev.ps1` — to samo przez skrypt.
- `cargo fmt --all -- --check` — formatowanie Rust.
- `cargo check --workspace` — kompilacja workspace.
- `cargo test --workspace` — testy (w tym `worldgen`).
- `trunk build` — ręczne budowanie frontendu do `dist/`.

Nie otwieraj `http://127.0.0.1:1420` w przeglądarce — aplikacja działa w oknie Tauri.
