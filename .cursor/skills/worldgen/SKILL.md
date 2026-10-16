---
name: worldgen
description: "Używaj przy projektowaniu, implementacji, testowaniu lub debugowaniu generatora historii świata fantasy, symulacji regionów, osad, państw, kultur, NPC, pogody i kroniki."
---

# Prace nad generatorem świata

Używaj tej umiejętności do zadań związanych z generatorem opisanym w [planie](../../../generato.md).
Kolejność slice’ów: [docs/ROADMAP_0.1.md](../../../docs/ROADMAP_0.1.md).
Zasady repozytorium i skrót architektury: [AGENTS.md](../../../AGENTS.md), [mapa projektu](../../../docs/PROJECT_OVERVIEW.md).

## Bieżący slice (przed pełnym 0.1)

- 4 osady; symulacja 10 lat + `advance` (+5 w UI); kronika z faktów.
- NPC: praca, dom, potrzeby, status; relacje (rodzina, sojusz, rywalizacja); migracje.
- Życie społeczne prezentuj jako **wpisy kroniki**, nie osobny panel UI NPC.
- `tauri dev` używa `trunk build` → `dist/` (bez `trunk serve`).

## Założenia wersji 0.1 (cel końcowy)

- Świat fantasy w realiach średniowiecznych.
- Wynik jest tekstowy; nie twórz mapy graficznej ani współrzędnych.
- Modeluj geografię jako regiony z biomami, cechami terenu i sąsiedztwem.
- Domyślny okres symulacji to 300 lat, z kalendarzem i porami roku.
- Symuluj ważne postacie, wspólnoty i wydarzenia, nie każdą codzienną czynność mieszkańców.
- Postacie mają ambicje, cele i relacje; działania i losy postaci mogą wpływać na innych.

## Procedura

1. Odczytaj `AGENTS.md`, mapę projektu i odpowiedni fragment `generato.md`; nie wczytuj ponownie całej dokumentacji, jeśli zadanie dotyczy jednego modułu.
2. Sprawdź status repozytorium i zidentyfikuj konkretne pliki źródłowe potrzebne do zadania.
3. Przedstaw użytkownikowi proponowany zakres, pliki do zmiany/dodania/usunięcia i plan implementacji. Nie edytuj żadnego pliku przed jednoznacznym zatwierdzeniem planu; sama prośba o funkcję nie wystarcza.
4. Jeśli podczas pracy wymagany zakres lub lista plików istotnie się zmieni, zatrzymaj edycję, przedstaw zaktualizowany plan i poczekaj na ponowne zatwierdzenie.
5. Ustal źródło prawdy dla danych domenowych. Oddziel symulację i zapis faktów od UI oraz formatu kroniki. Kod domenowy i symulacji umieszczaj w skrzyni `worldgen/`, bez zależności od Leptos ani Tauri.
6. Po zatwierdzeniu implementuj mały, kompletny zakres zgodny z planem. Zachowaj deterministyczność dla tego samego ziarna.
7. Dodaj lub zaktualizuj testy dla reguł i skutków wydarzeń, a nie tylko dla formatowania tekstu.
8. Uruchom adekwatne polecenia Rust/workspace i, jeśli zmieniał się frontend, `trunk build`.
9. Podsumuj decyzje, zmienione pliki, wyniki testów i nierozwiązane kwestie.

## Niezmienniki jakości

- Nie zapisuj wydarzeń bez prawidłowych odnośników do uczestników i lokacji.
- Ważne wydarzenia muszą mieć przyczynę i skutki odzwierciedlone w stanie świata.
- Zmarła postać nie wykonuje późniejszych działań; sukcesja musi wynikać z zasad lub jawnego konfliktu.
- Tekst kroniki opisuje zapisane wydarzenia i nie tworzy nowych faktów.
- Nie dodawaj pełnej mapy, taktycznych bitew ani rozbudowanej ekonomii do zakresu 0.1.
- Nie wprowadzaj zależności ani abstrakcji bez konkretnej potrzeby zadania.
