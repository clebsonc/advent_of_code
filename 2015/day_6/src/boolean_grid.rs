#[derive(Debug)]
pub struct LighGrid {
    grid: Vec<Vec<bool>>,
}

#[derive(Copy, Clone)]
pub struct Range {
    pub line_start: usize,
    pub col_start: usize,
    pub line_end: usize,
    pub col_end: usize,
}

pub enum State {
    On,
    Off,
    Toggle,
}

impl LighGrid {
    pub fn new(capacity: usize) -> Self {
        let mut grid = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            let line: Vec<bool> = vec![false; capacity];
            grid.push(line);
        }
        Self { grid }
    }

    pub fn change_light_state(&mut self, range: Range, state: &State) {
        for i in range.line_start..=range.line_end {
            for j in range.col_start..=range.col_end {
                if let Some(line) = self.grid.get_mut(i) {
                    if let Some(elem) = line.get_mut(j) {
                        match state {
                            State::On => *elem = true,
                            State::Off => *elem = false,
                            State::Toggle => *elem = !(*elem),
                        }
                    }
                }
            }
        }
    }

    pub fn get_total_lights_turned_on(&self) -> i32 {
        let mut total = 0;
        for rows in &self.grid {
            let s: i32 = rows.iter().map(|&x| x as i32).sum();
            total += s;
        }
        total
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_small_grid() {
        let range = [
            Range {
                line_start: 1,
                line_end: 1,
                col_start: 1,
                col_end: 3,
            },
            Range {
                line_start: 1,
                line_end: 2,
                col_start: 1,
                col_end: 1,
            },
            Range {
                line_start: 3,
                line_end: 3,
                col_start: 3,
                col_end: 3,
            },
            Range {
                line_start: 2,
                line_end: 3,
                col_start: 0,
                col_end: 0,
            },
        ];
        let state = [State::On, State::On, State::On, State::Toggle];
        let mut lg = LighGrid::new(4);
        let zipped = range.iter().zip(state.iter()).map(|(x, y)| (x, y));
        for (x, y) in zipped {
            lg.change_light_state(*x, y);
        }
        let result = lg.get_total_lights_turned_on();
        assert_eq!(result, 7);
    }
}
