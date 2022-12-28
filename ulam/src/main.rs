use std::{f32::consts::PI, process::exit, thread, time::Duration};

use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use minifb::{InputCallback, Key, MouseMode, Window, WindowOptions};
use raqote::{
    DrawOptions, DrawTarget, LineCap, PathBuilder, Point, SolidSource, Source, StrokeStyle,
};
const WIDTH: usize = 500;
const HEIGHT: usize = 500;

const GRID_SIZE: f32 = 10.0;

struct InputListener;

impl InputCallback for InputListener {
    fn add_char(&mut self, _: u32) {}

    fn set_key_state(&mut self, key: Key, _state: bool) {
        if key == Key::Escape {
            exit(0);
        }
    }
}

fn main() {
    let mut window = Window::new(
        "Ulam spiral",
        WIDTH,
        HEIGHT,
        WindowOptions {
            ..WindowOptions::default()
        },
    )
    .unwrap();
    window.set_input_callback(Box::new(InputListener));

    let _font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();

    let size = window.get_size();

    let solid_black = Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0, 0, 0));
    let solid_white = Source::Solid(SolidSource::from_unpremultiplied_argb(
        0xff, 0xff, 0xff, 0xff,
    ));
    let translucent_white =
        Source::Solid(SolidSource::from_unpremultiplied_argb(64, 0xff, 0xff, 0xff));
    let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);

    let options = DrawOptions::new();
    let stroke_style = StrokeStyle {
        cap: LineCap::Square,
        width: 2.0,
        ..Default::default()
    };

    let max_i = (HEIGHT.max(WIDTH) / GRID_SIZE as usize).pow(2);
    let primes = sieve(max_i);

    loop {
        if !window.is_open() {
            exit(0);
        }

        dt.clear(SolidSource::from_unpremultiplied_argb(
            0xff, 0x00, 0x00, 0x00,
        ));

        let (mut prev_x, mut prev_y) = to_coords(0, 0);

        for (i, prime) in primes.iter().enumerate().take(max_i + 1).skip(1) {
            let (x, y) = get_pos(i);
            let mut pb = PathBuilder::new();

            if GRID_SIZE >= 5.0 {
                pb.move_to(prev_x, prev_y);
                pb.line_to(x, y);
                dt.stroke(&pb.finish(), &translucent_white, &stroke_style, &options);
                (prev_x, prev_y) = (x, y);
            }

            if *prime {
                let mut pb = PathBuilder::new();

                pb.arc(x, y, GRID_SIZE / 4.0, 0.0, 2.0 * PI);
                let path = pb.finish();
                dt.fill(&path, &solid_white, &DrawOptions::new());
            }
        }

        if let Some(pos) = window.get_mouse_pos(MouseMode::Discard) {
            let i = reverse_pos(pos.0, pos.1);
            let pos_string = format!("{}", i);
            dt.fill_rect(0.0, 0.0, 60.0, 18.0, &solid_black, &options);
            dt.draw_text(
                &_font,
                16.,
                &pos_string,
                Point::new(2.0, 16.0),
                &solid_white,
                &options,
            );
        }

        window
            .update_with_buffer(dt.get_data(), size.0, size.1)
            .unwrap();

        thread::sleep(Duration::from_millis(10));
    }
}

fn reverse_pos(x: f32, y: f32) -> usize {
    for i in 1..=(HEIGHT.max(WIDTH) / GRID_SIZE as usize).pow(2) {
        let (maybe_x, maybe_y) = get_pos(i);

        if (maybe_x - x).abs() <= GRID_SIZE / 2.0 && (maybe_y - y).abs() <= GRID_SIZE / 2.0 {
            return i;
        }
    }
    0
}

fn to_coords(x: isize, y: isize) -> (f32, f32) {
    static CENTER_X: usize = WIDTH / 2;
    static CENTER_Y: usize = HEIGHT / 2;

    (
        CENTER_X as f32 + x as f32 * GRID_SIZE,
        CENTER_Y as f32 + y as f32 * GRID_SIZE,
    )
}

fn get_pos(i: usize) -> (f32, f32) {
    let i = i as isize - 1;

    let orbital = ((i as f32).sqrt().floor() / 2.0).ceil() as isize;
    let orbital_base = ((orbital * 2).saturating_sub(1)).pow(2);

    let mut x = orbital;
    let mut y = orbital.saturating_sub(1);
    let mut corner_value = orbital_base;
    if i == orbital_base {
        return to_coords(x, y);
    }
    if i < orbital_base + 2 * orbital {
        return to_coords(x, y - (i - orbital_base));
    }
    y -= 2 * orbital - 1;
    corner_value += 2 * orbital - 1;
    if i < orbital_base + 4 * orbital {
        return to_coords(x - (i - corner_value), y);
    }
    x -= 2 * orbital;
    corner_value += 2 * orbital;
    if i < orbital_base + 6 * orbital {
        return to_coords(x, y + (i - corner_value));
    }
    y += 2 * orbital;
    corner_value += 2 * orbital;
    to_coords(x + (i - corner_value), y)
}

fn sieve(n: usize) -> Vec<bool> {
    let mut primes = vec![true; n + 1];
    primes[0] = false;
    primes[1] = false;
    let mut p = 2;
    while p * p <= n {
        if primes[p] {
            let mut index = p * p;
            while index <= n {
                primes[index] = false;
                index += p;
            }
        }

        p += 1;
    }
    primes
}
