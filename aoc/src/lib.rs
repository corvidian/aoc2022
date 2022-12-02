use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_input_lines() -> Vec<String> {
    use std::env;
    let args = env::args().collect::<Vec<String>>();
    let filename = args.get(1).map(|s| s.as_str()).unwrap_or("input.txt");
    read_lines(filename)
}

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("File not found!");
    io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error reading line"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_lines_works() {
        let result = read_lines("example.txt");
        assert_eq!(result, vec!["1","2","3"]);
    }
}
