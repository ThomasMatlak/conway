// todo pause/resume button/key
// todo manually advance frame-by-frame
// todo draw the initial state
// todo export current state
// todo import a file that specifies initial state
// todo add ability to draw or erase while running
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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut next_grid = vec![vec![false; WIDTH]; HEIGHT];

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
                    n if n < 2 => false,
                    n if 2 <= n && n <= 3 => true,
                    _ => false
                }
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        grid = next_grid;
    }
}
