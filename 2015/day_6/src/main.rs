mod boolean_grid;
mod brightness_grid;

use boolean_grid::{LighGrid, Range, State};
use brightness_grid::{BrightnessGrid, Range as BRange, State as BState};
use std::fs::read_to_string;
// use crate::boolean_light_grid::{LighGrid, Range, State};

fn main() {
    let data = read_to_string("input.txt").unwrap();
    let data = data.to_lowercase();
    let data: Vec<&str> = data.split('\n').filter(|x| !x.is_empty()).collect();
    let data: Vec<&str> = data.iter().map(|&x| x.trim()).collect();

    let mut grid = LighGrid::new(1000);
    let mut bright = BrightnessGrid::new(1000);
    for d in data {
        let s = d
            .replace("turn ", "")
            .replace(" through ", ",")
            .replace(' ', ",");
        let s: Vec<&str> = s.split(',').collect();
        let state = match s[0] {
            "on" => State::On,
            "off" => State::Off,
            "toggle" => State::Toggle,
            _ => panic!("No valid"),
        };
        let bstate = match s[0] {
            "on" => BState::On,
            "off" => BState::Off,
            "toggle" => BState::Toggle,
            _ => panic!("No valid"),
        };
        let range = Range {
            line_start: s[2].parse::<usize>().unwrap(),
            col_start: s[1].parse::<usize>().unwrap(),
            line_end: s[4].parse::<usize>().unwrap(),
            col_end: s[3].parse::<usize>().unwrap(),
        };
        let brange = BRange {
            line_start: s[2].parse::<usize>().unwrap(),
            col_start: s[1].parse::<usize>().unwrap(),
            line_end: s[4].parse::<usize>().unwrap(),
            col_end: s[3].parse::<usize>().unwrap(),
        };
        grid.change_light_state(range, &state);
        bright.change_light_state(brange, &bstate);
    }
    println!("Total sum: {:?}", grid.get_total_lights_turned_on());
    println!(
        "Total brightness: {:?}",
        bright.get_total_lights_turned_on()
    );
}
