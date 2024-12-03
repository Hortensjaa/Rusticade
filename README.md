# Jak to ma działać?
Silnik zbudowany na bazie biblioteki ggez do tworzenia gier.
Reprezentuje dużo bardziej wysokopoziomowe podejście do projektowania gier platformowych w 2D poprzez gotowe klasy logiki:
- [x] Config 
- [x] Game 
oraz obiektów na ekranie; każdy obiekt ma swoją: 
- [x] fizykę (physics) 
- [x] grafikę (graphics)
- [ ] properties (słownik, który przechowuje wartości liczbowe z kluczami)
- [x] dodatkowe opcje zależne od typu np. hp gracza, efekty itemu.

Te obiekty to:
- [x] Player - gracz porusza się zgodnie z prawami fizyki i zasadami kolizji dla określonych obiektów
- [x] Platform - platformy mogą kolidować z różnych stron i przy uderzeniach z różnych stron mogą wywoływać różne efekty
- [x] Item - statyczne obiekty, które wykonują jakieś działanie podczas kolizji i pozniej znikaja
- [x] Creature - stworzenia poruszające się po określonym torze, na które nie działają prawa fizyki (łącznie z kolizjami)
Oraz domyślną implementację fizyki, która:
- [x] Zapobiega kolizjom kolizyjnych obiektów
- [x] Implementuje domyślnie ruchy w obydwie strony z określoną prędkością, skoki, grawitację

### Player
Gracz, który porusza się na planszy. Posiada: 
- swoją fizykę, którą można zmieniać (prędkość, wysokość skoku, położenie i wielkość) - physics
- swoją grafikę, zależną od ustawienia (stanie, skok, skręty) - graphics
oraz domyślne parametry:
1. hp - jeśli spadnie do 0, gra się kończy
2. score - jeśli osiągnie domyślną wartość ustaloną przez użytkownika, gra się kończy (jeśli użytkownik nie zdefiniuje jej w configu, to nic się nie dzieje)
3. props - inne własności dodane przez użytkownika, np. stamina albo coins.

### Obiekty statyczne
1. Platformy - domyślnie mają barierę tylko z góry (ale można dodać też po bokach i z dołu), mogą mieć akcję
2. Itemy - nie mają barier, muszą mieć zdefiniowaną akcję w przypadku kolizji, znikają po użyciu.

## Creatures
1. Potworki poruszają się po torze zdefiniowanym jako listy wektorów (ignorują kolizje i prawa fizyki).
2. Po kolizji wykonuje się akcja zdefniowana przez użytkownika; jeżeli zwróci 'false', to creature zostaje usunięty z listy ("pokonany").


## Możliwości 
1. Każdy obiekt (Player, Creature, Item, Platform) posiada mutowalny słownik własności (props), który może być dowolnie dostosowywany przez użytkownika. Daje to możliwość reprezentowania stanu i logiki akcji zależnej od tego stanu.
2. Domyślnie obiekty reprezentowane są jako kolorowe kwadraty, ale mogą to być też obrazy wprowadzone przez użytkownika.
3. Tryb latania (flying mode) - bez grawitacji
4. Dodanie dodatkowej logiki do aktualizacji stanu gry przed i po wywołaniu Game::update()

## Przykłady
1. Spychające stworki - zbierz wszystkie monety
2. Labirynt - bez grawitacji, jeśli jakiś zwierzak cię dotknie - przegrywasz

## MVP
- [x] poruszanie sie gracza w prozni bez fizyki
- [x] kolizja gracza i platformy
- [x] fizyka ruchu gracza (uuggghhh)
- [x] przeniesienie consts jako dostępne dla użytkownika
- [x] grafiki
- [x] itemy
- [x] dodawanie graczowi własnych pól (np. pieniędzy)
- [x] poprawić fizykę góra-dół
- [ ] dodawanie dodatkowej logiki do pętli zdarzeń (np. score == 100 jako koniec gry)
- [ ] mniej boilerplatu przy tworzeniu gry (przekazywanie configu i kontekstu? żal)
- [ ] grafika - wczytywanie z pliku
- [x] wyrzucenie timedelty
- [ ] posprzątanie configu (na koniec)

