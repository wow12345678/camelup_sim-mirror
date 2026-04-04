use crate::camel_map::CamelMap;
use crate::color::Color;
use crate::color_state::ColorState;
use std::cmp::max;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Dice {
    pub color: Color,
    pub value: u8,
}

// only use dice_queue in debug mode because not needed but nice for debugging
#[derive(Debug, Clone, Eq)]
pub struct Configuration {
    pub map: CamelMap,
    #[cfg(debug_assertions)]
    pub dice_queue: Vec<Dice>,
    pub available_colors: ColorState,
    pub done: bool,
}

impl Hash for Configuration {
    // dice_queue is not important
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.map.hash(state);
        self.available_colors.hash(state);
        self.done.hash(state);
    }
}

impl PartialEq for Configuration {
    fn eq(&self, other: &Self) -> bool {
        // dice_queue is excluded as it's only for debugging
        self.map == other.map
            && self.available_colors == other.available_colors
            && self.done == other.done
    }
}

impl Configuration {
    /// Creates a new ConfigurationBuilder for building Configuration instances
    pub fn builder() -> ConfigurationBuilder {
        ConfigurationBuilder::new()
    }

    pub fn clear_moveable_camels(&mut self) {
        self.available_colors.clear();
    }

    /// normalize the configuration and just keep the relative distances of the camels
    pub fn normalize(&mut self) {
        let mut positions: Vec<_> = Color::all()
            .iter()
            .map(|col| self.map.find_camel(*col))
            .collect::<Vec<_>>();
        positions.sort();
        let smallest_pos = *positions
            .first()
            .expect("There should always be at least 1 camel with a smallest position");
        if smallest_pos == 0 {
            return;
        }
        let shift = smallest_pos as usize;
        for i in 0..self.map.pos_color_map.len() {
            let camels = self.map.pos_color_map[i];
            let effect = self.map.effect_cards[i];
            let new_idx = max(i as i8 - shift as i8, 0) as usize;
            for cam in camels.iter() {
                self.map.color_pos_map[Into::<usize>::into(cam)] = new_idx as u8;
            }
            self.map.pos_color_map[i].clear();
            self.map.effect_cards[i] = None;

            self.map.pos_color_map[new_idx].replace(camels);
            self.map.effect_cards[new_idx] = effect;
        }
    }

    pub fn new_round(&mut self) {
        self.available_colors = ColorState::default();
        #[cfg(debug_assertions)]
        self.dice_queue.clear();
        self.map.clear_effects();
    }

    /// Creates a array as a leaderboard
    /// [1., 2., 3., 4., 5.]
    pub(crate) fn leaderboard(&self) -> [Color; 5] {
        let mut leaderboard: [Color; 5] = [Color::Blue; 5];
        let mut i = 0;

        for pos in self.map.pos_color_map.iter().rev() {
            for color in pos.iter().rev() {
                leaderboard[i] = color;
                i += 1;
            }
        }
        leaderboard
    }
}

/// Builder pattern for creating Configuration instances
pub struct ConfigurationBuilder {
    map: Option<CamelMap>,
    #[cfg(debug_assertions)]
    dice_queue: Option<Vec<Dice>>,
    available_colors: Option<ColorState>,
}

impl ConfigurationBuilder {
    /// Creates a new ConfigurationBuilder with default values
    pub fn new() -> Self {
        Self {
            map: None,
            #[cfg(debug_assertions)]
            dice_queue: None,
            available_colors: None,
        }
    }

    /// Sets the camel map from a vector of (position, color) pairs
    pub fn with_map(mut self, positions: Vec<(u8, Color)>) -> Self {
        self.map = Some(CamelMap::builder().with_positions(positions).build());
        self
    }

    /// Sets the camel map directly
    pub fn with_camel_map(mut self, map: CamelMap) -> Self {
        self.map = Some(map);
        self
    }

    /// Sets the available colors that can still be rolled
    pub fn with_available_colors(mut self, colors: Vec<Color>) -> Self {
        self.available_colors = Some(ColorState::new(colors));
        self
    }

    /// Sets the available colors using a ColorState directly
    pub fn with_color_state(mut self, color_state: ColorState) -> Self {
        self.available_colors = Some(color_state);
        self
    }

    /// Sets the dice queue from a vector of (Color, value) pairs
    /// Note: This field only exists in debug builds
    #[allow(unused)]
    pub fn with_dice_queue(mut self, dice_data: Vec<(Color, u8)>) -> Self {
        #[cfg(debug_assertions)]
        {
            let dice_vec: Vec<Dice> = dice_data
                .into_iter()
                .map(|(color, value)| Dice { color, value })
                .collect();
            self.dice_queue = Some(dice_vec);
        }
        self
    }

    /// Adds individual dice to the queue
    /// Note: This field only exists in debug builds
    #[cfg(debug_assertions)]
    pub fn add_dice(mut self, color: Color, value: u8) -> Self {
        let dice = Dice { color, value };
        match &mut self.dice_queue {
            Some(queue) => queue.push(dice),
            None => self.dice_queue = Some(vec![dice]),
        }
        self
    }

    /// Builds the Configuration, providing defaults for unspecified fields
    pub fn build(self) -> Configuration {
        Configuration {
            map: self.map.unwrap_or_else(|| {
                // Default starting positions as used in tests and main
                CamelMap::builder()
                    .with_positions(vec![
                        (0, Color::Blue),
                        (0, Color::Green),
                        (1, Color::White),
                        (1, Color::Yellow),
                        (2, Color::Orange),
                    ])
                    .build()
            }),
            #[cfg(debug_assertions)]
            dice_queue: self.dice_queue.unwrap_or_default(),
            available_colors: self.available_colors.unwrap_or_default(),
            done: false,
        }
    }
}

impl Default for ConfigurationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
