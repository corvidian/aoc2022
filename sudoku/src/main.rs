#![feature(slice_as_chunks)]

use intset::ShrinkSet;

type Sudoku = [[Option<usize>; 9]];
type Candidates = ShrinkSet;

fn main() {
    aoc::input_lines()
        .filter(|line| !line.starts_with('#'))
        .take(1000)
        .for_each(|line| {
            let mut all = line
                .chars()
                .map(|cell| cell.to_digit(10).map(|value| value as usize))
                .collect::<Vec<_>>();

            unsafe { solve(all.as_chunks_unchecked_mut::<9>()) }
        });
}

fn solve(sudoku: &mut Sudoku) {
    //_visualize(sudoku);

    step(sudoku, 0, 0);
}

fn _visualize(sudoku: &Sudoku) {
    println!("------------");
    for chunk in sudoku.chunks(3) {
        for row in chunk {
            for chunk in row.chunks(3) {
                for cell in chunk {
                    if let Some(value) = cell {
                        print!("{value}");
                    } else {
                        print!(" ");
                    }
                }
                print!("|")
            }
            println!();
        }
        println!("------------");
    }
    println!();
}

fn next(y: usize, x: usize) -> (usize, usize) {
    let x = x + 1;
    if x > 8 {
        (y + 1, 0)
    } else {
        (y, x)
    }
}

fn step(sudoku: &mut Sudoku, row: usize, column: usize) -> bool {
    if row > 8 {
        //_visualize(sudoku);
        return true;
    }

    let (next_y, next_x) = next(row, column);

    if sudoku[row][column].is_some() {
        return step(sudoku, next_y, next_x);
    }

    let mut candidates = ShrinkSet::new(10);
    candidates.remove(0);

    check_row(sudoku, row, &mut candidates);
    check_column(sudoku, column, &mut candidates);
    check_block(sudoku, row, column, &mut candidates);

    if candidates.len() == 0 {
        return false;
    }

    for candidate in candidates.iter() {
        sudoku[row][column] = Some(*candidate);
        if step(sudoku, next_y, next_x) {
            return true;
        }
        sudoku[row][column] = None;
    }
    false
}

fn check_row(sudoku: &Sudoku, row: usize, candidates: &mut Candidates) {
    for value in sudoku[row].iter().flatten() {
        candidates.remove(*value);
    }
}

fn check_column(sudoku: &Sudoku, x: usize, candidates: &mut Candidates) {
    for value in sudoku.iter().filter_map(|row| row[x]) {
        candidates.remove(value);
    }
}

fn check_block(sudoku: &Sudoku, y: usize, x: usize, candidates: &mut Candidates) {
    let y = get_chunk_start(y);
    let x = get_chunk_start(x);
    for row in sudoku.iter().skip(y).take(3) {
        for value in row.iter().skip(x).take(3).flatten() {
            candidates.remove(*value);
        }
    }
}

#[inline(always)]
fn get_chunk_start(coord: usize) -> usize {
    match coord {
        x if x < 3 => 0,
        x if x < 6 => 3,
        _ => 6,
    }
}
