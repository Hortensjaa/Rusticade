# Jak to ma działać?
Silnik ma dać możliwość użytkownikowi stworzenia gry w prosty sposób.
Każda gra ma losowo tworzoną planszę typu mario (to bedzie okropne, bo nie może sie wylosować za trudne).
Użytkownik może zdefiniować typy pól (oprócz zwykłych platform), które będzie obsługiwać jego plansza (np. pułapki, miejsca leczenia, windy).
Gra obsługuje jednego gracza, który ma określoną przez użytkownika liczbę hp (i ewentualnie inne moce?).
Celem gry jest zawsze zebranie jak największej liczby punktów, bo gra sie generuje w nieskonczonosc.
Użytkownik silnika może też dodawać do gry itemy (leczenie, zatrucie, przyspieszenie itd.), które mają wpływ na stan gracza i npc (wrogów i przyjaciół).

## MVP
[x] poruszanie sie gracza w prozni bez fizyki
[x] generowanie platform
[ ] kolizja gracza i platformy
[ ] fizyka ruchu gracza (uuggghhh)
[ ] przeniesienie consts jako dostępne dla użytkownika
[ ] rodzaje kamery gracza (dynamic, fixedX, fixedY)
[ ] ruch domyślny dla gracza (np. bieg do przodu, grawitacja)

## Rozwinięcie
- itemy
- efekty specjalne (spowolnienie przez określony czas, zatrucie itd.)
- nieskonczone generowanie platform (możliwość dla gracza)

## Dodatki
- dodawanie graczowi własnych pól (np. pieniędzy)
- przyjaciele i wrogowie
