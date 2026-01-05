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
    pub available_colours: ColorState,
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
    /// Creates a new ConfigurationBuilder for building Configuration instances
    pub fn builder() -> ConfigurationBuilder {
        ConfigurationBuilder::new()
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
        for i in 0..self.map.pos_color_map.len() {
            if let Some(camels) = self.map.pos_color_map[i].take() {
                let new_idx = max(i as i8 - smallest_pos as i8, 0) as usize;
                for cam in &camels {
                    self.map.color_pos_map[Into::<usize>::into(*cam)] = new_idx as u8;
                }
                self.map.pos_color_map[i] = None;
                self.map.pos_color_map[new_idx] = Some(camels);
            }
        }
    }

    /// Creates a array as a leaderboard
    /// [1., 2., 3., 4., 5.]
    pub(crate) fn leaderboard(&self) -> [Color; 5] {
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

/// Builder pattern for creating Configuration instances
pub struct ConfigurationBuilder {
    map: Option<CamelMap>,
    #[cfg(debug_assertions)]
    dice_queue: Option<Vec<Dice>>,
    available_colours: Option<ColorState>,
}

impl ConfigurationBuilder {
    /// Creates a new ConfigurationBuilder with default values
    pub fn new() -> Self {
        Self {
            map: None,
            #[cfg(debug_assertions)]
            dice_queue: None,
            available_colours: None,
        }
    }

    /// Sets the camel map from a vector of (position, color) pairs
    pub fn with_map(mut self, positions: Vec<(u8, Color)>) -> Self {
        self.map = Some(CamelMap::new(positions));
        self
    }

    /// Sets the camel map directly
    pub fn with_camel_map(mut self, map: CamelMap) -> Self {
        self.map = Some(map);
        self
    }

    /// Sets the available colors that can still be rolled
    pub fn with_available_colors(mut self, colors: Vec<Color>) -> Self {
        self.available_colours = Some(ColorState::new(colors));
        self
    }

    /// Sets the available colors using a ColorState directly
    pub fn with_color_state(mut self, color_state: ColorState) -> Self {
        self.available_colours = Some(color_state);
        self
    }

    /// Sets the dice queue from a vector of (Color, value) pairs
    /// Note: This field only exists in debug builds
    #[cfg(debug_assertions)]
    pub fn with_dice_queue(mut self, dice_data: Vec<(Color, u8)>) -> Self {
        let dice_vec: Vec<Dice> = dice_data
            .into_iter()
            .map(|(color, value)| Dice { color, value })
            .collect();
        self.dice_queue = Some(dice_vec);
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
                CamelMap::new(vec![
                    (0, Color::Blue),
                    (0, Color::Green),
                    (1, Color::White),
                    (1, Color::Yellow),
                    (2, Color::Orange),
                ])
            }),
            #[cfg(debug_assertions)]
            dice_queue: self.dice_queue.unwrap_or_default(),
            available_colours: self.available_colours.unwrap_or_default(),
        }
    }
}

impl Default for ConfigurationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
