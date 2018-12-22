# ![Menstruation. Regel dein Essen.](https://img.shields.io/badge/menstruation-Regel%20dein%20Essen.-red.svg?style=for-the-badge) [![Build Status](https://img.shields.io/travis/kmein/menstruation.rs.svg?style=flat-square&logo=travis)](https://travis-ci.org/kmein/menstruation.rs) ![Size](https://img.shields.io/github/languages/code-size/kmein/menstruation.rs.svg?style=flat-square&logo=rust&logoColor=white)

Stell dir vor, du studierst in Berlin und möchtest wissen, was es heute in deiner Mensa gibt.
Du möchtest aber

- weder die Wochenplan-PDF herunterladen müssen,
- noch die miserabel designte Webseite des Studentenwerks aufrufen,
- noch mehr als 3 € bezahlen,
- und bist darüber hinaus Veganer.

Unmöglich? Denkste!
All das ist nun bloß _einen_ Befehl entfernt! (Und im Terminal sogar bunt!)

```bash
$ menstruation -t vegan -p 3.0
SALATE
[1,75 €] Große Salatschale vegan
[0,65 €] Kleine Salatschale vegan

SUPPEN
[0,60 €] Curry-Kokos-Suppe mit Karotten vegan

AKTIONEN
[2,45 €] Mediterrane Tomaten-Gemüse-Sauce an Nudelauswahl vegan

ESSEN
[1,90 €] Frischer Winter-Gemüse-Eintopf mit Petersilie vegan öko
[1,35 €] Eine gebackene China-Knusper-Schnitte an bunter Sojasauce vegan

BEILAGEN
[0,60 €] Paprika-Ingwer-Gemüse vegan
[0,60 €] Erbsen vegan
[0,65 €] Grüne Bohnen bio vegan
[0,60 €] Petersilienkartoffeln vegan
[0,60 €] Mandelreis vegan
[0,35 €] Kräuterjus vegan bio
[0,30 €] Bunte Sojasauce vegan

DESSERTS
[0,60 €] Aprikosenkompott vegan
```
