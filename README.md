# AiZO - Projekt 1

## Technologie:
- **Język programowania**: Rust
- **Opis projektu oraz sprawozdanie**: Markdown

## Założenia:
1. **Podstawowy element sortowanych tablic**: 4 bajtowa liczba całkowita (`i32` / `int`).
2. **Dodatkowo rozpatrywane typy danych**: `char`, `double`, `float`.
3. Sortowanie tablic są alokowane dynamicznie.
4. Dla konkretnego rozmiaru tablicy, pomiar czasu sortowania należy dokonać wielokrotnie (np. 100 razy, przy czym za każdym razem są generowane nowe dane).
5. **Badania są przeprowadzone dla**: 7 reprezentatywnych rozmiarów tablic (np. 10000, 20000, 100000, ... lub 100000, 200000, 400000, 800000, ...).
6. Pomiar zostaje dobrany w taki sposób, aby sortowanie zajęło czas liczony na poziomie `ms` lub więcej. 
7. Pomiar czasowy uwzględnia wyłącznie czas sortowania tablicy.
8. **Rozpatrywane przypadki szczególne**:
    - tablica całkowicie losowa
    - tablica posortowana rosnąco
    - tablica posortowana malejąco
    - tablica posortowana w `33%`
    - tablica posortowana w `66%`
9. **Dodatkowa funkcjonalność programu**: możliwość sprawdzenia poprawności zaimplementowanych algorytmów sortowania (wczytanie danych z pliku i wyświetlenia tablicy przed i po sortowaniu)


## Sprawdzenie poprawności algorytmów sortowania obejmuje:
1. Utworzenie dynamicznej tablicy na podstawie danych zapisanych w pliku tekstowym (`data.csv`).
2. Wyświetlanie na ekranie tablicy przed sortowaniem.
3. Wyświetlanie na ekranie tablicy po sortowaniu.
4. Utworzenie `menu` programu umożliwiające:
    - wczytanie tablicy z pliku o zadanej nazwie.
    - wygenerowanie tablicy z pliku o zadanym rozmiarze zawierające losowe wartości (program ma zapytać o rozmiar tablicy).
    - wyświetlenie ostatnio utworzonej tablicy na ekranie (wygenerowanej lub wvzytanej)
    - uruchomienie wybranego algorytmu na ostatnio utworzonej tablicy (do uruchomienia algorytmu używamy kopii utworzonej tablicy, program powinien umożliwiać testowanie algorytmów dla tych samych danych)
    - wyświetlanie posortowanej tablicy na ekranie.

5. Menu powinno zostać rozszerzone o własne opcje. W przypadku badania wpływu typu danych na czas sortowania można stworzyć menu dwupoziomowe, gdzie na pierwszym poziomie wybieramy typ danych (np. `float` lub `i32`/`int`), a drugi poziom zawiera menu przedstawione powyżej.

## Algorymty sortowania na ocene `5.0`:
1. Sortowanie przez wstawianie (insertion sort).
2. Sortowanie przez kopcowanie (heapsort).
3. Sortowanie Shella:
4. Sortowanie szybkie (quick sort).

## Dodatkowe warunki na `5.0`:
1. Program napisany w wersji obiektowej.
2. Badanie wpływu typu danych np. `int` i `float`.
3. Wpływ wyboru odstępów dla algorytmu Shella (2 różne skrajne wzory tworzące algorytmy o różnych złożonościach).
4. Sposób wyboru `pivota` (skrajny lewy, skrajny prawy, środkowy bądź losowy) - `quicksort`.

