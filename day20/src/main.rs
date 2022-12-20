use std::fmt::Debug;

const DECRYPT_KEY: i64 = 811589153;

fn main() {
    let mut file: Vec<_> = aoc::input_lines()
        .map(|value| value.parse::<i64>().unwrap())
        .collect();

    let mut indexes = (0..file.len()).collect();
    mix(&file, &mut indexes);
    result("Part 1", &file, &indexes);

    for item in &mut file {
        *item *= DECRYPT_KEY
    }

    let mut indexes = (0..file.len()).collect();
    for _ in 0..10 {
        mix(&file, &mut indexes)
    }
    result("Part 2", &file, &indexes);
}

fn result(part: &str, file: &[i64], indexes: &[usize]) {
    let pos_zero = indexes.iter().position(|a| file[*a] == 0).unwrap();
    let thousand = file[indexes[(pos_zero + 1000).rem_euclid(file.len())]];
    let two_thousand = file[indexes[(pos_zero + 2000).rem_euclid(file.len())]];
    let three_thousand = file[indexes[(pos_zero + 3000).rem_euclid(file.len())]];

    println!("{part}: {}", thousand + two_thousand + three_thousand);
}

fn mix(file: &[i64], indexes: &mut Vec<usize>) {
    for i in 0..file.len() {
        let index = indexes.iter().position(|a| *a == i).unwrap();
        shift(indexes, index, file[indexes[index]]);
    }
}

fn shift<T: Debug>(indexes: &mut Vec<T>, i: usize, amount: i64) {
    let n = indexes.len();
    let moves = amount.rem_euclid(n as i64 - 1) as usize;
    let target = (i + moves).rem_euclid(n) + 1;
    if i < target {
        indexes[i..target].rotate_left(1)
    } else if i > target {
        indexes[target..=i].rotate_right(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(vec![1, 2, -3, 3, -2, 0, 4],0,vec![2, 1, -3, 3, -2, 0, 4])]
    #[case(vec![2, 1, -3, 3, -2, 0, 4],0,vec![1, -3, 2, 3, -2, 0, 4])]
    #[case(vec![1, -3, 2, 3, -2, 0, 4],1,vec![1, 2, 3, -2, -3, 0, 4])]
    #[case(vec![1, 2, 3, -2, -3, 0, 4],2,vec![1, 2, -2, -3, 0, 3, 4])]
    #[case(vec![1, 2, -2, -3, 0, 3, 4],2,vec![1, 2, -3, 0, 3, 4, -2])]
    #[case(vec![1, 2, -3, 0, 3, 4, -2],3,vec![1, 2, -3, 0, 3, 4, -2])]
    #[case(vec![1, 2, -3, 0, 3, 4, -2],5,vec![1, 2, -3, 4, 0, 3, -2])]
    fn case1(#[case] mut input: Vec<i64>, #[case] index: usize, #[case] output: Vec<i64>) {
        let amount = input[index];
        shift(&mut input, index, amount);
        assert_eq!(input, output)
    }
}
