use std::collections::HashSet;
use std::str::FromStr;

use crate::coord::{Coord, MatchSet, MatchSets};
use crate::errors::BoardParseError;
use crate::tile::{Matchable, Tile};

pub const fn board_area<const S: usize>() -> usize {
    1 + 3 * S * (S - 1)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board<const S: usize>
where
    [(); board_area::<S>()]: Sized,
{
    tiles: [Tile; board_area::<S>()],
}

impl<const S: usize> Board<S>
where
    [(); board_area::<S>()]: Sized,
{
    pub const fn empty() -> Self {
        Self {
            tiles: [Tile::Empty; board_area::<S>()],
        }
    }
    pub fn from_tiles(tiles: [Tile; board_area::<S>()]) -> Self {
        Self { tiles }
    }

    pub const fn row_count() -> usize {
        2 * S - 1
    }
    pub const fn row_length(row: usize) -> usize {
        assert!(row < Self::row_count());
        if row < S { S + row } else { 3 * S - 2 - row }
    }

    fn coord_to_idx(coord: &Coord) -> usize {
        assert!(coord.row < Self::row_count());

        (0..coord.row).fold(0, |acc, r| acc + Self::row_length(r)) + coord.col
    }

    fn index_to_coord(mut idx: usize) -> Coord {
        let mut row = 0usize;
        let mut row_len = S;

        loop {
            if idx < row_len {
                return Coord { row, col: idx };
            }
            idx -= row_len;
            row += 1;

            if row < S {
                row_len += 1;
            } else {
                row_len -= 1;
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tiles.iter().all(|t| *t == Tile::Empty)
    }

    pub fn get_tile(&self, coord: &Coord) -> &Tile {
        let idx = Self::coord_to_idx(coord);
        &self.tiles[idx]
    }
    pub fn set_tile(&mut self, coord: &Coord, tile: Tile) {
        let idx = Self::coord_to_idx(coord);
        self.tiles[idx] = tile;
    }
    pub fn remove_tile(&mut self, coord: &Coord) {
        self.set_tile(coord, Tile::Empty);
    }
    pub fn remove_match_set(&mut self, match_set: &MatchSet) {
        for coord in match_set.iter() {
            self.remove_tile(coord);
        }
    }
    pub fn without_match_set(&self, match_set: &MatchSet) -> Self {
        let mut new_board = self.clone();
        new_board.remove_match_set(match_set);
        new_board
    }

    pub fn tiles(&self) -> std::slice::Iter<'_, Tile> {
        self.tiles.iter()
    }
    pub fn nonempty_tiles(&self) -> impl Iterator<Item = (Coord, &Tile)> {
        self.tiles.iter().enumerate().filter_map(|(idx, tile)| {
            if *tile == Tile::Empty {
                None
            } else {
                Some((Self::index_to_coord(idx), tile))
            }
        })
    }

    pub fn get_upper_left_neighbor(&self, coord: &Coord) -> &Tile {
        let is_upper_half = coord.row < S;
        if coord.row == 0 {
            &Tile::Empty
        } else if is_upper_half {
            if coord.col == 0 {
                &Tile::Empty
            } else {
                self.get_tile(&Coord {
                    row: coord.row - 1,
                    col: coord.col - 1,
                })
            }
        } else {
            self.get_tile(&Coord {
                row: coord.row - 1,
                col: coord.col,
            })
        }
    }
    pub fn get_upper_right_neighbor(&self, coord: &Coord) -> &Tile {
        let is_upper_half = coord.row < S;
        let upper_col_index = if is_upper_half {
            coord.col
        } else {
            coord.col + 1
        };

        if coord.row == 0 || upper_col_index >= Self::row_length(coord.row - 1) {
            &Tile::Empty
        } else {
            self.get_tile(&Coord {
                row: coord.row - 1,
                col: upper_col_index,
            })
        }
    }
    pub fn get_left_neighbor(&self, coord: &Coord) -> &Tile {
        if coord.col == 0 {
            &Tile::Empty
        } else {
            self.get_tile(&Coord {
                row: coord.row,
                col: coord.col - 1,
            })
        }
    }
    pub fn get_right_neighbor(&self, coord: &Coord) -> &Tile {
        if coord.col >= Self::row_length(coord.row) - 1 {
            &Tile::Empty
        } else {
            self.get_tile(&Coord {
                row: coord.row,
                col: coord.col + 1,
            })
        }
    }
    pub fn get_lower_left_neighbor(&self, coord: &Coord) -> &Tile {
        let is_lower_half = coord.row >= S - 1;
        if coord.row == Self::row_count() - 1 {
            &Tile::Empty
        } else if is_lower_half {
            if coord.col == 0 {
                &Tile::Empty
            } else {
                self.get_tile(&Coord {
                    row: coord.row + 1,
                    col: coord.col - 1,
                })
            }
        } else {
            self.get_tile(&Coord {
                row: coord.row + 1,
                col: coord.col,
            })
        }
    }
    pub fn get_lower_right_neighbor(&self, coord: &Coord) -> &Tile {
        let is_lower_half = coord.row >= S - 1;
        let lower_col_index = if is_lower_half {
            coord.col
        } else {
            coord.col + 1
        };

        if coord.row == Self::row_count() - 1 || lower_col_index >= Self::row_length(coord.row + 1)
        {
            &Tile::Empty
        } else {
            self.get_tile(&Coord {
                row: coord.row + 1,
                col: lower_col_index,
            })
        }
    }
    pub fn neighbors(&self, coord: &Coord) -> [&Tile; 6] {
        [
            self.get_upper_left_neighbor(coord),
            self.get_upper_right_neighbor(coord),
            self.get_right_neighbor(coord),
            self.get_lower_right_neighbor(coord),
            self.get_lower_left_neighbor(coord),
            self.get_left_neighbor(coord),
        ]
    }

    // Return true if tile at `coord` is selectable (>=3 consecutive empty neighbors)
    pub fn is_selectable(&self, coord: &Coord) -> bool {
        let neighbors = self.neighbors(coord);

        let starting_run = neighbors
            .iter()
            .take_while(|&&tile| tile == &Tile::Empty)
            .count();

        let mut run_size = 0usize;
        for &tile in neighbors.iter().skip(starting_run + 1) {
            if tile == &Tile::Empty {
                run_size += 1;
            } else {
                run_size = 0;
            }
            if run_size >= 3 {
                return true;
            }
        }

        // Checks for wraparound.
        run_size + starting_run >= 3
    }

    pub fn selectable_tiles(&self) -> HashSet<(Coord, &Tile)> {
        self.tiles
            .iter()
            .enumerate()
            .filter_map(|(idx, tile)| {
                if *tile == Tile::Empty {
                    None
                } else {
                    let coord = Self::index_to_coord(idx);
                    if self.is_selectable(&coord) {
                        Some((coord, tile))
                    } else {
                        None
                    }
                }
            })
            .collect::<HashSet<_>>()
    }

    pub fn find_match_sets(&self) -> MatchSets {
        let candidates = self.selectable_tiles();

        candidates
            .iter()
            .flat_map(|(c, t)| t.filter_matches(c, self, candidates.iter().cloned()))
            .collect::<MatchSets>()
    }
}

impl<const S: usize> FromIterator<(Coord, Tile)> for Board<S>
where
    [(); board_area::<S>()]: Sized,
{
    fn from_iter<T: IntoIterator<Item = (Coord, Tile)>>(iter: T) -> Self {
        let mut tile_array = [Tile::Empty; board_area::<S>()];
        for (c, t) in iter {
            tile_array[Self::coord_to_idx(&c)] = t;
        }
        Self { tiles: tile_array }
    }
}

impl<const S: usize> FromStr for Board<S>
where
    [(); board_area::<S>()]: Sized,
{
    type Err = BoardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = [Tile::Empty; board_area::<S>()];
        let line_count = s.matches("\n").count() + 1;
        if line_count != Self::row_count() {
            return Err(BoardParseError::InvalidRowCount(
                Self::row_count(),
                line_count,
            ));
        }

        for (row_idx, line) in s.lines().enumerate() {
            if Board::<S>::row_length(row_idx) != line.len() {
                return Err(BoardParseError::InvalidRowLength(
                    Board::<S>::row_length(row_idx),
                    line.len(),
                ));
            }

            for (col_idx, c) in line.chars().enumerate() {
                let tile = Tile::try_from(c)?;
                tiles[Board::<S>::coord_to_idx(&Coord::new(row_idx, col_idx))] = tile;
            }
        }

        Ok(Self { tiles })
    }
}

#[cfg(test)]
mod tests {
    use crate::tile::ElementTile;

    use super::*;

    #[test]
    fn test_index_to_coord_size_1() {
        // Only one tile at (0, 0)
        assert_eq!(Board::<1>::index_to_coord(0), Coord::new(0, 0));
    }

    #[test]
    fn test_index_to_coord_size_2() {
        // Row 0: 2 tiles
        assert_eq!(Board::<2>::index_to_coord(0), Coord::new(0, 0));
        assert_eq!(Board::<2>::index_to_coord(1), Coord::new(0, 1));
        // Row 1: 3 tiles
        assert_eq!(Board::<2>::index_to_coord(2), Coord::new(1, 0));
        assert_eq!(Board::<2>::index_to_coord(3), Coord::new(1, 1));
        assert_eq!(Board::<2>::index_to_coord(4), Coord::new(1, 2));
        // Row 2: 2 tiles
        assert_eq!(Board::<2>::index_to_coord(5), Coord::new(2, 0));
        assert_eq!(Board::<2>::index_to_coord(6), Coord::new(2, 1));
    }

    #[test]
    fn test_index_to_coord_size_3() {
        // Row 0: 3 tiles
        assert_eq!(Board::<3>::index_to_coord(0), Coord::new(0, 0));
        assert_eq!(Board::<3>::index_to_coord(1), Coord::new(0, 1));
        assert_eq!(Board::<3>::index_to_coord(2), Coord::new(0, 2));
        // Row 1: 4 tiles
        assert_eq!(Board::<3>::index_to_coord(3), Coord::new(1, 0));
        assert_eq!(Board::<3>::index_to_coord(4), Coord::new(1, 1));
        assert_eq!(Board::<3>::index_to_coord(5), Coord::new(1, 2));
        assert_eq!(Board::<3>::index_to_coord(6), Coord::new(1, 3));
        // Row 2: 5 tiles
        assert_eq!(Board::<3>::index_to_coord(7), Coord::new(2, 0));
        assert_eq!(Board::<3>::index_to_coord(8), Coord::new(2, 1));
        assert_eq!(Board::<3>::index_to_coord(9), Coord::new(2, 2));
        assert_eq!(Board::<3>::index_to_coord(10), Coord::new(2, 3));
        assert_eq!(Board::<3>::index_to_coord(11), Coord::new(2, 4));
        // Row 3: 4 tiles
        assert_eq!(Board::<3>::index_to_coord(12), Coord::new(3, 0));
        assert_eq!(Board::<3>::index_to_coord(13), Coord::new(3, 1));
        assert_eq!(Board::<3>::index_to_coord(14), Coord::new(3, 2));
        assert_eq!(Board::<3>::index_to_coord(15), Coord::new(3, 3));
        // Row 4: 3 tiles
        assert_eq!(Board::<3>::index_to_coord(16), Coord::new(4, 0));
        assert_eq!(Board::<3>::index_to_coord(17), Coord::new(4, 1));
        assert_eq!(Board::<3>::index_to_coord(18), Coord::new(4, 2));
    }

    #[test]
    fn test_index_to_coord_size_6() {
        let total_tiles = board_area::<6>();
        for idx in 0..total_tiles {
            let coord = Board::<6>::index_to_coord(idx);
            let back_idx = Board::<6>::coord_to_idx(&coord);
            assert_eq!(
                idx, back_idx,
                "Index to coord and back failed for index {}",
                idx
            );
        }

        assert_eq!(Board::<6>::index_to_coord(16), Coord::new(2, 3));
    }

    #[test]
    fn test_no_matches_when_blocked() {
        // Place two Fire tiles and one Water tile in a line
        // i.e. no matches are selectable
        let board = Board::<3>::from_iter([
            (Coord::new(0, 0), Tile::Element(ElementTile::Fire)),
            (Coord::new(1, 1), Tile::Element(ElementTile::Fire)),
            (Coord::new(2, 2), Tile::Element(ElementTile::Water)),
        ]);
        let match_sets = board.find_match_sets();

        assert!(match_sets.is_empty());
    }
}
