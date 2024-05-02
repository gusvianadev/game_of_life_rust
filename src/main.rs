// use std::io;
use std::thread::sleep;
use std::time::Duration;

use hsv::hsv_to_rgb;

fn count_neighbors(x: usize, y: usize, world: &[Vec<bool>]) -> u8 {
    let mut count = 0;
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dx, dy) in directions {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;

        if nx >= 0
            && nx < world.len() as i32
            && ny >= 0
            && ny < world[0].len() as i32
            && world[ny as usize][nx as usize]
        {
            count += 1;
        }
    }

    count
}

fn main() {
    // let mut len = String::new();
    // println!("Enter the amount of rows and cols: ");
    // io::stdin()
    //     .read_line(&mut len)
    //     .expect("Whoops! Something went wrong. Try again.");

    let mut world = vec![
        vec![
            false;
            // len.trim()
            //     .parse::<usize>()
            //     .expect("That's not a number pal.")
            17
        ];
        // len.trim()
        //     .parse::<usize>()
        //     .expect("That's not a number pal.")
        17
    ];

    world[4][2] = true;
    world[5][2] = true;
    world[6][2] = true;

    world[4][7] = true;
    world[5][7] = true;
    world[6][7] = true;

    world[10][2] = true;
    world[11][2] = true;
    world[12][2] = true;

    world[10][7] = true;
    world[11][7] = true;
    world[12][7] = true;

    world[4][14] = true;
    world[5][14] = true;
    world[6][14] = true;

    world[4][9] = true;
    world[5][9] = true;
    world[6][9] = true;

    world[10][14] = true;
    world[11][14] = true;
    world[12][14] = true;

    world[10][9] = true;
    world[11][9] = true;
    world[12][9] = true;

    world[2][4] = true;
    world[2][5] = true;
    world[2][6] = true;

    world[7][4] = true;
    world[7][5] = true;
    world[7][6] = true;

    world[2][10] = true;
    world[2][11] = true;
    world[2][12] = true;

    world[7][10] = true;
    world[7][11] = true;
    world[7][12] = true;

    world[14][4] = true;
    world[14][5] = true;
    world[14][6] = true;

    world[9][4] = true;
    world[9][5] = true;
    world[9][6] = true;

    world[14][10] = true;
    world[14][11] = true;
    world[14][12] = true;

    world[9][10] = true;
    world[9][11] = true;
    world[9][12] = true;

    let mut hue: u16 = 0;
    let mut count: u16 = 0;
    let count_break: u16 = 20;

    loop {
        // Clears the screen
        print!("\x1B[2J\x1B[1;1H");
        let new_world = world.clone();

        for (y, row) in new_world.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let neighbor_count = count_neighbors(x, y, &new_world);

                if *cell {
                    if count == count_break {
                        match neighbor_count {
                            _ if !(2..=3).contains(&neighbor_count) => {
                                world[y][x] = false;
                            }
                            _ => (),
                        }
                    }
                    let (r, g, b) = hsv_to_rgb(hue as f64, 0.9, 0.9);
                    let color_string = format!("\x1B[38;2;{};{};{}m■\x1B[0m", r, g, b);

                    print!("{color_string}");
                } else {
                    if neighbor_count == 3 && count == count_break {
                        world[y][x] = true;
                    }

                    let (r, g, b) = hsv_to_rgb(hue as f64, 0.4, 0.3);
                    let color_string = format!("\x1B[38;2;{};{};{}m□\x1B[0m", r, g, b);

                    print!("{color_string}");
                }

                hue = (hue + 1) % 256;
                print!(" ");
            }

            println!();
        }

        sleep(Duration::from_millis(90));
        if count < count_break {
            count += 1;
        } else {
            count = 0;
        }
    }
}
