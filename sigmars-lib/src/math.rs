pub const fn board_area<const S: usize>() -> usize {
    1 + 3 * S * (S - 1)
}

pub const fn row_count<const S: usize>() -> usize {
    2 * S - 1
}
pub const fn row_length<const S: usize>(row: usize) -> usize {
    assert!(row < row_count::<S>());
    if row < S { S + row } else { 3 * S - 2 - row }
}
