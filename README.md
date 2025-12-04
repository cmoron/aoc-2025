# Advent of Code 2025

Solutions pour les défis [Advent of Code 2025](https://adventofcode.com/2025) en Rust.

## Structure du projet

Ce projet utilise un workspace Cargo pour organiser les solutions par jour :

```
solutions/
└── 2025/
    ├── day01/
    │   ├── src/main.rs
    │   ├── example.txt
    │   ├── input.txt
    │   └── Cargo.toml
    ├── day02/
    └── day03/
    └── ...
```

Chaque jour contient :
- `src/main.rs` : Solution avec les fonctions `part1()` et `part2()`
- `example.txt` : Exemple d'entrée pour tester
- `input.txt` : Entrée personnelle du puzzle
- `Cargo.toml` : Configuration du binaire

## Exécuter une solution

Pour exécuter la solution d'un jour spécifique :

```bash
cargo run -p day01-2025
```

Ou depuis le répertoire du jour :

```bash
cd solutions/2025/day01
cargo run
```

## Exécuter tous les tests

```bash
cargo test --workspace
```

## Compiler toutes les solutions

```bash
cargo build --workspace --release
```
