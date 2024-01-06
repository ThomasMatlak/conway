// todo export current state
// todo import a file that specifies initial state
// todo make random start, giant block, maybe other easy to implement a command line option
// todo make refresh time an input parameter (decouple simulation ticks from fps?)

use minifb::{self, Window, WindowOptions, Key, Scale, KeyRepeat};
use rand::{self, random};


const WIDTH: usize = 400;
const HEIGHT: usize = 300;

fn main() {
    let mut grid = vec![vec![false; WIDTH]; HEIGHT];

    // for y in 0..grid.len() {
    //     for x in 0..grid[y].len() {
            // random initial state
            // todo why does this leave the right side of the window empty?
            // if random() {
            //     grid[y][x] = true;
            // }
            // giant block in the middle
            // if y >= 100 && y < 200 && x >= 100 && x < 300 {
            //     grid[y][x] = true;
            // }
    //     }
    // }

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
        for x in 0..grid.len() {
            buffer[(y * WIDTH) + x] = if grid[y][x] {u32::MAX} else {u32::MIN};
        }
    }

    window.limit_update_rate(Some(std::time::Duration::from_millis(100)));
    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

    let mut paused = true;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Space) {
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
                for x in 0..grid[y].len() {
                    let mut num_living_neighbors: u8 = 0;
                    // todo implement wrap-around (optional?)

                    for y_d in 0..3 {
                        for x_d in 0..3 {
                            let y_i = y as isize;
                            let x_i = x as isize;
                            let y_delta = y_d - 1;
                            let x_delta = x_d - 1;

                            if (y_delta != 0 || x_delta != 0) && y_i + y_delta >= 0 && y_i + y_delta < (grid.len() as isize) && x_i + x_delta >= 0 && x_i + x_delta < (grid[y].len() as isize) {
                                num_living_neighbors += match grid[(y_i + y_delta) as usize][(x_i + x_delta) as usize] {
                                    true => 1,
                                    false => 0
                                }
                            }
                        }
                    }

                    next_grid[y][x] = match (grid[y][x], num_living_neighbors) {
                        (true, 2) => true,
                        (_, 3) => true,
                        _ => false
                    };
                    buffer[(y * WIDTH) + x] = if next_grid[y][x] {u32::MAX} else {u32::MIN};
                }
            }
            grid = next_grid;
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
