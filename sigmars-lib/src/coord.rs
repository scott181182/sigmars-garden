use std::collections::HashSet;

use crate::errors::MatchSetError;
use crate::math::{board_area, row_count, row_length};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoardCoord {
    pub row: usize,
    pub col: usize,
}
impl BoardCoord {
    pub const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub(crate) fn as_index<const S: usize>(&self) -> usize {
        assert!(self.row < row_count::<S>());

        (0..self.row).fold(0, |acc, r| acc + row_length::<S>(r)) + self.col
    }

    pub(crate) fn from_index<const S: usize>(mut index: usize) -> Self {
        assert!(index < board_area::<S>());

        let mut row = 0usize;
        let mut row_len = S;

        loop {
            if index < row_len {
                return BoardCoord { row, col: index };
            }
            index -= row_len;
            row += 1;

            if row < S {
                row_len += 1;
            } else {
                row_len -= 1;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MatchSet(HashSet<BoardCoord>);
impl MatchSet {
    pub fn from<const N: usize>(coords: [BoardCoord; N]) -> Self {
        assert!(N > 0, "MatchSet must contain at least one item");
        Self(coords.into())
    }
    pub fn try_from_iter<T: IntoIterator<Item = BoardCoord>>(
        iter: T,
    ) -> Result<Self, MatchSetError> {
        let set = HashSet::from_iter(iter);
        if set.is_empty() {
            Err(MatchSetError::EmptyMatchSet)
        } else {
            Ok(Self(set))
        }
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn contains(&self, coord: &BoardCoord) -> bool {
        self.0.contains(coord)
    }
    pub fn iter(&self) -> impl Iterator<Item = &BoardCoord> {
        self.0.iter()
    }
}
impl IntoIterator for MatchSet {
    type Item = BoardCoord;
    type IntoIter = std::collections::hash_set::IntoIter<BoardCoord>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl std::hash::Hash for MatchSet {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut coords: Vec<&BoardCoord> = self.0.iter().collect();
        coords.sort();
        for coord in coords {
            coord.hash(state);
        }
    }
}
pub type MatchSets = HashSet<MatchSet>;
