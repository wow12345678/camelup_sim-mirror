use std::collections::HashMap;
use std::convert::Into;
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

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
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
#[derive(Clone, Eq)]
struct ColorState {
    state: u8,
}

impl Hash for ColorState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let colors_wo_index = self.state & 0b1111_1000;
        colors_wo_index.hash(state);
    }
}

impl PartialEq for ColorState {
    fn eq(&self, other: &Self) -> bool {
        let self_state_wo_index = self.state & 0b1111_1000;
        let other_state_wo_index = other.state & 0b1111_1000;
        self_state_wo_index == other_state_wo_index
    }
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

    fn remove_color(&mut self, col: Color) {
        self.assign_to_index(col.into(), false);
    }

    fn add_color(&mut self, col: Color) {
        self.assign_to_index(col.into(), true);
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct CamelMap {
    pos_color_map: [Option<Vec<Color>>; 16],
    // colors are encoded by index like the enum
    color_pos_map: [u8; 5],
}

impl CamelMap {
    fn new(init_positions: Vec<(u8, Color)>) -> Self {
        let mut res = Self {
            pos_color_map: [const { None }; 16],
            color_pos_map: [0; 5],
        };

        for pos in init_positions {
            res.insert_camel(pos);
        }
        res
    }

    //inserts camel at postion
    fn insert_camel(&mut self, (pos, color): (u8, Color)) {
        if let Some(vec) = &mut self.pos_color_map[pos as usize] {
            vec.push(color);
        } else {
            self.pos_color_map[pos as usize] = Some(vec![color]);
        }
        self.color_pos_map[color as usize] = pos;
    }

    // moves camel to position along with all camels on top of it
    fn move_camel(&mut self, camel: Color, by: u8) {
        let old_field_pos = self.find_camel(camel);
        let new_pos = old_field_pos + by;

        let old_pos_in_stack = &mut self.pos_color_map[old_field_pos as usize]
            .iter()
            .find_map(|v| v.iter().position(|c| *c == camel))
            .unwrap();

        let mut moving_camels = self.pos_color_map[old_field_pos as usize]
            .as_mut()
            .unwrap()
            .split_off(*old_pos_in_stack);

        // update positions
        for col in &moving_camels {
            self.color_pos_map[Into::<u8>::into(*col) as usize] = new_pos;
        }

        if let Some(vec) = &self.pos_color_map[old_field_pos as usize]
            && vec.is_empty()
        {
            self.pos_color_map[old_field_pos as usize] = None;
        }

        if let Some(vec_new_pos) = self.pos_color_map[new_pos as usize].as_mut() {
            vec_new_pos.append(&mut moving_camels);
        } else {
            self.pos_color_map[new_pos as usize] = Some(moving_camels);
        }
    }

    fn find_camel(&self, color: Color) -> u8 {
        self.color_pos_map[Into::<u8>::into(color) as usize]
    }
}

// only use dice_queue in debug mode because not needed but nice for debugging
#[derive(Debug, Clone, Eq)]
struct Configuration {
    map: CamelMap,
    #[cfg(debug_assertions)]
    dice_queue: Vec<Dice>,
    available_colours: ColorState,
}

impl Hash for Configuration {
    // dice_queue is not important
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.map.hash(state);
        self.available_colours.hash(state);
    }
}

impl PartialEq for Configuration {
    fn eq(&self, other: &Self) -> bool {
        // dice_queue is excluded as it's only for debugging
        self.map == other.map && self.available_colours == other.available_colours
    }
}

impl Configuration {
    fn leaderboard(&self) -> [Color; 5] {
        let mut positions: Vec<(usize, &Vec<Color>)> = Vec::new();
        for (i, pos) in self.map.pos_color_map.iter().enumerate() {
            if let Some(val) = pos {
                positions.push((i, val));
            }
        }
        positions.sort_by(|a, b| b.0.cmp(&a.0));
        let mut leaderboard: [Color; 5] = [Color::None; 5];

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

fn aggragate_placements(placements_vec: &Vec<Placements>) -> [[u32; 5]; 5] {
    let mut placements: [[u32; 5]; 5] = [[0; 5]; 5];

    for placement in placements_vec {
        for (i, &color_index) in placement.numbers.iter().enumerate() {
            placements[color_index as usize][i] += 1;
        }
    }

    placements
}

#[derive(Debug, Default)]
struct CacheStatistics {
    cache_hits: u32,
    cache_misses: u32,
    total_function_calls: u32,
}

impl CacheStatistics {
    fn new() -> Self {
        Self {
            cache_hits: 0,
            cache_misses: 0,
            total_function_calls: 0,
        }
    }

    fn record_hit(&mut self) {
        self.cache_hits += 1;
        self.total_function_calls += 1;
    }

    fn record_miss(&mut self) {
        self.cache_misses += 1;
        self.total_function_calls += 1;
    }

    fn print_stats(&self) {
        println!("=== Cache Statistics ===");
        println!("Total function calls: {}", self.total_function_calls);
        println!("Cache hits: {}", self.cache_hits);
        println!("Cache misses: {}", self.cache_misses);
    }
}

#[derive(Debug, Hash, Clone)]
struct Placements {
    numbers: [u8; 5],
}

fn simulate_rounds(init_config: Configuration) -> (Vec<Placements>, CacheStatistics) {
    let mut cache: HashMap<Configuration, Vec<Placements>> = HashMap::new();
    let mut stats = CacheStatistics::new();
    let placements = simulate_round_rec(
        init_config,
        &mut cache,
        #[cfg(debug_assertions)]
        &mut stats,
    );
    (placements, stats)
}

fn simulate_round_rec(
    conf: Configuration,
    cache: &mut HashMap<Configuration, Vec<Placements>>,
    #[cfg(debug_assertions)] stats: &mut CacheStatistics,
) -> Vec<Placements> {
    // Base case
    if conf.available_colours.len() == 0 {
        #[cfg(debug_assertions)]
        stats.record_miss();
        return vec![Placements {
            numbers: conf.leaderboard().map(|color| color.into()),
        }];
    }

    // check cache
    if let Some(cached_result) = cache.get(&conf) {
        #[cfg(debug_assertions)]
        stats.record_hit();
        return cached_result.clone();
    }

    #[cfg(debug_assertions)]
    stats.record_miss();

    let mut all_placements = Vec::new();

    // For each available color, simulate all possible dice outcomes (1, 2, 3)
    for color_code in &conf.available_colours {
        let dice_color = Color::from_byte(color_code);
        assert_ne!(dice_color, Color::None);

        for dice_value in 1..=3 {
            let mut new_conf = conf.clone();

            #[cfg(debug_assertions)]
            {
                new_conf.dice_queue.push(Dice {
                    color: dice_color,
                    value: dice_value,
                });
            }

            new_conf.available_colours.remove_color(dice_color);
            new_conf.map.move_camel(dice_color, dice_value);

            // recursive call
            let mut sub_placements = simulate_round_rec(
                new_conf,
                cache,
                #[cfg(debug_assertions)]
                stats,
            );

            all_placements.append(&mut sub_placements);
        }
    }

    cache.insert(conf, all_placements.clone());

    all_placements
}

fn main() {
    const COUNT_ALL: u32 = 5 * 4 * 3 * 2 * 3_u32.pow(5);

    let init_conf = Configuration {
        map: CamelMap::new(vec![
            (0, Color::Blue),
            (0, Color::Green),
            (1, Color::White),
            (1, Color::Yellow),
            (2, Color::Orange),
        ]),
        #[cfg(debug_assertions)]
        dice_queue: Vec::new(),
        available_colours: ColorState::default(),
    };

    let (new_placements_vec, cache_stats) = simulate_rounds(init_conf);

    cache_stats.print_stats();

    let new_placements = aggragate_placements(&new_placements_vec);
    println!("{:?}", new_placements);

    let new_prob_blue = new_placements[0][0] as f64 / COUNT_ALL as f64;
    let new_prob_green = new_placements[1][0] as f64 / COUNT_ALL as f64;
    let new_prob_orange = new_placements[2][0] as f64 / COUNT_ALL as f64;
    let new_prob_white = new_placements[3][0] as f64 / COUNT_ALL as f64;
    let new_prob_yellow = new_placements[4][0] as f64 / COUNT_ALL as f64;

    println!("Blue: {new_prob_blue}");
    println!("Green: {new_prob_green}");
    println!("Orange: {new_prob_orange}");
    println!("White: {new_prob_white}");
    println!("Yellow: {new_prob_yellow}");
}

#[cfg(test)]
mod test {
    use crate::{CamelMap, Color, ColorState, Configuration, simulate_rounds};

    fn simple_test_config() -> Configuration {
        Configuration {
            map: CamelMap::new(vec![
                (0, Color::Blue),
                (0, Color::Green),
                (1, Color::Yellow),
                (1, Color::White),
                (2, Color::Orange),
            ]),
            #[cfg(debug_assertions)]
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
    fn test_simulate_round_new() {
        let mut small_config = simple_test_config();
        small_config.available_colours = ColorState::new(vec![Color::Blue]);

        let (placements, chachestats) = simulate_rounds(small_config);
        assert_eq!(placements.len(), 3);

        for placement in placements {
            for &pos in &placement.numbers {
                assert!(pos <= 4, "Position should be 0-4, got {}", pos);
            }
        }
    }

    #[test]
    fn test_move_camel() {
        let mut map = CamelMap::new(vec![
            (0, Color::Blue),
            (0, Color::Green),
            (1, Color::Orange),
        ]);

        map.move_camel(Color::Green, 3);
        assert_eq!(map.find_camel(Color::Green), 3);
        assert_eq!(map.find_camel(Color::Blue), 0);
        assert_eq!(map.find_camel(Color::Orange), 1);
        assert_eq!(map.pos_color_map[3], Some(vec![Color::Green]));
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
