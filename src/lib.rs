pub mod game {
    pub fn play(height: &u32, width: &u32, cells: &Vec<(u32, u32)>) -> Vec<(u32, u32)> {
        let mut grid = vec![vec![false; *height as usize]; *width as usize];

        for (x, y) in cells.iter() {
            grid[(x - 1) as usize][(y - 1) as usize] = true;
        }

        let mut next: Vec<(u32, u32)> = vec![];

        for (x, col) in grid.iter().enumerate() {
            for (y, cell_status) in col.iter().enumerate() {
                match scan(cell_status, &x, &y, &grid) {
                    State::Alive => next.push((x as u32, y as u32)),
                    State::Reproduction => next.push((x as u32, y as u32)),
                    _ => ()
                };
            }
        }

        next
    }

    fn scan(is_alive: &bool, x: &usize, y: &usize, grid: &Vec<Vec<bool>>) -> State {
        let scan_range = 2;
        let s_x = x - 2;
        let s_y = y - 2;
        let mut alive_neighbours = 0;

        for i_x in s_x..(s_x + scan_range) {
            for i_y in s_y..(s_y + scan_range) {
                if (i_x != *x || i_y != *y) && grid[i_x][i_y] {
                    alive_neighbours += 1;
                }
            }
        }

        if *is_alive && alive_neighbours < 2 {
            State::Underpopulation
        } else if *is_alive && 2 <= alive_neighbours && alive_neighbours <= 3 {
            State::Alive
        } else if *is_alive && alive_neighbours > 3 {
            State::Overpopulation
        } else if !is_alive && alive_neighbours == 3 {
            State::Reproduction
        } else {
            State::Dead
        }
    }

    enum State {
        Alive,
        Dead,
        Underpopulation,
        Overpopulation,
        Reproduction
    }
}