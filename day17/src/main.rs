use std::{collections::VecDeque, slice::Iter};

type Position = (i64, i64);

#[derive(Clone, Copy)]
enum Shape {
    Horizontal,
    Cross,
    Angle,
    Vertical,
    Square,
}

static HORIZONTAL_DRAW: &[Position] = &[(0, 0), (0, 1), (0, 2), (0, 3)];
static CROSS_DRAW: &[Position] = &[(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)];
static ANGLE_DRAW: &[Position] = &[(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)];
static VERTICAL_DRAW: &[Position] = &[(0, 0), (1, 0), (2, 0), (3, 0)];
static SQUARE_DRAW: &[Position] = &[(0, 0), (0, 1), (1, 0), (1, 1)];

static HORIZONTAL_DOWN: &[Position] = &[(-1, 0), (-1, 1), (-1, 2), (-1, 3)];
static CROSS_DOWN: &[Position] = &[(0, 0), (-1, 1), (0, 2)];
static ANGLE_DOWN: &[Position] = &[(-1, 0), (-1, 1), (-1, 2)];
static VERTICAL_DOWN: &[Position] = &[(-1, 0)];
static SQUARE_DOWN: &[Position] = &[(-1, 0), (-1, 1)];

static HORIZONTAL_LEFT: &[Position] = &[(0, -1)];
static CROSS_LEFT: &[Position] = &[(0, 0), (1, -1), (2, 0)];
static ANGLE_LEFT: &[Position] = &[(0, -1), (1, 1), (2, 1)];
static VERTICAL_LEFT: &[Position] = &[(0, -1), (1, -1), (2, -1), (3, -1)];
static SQUARE_LEFT: &[Position] = &[(0, -1), (1, -1)];

static HORIZONTAL_RIGHT: &[Position] = &[(0, 4)];
static CROSS_RIGHT: &[Position] = &[(0, 2), (1, 3), (2, 2)];
static ANGLE_RIGHT: &[Position] = &[(0, 3), (1, 3), (2, 3)];
static VERTICAL_RIGHT: &[Position] = &[(0, 1), (1, 1), (2, 1), (3, 1)];
static SQUARE_RIGHT: &[Position] = &[(0, 2), (1, 2)];

impl Shape {
    fn all() -> Iter<'static, Shape> {
        static SHAPES: [Shape; 5] = [
            Shape::Horizontal,
            Shape::Cross,
            Shape::Angle,
            Shape::Vertical,
            Shape::Square,
        ];
        SHAPES.iter()
    }

    fn height(&self) -> i64 {
        match self {
            Shape::Horizontal => 0,
            Shape::Cross => 2,
            Shape::Angle => 2,
            Shape::Vertical => 3,
            Shape::Square => 1,
        }
    }

    fn draw_blocks(&self) -> &[Position] {
        match self {
            Shape::Horizontal => HORIZONTAL_DRAW,
            Shape::Cross => CROSS_DRAW,
            Shape::Angle => ANGLE_DRAW,
            Shape::Vertical => VERTICAL_DRAW,
            Shape::Square => SQUARE_DRAW,
        }
    }

    fn down_blocks(&self) -> &[Position] {
        match self {
            Shape::Horizontal => HORIZONTAL_DOWN,
            Shape::Cross => CROSS_DOWN,
            Shape::Angle => ANGLE_DOWN,
            Shape::Vertical => VERTICAL_DOWN,
            Shape::Square => SQUARE_DOWN,
        }
    }

    fn left_blocks(&self) -> &[Position] {
        match self {
            Shape::Horizontal => HORIZONTAL_LEFT,
            Shape::Cross => CROSS_LEFT,
            Shape::Angle => ANGLE_LEFT,
            Shape::Vertical => VERTICAL_LEFT,
            Shape::Square => SQUARE_LEFT,
        }
    }

    fn right_blocks(&self) -> &[Position] {
        match self {
            Shape::Horizontal => HORIZONTAL_RIGHT,
            Shape::Cross => CROSS_RIGHT,
            Shape::Angle => ANGLE_RIGHT,
            Shape::Vertical => VERTICAL_RIGHT,
            Shape::Square => SQUARE_RIGHT,
        }
    }
}

struct Rock {
    y: i64, // vasen alanurkka
    x: i64,
    shape: Shape,
}

impl Rock {
    fn move_down(&mut self, cavern: &mut Cavern) -> bool {
        if self
            .shape
            .down_blocks()
            .iter()
            .all(|(dy, dx)| cavern.map[(self.y + dy) as usize][(self.x + dx) as usize])
        {
            self.y -= 1;
            true
        } else {
            for (dy, dx) in self.shape.draw_blocks() {
                cavern.map[(self.y + dy) as usize][(self.x + dx) as usize] = false;
            }
            let rock_height = self.y + self.shape.height();
            if rock_height > cavern.max_height as i64 {
                cavern.max_height = rock_height;
            }
            false
        }
    }

    fn move_left(&mut self, cavern: &Cavern) {
        if self
            .shape
            .left_blocks()
            .iter()
            .all(|(dy, dx)| cavern.map[(self.y + dy) as usize][(self.x + dx) as usize])
        {
            self.x -= 1;
        }
    }

    fn move_right(&mut self, cavern: &Cavern) {
        if self
            .shape
            .right_blocks()
            .iter()
            .all(|(dy, dx)| cavern.map[(self.y + dy) as usize][(self.x + dx) as usize])
        {
            self.x += 1;
        }
    }

    fn move_with_the_wind(&mut self, wind: u8, cavern: &Cavern) {
        match wind {
            b'<' => self.move_left(cavern),
            b'>' => self.move_right(cavern),
            other => panic!("Unknown wind: {} ({})", other as char, other),
        };
    }
}

struct Cavern {
    max_height: i64,
    map: Vec<[bool; 9]>,
}

impl Cavern {
    fn new() -> Cavern {
        let mut cavern = Cavern {
            max_height: 0,
            map: vec![],
        };
        cavern.map.push([false; 9]);
        cavern.resize();

        cavern
    }

    fn _visualize(&self) {
        for (y, line) in self.map.iter().enumerate().rev() {
            print!("{:10} ", y);
            for is_air in line {
                if *is_air {
                    print!(".");
                } else {
                    print!("#")
                }
            }
            if self.max_height == y as i64 {
                print!(" <-- max");
            }
            println!();
        }
        println!();
    }

    fn resize(&mut self) {
        while self.max_height + 8 > self.map.len() as i64 {
            self.map
                .push([false, true, true, true, true, true, true, true, false]);
        }
    }
}

fn main() {
    let input = aoc::read_input_string();
    println!("input length: {}", input.len());
    let mut wind_chars = input.bytes().cycle();

    let mut cavern = Cavern::new();
    //cavern.visualize();

    let _magic_number_input = 344i64;
    let _magic_number_example = 7i64;

    let one_pass_size = input.len() as i64 * 5 * _magic_number_input;
    println!("One pass size: {one_pass_size}");

    let mut history = VecDeque::new();
    history.push_front(1 as usize);

    for i in 1..400000i64 {
        let shapes = Shape::all().cycle();
        for shape in shapes.take(one_pass_size as usize) {
            let mut rock = Rock {
                y: (cavern.max_height + 4) as i64,
                x: 3,
                shape: *shape,
            };
            rock.move_with_the_wind(wind_chars.next().unwrap(), &cavern);
            while rock.move_down(&mut cavern) {
                rock.move_with_the_wind(wind_chars.next().unwrap(), &cavern);
            }
            cavern.resize();
        }

        history.push_front(cavern.max_height as usize);

        println!(
            "After {i} passes, difference between heights: {}",
            cavern.max_height as usize - history[1]
        );

        if let (Some(&a), Some(&b), Some(&c)) = (history.get(3), history.get(2), history.get(1)) {
            if c - b == b - a {
                if cavern.map[a..=b] == cavern.map[b..=c] {
                    println!("Stablized between {}-{} and {}-{}", a, b, b, c);

                    println!("passes: {i} {}", history.len() - 1);

                    let during_one_pass = (c - b) as i64;

                    let went = one_pass_size * (i) as i64;
                    println!("Went: {}", went);

                    let simulation_size = 1_000_000_000_000i64;

                    let passes_needed = simulation_size / one_pass_size;
                    println!("Total passes needed {}", passes_needed);
                    let passes_remaining = passes_needed - i;
                    println!("Passes remaining {}", passes_remaining);
                    let rocks_remaining = simulation_size - passes_needed * one_pass_size;
                    println!("Rocks remaining after final pass {}", rocks_remaining);

                    let shapes = Shape::all().cycle();
                    for shape in shapes.take(rocks_remaining as usize) {
                        let mut rock = Rock {
                            y: (cavern.max_height + 4) as i64,
                            x: 3,
                            shape: *shape,
                        };
                        rock.move_with_the_wind(wind_chars.next().unwrap(), &cavern);
                        while rock.move_down(&mut cavern) {
                            rock.move_with_the_wind(wind_chars.next().unwrap(), &cavern);
                        }
                        cavern.resize();
                    }

                    let result = passes_remaining * during_one_pass + cavern.max_height;
                    println!("Result: {result}");
                    return;
                }
            }
        }
    }
}
