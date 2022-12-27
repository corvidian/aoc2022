use intset::ShrinkSet;
use std::process::exit;

type Sudoku = Vec<Vec<Option<usize>>>;

fn main() {
    let mut sudoku = aoc::input_lines()
        .take(9)
        .map(|line| {
            line.split(',')
                .map(|section| {
                    if let Ok(value) = section.parse::<usize>() {
                        Some(value)
                    } else {
                        None
                    }
                })
                .collect::<Vec<Option<usize>>>()
        })
        .take(9)
        .collect::<Vec<_>>();

    visualize(&sudoku);

    step(&mut sudoku, 0, 0);
}

fn visualize(sudoku: &Sudoku) {
    println!("------------");
    for chunk in sudoku.chunks(3) {
        for row in chunk {
            for chunk in row.chunks(3) {
                for ele in chunk {
                    if let Some(value) = ele {
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

fn step(sudoku: &mut Sudoku, row: usize, column: usize) {
    if row > 8 {
        visualize(sudoku);
        exit(0);
    }

    let (next_y, next_x) = next(row, column);

    if sudoku[row][column].is_some() {
        step(sudoku, next_y, next_x);
        return;
    }

    let mut candidates = ShrinkSet::new(10);
    candidates.remove(0);

    check_row(sudoku, row, &mut candidates);
    check_column(sudoku, column, &mut candidates);
    check_block(sudoku, row, column, &mut candidates);

    if candidates.len() == 0 {
        return;
    }

    for candidate in candidates.iter() {
        sudoku[row][column] = Some(*candidate);
        step(sudoku, next_y, next_x);
        sudoku[row][column] = None;
    }
}

fn check_row(sudoku: &Sudoku, row: usize, candidates: &mut ShrinkSet) {
    for value in sudoku[row].iter().flatten() {
        candidates.remove(*value);
    }
}

fn check_column(sudoku: &Sudoku, x: usize, candidates: &mut ShrinkSet) {
    for value in sudoku.iter().filter_map(|row| row[x]) {
        candidates.remove(value);
    }
}

fn check_block(sudoku: &Sudoku, y: usize, x: usize, candidates: &mut ShrinkSet) {
    let y = y / 3 * 3;
    let x = x / 3 * 3;
    for row in sudoku.iter().skip(y).take(3) {
        for value in row.iter().skip(x).take(3).flatten() {
            candidates.remove(*value);
        }
    }
}
