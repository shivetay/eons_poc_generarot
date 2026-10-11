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

### Iteracja 1 — jedna wieś, 10 lat, konsola w oknie (obecna)

- Skrzynia `worldgen/`: model wsi, postaci, wydarzeń; pętla 10 lat.
- 1–10 ważnych mieszkańców (losowo), proste wydarzenia (urodzaj, śmierć, sukcesja, konflikt, narodziny).
- Główne okno aplikacji (`cargo tauri dev`): konsola z kroniką tekstową.
- Seed losowy przy każdej generacji; bez nacisku na UX odtwarzalności.

**Kryterium ukończenia:** uruchomienie aplikacji pokazuje kronikę jednej wsi po 10 latach; przycisk generuje kolejną historię.

### Iteracja 2 — odtwarzalność i bogatsze wydarzenia

- Jawne ziarno (opcjonalnie w UI), ten sam seed → ten sam wynik.
- Więcej typów wydarzeń i silniejsze powiązanie przyczyn → skutków w stanie świata.
- Testy determinizmu jako wymaganie, nie tylko pomocnicze.

### Iteracja 3 — region i kilka osad

- Region z biomem / cechami terenu (bez mapy).
- Więcej niż jedna osada; proste sąsiedztwo lub odległość logiczna.
- Wydarzenia mogą dotyczyć więcej niż jednej lokacji.

### Iteracja 4 — państwa, kultury, polityka

- Państwo / władza ponad osadami, kultura niezależna od państwa.
- Relacje między wspólnotami; podstawowe zmiany władzy lub praw.

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
| 1 | zaimplementowana (oczekuje weryfikacji użytkownika) |
| 2–6 | zaplanowane |
