# EONS POC — generator historii świata

Aplikacja desktopowa **Tauri 2 + Leptos 0.8** z domeną w skrzyni Rust `worldgen/`.
Tekstowy generator świata fantasy (realia średniowieczne): osady, NPC, wydarzenia
z przyczynami i skutkami, kronika — bez mapy graficznej.

Cel końcowy v0.1 i kryteria: [generato.md](generato.md),
[issue #10](https://github.com/shivetay/tauri_game_POC/issues/10).
Kolejność dojścia: [docs/ROADMAP_0.1.md](docs/ROADMAP_0.1.md).
Mapa repo: [docs/PROJECT_OVERVIEW.md](docs/PROJECT_OVERVIEW.md).

## Stan (branch `v1`)

- **2 państwa × 2 osady:** ustrój, stolica, władca; osady z populacją, dzielnicami, budynkami
- **Symulacja** 10 lat na start; przycisk **+5 lat** kontynuuje tę samą historię
- **NPC:** praca, dom, potrzeby, status; relacje (rodzina, sojusz, rywalizacja); migracje
- **UI:** karty osad (z państwem) + kronika (życie i polityka jako wpisy)
- Frontend budowany przez `trunk build` do `dist/` (bez `trunk serve` / portu 1420)

## Uruchomienie

```bash
cargo tauri dev
```

albo:

```powershell
powershell ./scripts/dev.ps1
```

Nie otwieraj `http://127.0.0.1:1420` w przeglądarce — aplikacja działa w oknie Tauri.

## Testy

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
trunk build
```

## IDE

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
