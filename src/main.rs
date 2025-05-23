use std::{io::{stdout, Write}, process::Command, process::exit, thread::sleep, time::{self, Duration}};
use terminal_size::{Width, Height, terminal_size};

const FRAME_DURATION: Duration = time::Duration::new(0, 100000000);
const GRAPHIC_WIDTH: usize = 27;
const GRAPHIC_HEIGHT: usize = 7;
const GRAPHIC: [[bool; GRAPHIC_HEIGHT]; GRAPHIC_WIDTH] = [[false, false, false, true, false, true, false], [false, true, true, true, false, true, false], [true, true, true, true, false, true, false], [true, true, true, true, false, true, false], [true, false, false, true, false, true, true], [true, false, false, true, false, true, true], [true, false, false, true, false, true, true], [true, true, true, true, false, true, true], [true, true, true, false, false, true, true], [true, true, true, false, false, true, true], [true, true, false, false, false, false, true], [true, true, true, false, false, false, true], [false, true, true, true, true, false, true], [false, false, true, true, false, false, true], [false, false, true, true, false, true, true], [false, true, true, false, false, true, true], [false, true, false, false, false, true, true], [true, true, false, false, false, true, true], [true, false, true, true, false, true, true], [true, true, true, true, false, true, true], [true, true, true, true, false, true, true], [true, false, false, true, false, true, false], [true, false, false, true, false, true, false], [true, false, false, true, false, true, false], [true, true, true, true, false, true, false], [false, true, true, false, false, true, false], [false, true, true, false, false, false, false]];

fn main() {
    ctrlc::set_handler(move || {
        let mut lock = stdout().lock();
        // This sequence reenables the cursor (we disable it below)
        write!(lock, "{}c\x1b[?25h\n", 27 as char).unwrap();
        stdout().flush().unwrap();
        // This command resets terminal colors
        Command::new("tput").arg("sgr0").status().unwrap();
        exit(0)
    })
    .expect("Error setting Ctrl-C handler");

    let mut width: usize = 0;
    let mut height: usize = 0;
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut vx: i32 = 1;
    let mut vy: i32 = 1;

    let mut screen: Vec<Vec<bool>> = vec![vec![false; height]; width];

    loop {
        let (Width(w), Height(h)) = terminal_size().unwrap(); 
        let new_width = usize::from(w);
        let new_height = usize::from(h);
        if new_width != width || new_height != height {
            // Reset position, velocity, and screen state on resize. Avoids 
            // panics caused by indexing out of bounds.
            x = 0;
            y = 0;
            vx = 1;
            vy = 1;
            screen = vec![vec![false; new_height]; new_width];
            write_initial_screen(new_width, new_height);
        }
        width = new_width;
        height = new_height;


        if width <= GRAPHIC_WIDTH || height <= GRAPHIC_HEIGHT {
            let mut lock = stdout().lock();
            Command::new("tput").arg("sgr0").status().unwrap();
            // Same escape here to reenable the cursor.
            write!(lock, "{}c\x1b[?25hTerminal is too small. Must be at least {} characters wide and {} characters tall.\n", 27 as char, GRAPHIC_WIDTH, GRAPHIC_HEIGHT).unwrap();
            stdout().flush().unwrap();
            exit(1);
        }

        screen = do_screen(width, height, x, y, screen);

        // Handle x boundary collisions
        if x + GRAPHIC_WIDTH + 1 > width {
            vx = -1;
        } else if x == 0 {
            vx = 1;
        }

        // Increment x
        if vx.is_negative() {
            x -= usize::try_from(vx.abs()).unwrap();
        } else {
            x += usize::try_from(vx.abs()).unwrap();
        }

        // Handle y boundary collisions
        if y + GRAPHIC_HEIGHT + 1 > height {
            vy = -1;
        } else if y == 0 {
            vy = 1;
        }

        // Increment y
        if vy.is_negative() {
            y -= usize::try_from(vy.abs()).unwrap();
        } else {
            y += usize::try_from(vy.abs()).unwrap();
        }

        sleep(FRAME_DURATION);
    } 
}

fn write_initial_screen(w: usize, h: usize) {
    let mut screen_string: String = "".to_owned();

    for _ in 0..h {
        for _ in 0..w {
            screen_string.push_str(&"\x1b[48;2;0;0;0m ");
        }
    }
    
    print!("{}{}c{}", "\x1b[?25l", 27 as char, screen_string);
}

fn do_screen(w: usize, h: usize, x: usize, y: usize, screen: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_screen: Vec<Vec<bool>> = vec![vec![false; h]; w];

    // Insert graphic at coordinates
    for j in 0..GRAPHIC_HEIGHT {
        for i in 0..GRAPHIC_WIDTH {
            new_screen[i + x][j + y] = GRAPHIC[i][j];
        }
    }
    let mut out = stdout();

    // Go through both screens and rewrite only the difference
    for j in 0..h {
        for i in 0..w {
            if screen[i][j] != new_screen[i][j] {
                write!(out, "\x1b[{};{}H{}", j + 1, i + 1, if new_screen[i][j] { "\x1b[48;2;255;255;255m " } else { "\x1b[48;2;0;0;0m " }).unwrap();
                stdout().flush().unwrap();
            }
        }
    }

    // \x1b[?25l disables the terminal cursor, which likes to reappear
    write!(out, "\x1b[?25l").unwrap();
    stdout().flush().unwrap();

    return new_screen;
}
