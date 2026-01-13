use std::collections::HashSet;

use crate::board::Board;
use crate::coord::MatchSet;
use crate::math::board_area;
use crate::tile::Tile;

/// A trait for types that can be solved using a sequence of steps.
///
/// Types implementing `Solvable` must define:
/// - The type of step (`Step`) that can be applied to the state.
/// - How to determine if the current state is a goal state (`is_goal`).
/// - How to generate all possible next steps from the current state (`next_steps`).
/// - How to apply a step to the current state to produce a new state (`apply_step`).
///
/// This trait is intended for use in generic puzzle solvers and search algorithms.
///
/// # Requirements
/// - The type must be `Clone`, `Eq`, and implement `std::hash::Hash`.
/// - The associated `Step` type must be `Clone`.
pub trait Solvable: Clone + Eq + std::hash::Hash {
    /// A type representing the transition between states.
    type Step: Clone;

    /// Whether the current state is a goal state.
    fn is_goal(&self) -> bool;
    /// List the steps that are possible from the current state.
    /// This should return steps in priority order, or the order that should be attempted during a solution.
    fn next_steps(&self) -> Vec<Self::Step>;
    /// Apply a step to the current state, returning the resulting state.
    fn apply_step(&self, step: &Self::Step) -> Self;
}

enum MoveType {
    Element,
    ElementTheta,
    ThetaTheta,
    Duality,
    Metal,
    Gold,
    Unknown,
}
impl MoveType {
    fn identify<const S: usize>(board: &Board<S>, match_set: &MatchSet) -> MoveType
    where
        [(); board_area::<S>()]: Sized,
    {
        let tiles: Vec<&Tile> = match_set
            .iter()
            .map(|coord| board.get_tile(coord))
            .collect();

        if tiles.iter().all(|t| matches!(t, Tile::Element(_))) {
            MoveType::Element
        } else if tiles.iter().all(|t| matches!(t, Tile::Theta)) {
            MoveType::ThetaTheta
        } else if tiles
            .iter()
            .all(|t| matches!(t, Tile::Element(_) | Tile::Theta))
        {
            MoveType::ElementTheta
        } else if tiles.iter().all(|t| matches!(t, Tile::Binary(_))) {
            MoveType::Duality
        } else if tiles
            .iter()
            .all(|t| matches!(t, Tile::Metal(_) | Tile::Quicksilver))
        {
            MoveType::Metal
        } else if tiles.iter().all(|t| **t == Tile::Gold) {
            MoveType::Gold
        } else {
            MoveType::Unknown
        }
    }
}

impl<const S: usize> Solvable for Board<S>
where
    [(); board_area::<S>()]: Sized,
{
    type Step = MatchSet;

    fn is_goal(&self) -> bool {
        self.is_empty()
    }

    fn apply_step(&self, step: &Self::Step) -> Self {
        self.without_match_set(step)
    }

    fn next_steps(&self) -> Vec<Self::Step> {
        let mut steps: Vec<MatchSet> = self.find_match_sets().into_iter().collect();

        // Determine priority for making specific moves (lower is tried first).
        // Using 50 as a neutral value.
        steps.sort_by_key(|step| {
            match MoveType::identify(self, step) {
                // Always go for gold.
                MoveType::Gold => 0,
                MoveType::Metal => 50,
                // Go for element match if it's the last pair
                MoveType::Element => {
                    let element_coord = step.iter().next().unwrap();
                    let element_tile = self.get_tile(element_coord);
                    let elements_left = self
                        .nonempty_tiles()
                        .filter(|(_, t)| t == &element_tile)
                        .count();
                    if elements_left <= 2 { 20 } else { 50 }
                }
                MoveType::Duality => 50,
                MoveType::Unknown => 51,
                MoveType::ThetaTheta => 75,
                // Don't prefer this, since it opens us up to holes.
                MoveType::ElementTheta => 100,
            }
        });

        steps
    }
}

pub fn solve_dfs<G: Solvable>(board: &G) -> Option<Vec<G::Step>> {
    let mut seen = HashSet::new();
    let mut path = Vec::new();

    dfs(board, &mut path, &mut seen);

    Some(path)
}

fn dfs<G: Solvable>(
    game: &G,
    path: &mut Vec<G::Step>,
    seen: &mut HashSet<G>,
) -> Option<Vec<G::Step>> {
    if game.is_goal() {
        return Some(path.clone());
    }
    // Prune if we've seen this board before.
    if seen.contains(game) {
        return None;
    }
    seen.insert(game.clone());

    for neighbor in game.next_steps() {
        let next_board = game.apply_step(&neighbor);
        path.push(neighbor);
        if let Some(solution) = dfs(&next_board, path, seen) {
            return Some(solution);
        }
        path.pop();
    }
    None
}
