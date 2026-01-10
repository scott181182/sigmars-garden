use crate::board::Board;
use crate::coord::{BoardCoord, MatchSet, MatchSets};
use crate::errors::BoardParseError;
use crate::math::board_area;

pub trait Matchable {
    fn filter_matches<'a, const S: usize, I>(
        &self,
        coord: &BoardCoord,
        board: &'a Board<S>,
        candidates: I,
    ) -> MatchSets
    where
        I: Iterator<Item = (BoardCoord, &'a Tile)>,
        [(); board_area::<S>()]:;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementTile {
    Air,
    Fire,
    Water,
    Earth,
}
impl Matchable for ElementTile {
    fn filter_matches<'a, const S: usize, I>(
        &self,
        coord: &BoardCoord,
        _board: &'a Board<S>,
        candidates: I,
    ) -> MatchSets
    where
        I: Iterator<Item = (BoardCoord, &'a Tile)>,
        [(); board_area::<S>()]:,
    {
        candidates
            .filter(|(c, t)| matches!(t, Tile::Element(e) if e == self && c != coord))
            .map(|(c, _)| MatchSet::from([*coord, c]))
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetalTile {
    Lead = 0,
    Tin = 1,
    Iron = 2,
    Copper = 3,
    Silver = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryTile {
    Life,
    Death,
}
impl Matchable for BinaryTile {
    fn filter_matches<'a, const S: usize, I>(
        &self,
        coord: &BoardCoord,
        _board: &'a Board<S>,
        tiles: I,
    ) -> MatchSets
    where
        I: Iterator<Item = (BoardCoord, &'a Tile)>,
        [(); board_area::<S>()]:,
    {
        tiles
            .filter_map(|(c, t)| match t {
                Tile::Binary(e) if e != self => Some(c),
                _ => None,
            })
            .map(|c| MatchSet::from([*coord, c]))
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Empty,
    Theta,
    Element(ElementTile),
    Binary(BinaryTile),
    Quicksilver,
    Metal(MetalTile),
    Gold,
}
impl TryFrom<char> for Tile {
    type Error = BoardParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            ' ' | '_' => Ok(Tile::Empty),
            'F' => Ok(Tile::Element(ElementTile::Fire)),
            'W' => Ok(Tile::Element(ElementTile::Water)),
            'A' => Ok(Tile::Element(ElementTile::Air)),
            'E' => Ok(Tile::Element(ElementTile::Earth)),
            'L' => Ok(Tile::Binary(BinaryTile::Life)),
            'D' => Ok(Tile::Binary(BinaryTile::Death)),
            'T' => Ok(Tile::Theta),
            'Q' => Ok(Tile::Quicksilver),
            '0' => Ok(Tile::Metal(MetalTile::Lead)),
            '1' => Ok(Tile::Metal(MetalTile::Tin)),
            '2' => Ok(Tile::Metal(MetalTile::Iron)),
            '3' => Ok(Tile::Metal(MetalTile::Copper)),
            '4' => Ok(Tile::Metal(MetalTile::Silver)),
            '5' => Ok(Tile::Gold),
            _ => Err(BoardParseError::UnexpectedTileCharacter(value)),
        }
    }
}

impl Matchable for Tile {
    fn filter_matches<'a, const S: usize, I>(
        &self,
        coord: &BoardCoord,
        board: &'a Board<S>,
        mut candidates: I,
    ) -> MatchSets
    where
        I: Iterator<Item = (BoardCoord, &'a Tile)>,
        [(); board_area::<S>()]:,
    {
        match self {
            Tile::Empty => MatchSets::default(),

            Tile::Element(element_tile) => element_tile.filter_matches(coord, board, candidates),
            // Covers matches with any other ElementTile, and other Thetas.
            Tile::Theta => candidates
                .filter_map(|(c, t)| match t {
                    Tile::Theta if &c != coord => Some(c),
                    Tile::Element(_) => Some(c),
                    _ => None,
                })
                .map(|c| MatchSet::from([*coord, c]))
                .collect(),

            Tile::Binary(binary_tile) => binary_tile.filter_matches(coord, board, candidates),

            // Metal matches are covered by below Quicksilver.
            Tile::Metal(_) => MatchSets::default(),
            // Gold, if selectable, is always clearable.
            Tile::Gold => MatchSets::from([MatchSet::from([*coord])]),
            Tile::Quicksilver => {
                let earliest_metal = board.tiles().fold(None, |acc, val| match (acc, val) {
                    (None, Tile::Metal(m)) => Some(m),
                    (Some(m0), Tile::Metal(m1)) => {
                        if (*m1 as u8) < (*m0 as u8) {
                            Some(m1)
                        } else {
                            acc
                        }
                    }
                    _ => acc,
                });

                candidates
                    .find_map(|(c, t)| match (t, earliest_metal) {
                        (Tile::Metal(m), Some(em)) if m == em => Some(c),
                        _ => None,
                    })
                    .map(|c| MatchSets::from([MatchSet::from([*coord, c])]))
                    .unwrap_or_default()
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    #[test]
    fn test_element_tile_matches_same_type() {
        // Place two Fire tiles and one Water tile
        let tiles = [
            (BoardCoord::new(0, 0), Tile::Element(ElementTile::Fire)),
            (BoardCoord::new(0, 1), Tile::Element(ElementTile::Fire)),
            (BoardCoord::new(2, 1), Tile::Element(ElementTile::Water)),
        ];
        let board = Board::<2>::from_iter(tiles);
        let match_sets = board.find_match_sets();

        assert!(match_sets.len() == 1);
        let match_set = match_sets.into_iter().next().unwrap();
        assert!(match_set.len() == 2);
        // Should only have fire matches.
        assert!(match_set.contains(&BoardCoord::new(0, 0)));
        assert!(match_set.contains(&BoardCoord::new(0, 1)));
    }

    #[test]
    fn test_element_tile_does_not_match_other_types() {
        // Place two Fire tiles and one Water tile
        let tiles = [
            (BoardCoord::new(0, 0), Tile::Element(ElementTile::Fire)),
            (BoardCoord::new(0, 1), Tile::Element(ElementTile::Earth)),
            (BoardCoord::new(2, 1), Tile::Element(ElementTile::Water)),
        ];
        let board = Board::<2>::from_iter(tiles);
        let match_sets = board.find_match_sets();

        assert!(match_sets.is_empty());
    }

    #[test]
    fn test_element_tile_does_not_match_self() {
        // Place one Fire tile
        let tiles = [(BoardCoord::new(1, 1), Tile::Element(ElementTile::Fire))];
        let board = Board::<2>::from_iter(tiles);
        let match_sets = board.find_match_sets();

        // Should not match with itself
        assert!(match_sets.is_empty());
    }

    #[test]
    fn test_element_tile_multiple_matches() {
        // Place three Water tiles
        let tiles = [
            (BoardCoord::new(0, 0), Tile::Element(ElementTile::Water)),
            (BoardCoord::new(1, 2), Tile::Element(ElementTile::Water)),
            (BoardCoord::new(2, 0), Tile::Element(ElementTile::Water)),
        ];
        let board = Board::<2>::from_iter(tiles);
        let match_sets = board.find_match_sets();

        // Should match with both other Water tiles
        assert_eq!(match_sets.len(), 3);
        assert!(match_sets.contains(&MatchSet::from([
            BoardCoord::new(0, 0),
            BoardCoord::new(1, 2)
        ])));
        assert!(match_sets.contains(&MatchSet::from([
            BoardCoord::new(0, 0),
            BoardCoord::new(2, 0)
        ])));
        assert!(match_sets.contains(&MatchSet::from([
            BoardCoord::new(1, 2),
            BoardCoord::new(2, 0)
        ])));
    }
}
