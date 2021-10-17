pub mod game {
    use std::fmt;

    pub fn play(height: &usize, width: &usize, cells: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut grid = vec![vec![false; *height]; *width];

        for (x, y) in cells.iter() {
            grid[x - 1][y - 1] = true;
        }

        let mut next: Vec<(usize, usize)> = vec![];

        for (x, col) in grid.iter().enumerate() {
            for (y, cell_status) in col.iter().enumerate() {
                match scan(cell_status, &x, &y, &grid) {
                    State::Alive => next.push((x + 1, y + 1)),
                    State::Reproduction => next.push((x + 1, y + 1)),
                    _ => ()
                };
            }
        }

        next
    }

    fn scan(is_alive: &bool, x: &usize, y: &usize, grid: &Vec<Vec<bool>>) -> State {
        let scan_range = 3;
        let s_x = if 2 > *x {
            0
        } else {
            x - 2
        };
        let s_y = if 2 > *y {
            0
        } else {
            y - 2
        };
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

    #[derive(Debug, PartialEq)]
    enum State {
        Alive,
        Dead,
        Underpopulation,
        Overpopulation,
        Reproduction
    }

    impl fmt::Display for State {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                State::Alive => write!(f, "Alive"),
                State::Dead => write!(f, "Dead"),
                State::Underpopulation => write!(f, "Underpopulation"),
                State::Overpopulation => write!(f, "Overpopulation"),
                State::Reproduction => write!(f, "Reproduction")
            }
        }
    }

    #[cfg(test)]
    mod scan_should {
        use super::*;

        #[test]
        fn find_underpopulation() {
            let cell_x = 1usize;
            let cell_y = 1usize;

            let mut grid = vec![vec![false; 5]; 5];
            grid[cell_x - 1][cell_y - 1] = true;
            grid[1][1] = true;

            let state = scan(&true, &cell_x, &cell_y, &grid);

            assert_eq!(state, State::Underpopulation, "State [{}] was expected to be [{}]", state, State::Underpopulation);
        }

        #[test]
        fn find_alive() {
            let cell_x = 1usize;
            let cell_y = 1usize;

            let mut grid = vec![vec![false; 5]; 5];
            grid[cell_x - 1][cell_y - 1] = true;
            grid[0][1] = true;
            grid[1][1] = true;
            grid[1][0] = true;

            let state = scan(&true, &cell_x, &cell_y, &grid);

            assert_eq!(state, State::Alive, "State [{}] was expected to be [{}]", state, State::Alive);
        }

        #[test]
        fn find_overpopulation() {
            let cell_x = 2usize;
            let cell_y = 2usize;

            let mut grid = vec![vec![false; 5]; 5];
            grid[cell_x - 1][cell_y - 1] = true;
            grid[0][0] = true;
            grid[1][0] = true;
            grid[2][0] = true;
            grid[2][1] = true;

            let state = scan(&true, &cell_x, &cell_y, &grid);

            assert_eq!(state, State::Overpopulation, "State [{}] was expected to be [{}]", state, State::Overpopulation);
        }

        #[test]
        fn find_reproduction() {
            let cell_x = 1usize;
            let cell_y = 1usize;

            let mut grid = vec![vec![false; 5]; 5];
            grid[1][2] = true;
            grid[2][2] = true;
            grid[2][1] = true;

            let state = scan(&false, &cell_x, &cell_y, &grid);

            assert_eq!(state, State::Reproduction, "State [{}] was expected to be [{}]", state, State::Reproduction);
        }

        #[test]
        fn find_dead() {
            let cell_x = 1usize;
            let cell_y = 1usize;

            let grid = vec![vec![false; 5]; 5];

            let state = scan(&false, &cell_x, &cell_y, &grid);

            assert_eq!(state, State::Dead, "State [{}] was expected to be [{}]", state, State::Dead);
        }
    }

    #[cfg(test)]
    mod play_should {
        use super::*;
        use std::collections::HashSet;

        #[test]
        fn exec_blinker() {
            let height = 5usize;
            let width = 5usize;
            let initial_state: Vec<(usize, usize)> = vec![(3, 2), (3, 3), (3, 4)];
            let expect_state_1: Vec<(usize, usize)> = vec![(2, 3), (3, 3), (4, 3)];
            let expect_state_2: Vec<(usize, usize)> = vec![(3, 2), (3, 3), (3, 4)];

            let state_1 = play(&height, &width, &initial_state);
            let state_2 = play(&height, &width, &state_1);

            assert!(equal_anyorder(&state_1, &expect_state_1), "Expected state after the first tick was {:?} but got {:?}", expect_state_1, state_1);
            assert!(equal_anyorder(&state_2, &expect_state_2), "Expected state after the first tick was {:?} but got {:?}", expect_state_2, state_2);
        }

        fn equal_anyorder(i1: &Vec<(usize, usize)>, i2: &Vec<(usize, usize)>) -> bool {
            let set:HashSet<(usize, usize)> = i2.iter().cloned().collect();
            i1.iter().all(|x| set.contains(&x))
        }
    }
}