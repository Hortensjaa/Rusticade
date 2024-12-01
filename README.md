# Jak to ma działać?
Silnik zbudowany na bazie biblioteki ggez do tworzenia gier.
Reprezentuje dużo bardziej wysokopoziomowe podejście do projektowania gier platformowych w 2D poprzez gotowe klasy:
- [x] Config 
- [x] Game 
- [x] Player - gracz porusza się zgodnie z prawami fizyki i zasadami kolizji dla określonych obiektów
- [x] Platform - platformy mogą kolidować z różnych stron i przy uderzeniach z różnych stron mogą wywoływać różne efekty
- [ ] Item - statyczne obiekty, które wykonują jakieś działanie podczas kolizji i pozniej znikaja
- [ ] Creature - poruszające się stworzenia, na które również działają prawa fizyki
Oraz domyślną implementację fizyki, która:
- [ ] Zapobiega kolizjom kolizyjnych obiektów
- [x] Implementuje domyślnie ruchy w obydwie strony z określoną prędkością, skoki, grawitację

### Logika kolizji obiektów fizycznych
Dwa obiekty statyczne nic nie robią w przypadku kolizji.
Dwa obiekty dynamiczne ??.
Obiekt dynamiczny odbija sie od statycznego, jesli statyczny ma zdefiniowanią barierę kolizji z tej strony wpw. przenika.
Domyślnie, bariera kolizji jest tylko z góry.

## MVP
- [x] poruszanie sie gracza w prozni bez fizyki
- [x] generowanie platform
- [x] kolizja gracza i platformy
- [x] fizyka ruchu gracza (uuggghhh)
- [x] przeniesienie consts jako dostępne dla użytkownika

## Rozwinięcie
- itemy
- efekty specjalne (spowolnienie przez określony czas, zatrucie itd.)

## Dodatki
- dodawanie graczowi własnych pól (np. pieniędzy)
- przyjaciele i wrogowie
- nieskonczone generowanie platform (możliwość dla gracza)
- rodzaje kamery gracza (dynamic, fixedX, fixedY)
