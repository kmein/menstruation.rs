# Menstruation. Regel dein Essen. [![Build Status](https://img.shields.io/docker/cloud/build/kmein/menstruation.svg?logo=docker&logoColor=white&style=flat-square)](https://hub.docker.com/r/kmein/menstruation)

## CLI

### Installation

```bash
git clone https://github.com/kmein/menstruation.rs && cd menstruation.rs
cargo install --bin menstruation --path .
```

### Benutzungsbeispiele

- `menstruation codes HU` listet alle Mensen der HU mit Nummer auf.
- `menstruation menu -m 191` zeigt den heutigen Speiseplan der Mensa 191 (HU Oase Adlershof).
- `menstruation menu -p 2.5 -t vegan -d 2019-04-04` zeigt die veganen Angebote unter 2,50€ der Mensa Adlershof für den 4.4.2019 an.
- `menstruation menu --green` zeigt nur grün auf der Lebensmittelampel markierte Angebote an.

## REST API

### Installation

#### Docker

```bash
docker pull kmein/menstruation

docker run --rm -p 8000:8000 -ti kmein/menstruation
```

#### Manuell

```bash
git clone https://github.com/kmein/menstruation.rs && cd menstruation.rs
cargo install --bin menstruation_server --path .

menstruation_server  # runs on port 8000
```

### Routen

- GET `/codes` gibt alle Mensen mit Nummer und Adresse zurück. Query-Parameter:
  - `pattern=PATTERN` durchsucht die Mensanamen nach `PATTERN` (optional)
- GET `/menu` gibt einen Speiseplan zurück. Query-Parameter:
  - `mensa=CODE` wählt die Mensa aus
  - `color=FARBEN...` filtert nach bestimmten Farben auf der Lebensmittelampel (optional)
  - `tag=TAGS...` filtert nach bestimmten Kriterien, z.B. vegan (optional)
  - `max_price=CENTS` filtert nach Preis (optional)
  - `date=YYYY-MM-DD` wählt das Datum aus (optional)
