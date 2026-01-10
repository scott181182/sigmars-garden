use sigmars_lib::{BinaryTile, Board, BoardCoord, ElementTile, Tile};
use std::fs;
use std::path::Path;
use std::str::FromStr;

const GOOD_BOARD_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/");

fn read_board_file(path: &Path) -> Board<6> {
    let filedata = fs::read_to_string(path).expect("Failed to read board file");

    Board::<6>::from_str(&filedata)
        .unwrap_or_else(|_| panic!("Could not parse board file {:?}", path))
}

#[test]
fn test_parse_good_boards() {
    let dir_path = Path::new(GOOD_BOARD_DIR);
    for entry in fs::read_dir(dir_path).expect("Failed to read good boards directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        read_board_file(&path);
    }
}

#[test]
fn test_solve_good_boards() {
    let dir_path = Path::new(GOOD_BOARD_DIR);
    for entry in fs::read_dir(dir_path).expect("Failed to read good boards directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let mut board = read_board_file(&path);
        let solution = sigmars_lib::solve_board(&board);
        assert!(
            solution.is_some(),
            "No solution found for board file {:?}",
            path
        );

        let mut solution = solution.unwrap();
        solution.reverse();
        while let Some(match_set) = solution.pop() {
            // Ensure all tiles in the match set are selectable.
            let selectables = board.selectable_tiles();
            for coord in match_set.iter() {
                assert!(
                    selectables.iter().any(|(c, _)| c == coord),
                    "Move set contains non-selectable tile {:?} for board file {:?}",
                    coord,
                    path
                );
            }

            board.remove_match_set(&match_set);
        }
        assert!(
            board.is_empty(),
            "Board not empty after solution for board file {:?}",
            path
        );
    }
}

#[test]
fn test_parse_board1() {
    const BOARD_1_PATH_STR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/board1.txt");
    let board_path = Path::new(BOARD_1_PATH_STR);
    let board = read_board_file(board_path);

    assert_eq!(board.tiles().filter(|t| t == &&Tile::Empty).count(), 36);
    let selectable_tiles = board.selectable_tiles();
    assert_eq!(selectable_tiles.len(), 6);
    assert!(selectable_tiles.contains(&(BoardCoord::new(0, 2), &Tile::Binary(BinaryTile::Life))));
    assert!(selectable_tiles.contains(&(BoardCoord::new(2, 7), &Tile::Binary(BinaryTile::Death))));
    assert!(selectable_tiles.contains(&(BoardCoord::new(3, 0), &Tile::Element(ElementTile::Air))));
    assert!(selectable_tiles.contains(&(BoardCoord::new(7, 8), &Tile::Element(ElementTile::Water))));
    assert!(selectable_tiles.contains(&(BoardCoord::new(8, 0), &Tile::Element(ElementTile::Fire))));
    assert!(selectable_tiles.contains(&(BoardCoord::new(10, 3), &Tile::Element(ElementTile::Water))));
}
