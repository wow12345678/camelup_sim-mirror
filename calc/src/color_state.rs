use crate::color::Color;
use std::{convert::Into, fmt::Debug, hash::Hash};

/// for saving memory
/// Bit 0-4:
/// 0: Blue
/// 1: Green
/// 2: Orange
/// 3: White
/// 4: Yellow
/// 5-7: current color index (for iterator)
#[derive(Clone, Eq)]
pub struct ColorState {
    pub state: u8,
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

impl Default for ColorState {
    //all colors, no current index
    fn default() -> Self {
        Self { state: 0b1111_1000 }
    }
}

impl ColorState {
    pub fn len(&self) -> u8 {
        let colors = self.state & 0b1111_1000;
        // has to be in 0..6
        colors.count_ones() as u8
    }

    #[inline]
    pub fn assign_to_index(&mut self, index: u8, value: bool) {
        if value {
            self.state |= 0b1000_0000 >> index;
        } else {
            self.state &= !(0b1000_0000 >> index);
        }
    }

    pub fn remove_color(&mut self, col: Color) {
        self.assign_to_index(col.into(), false);
    }

    pub fn add_color(&mut self, col: Color) {
        self.assign_to_index(col.into(), true);
    }

    pub fn new<T: Into<Color>>(conf: Vec<T>) -> Self {
        let mut state = 0b0000_0000;
        for col in conf {
            state |= col.into().as_byte();
        }
        Self { state }
    }

    pub(crate) fn retain(&mut self, predicate: impl Fn(u8) -> bool) {
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
