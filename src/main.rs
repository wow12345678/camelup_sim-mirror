use std::collections::HashMap;
use std::f64;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    Blue,
    Green,
    Orange,
    White,
    Yellow,
    None,
}

impl Color {
    fn as_byte(&self) -> u8 {
        let mask = 0b0000_0001;
        let index: u8 = (*self).into();
        mask << (7 - index)
    }

    fn from_byte(color_code: u8) -> Self {
        match color_code {
            0b1000_0000 => Color::Blue,
            0b0100_0000 => Color::Green,
            0b0010_0000 => Color::Orange,
            0b0001_0000 => Color::White,
            0b0000_1000 => Color::Yellow,
            _ => Color::None,
        }
    }
}

impl From<Color> for u8 {
    fn from(value: Color) -> Self {
        match value {
            Color::Blue => 0,
            Color::Green => 1,
            Color::Orange => 2,
            Color::White => 3,
            Color::Yellow => 4,
            Color::None => 5,
        }
    }
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Color::Blue,
            1 => Color::Green,
            2 => Color::Orange,
            3 => Color::White,
            4 => Color::Yellow,
            _ => Color::None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Dice {
    color: Color,
    value: u8,
}

/// for saving memory
/// Bit 0-4:
/// 0: Blue
/// 1: Green
/// 2: Orange
/// 3: White
/// 4: Yellow
/// 5-7: current color index (for iterator)
#[derive(Clone, PartialEq, Eq)]
struct ColorState {
    state: u8,
}

impl ColorState {
    fn len(&self) -> u8 {
        let colors = self.state & 0b1111_1000;
        // has to be in 0..6
        colors.count_ones() as u8
    }

    #[inline]
    fn assign_to_index(&mut self, index: u8, value: bool) {
        if value {
            self.state |= 0b1000_0000 >> index;
        } else {
            self.state &= !(0b1000_0000 >> index);
        }
    }

    //all colors, no current index
    fn default() -> Self {
        Self { state: 0b1111_1000 }
    }

    fn new<T: Into<Color>>(conf: Vec<T>) -> Self {
        let mut state = 0b0000_0000;
        for col in conf {
            state |= col.into().as_byte();
        }
        Self { state }
    }

    fn retain(&mut self, predicate: impl Fn(u8) -> bool) {
        for i in 0..6 {
            let elem = 0b1000_0000 >> i;
            // Only keep the bit if it's currently set AND the predicate returns true
            let is_currently_set = (self.state & elem) != 0;
            if is_currently_set && !predicate(elem) {
                self.assign_to_index(i, false);
            }
        }
    }
}

impl Debug for ColorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ColorState")
            .field("state", &self.state)
            .field("state binary", &format!("{:08b}", self.state))
            .finish()
    }
}

impl IntoIterator for &ColorState {
    type Item = u8;
    type IntoIter = ColorState;

    fn into_iter(self) -> Self::IntoIter {
        ColorState { state: self.state }
    }
}

impl Iterator for ColorState {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_index: u8 = self.state & 0b0000_0111;

        while current_index < 5 {
            // Check if bit at current_index is set
            if (self.state & (0b1000_0000 >> current_index)) != 0 {
                // remove color from available ones (consume iterator value)
                self.state &= !(0b1000_0000 >> current_index);

                // update index (erase last 3 bits, change to current index)
                self.state &= 0b1111_1000;
                self.state |= current_index + 1;

                // Return the bit value
                let return_val = 0b1000_0000 >> current_index;
                return Some(return_val);
            }
            current_index += 1;
        }
        None
    }
}

#[derive(Debug, Clone)]
struct Configuration {
    pos_color_map: HashMap<u8, Vec<Color>>,
    dice_queue: Vec<Dice>,
    available_colours: ColorState,
}

impl Configuration {
    fn leaderboard(&self) -> [Color; 5] {
        let mut positions: Vec<(u8, &Vec<Color>)> =
            self.pos_color_map.iter().map(|(k, v)| (*k, v)).collect();
        positions.sort_by(|a, b| b.0.cmp(&a.0));
        let mut leaderboard: [Color; 5] = [Color::None; 5];
        //Safety: i is always <= 5
        let mut i = 0;
        for pos in positions {
            for color in pos.1.iter().rev() {
                leaderboard[i] = *color;
                i += 1;
            }
        }
        leaderboard
    }
}

fn main() {
    const COUNT_ALL: u32 = 5 * 4 * 3 * 2 * 3_u32.pow(5);
    let init_pos_color_map: HashMap<u8, Vec<Color>> = HashMap::from([
        (0, vec![Color::Blue, Color::Green]),
        (1, vec![Color::Yellow, Color::White]),
        (2, vec![Color::Orange]),
    ]);

    let init_conf = Configuration {
        pos_color_map: init_pos_color_map,
        dice_queue: Vec::new(),
        available_colours: ColorState::default(),
    };

    let configs = simulate_round(init_conf);

    // let mut file = OpenOptions::new().append(true).open("test.txt").unwrap();
    // for (i, conf) in configs.iter().enumerate() {
    //     let _ = writeln!(file, "Config {i}:\n{conf:?}");
    // }

    let placements = aggragate_placements(&configs);

    println!("{:?}", placements);
    let prob_blue = placements[0][0] as f64 / COUNT_ALL as f64;
    let prob_green = placements[1][0] as f64 / COUNT_ALL as f64;
    let prob_orange = placements[2][0] as f64 / COUNT_ALL as f64;
    let prob_white = placements[3][0] as f64 / COUNT_ALL as f64;
    let prob_yellow = placements[4][0] as f64 / COUNT_ALL as f64;

    println!("Blue: {prob_blue}");
    println!("Green: {prob_green}");
    println!("Orange: {prob_orange}");
    println!("White: {prob_white}");
    println!("Yellow: {prob_yellow}");

    // TODO: this crashes my pc :(
    // let mut new_configs: Vec<Configuration> = Vec::new();
    //
    // for mut conf in configs {
    //     conf.available_colours = vec![
    //         Color::Blue,
    //         Color::Green,
    //         Color::Orange,
    //         Color::White,
    //         Color::Yellow,
    //     ];
    //     new_configs.append(&mut simulate_round(conf));
    // }
    //
    // let new_placements = aggragate_placements(&new_configs);
    //
    // println!("{:?}", new_placements);
    // let new_prob_blue = new_placements[0][0] as f64 / COUNT_ALL as f64;
    // let new_prob_green = new_placements[1][0] as f64 / COUNT_ALL as f64;
    // let new_prob_orange = new_placements[2][0] as f64 / COUNT_ALL as f64;
    // let new_prob_white = new_placements[3][0] as f64 / COUNT_ALL as f64;
    // let new_prob_yellow = new_placements[4][0] as f64 / COUNT_ALL as f64;
    //
    // println!("Blue: {new_prob_blue}");
    // println!("Green: {new_prob_green}");
    // println!("Orange: {new_prob_orange}");
    // println!("White: {new_prob_white}");
    // println!("Yellow: {new_prob_yellow}");
}

// [Color] -> [Placements]
fn aggragate_placements(configs: &Vec<Configuration>) -> [[u32; 5]; 5] {
    let mut placements: [[u32; 5]; 5] = [[0; 5]; 5];

    for conf in configs {
        for (i, col) in conf.leaderboard().iter().enumerate() {
            let index: u8 = (*col).into();
            placements[index as usize][i] += 1;
        }
    }

    placements
}

//simulate round (remaining dice throws)
fn simulate_round(init_config: Configuration) -> Vec<Configuration> {
    let amount_throws = init_config.available_colours.len();
    let mut configs: Vec<Configuration> = vec![init_config];

    for _ in 0..amount_throws {
        let mut new_confs: Vec<Configuration> = Vec::new();
        for conf in &configs {
            for color_code in &conf.available_colours {
                new_confs.append(&mut simulate_dice_throw(conf, color_code));
            }
        }

        configs = new_confs;
    }

    configs
}

///returns all possible 3 dice values as new Configurations
fn simulate_dice_throw(conf: &Configuration, color_code: u8) -> Vec<Configuration> {
    let mut confs: Vec<Configuration> = Vec::new();
    let dice_color: Color = Color::from_byte(color_code);
    assert_ne!(dice_color, Color::None);

    for n in 1..=3 {
        let mut new_conf = conf.clone();
        new_conf.dice_queue.push(Dice {
            color: dice_color,
            value: n,
        });

        new_conf
            .available_colours
            .retain(|b| b != dice_color.as_byte());

        let old_pos = *new_conf
            .pos_color_map
            .iter()
            .filter_map(|(pos, colors)| {
                if colors.contains(&dice_color) {
                    Some(pos)
                } else {
                    None
                }
            })
            .next()
            .unwrap();

        let old_pos_camels = new_conf.pos_color_map.get_mut(&old_pos).unwrap();

        let mut moving_camels: Vec<Color> = Vec::new();
        while let Some(last) = old_pos_camels.pop() {
            if last != dice_color {
                moving_camels.push(last);
            } else {
                moving_camels.push(last);
                break;
            }
        }
        if old_pos_camels.is_empty() {
            new_conf.pos_color_map.remove(&old_pos);
        }

        moving_camels.reverse();
        let new_pos = old_pos + n;

        if let Some(camels_new_pos) = new_conf.pos_color_map.get_mut(&new_pos) {
            camels_new_pos.append(&mut moving_camels);
        } else {
            new_conf.pos_color_map.insert(new_pos, moving_camels);
        }

        confs.push(new_conf);
    }
    confs
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{Color, ColorState, Configuration};

    fn simple_config() -> Configuration {
        let init_pos_color_map: HashMap<u8, Vec<Color>> = HashMap::from([
            (0, vec![Color::Blue, Color::Green]),
            (1, vec![Color::Yellow, Color::White]),
            (2, vec![Color::Orange]),
        ]);

        Configuration {
            pos_color_map: init_pos_color_map,
            dice_queue: Vec::new(),
            available_colours: ColorState::new(vec![
                Color::Blue,
                Color::Green,
                Color::Orange,
                Color::White,
                Color::Yellow,
            ]),
        }
    }

    #[test]
    fn color_state_retain() {
        let mut test_state = simple_config();
        test_state.available_colours.retain(|c| {
            let val = c == Color::Blue.as_byte();
            println!("{c}, {}, {val}", Color::Blue.as_byte());
            val
        });
        let first_val = test_state.available_colours.next();
        assert!(first_val.is_some());
        assert_eq!(test_state.available_colours.next(), None);
    }

    #[test]
    fn color_state_assign() {
        let mut test_state = ColorState::default();
        test_state.assign_to_index(0, false);
        test_state.assign_to_index(2, false);
        test_state.assign_to_index(3, false);
        test_state.assign_to_index(4, false);
        assert_eq!(test_state.len(), 1);
        assert_eq!(test_state.state, 0b0100_0000);
        test_state.assign_to_index(3, true);
        test_state.assign_to_index(4, true);
        assert_eq!(test_state.len(), 3);
        assert_eq!(test_state.state, 0b0101_1000);
    }

    #[test]
    fn color_state_iter() {
        let mut test_state = ColorState::default();
        assert_eq!(test_state.next(), Some(Color::Blue.as_byte()));
        assert_eq!(test_state.next(), Some(Color::Green.as_byte()));
        assert_eq!(test_state.next(), Some(Color::Orange.as_byte()));
        assert_eq!(test_state.next(), Some(Color::White.as_byte()));
        assert_eq!(test_state.next(), Some(Color::Yellow.as_byte()));
        assert_eq!(test_state.next(), None);
    }
}
