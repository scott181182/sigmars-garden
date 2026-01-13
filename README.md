<img src="./assets/logo.png" alt="Sigmar's Garden" height="80">

A tool for automatically solving Sigmar's Garden, the [Zachtronics](https://zachtronics.com/) game-within-a-game seen in [Opus Magnum](https://store.steampowered.com/app/558990/Opus_Magnum/).

# Usage

## Installation

First, install this program by cloning the git repository and making a build. Then the executable can be found in `sigmars-cli/target`

```sh
git clone https://github.com/scott181182/sigmars-garden.git
cd sigmars-garden
# NOTE: this project current requires the nightly toolchain.
cargo build --release
```

## Puzzle Input

The program currently reads a text file with a Sigmar's Garden puzzle in it to solve (see [Future Plans](#future-plans)). The file format is plain text and uses single characters to represent each tile in the game:

- Empty space or `_` (underscore): Empty tile
- `T`: Salt (theta)
- `L`: Vitae/Life
- `D`: Mors/Death
- `Q`: Quicksilver
- Elements
  - `F`: Fire
  - `W`: Water
  - `A`: Air
  - `E`: Earth
- Metals
  - `0`: Lead
  - `1`: Tin
  - `2`: Iron
  - `3`: Copper
  - `4`: Silver
  - `5`: Gold

Each line of the file corresponds to a single row on the board.

Examples of this format can be seen in the [test data](./sigmars-lib/tests/data)

## Running the Solver

```sh
# Optional: install this binary to another location
./target/release/sigmars-cli <puzzle-input>
```

This will spit out several lines of moves you should make in order to get to a solution. Each move includes the tile types you should click, and where they are on the board. Board coordinates are 0-based and given in `(row, index)` form, so the top row is row 0, and the 0th index on a given row is always the left-most tile.

# Future Plans

- Support Quintessence
- Add library for image recognition and deriving board state from your screen
- Implement robot to input the puzzle solution for you
- Refine heuristics for solution planning
  - e.g. avoid using salt in ways that make solving impossible
- Refactor [MatchSet](./sigmars-lib/src/coord.rs) to include [MoveType](./sigmars-lib/src/solve.rs) information to avoid unnecessary recalculation
- Make solution printout easier to read
