use std::collections::HashSet;

type Position = (i32, i32);

fn main() {
    let input = aoc::read_input_lines();
    let mut rope = [(0, 0); 2];
    let mut visited: HashSet<Position> = HashSet::new();
    input
        .iter()
        .for_each(|line| instruction(line, &mut rope, &mut visited));

    println!("Part 1: {}", visited.len());

    let mut rope = [(0, 0); 10];
    visited.clear();
    input
        .iter()
        .for_each(|line| instruction(line, &mut rope, &mut visited));

    println!("Part 2: {}", visited.len());
}

fn instruction<const N: usize>(
    instruction: &str,
    rope: &mut [Position; N],
    visited: &mut HashSet<Position>,
) {
    let direction = instruction.chars().next().unwrap();
    let delta = match direction {
        'R' => (1, 0),
        'L' => (-1, 0),
        'U' => (0, 1),
        'D' => (0, -1),
        _ => panic!("Invalid instruction"),
    };
    let amount: usize = instruction
        .split_once(' ')
        .unwrap()
        .1
        .parse()
        .expect("Invalid move amount");
    (0..amount).for_each(|_| movement(&delta, rope, visited));
}

fn movement<const N: usize>(
    delta: &Position,
    rope: &mut [Position; N],
    visited: &mut HashSet<Position>,
) {
    rope[0].0 += delta.0;
    rope[0].1 += delta.1;

    for i in 1..N {
        let head = rope[i - 1];
        move_rope(&head, &mut rope[i]);
    }

    //println!("{rope:?}");

    visited.insert(rope[N - 1]);
}

fn move_rope(head: &Position, next: &mut Position) {
    if (head.0 - next.0).abs() > 1 || (head.1 - next.1).abs() > 1 {
        next.0 += (head.0 - next.0).signum();
        next.1 += (head.1 - next.1).signum();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case((0,0),(0,0),(0,0))]
    #[case((1,1),(0,0),(0,0))]
    #[case((2,0),(0,0),(1,0))]
    #[case((0,2),(0,0),(0,1))]
    #[case((-2,0),(0,0),(-1,0))]
    #[case((0,-2),(0,0),(0,-1))]
    #[case((2,1),(0,0),(1,1))]
    #[case((1,2),(0,0),(1,1))]
    #[case((-2,-1),(0,0),(-1,-1))]
    #[case((-1,-2),(0,0),(-1,-1))]
    #[case((6,5),(4,4),(5,5))]
    fn test_move_rope(#[case] head: Position, #[case] next: Position, #[case] expected: Position) {
        let mut next = next;

        move_rope(&head, &mut next);

        assert_eq!(next, expected);
    }
}
