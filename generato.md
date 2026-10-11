# Plan generatora świata — wersja 0.1

## Cel

Stworzyć generator tekstowego świata fantasy osadzonego w realiach średniowiecznych. Generator tworzy świat, a następnie symuluje jego historię przez zadany okres — domyślnie 300 lat. Wersja 0.1 nie tworzy ani nie wyświetla mapy.

Geografia istnieje jako model danych: regiony mają biomy, cechy terenu i połączenia z sąsiadami. Dzięki temu podróże, handel i konflikty mogą wynikać z położenia, mimo że mapa nie jest renderowana.

## Zasady projektowe

- **Najpierw stan świata, potem opowieść.** Symulacja zapisuje fakty oraz zmiany stanu. Kronika i opisy są tworzone na podstawie tych danych.
- **Przyczyny i skutki.** Istotne wydarzenie wskazuje powód, uczestników oraz konsekwencje odzwierciedlone w stanie świata.
- **Powtarzalność.** Ziarno losowości i te same parametry wejściowe odtwarzają ten sam wynik.
- **Spójne referencje.** Relacje między postaciami, lokacjami i państwami wskazują na istniejące obiekty.
- **Czytelny podział.** Model domenowy i symulacja żyją w skrzyni `worldgen/` i nie zależą od UI (Leptos) ani od komend Tauri; tekst kroniki jest warstwą prezentacji nad zapisanymi faktami.
- **Skala wersji 0.1.** Symulowane są ważne postacie, wspólnoty i wydarzenia, a nie codzienne życie każdego mieszkańca.

## Model świata

- **Region:** nazwa, biom, klimat, cechy terenu, zasoby, zagrożenia, sąsiednie regiony oraz znajdujące się w nim lokacje. Cechy geograficzne mogą obejmować góry, lasy, równiny, mokradła, rzeki, jeziora, morza i oceany.
- **Lokacja:** miejsce osadnicze lub specjalne, np. miasto, wieś, osada, twierdza, port, ruina albo miejsce kultu.
- **Miasto i dzielnice:** miasto może zawierać dzielnice o różnych funkcjach, mieszkańcach, wpływach i problemach.
- **Państwo:** terytoria i lokacje, stolica, ustrój, władza, zasoby, prawa, zasady oraz relacje z innymi państwami.
- **Kultura:** obyczaje i wartości wspólnoty. Kultura jest niezależna od państwa: jedno państwo może obejmować kilka kultur, a kultura może występować w kilku państwach.
- **Postać:** pochodzenie, rola, przynależność, ambicje, bieżące cele, cechy i stan życia.
- **Relacja:** powiązanie postaci lub wspólnot, np. pokrewieństwo, sojusz, rywalizacja, zależność albo wrogość.
- **Czas i pogoda:** kalendarz, lata, miesiące i pory roku; pogoda symulowana sezonowo dla regionów.
- **Zasoby:** podstawowe zasoby i zapasy osad, wpływające m.in. na handel, migracje i ryzyko kryzysów.
- **Wydarzenie:** data, typ, przyczyna, uczestnicy, miejsce, skutki oraz odnośniki do powiązanych wydarzeń.

## Przebieg generacji

1. Odczytaj ziarno losowości i konfigurację, w tym długość symulacji (domyślnie 300 lat).
2. Utwórz regiony, ich biomy, cechy geograficzne i sąsiedztwo.
3. Utwórz lokacje, państwa, kultury, ustroje, prawa i początkowe zasoby.
4. Utwórz ważne postacie oraz ich cele i relacje.
5. Symuluj czas rok po roku, aktualizując sezony, pogodę, zasoby, cele, działania i wydarzenia.
6. Zapisz końcowy stan świata wraz z chronologią wydarzeń.
7. Wygeneruj kronikę i opisy państw, osad oraz postaci z zapisanych danych.

## Pętla symulacji

W każdej turze czasowej:

1. Zaktualizuj kalendarz i regionalne warunki pogodowe.
2. Zaktualizuj zasoby i problemy wspólnot na podstawie warunków.
3. Wyznacz możliwe działania NPC i państw na podstawie celów, ambicji, relacji i wiedzy.
4. Rozstrzygnij działania, uwzględniając sąsiedztwo regionów i istniejące ograniczenia.
5. Zapisz istotne wydarzenia wraz z przyczynami i skutkami.
6. Zaktualizuj stan świata oraz sprawdź spójność referencji.

Przykładowy łańcuch przyczyn: nieurodzaj → niedobór zapasów → wzrost cen → protesty w dzielnicy miasta → utrata poparcia władcy → bunt → zmiana władzy lub praw.

## Etapy prac

1. **Model domenowy i losowość** — skrzynia `worldgen/` w workspace, typy danych dla świata i wydarzeń, stabilne identyfikatory, generator pseudolosowy z ziarnem oraz walidacja referencji.
2. **Świat początkowy** — regiony i graf sąsiedztwa, biomy, geografia, osady z dzielnicami, państwa, kultury, ustroje, prawa i zasoby początkowe.
3. **Kalendarz i pogoda** — przejście czasu, pory roku, regionalna pogoda i sezonowe zmiany zasobów.
4. **Postacie i relacje** — ambicje, cele, decyzje, relacje, śmierć i podstawowa sukcesja.
5. **Symulacja wydarzeń** — handel, migracje, negocjacje, kryzysy, bunty, wojny i zmiany polityczne; wydarzenia aktualizują stan świata.
6. **Kronika tekstowa** — chronologia i podsumowania tworzone wyłącznie z zapisanych faktów.
7. **Walidacja generacji** — testy deterministyczności, spójności, wpływu przyczyn i skutków oraz przykładowe generacje wieloletnie.

## Kryteria akceptacji

- Ten sam seed i konfiguracja dają powtarzalny stan świata oraz chronologię.
- Generator tworzy tekstowo opisywalne regiony i lokacje bez mapy graficznej.
- Sąsiedztwo regionów może ograniczać lub uzasadniać podróże, handel i konflikty.
- Symulacja obsługuje konfigurowalny czas, domyślnie 300 lat, oraz kalendarz i pory roku.
- Postacie mają cele i relacje, a ich działania i losy mogą zmieniać sytuację innych postaci lub wspólnot.
- Pogoda i zasoby mogą wpływać na wydarzenia.
- Wydarzenia mają przyczyny, uczestników i skutki zgodne z końcowym stanem świata.
- Kronika nie dodaje faktów sprzecznych z zapisanymi danymi.
- Testy obejmują odtwarzalność, poprawność referencji, przejścia czasu oraz aktualizację stanu po wydarzeniach.

## Poza zakresem wersji 0.1

- Graficzna mapa, współrzędne i wizualna nawigacja.
- Pełna symulacja codziennego życia każdego mieszkańca.
- Taktyczny system walki i szczegółowe rozstrzyganie bitew.
- Rozbudowana ekonomia, edytor reguł generacji i zaawansowana konfiguracja.

## Do doprecyzowania w kolejnych iteracjach

- Dokładny zestaw ras, magii, potworów i ich wpływ na symulację.
- Szczegółowe reguły klimatu, kalendarza i długości sezonów.
- Skala początkowego świata: liczba regionów, państw, osad i ważnych postaci.
- Format zapisu i eksportu wygenerowanego świata.
