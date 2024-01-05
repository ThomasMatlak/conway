// todo manually advance frame-by-frame
// todo draw the initial state
// todo draw or erase while running
// todo export current state
// todo import a file that specifies initial state
// todo make refresh time an input parameter (decouple simulation ticks from fps?)

use minifb::{self, Window, WindowOptions, Key, Scale};
use rand::{self, random};


const WIDTH: usize = 400;
const HEIGHT: usize = 300;

fn main() {
    let mut grid = vec![vec![false; WIDTH]; HEIGHT];

    // random initial state
    // for y in 0..grid.len() {
    //     for x in 0..grid[y].len() {
    //         if random() {
    //             grid[y][x] = true;
    //         }
    //     }
    // }

    // giant block in the middle
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if y >= 100 && y < 200 && x >= 100 && x < 300 {
                grid[y][x] = true;
            }
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

    window.limit_update_rate(Some(std::time::Duration::from_millis(100)));

    let mut paused = true;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Space) {
            paused = !paused;
        }

        let mut next_grid = grid.clone();

        if !paused {
            for y in 0..grid.len() {
                for x in 0..grid[y].len() {
                    buffer[(y * WIDTH) + x] = if grid[y][x] {u32::MAX} else {u32::MIN};

                    let mut num_living_neighbors: u8 = 0;
                    // todo can this seires of ifs be turned nicely into a loop?
                    // todo implement wrap-around (optional?)

                    if y > 0 && x > 0 && grid[y - 1][x - 1] {
                        num_living_neighbors += 1;
                    }
                    if y > 0 && grid[y - 1][x] {
                        num_living_neighbors += 1;
                    }
                    if y > 0 && x < grid[y].len() - 1 && grid[y - 1][x + 1] {
                        num_living_neighbors += 1;
                    }
                    if x < grid[y].len() - 1 && grid[y][x + 1] {
                        num_living_neighbors += 1;
                    }
                    if y < grid.len() - 1 && x < grid[y].len() - 1 && grid[y + 1][x + 1] {
                        num_living_neighbors += 1;
                    }
                    if y < grid.len() - 1 && grid[y + 1][x] {
                        num_living_neighbors += 1;
                    }
                    if y < grid.len() - 1 && x > 0 && grid[y + 1][x - 1] {
                        num_living_neighbors += 1;
                    }
                    if x > 0 && grid[y][x - 1] {
                        num_living_neighbors += 1;
                    }

                    next_grid[y][x] = match num_living_neighbors {
                        n if 2 <= n && n <= 3 => true,
                        _ => false
                    }
                }
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        grid = next_grid;
    }
}
