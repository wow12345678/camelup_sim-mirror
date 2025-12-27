use crate::{
    camel_map::CamelMap,
    color::Color,
    color_state::ColorState,
    configuration::{Configuration, Dice},
};
use std::{collections::HashMap, convert::Into};

pub fn aggragate_placements(placements_vec: &Vec<Placement>) -> [[u32; 5]; 5] {
    let mut placements: [[u32; 5]; 5] = [[0; 5]; 5];

    for placement in placements_vec {
        for (i, &color_index) in placement.iter().enumerate() {
            placements[color_index as usize][i] += 1;
        }
    }

    placements
}

pub const ALL_GAME_STATES_COUNT: u32 = 5 * 4 * 3 * 2 * 3_u32.pow(5);

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

    pub(crate) fn print_stats(&self) {
        println!("=== Cache Statistics ===");
        println!("Total function calls: {}", self.total_function_calls);
        println!("Cache hits: {}", self.cache_hits);
        println!("Cache misses: {}", self.cache_misses);
    }
}

pub type Placement = [u8; 5];

pub struct SimulationResult {
    placements: Vec<Placement>,
    #[cfg(debug_assertions)]
    stats: CacheStatistics,
}

impl SimulationResult {
    pub fn print_stats(&self) {
        if cfg!(debug_assertions) {
            #[cfg(debug_assertions)]
            self.stats.print_stats();
        }
    }

    pub fn placements(&self) -> &Vec<Placement> {
        &self.placements
    }
}

pub fn simulate_rounds(init_config: Configuration) -> SimulationResult {
    let mut cache: HashMap<Configuration, Vec<Placement>> = HashMap::new();
    #[cfg(debug_assertions)]
    let mut stats = CacheStatistics::new();
    let placements = simulate_round_rec(
        init_config,
        &mut cache,
        #[cfg(debug_assertions)]
        &mut stats,
    );
    SimulationResult {
        placements,
        #[cfg(debug_assertions)]
        stats,
    }
}

fn simulate_round_rec(
    conf: Configuration,
    cache: &mut HashMap<Configuration, Vec<Placement>>,
    #[cfg(debug_assertions)] stats: &mut CacheStatistics,
) -> Vec<Placement> {
    // Base case
    if conf.available_colours.len() == 0 {
        #[cfg(debug_assertions)]
        stats.record_miss();
        return vec![conf.leaderboard().map(|color| color.into())];
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

pub fn main() {
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

    let res = simulate_rounds(init_conf);

    res.print_stats();

    let new_placements = aggragate_placements(res.placements());
    println!("{:?}", new_placements);

    let new_prob_blue = new_placements[0][0] as f64 / ALL_GAME_STATES_COUNT as f64;
    let new_prob_green = new_placements[1][0] as f64 / ALL_GAME_STATES_COUNT as f64;
    let new_prob_orange = new_placements[2][0] as f64 / ALL_GAME_STATES_COUNT as f64;
    let new_prob_white = new_placements[3][0] as f64 / ALL_GAME_STATES_COUNT as f64;
    let new_prob_yellow = new_placements[4][0] as f64 / ALL_GAME_STATES_COUNT as f64;

    println!("Blue: {new_prob_blue}");
    println!("Green: {new_prob_green}");
    println!("Orange: {new_prob_orange}");
    println!("White: {new_prob_white}");
    println!("Yellow: {new_prob_yellow}");
}
