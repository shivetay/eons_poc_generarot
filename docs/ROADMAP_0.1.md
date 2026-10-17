# Droga do wersji 0.1 generatora świata

Ten dokument opisuje **kolejne iteracje** prowadzące do pełnej wersji 0.1.
Plan domenowy i kryteria końcowe pozostają w [generato.md](../generato.md)
oraz w [issue #10](https://github.com/shivetay/tauri_game_POC/issues/10).

Wersja 0.1 **nie jest** zamknięta po pierwszej iteracji — każda iteracja
dostarcza działający, wąski przyrost.

## Cel końcowy (v0.1)

Tekstowy generator świata fantasy (realia średniowieczne): spójny model,
symulacja historii (domyślnie 300 lat), kronika z faktów z przyczynami i skutkami,
bez mapy graficznej. Logika w skrzyni `worldgen/`, niezależna od UI.

## Iteracje

### Iteracja 1 — jedna wieś, 10 lat, konsola w oknie

- Skrzynia `worldgen/`: model wsi, postaci, wydarzeń; pętla 10 lat.
- 1–10 ważnych mieszkańców (losowo), proste wydarzenia (urodzaj, śmierć, sukcesja, konflikt, narodziny).
- Główne okno aplikacji (`cargo tauri dev`): konsola z kroniką tekstową.
- Seed losowy przy każdej generacji; bez nacisku na UX odtwarzalności.

**Kryterium ukończenia:** uruchomienie aplikacji pokazuje kronikę jednej wsi po 10 latach; przycisk generuje kolejną historię.

### Iteracja 1b — cztery osady

- Dokładnie 4 wsie: populacja, wielkość, zajęcie, dzielnice, lista budynków.
- Symulacja i +5 lat obejmują wszystkie osady; wydarzenia mają miejsce (wieś).
- UI: sekcja „Osady” z faktami ze stanu świata; kronika opisuje wszystkie osady.

**Kryterium ukończenia:** generacja pokazuje 4 karty osad + kronikę z wydarzeniami per wieś.

### Iteracja 1c — życie NPC i system społeczny

- NPC: praca, dom, potrzeby (żywność/schronienie/bezpieczeństwo), status społeczny.
- Relacje: małżeństwo, rodzic–dziecko, sojusz, rywalizacja.
- Migracja między osadami; sukcesja preferuje sojuszników.
- Efekty życia widać **tylko jako wpisy w kronice** (bez osobnego UI NPC).

**Kryterium ukończenia:** kronika zawiera pracę, niedostatek, migracje, małżeństwa i więzi.

### Iteracja 1d — dwa państwa

- Dokładnie 2 państwa (po 2 osady): nazwa, ustrój, stolica, władca.
- UI: przynależność państwowa przy karcie osady; szczegóły w kronice.
- Wydarzenia: sukcesja władcy, napięcie międzypaństwowe.

**Kryterium ukończenia:** generacja ma 2 państwa; karty osad pokazują państwo; kronika opisuje władzę.

### Iteracja 2 — odtwarzalność i bogatsze wydarzenia (następna)

- Jawne ziarno (opcjonalnie w UI), ten sam seed → ten sam wynik.
- Więcej typów wydarzeń i silniejsze powiązanie przyczyn → skutków w stanie świata.
- Testy determinizmu jako wymaganie, nie tylko pomocnicze.

### Iteracja 3 — region i sąsiedztwo

- Region z biomem / cechami terenu (bez mapy) ponad istniejącymi 4 osadami.
- Graf sąsiedztwa / odległość logiczna uzasadniająca migracje i konflikty.
- Wydarzenia międzyosadowe wynikające z sąsiedztwa.

### Iteracja 4 — kultury i bogatsza polityka

- Kultury niezależne od państwa; relacje dyplomatyczne; reformy / prawa.

### Iteracja 5 — postacie i relacje w pełniejszym zakresie

- Ambicje, cele, relacje (rodzina, sojusz, rywalizacja).
- Decyzje NPC wpływające na inne postacie i wspólnoty; sukcesja zgodnie z regułami.

### Iteracja 6 — domknięcie v0.1

- Konfigurowalny czas symulacji, domyślnie 300 lat.
- Kalendarz i pory roku w zakresie z `generato.md`.
- Spełnienie kryteriów akceptacji z issue #10.
- Kronika i podsumowania wyłącznie z zapisanych faktów.

## Zasady wspólne dla wszystkich iteracji

- Najpierw stan świata i wydarzenia-dane, potem tekst kroniki.
- Ważne wydarzenia mają przyczynę, uczestników i skutki w stanie świata.
- Brak mapy graficznej w drodze do 0.1.
- Zmiany tylko po zatwierdzonym planie (`AGENTS.md`).

## Status

| Iteracja | Status |
| --- | --- |
| 1 | zaimplementowana |
| 1b | zaimplementowana (4 osady) |
| 1c | zaimplementowana (życie NPC / społeczeństwo w kronice) |
| 1d | zaimplementowana (2 państwa) |
| 2 | następna |
| 3–6 | zaplanowane |
