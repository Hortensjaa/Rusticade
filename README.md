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
- [ ] Item - statyczne obiekty, które wykonują jakieś działanie podczas kolizji i pozniej znikaja
- [ ] Creature - poruszające się stworzenia, na które również działają prawa fizyki
Oraz domyślną implementację fizyki, która:
- [x] Zapobiega kolizjom kolizyjnych obiektów
- [x] Implementuje domyślnie ruchy w obydwie strony z określoną prędkością, skoki, grawitację

### Logika kolizji obiektów fizycznych
Dwa obiekty statyczne nic nie robią w przypadku kolizji.
Dwa obiekty dynamiczne ??.
Obiekt dynamiczny odbija sie od statycznego, jesli statyczny ma zdefiniowanią barierę kolizji z tej strony wpw. przenika.
Domyślnie, bariera kolizji platform jest tylko z góry.
Itemy nie mają w ogóle kolizji i znikają po użyciu, tym się różnią od platform.

## MVP
- [x] poruszanie sie gracza w prozni bez fizyki
- [x] kolizja gracza i platformy
- [x] fizyka ruchu gracza (uuggghhh)
- [x] przeniesienie consts jako dostępne dla użytkownika
- [x] grafiki
- [ ] itemy
- [x] dodawanie graczowi własnych pól (np. pieniędzy)
- [ ] dodawanie dodatkowej logiki do pętli zdarzeń (np. score == 100 jako koniec gry)
- [ ] mniej boilerplatu przy tworzeniu gry (przekazywanie configu i kontekstu? żal)

## Rozwinięcie
- [ ] efekty specjalne (spowolnienie przez określony czas, zatrucie itd.)
- [ ] przyjaciele i wrogowie

## Dodatki
- nieskonczone generowanie platform (możliwość dla gracza)
- rodzaje kamery gracza (dynamic, fixedX, fixedY)
