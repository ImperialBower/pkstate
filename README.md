[![Build and Test](https://github.com/ImperialBower/pkstate/actions/workflows/CI.yaml/badge.svg)](https://github.com/ImperialBower/pkstate/actions/workflows/CI.yaml)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](LICENSE-APACHE)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)
[![Crates.io Version](https://img.shields.io/crates/v/pkstate.svg)](https://crates.io/crates/pkstate)
[![Rustdocs](https://docs.rs/pkstate/badge.svg)](https://docs.rs/pkstate/)

---

# PKState

A Rust library for representing, serializing, and deserializing the state of a poker game. `pkstate`
models everything needed to describe a hand in progress or a complete hand history: the game type,
forced bets, the players at the table, the community board cards, and the full sequence of actions
across every street — all serializable to and from human-readable YAML.

## Features

- **Full game state** — captures game type, button position, forced bets, players, board cards, and
  action history in a single `PKState` struct.
- **Up to 12 players** — `Action` variants cover seats P0 through P11.
- **Rich action model** — per-player `Dealt`, `Check`, `CBR` (call/bet/raise), `Fold`, `Wins`, and
  `Loses` actions, plus a `DealCommon` action for community cards.
- **Human-readable YAML** — card piles are serialized as Unicode card strings (e.g. `"A♠ K♥"`)
  rather than nested structures, keeping hand histories easy to read and edit.
- **Optional fields** — `id`, `datetime`, and `board` are skipped when `None`, keeping YAML output
  clean.

## Supported Game Types

| Variant          | Cards Dealt | Board Cards |
|------------------|-------------|-------------|
| `NoLimitHoldem`  | 2           | 5           |
| `LimitHoldem`    | 2           | 5           |
| `PLO`            | 4           | 5           |
| `Razz`           | 7           | 0           |

## Quick Start

Add `pkstate` to your `Cargo.toml`:

```toml
[dependencies]
pkstate = "0.1.0"
```

### Building a hand and serializing to YAML

```rust
use pkstate::{PKState, act::{Action, Round}, game::{ForcedBets, GameType}, seat::Seat};
use cardpack::prelude::*;

let players = vec![
    Seat { id: None, name: "Alice".to_string(), stack: 1_000 },
    Seat { id: None, name: "Bob".to_string(),   stack: 1_000 },
];

let preflop = Round(vec![
    Action::P0Dealt(basic!("A♠ K♠")),
    Action::P1Dealt(basic!("7♦ 2♣")),
    Action::P0CBR(100),
    Action::P1Fold,
    Action::P0Wins(100),
]);

let state = PKState {
    id: Some("example-hand".to_string()),
    datetime: None,
    game: GameType::NoLimitHoldem,
    button: 0,
    forced_bets: ForcedBets::new(50, 100),
    board: None,
    players,
    rounds: vec![preflop],
};

let yaml = serde_yaml_bw::to_string(&state).unwrap();
println!("{}", yaml);
```

Output:

```yaml
id: example-hand
game: NoLimitHoldem
button: 0
forced_bets:
  small: 50
  big: 100
players:
- name: Alice
  stack: 1000
- name: Bob
  stack: 1000
rounds:
- - P0Dealt: A♠ K♠
  - P1Dealt: 7♦ 2♣
  - P0CBR: 100
  - P1Fold
  - P0Wins: 100
```

### Deserializing from YAML

```rust
let state: PKState = serde_yaml_bw::from_str(&yaml).unwrap();
```

## YAML Format

### `PKState`

| Field         | Type                     | Optional | Description                                  |
|---------------|--------------------------|----------|----------------------------------------------|
| `id`          | `String`                 | ✅       | Unique identifier for the hand               |
| `datetime`    | `DateTime<Utc>`          | ✅       | Timestamp of the hand                        |
| `game`        | `GameType`               | ❌       | Variant of poker being played                |
| `button`      | `usize`                  | ❌       | Seat index of the dealer button              |
| `forced_bets` | `ForcedBets`             | ❌       | Small blind, big blind, optional straddles / ante |
| `board`       | `BasicPile` (as string)  | ✅       | Community cards, e.g. `"9♣ 6♦ 5♥ 5♠ 8♠"`   |
| `players`     | `Vec<Seat>`              | ❌       | Ordered list of players at the table         |
| `rounds`      | `Vec<Round>`             | ❌       | Ordered list of betting rounds               |

### `Action` YAML representations

| Variant            | YAML                     |
|--------------------|--------------------------|
| `DealCommon(pile)` | `DealCommon: "9♣ 6♦ 5♥"` |
| `PnDealt(pile)`    | `P0Dealt: "A♠ K♠"`       |
| `PnCheck`          | `P0Check`                |
| `PnCBR(amount)`    | `P0CBR: 200`             |
| `PnFold`           | `P1Fold`                 |
| `PnWins(amount)`   | `P3Wins: 1000`           |
| `PnLoses(amount)`  | `P4Loses: 1000`          |

## Modules

- **`act`** — `Action` enum and `Round` newtype. Encodes every per-player and community-card action
  that can occur during a hand, with custom serde implementations that represent `BasicPile` values
  as Unicode card strings.
- **`game`** — `GameType` enum and `ForcedBets` struct. Describes the variant being played and the
  mandatory bets required to start each hand.
- **`seat`** — `Seat` struct. Represents a player at the table: their optional id, display name,
  and chip stack.
- **`util`** — Shared serde helper functions for serializing and deserializing `BasicPile` and
  `Option<BasicPile>` as card strings.

## Resources

- [Poker Hand History File Format Specification](https://github.com/uoftcprg/phh-std)
- [`cardpack` crate](https://crates.io/crates/cardpack) — playing card types used throughout

## Development

This project uses [cargo-make](https://github.com/sagiegurari/cargo-make) to manage tasks. Install
it with:

```shell
cargo install cargo-make
```

The default `cargo make` task runs:

- `cargo fmt`
- `cargo clean`
- `cargo build`
- `cargo test`
- `cargo clippy` with `clippy::pedantic` lint settings
- `cargo doc --no-deps`

```shell
cargo make
```

To open the generated docs in your browser:

```shell
cargo make docs
```

## License

Licensed under either of:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.
