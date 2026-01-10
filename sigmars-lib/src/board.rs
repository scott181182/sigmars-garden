use std::collections::HashSet;
use std::str::FromStr;

use crate::coord::{BoardCoord, MatchSet, MatchSets};
use crate::errors::BoardParseError;
use crate::math::{board_area, row_count, row_length};
use crate::tile::{Matchable, Tile};

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

    pub fn is_empty(&self) -> bool {
        self.tiles.iter().all(|t| *t == Tile::Empty)
    }

    pub fn get_tile(&self, coord: &BoardCoord) -> &Tile {
        &self.tiles[coord.as_index::<S>()]
    }
    pub fn set_tile(&mut self, coord: &BoardCoord, tile: Tile) {
        self.tiles[coord.as_index::<S>()] = tile;
    }
    pub fn remove_tile(&mut self, coord: &BoardCoord) {
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
    pub fn nonempty_tiles(&self) -> impl Iterator<Item = (BoardCoord, &Tile)> {
        self.tiles.iter().enumerate().filter_map(|(idx, tile)| {
            if *tile == Tile::Empty {
                None
            } else {
                Some((BoardCoord::from_index::<S>(idx), tile))
            }
        })
    }

    pub fn get_upper_left_neighbor(&self, coord: &BoardCoord) -> &Tile {
        let is_upper_half = coord.row < S;
        if coord.row == 0 {
            &Tile::Empty
        } else if is_upper_half {
            if coord.col == 0 {
                &Tile::Empty
            } else {
                self.get_tile(&BoardCoord {
                    row: coord.row - 1,
                    col: coord.col - 1,
                })
            }
        } else {
            self.get_tile(&BoardCoord {
                row: coord.row - 1,
                col: coord.col,
            })
        }
    }
    pub fn get_upper_right_neighbor(&self, coord: &BoardCoord) -> &Tile {
        let is_upper_half = coord.row < S;
        let upper_col_index = if is_upper_half {
            coord.col
        } else {
            coord.col + 1
        };

        if coord.row == 0 || upper_col_index >= row_length::<S>(coord.row - 1) {
            &Tile::Empty
        } else {
            self.get_tile(&BoardCoord {
                row: coord.row - 1,
                col: upper_col_index,
            })
        }
    }
    pub fn get_left_neighbor(&self, coord: &BoardCoord) -> &Tile {
        if coord.col == 0 {
            &Tile::Empty
        } else {
            self.get_tile(&BoardCoord {
                row: coord.row,
                col: coord.col - 1,
            })
        }
    }
    pub fn get_right_neighbor(&self, coord: &BoardCoord) -> &Tile {
        if coord.col >= row_length::<S>(coord.row) - 1 {
            &Tile::Empty
        } else {
            self.get_tile(&BoardCoord {
                row: coord.row,
                col: coord.col + 1,
            })
        }
    }
    pub fn get_lower_left_neighbor(&self, coord: &BoardCoord) -> &Tile {
        let is_lower_half = coord.row >= S - 1;
        if coord.row == row_count::<S>() - 1 {
            &Tile::Empty
        } else if is_lower_half {
            if coord.col == 0 {
                &Tile::Empty
            } else {
                self.get_tile(&BoardCoord {
                    row: coord.row + 1,
                    col: coord.col - 1,
                })
            }
        } else {
            self.get_tile(&BoardCoord {
                row: coord.row + 1,
                col: coord.col,
            })
        }
    }
    pub fn get_lower_right_neighbor(&self, coord: &BoardCoord) -> &Tile {
        let is_lower_half = coord.row >= S - 1;
        let lower_col_index = if is_lower_half {
            coord.col
        } else {
            coord.col + 1
        };

        if coord.row == row_count::<S>() - 1 || lower_col_index >= row_length::<S>(coord.row + 1) {
            &Tile::Empty
        } else {
            self.get_tile(&BoardCoord {
                row: coord.row + 1,
                col: lower_col_index,
            })
        }
    }
    pub fn neighbors(&self, coord: &BoardCoord) -> [&Tile; 6] {
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
    pub fn is_selectable(&self, coord: &BoardCoord) -> bool {
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

    pub fn selectable_tiles(&self) -> HashSet<(BoardCoord, &Tile)> {
        self.tiles
            .iter()
            .enumerate()
            .filter_map(|(idx, tile)| {
                if *tile == Tile::Empty {
                    None
                } else {
                    let coord = BoardCoord::from_index::<S>(idx);
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

impl<const S: usize> FromIterator<(BoardCoord, Tile)> for Board<S>
where
    [(); board_area::<S>()]: Sized,
{
    fn from_iter<T: IntoIterator<Item = (BoardCoord, Tile)>>(iter: T) -> Self {
        let mut tile_array = [Tile::Empty; board_area::<S>()];
        for (c, t) in iter {
            tile_array[c.as_index::<S>()] = t;
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
        if line_count != row_count::<S>() {
            return Err(BoardParseError::InvalidRowCount(
                row_count::<S>(),
                line_count,
            ));
        }

        for (row_idx, line) in s.lines().enumerate() {
            if row_length::<S>(row_idx) != line.len() {
                return Err(BoardParseError::InvalidRowLength(
                    row_length::<S>(row_idx),
                    line.len(),
                ));
            }

            for (col_idx, c) in line.chars().enumerate() {
                let tile = Tile::try_from(c)?;
                tiles[BoardCoord::new(row_idx, col_idx).as_index::<S>()] = tile;
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
        assert_eq!(BoardCoord::from_index::<1>(0), BoardCoord::new(0, 0));
    }

    #[test]
    fn test_index_to_coord_size_2() {
        // Row 0: 2 tiles
        assert_eq!(BoardCoord::from_index::<2>(0), BoardCoord::new(0, 0));
        assert_eq!(BoardCoord::from_index::<2>(1), BoardCoord::new(0, 1));
        // Row 1: 3 tiles
        assert_eq!(BoardCoord::from_index::<2>(2), BoardCoord::new(1, 0));
        assert_eq!(BoardCoord::from_index::<2>(3), BoardCoord::new(1, 1));
        assert_eq!(BoardCoord::from_index::<2>(4), BoardCoord::new(1, 2));
        // Row 2: 2 tiles
        assert_eq!(BoardCoord::from_index::<2>(5), BoardCoord::new(2, 0));
        assert_eq!(BoardCoord::from_index::<2>(6), BoardCoord::new(2, 1));
    }

    #[test]
    fn test_index_to_coord_size_3() {
        // Row 0: 3 tiles
        assert_eq!(BoardCoord::from_index::<3>(0), BoardCoord::new(0, 0));
        assert_eq!(BoardCoord::from_index::<3>(1), BoardCoord::new(0, 1));
        assert_eq!(BoardCoord::from_index::<3>(2), BoardCoord::new(0, 2));
        // Row 1: 4 tiles
        assert_eq!(BoardCoord::from_index::<3>(3), BoardCoord::new(1, 0));
        assert_eq!(BoardCoord::from_index::<3>(4), BoardCoord::new(1, 1));
        assert_eq!(BoardCoord::from_index::<3>(5), BoardCoord::new(1, 2));
        assert_eq!(BoardCoord::from_index::<3>(6), BoardCoord::new(1, 3));
        // Row 2: 5 tiles
        assert_eq!(BoardCoord::from_index::<3>(7), BoardCoord::new(2, 0));
        assert_eq!(BoardCoord::from_index::<3>(8), BoardCoord::new(2, 1));
        assert_eq!(BoardCoord::from_index::<3>(9), BoardCoord::new(2, 2));
        assert_eq!(BoardCoord::from_index::<3>(10), BoardCoord::new(2, 3));
        assert_eq!(BoardCoord::from_index::<3>(11), BoardCoord::new(2, 4));
        // Row 3: 4 tiles
        assert_eq!(BoardCoord::from_index::<3>(12), BoardCoord::new(3, 0));
        assert_eq!(BoardCoord::from_index::<3>(13), BoardCoord::new(3, 1));
        assert_eq!(BoardCoord::from_index::<3>(14), BoardCoord::new(3, 2));
        assert_eq!(BoardCoord::from_index::<3>(15), BoardCoord::new(3, 3));
        // Row 4: 3 tiles
        assert_eq!(BoardCoord::from_index::<3>(16), BoardCoord::new(4, 0));
        assert_eq!(BoardCoord::from_index::<3>(17), BoardCoord::new(4, 1));
        assert_eq!(BoardCoord::from_index::<3>(18), BoardCoord::new(4, 2));
    }

    #[test]
    fn test_index_to_coord_size_6() {
        let total_tiles = board_area::<6>();
        for idx in 0..total_tiles {
            let coord = BoardCoord::from_index::<6>(idx);
            let back_idx = coord.as_index::<6>();
            assert_eq!(
                idx, back_idx,
                "Index to coord and back failed for index {}",
                idx
            );
        }

        assert_eq!(BoardCoord::from_index::<6>(16), BoardCoord::new(2, 3));
    }

    #[test]
    fn test_no_matches_when_blocked() {
        // Place two Fire tiles and one Water tile in a line
        // i.e. no matches are selectable
        let board = Board::<3>::from_iter([
            (BoardCoord::new(0, 0), Tile::Element(ElementTile::Fire)),
            (BoardCoord::new(1, 1), Tile::Element(ElementTile::Fire)),
            (BoardCoord::new(2, 2), Tile::Element(ElementTile::Water)),
        ]);
        let match_sets = board.find_match_sets();

        assert!(match_sets.is_empty());
    }
}
