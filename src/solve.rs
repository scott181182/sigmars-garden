use std::collections::HashMap;

use crate::board::{Board, board_area};
use crate::coord::MatchSet;
use crate::tile::Tile;

enum MoveType {
    Element,
    ElementTheta,
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

pub fn solve_board<const S: usize>(board: &Board<S>) -> Option<Vec<MatchSet>>
where
    [(); board_area::<S>()]: Sized,
{
    let mut seen: HashMap<Board<S>, usize> = HashMap::new();
    let mut path = Vec::new();
    depth_first_solution(
        board,
        &mut path,
        &mut seen,
        &|board, move_set| {
            match MoveType::identify(board, move_set) {
                // Always go for gold.
                MoveType::Gold => 99,
                MoveType::Metal => 20,
                // Go for element match if it's the last pair
                MoveType::Element => {
                    let element_coord = move_set.iter().next().unwrap();
                    let element_tile = board.get_tile(element_coord);
                    if board
                        .nonempty_tiles()
                        .filter(|(_, t)| t == &element_tile)
                        .count()
                        <= 2
                    {
                        80
                    } else {
                        20
                    }
                }
                MoveType::Duality => 20,
                MoveType::Unknown => 20,
                // Don't prefer this, since it opens us up to holes.
                MoveType::ElementTheta => 0,
            }
        },
        0,
    )
}

fn depth_first_solution<const S: usize, F>(
    board: &Board<S>,
    path: &mut Vec<MatchSet>,
    seen: &mut HashMap<Board<S>, usize>,
    move_priority: &F,
    depth: usize,
) -> Option<Vec<MatchSet>>
where
    [(); board_area::<S>()]: Sized,
    F: Fn(&Board<S>, &MatchSet) -> usize,
{
    if board.is_empty() {
        return Some(path.clone());
    }
    // Prune if we've seen this board with a shorter or equal path
    if let Some(&min_depth) = seen.get(board)
        && depth >= min_depth
    {
        return None;
    }
    seen.insert(board.clone(), depth);

    let mut neighbors: Vec<MatchSet> = board.find_match_sets().into_iter().collect();
    // Rank moves by move_priority, descending (higher = more preferred)
    neighbors.sort_by_key(|m| std::cmp::Reverse(move_priority(board, m)));

    for neighbor in neighbors {
        let next_board = board.without_match_set(&neighbor);
        path.push(neighbor);
        if let Some(solution) =
            depth_first_solution(&next_board, path, seen, move_priority, depth + 1)
        {
            return Some(solution);
        }
        path.pop();
    }
    None
}
