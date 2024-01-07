// todo export current state
// todo import a file that specifies initial state
// todo make random start, giant block, maybe other easy to implement a command line option
// todo make refresh time an input parameter (decouple simulation ticks from fps?)
// todo drawing while not paused
// todo make different rulesets selectable at runtime

mod ruleset;

use minifb::{self, Window, WindowOptions, Key, Scale, KeyRepeat};
use rand::{self, random};
use ruleset::Ruleset;


// todo this is maxing out one CPU core - try to optimize
const WIDTH: usize = 400;
const HEIGHT: usize = 300;

const RULESET: Ruleset = ruleset::Ruleset::Life;

fn main() {
    let mut grid = vec![vec![false; WIDTH]; HEIGHT];

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            // random initial state
            grid[y][x] = random();
            // giant block in the middle
            // if y >= 100 && y < 200 && x >= 100 && x < 300 {
            //     grid[y][x] = true;
            // }
        }
    }

    let mut window = Window::new(
        "Conway's Game of Life - Press ESC to quit",
        WIDTH,
        HEIGHT,
        WindowOptions{
            scale: Scale::X4,
            ..WindowOptions::default()
        }
    ).expect("Unable to create window :(");

    // todo handle resizing
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            buffer[(y * WIDTH) + x] = if grid[y][x] {u32::MAX} else {u32::MIN};
        }
    }

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

    let mut paused = true;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            paused = !paused;
        }

        if paused && window.is_active() && window.get_mouse_down(minifb::MouseButton::Left) {
            let pos = window.get_mouse_pos(minifb::MouseMode::Discard);
            pos.map(|mouse| {
                let x = mouse.0 as usize;
                let y = mouse.1 as usize;
                grid[y][x] = true;
                buffer[(y * WIDTH) + x] = u32::MAX;
            });
        }

        if window.is_key_pressed(Key::C, KeyRepeat::No) {
            grid = vec![vec![false; WIDTH]; HEIGHT];
            buffer = vec![0; WIDTH * HEIGHT];
            paused = true;
        }

        if !paused || window.is_key_pressed(Key::L, KeyRepeat::No) {
            let mut next_grid = grid.clone();

            for y in 0..grid.len() {
                let (y_minus_one, y_plus_one) = match y {
                    0 => (grid.len() - 1, y + 1),
                    i if i == grid.len() - 1 => (y - 1, 0),
                    _ => (y - 1, y + 1)
                };

                for x in 0..grid[y].len() {
                    let (x_minus_one, x_plus_one) = match x {
                        0 => (grid[y].len() - 1, x + 1),
                        i if i == grid[y].len() - 1 => (x - 1, 0),
                        _ => (x - 1, x + 1)
                    };

                    let neighbors = [
                        (x_minus_one, y_minus_one),
                        (x_minus_one, y),
                        (x_minus_one, y_plus_one),
                        (x, y_plus_one),
                        (x_plus_one, y_plus_one),
                        (x_plus_one, y),
                        (x_plus_one, y_minus_one),
                        (x, y_minus_one)
                    ];

                    let mut num_living_neighbors: u8 = 0;
                    for neighbor in neighbors {
                        num_living_neighbors += match grid[neighbor.1][neighbor.0] {
                            true => 1,
                            false => 0
                        }
                    }

                    next_grid[y][x] = match RULESET {
                        Ruleset::Life => life!(grid[y][x], num_living_neighbors),
                        Ruleset::Replicator => replicator!(grid[y][x], num_living_neighbors),
                        Ruleset::Seeds => seeds!(grid[y][x], num_living_neighbors),
                        Ruleset::LifeWithoutDeath => life_without_death!(grid[y][x], num_living_neighbors),
                        Ruleset::ThreeFourLife => three_four_life!(grid[y][x], num_living_neighbors),
                        Ruleset::Diamoeba => diamoeba!(grid[y][x], num_living_neighbors),
                        Ruleset::TwoByTwo => two_by_two!(grid[y][x], num_living_neighbors),
                        Ruleset::HighLife => high_life!(grid[y][x], num_living_neighbors),
                        Ruleset::NightAndDay => night_and_day!(grid[y][x], num_living_neighbors),
                        Ruleset::Morley => morley!(grid[y][x], num_living_neighbors),
                        Ruleset::Anneal => anneal!(grid[y][x], num_living_neighbors),
                    };
                    buffer[(y * WIDTH) + x] = if next_grid[y][x] {u32::MAX} else {u32::MIN};
                }
            }
            grid = next_grid;
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
