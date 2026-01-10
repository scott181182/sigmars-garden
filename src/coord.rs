use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}
impl Coord {
    pub const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MatchSet(HashSet<Coord>);
impl MatchSet {
    pub fn from<const N: usize>(coords: [Coord; N]) -> Self {
        assert!(N > 0, "MatchSet must contain at least one item");
        Self(coords.into())
    }
    fn try_from_iter<T: IntoIterator<Item = Coord>>(iter: T) -> Result<Self, ()> {
        let set = HashSet::from_iter(iter);
        if set.is_empty() {
            Err(())
        } else {
            Ok(Self(set))
        }
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn contains(&self, coord: &Coord) -> bool {
        self.0.contains(coord)
    }
    pub fn iter(&self) -> impl Iterator<Item = &Coord> {
        self.0.iter()
    }
}
impl IntoIterator for MatchSet {
    type Item = Coord;
    type IntoIter = std::collections::hash_set::IntoIter<Coord>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl std::hash::Hash for MatchSet {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut coords: Vec<&Coord> = self.0.iter().collect();
        coords.sort();
        for coord in coords {
            coord.hash(state);
        }
    }
}
pub type MatchSets = HashSet<MatchSet>;
