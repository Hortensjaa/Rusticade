# Jak to ma działać?
Silnik ma dać możliwość użytkownikowi stworzenia gry w prosty sposób..
Użytkownik może zdefiniować typy pól (oprócz zwykłych platform), które będzie obsługiwać jego plansza (np. pułapki, miejsca leczenia, windy).
Gra obsługuje jednego gracza, który ma określoną przez użytkownika liczbę hp (i ewentualnie inne moce?).
Użytkownik silnika może też dodawać do gry itemy (leczenie, zatrucie, przyspieszenie itd.), które mają wpływ na stan gracza.

## MVP
- [x] poruszanie sie gracza w prozni bez fizyki
- [x] generowanie platform
- [x] kolizja gracza i platformy
- [x] fizyka ruchu gracza (uuggghhh)
- [x] przeniesienie consts jako dostępne dla użytkownika
- [ ] rodzaje kamery gracza (dynamic, fixedX, fixedY)
- [ ] ruch domyślny dla gracza (np. bieg do przodu, grawitacja)

## Rozwinięcie
- itemy
- efekty specjalne (spowolnienie przez określony czas, zatrucie itd.)

## Dodatki
- dodawanie graczowi własnych pól (np. pieniędzy)
- przyjaciele i wrogowie
- nieskonczone generowanie platform (możliwość dla gracza)
