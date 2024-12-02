# Jak to ma działać?
Silnik zbudowany na bazie biblioteki ggez do tworzenia gier.
Reprezentuje dużo bardziej wysokopoziomowe podejście do projektowania gier platformowych w 2D poprzez gotowe klasy logiki:
- [x] Config 
- [x] Game 
oraz obiektów na ekranie; każdy obiekt ma swoją: 
- [x] fizykę (physics) 
- [x] grafikę (graphics)
- [x] dodatkowe opcje zależne od typu np. hp gracza, efekty itemu.

Te obiekty to:
- [x] Player - gracz porusza się zgodnie z prawami fizyki i zasadami kolizji dla określonych obiektów
- [x] Platform - platformy mogą kolidować z różnych stron i przy uderzeniach z różnych stron mogą wywoływać różne efekty
- [x] Item - statyczne obiekty, które wykonują jakieś działanie podczas kolizji i pozniej znikaja
- [ ] Creature - stworzenia poruszające się po określonym torze, na które nie działają prawa fizyki (łącznie z kolizjami)
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

## MVP
- [x] poruszanie sie gracza w prozni bez fizyki
- [x] kolizja gracza i platformy
- [x] fizyka ruchu gracza (uuggghhh)
- [x] przeniesienie consts jako dostępne dla użytkownika
- [x] grafiki
- [x] itemy
- [x] dodawanie graczowi własnych pól (np. pieniędzy)
- [ ] poprawić fizykę góra-dół
- [ ] dodawanie dodatkowej logiki do pętli zdarzeń (np. score == 100 jako koniec gry)
- [ ] mniej boilerplatu przy tworzeniu gry (przekazywanie configu i kontekstu? żal)

## Rozwinięcie
- [ ] efekty specjalne (spowolnienie przez określony czas, zatrucie itd.)
- [ ] przyjaciele i wrogowie

## Dodatki
- nieskonczone generowanie platform (możliwość dla gracza)
- rodzaje kamery gracza (dynamic, fixedX, fixedY)
