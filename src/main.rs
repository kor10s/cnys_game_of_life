use std::env;
use std::{thread, time};
use cnys_game_of_life::game;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        out_help();
        return;
    }

    let height = &args[1].parse::<usize>().unwrap();
    let width = &args[2].parse::<usize>().unwrap();
    let init_cells: Vec<(usize, usize)> = args[3].split(',')
        .map(|v| {
            let coords: Vec<usize> = v.split('-').map(|iv| iv.parse::<usize>().unwrap()).collect();

            (coords[0], coords[1])
        }).collect();

    println!("Starting the game at state:");
    out_state(&height, &width, &init_cells);
    println!();
    
    let tick_duration = time::Duration::from_millis(500);
    let mut state = init_cells;

    loop {
        state = game::play(&height, &width, &state);

        out_state(&height, &width, &state);
        println!();
        thread::sleep(tick_duration);
    }
}

fn out_help() {
    println!("cnys_game_of_life <height::i32> <width::i32> <comma separated coordinates of alive cells like: <latitude>-<longitude>,<latitude>-<longitude>>");
}

fn out_state(height: &usize, width: &usize, cells: &Vec<(usize, usize)>) {
    let mut grid = vec![vec![false; *height]; *width];

    for (x, y) in cells.iter() {
        grid[x - 1][y - 1] = true;
    }
    
    out_header_line(&width);

    for row in grid.iter().rev() {
        let row_cells: Vec<&str> = row.iter().map(|c| match c {
            true => "X",
            false => "O"
        }).collect();

        println!("|{}|", row_cells.join("|"));
        out_header_line(&width);
    }
}

fn out_header_line(length: &usize) {
    let cells: Vec<&str> = vec![0; *length].iter().map(|_| "-").collect();

    println!("+{}+", cells.join("+"));
}