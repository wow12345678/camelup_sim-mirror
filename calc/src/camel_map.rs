use crate::camel_stack::CamelStack;
use crate::color::Color;

/// first camel at a position is at the bottom of a stack
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CamelMap {
    pub pos_color_map: [CamelStack; 20],
    // colors are encoded by index like the enum
    pub color_pos_map: [u8; 5],
    pub effect_cards: [Option<EffectCard>; 20],
}

/// plates which have a effect when a camel lands on a field with them
/// Oasis => +1 field to top of camels on the next field
/// Desert => -1 field to the bottom of the camels on the previous field
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum EffectCard {
    Oasis,
    Desert,
}

impl EffectCard {
    pub fn to_color(&self) -> (u8, u8, u8) {
        match self {
            EffectCard::Oasis => (0, 100, 0),
            EffectCard::Desert => (100, 0, 0),
        }
    }
}

impl CamelMap {
    pub fn builder() -> CamelMapBuilder {
        CamelMapBuilder::default()
    }

    pub fn clear_effects(&mut self) {
        self.effect_cards = [const { None }; 20];
    }

    pub fn camel_has_won(&self) -> bool {
        self.color_pos_map.iter().any(|pos| *pos >= 15)
    }

    // moves camel to position along with all camels on top of it
    pub fn move_camel(&mut self, camel: Color, by: i8) {
        let max_pos = (self.pos_color_map.len() - 1) as i8;
        let old_field_pos = self.find_camel(camel);
        let mut new_pos = (old_field_pos as i8 + by).clamp(0, max_pos) as u8;
        let card_effect = self.effect_cards[new_pos as usize];

        match card_effect {
            Some(EffectCard::Oasis) => new_pos += 1,
            Some(EffectCard::Desert) => new_pos -= 1,
            None => (),
        }
        new_pos = (new_pos as i8).clamp(0, max_pos) as u8;

        let old_pos_in_stack = self.pos_color_map[old_field_pos as usize]
            .iter()
            .enumerate()
            .filter_map(|(pos, color)| if color == camel { Some(pos) } else { None })
            .next()
            .unwrap();

        let moving_camels = self.pos_color_map[old_field_pos as usize].split_off(old_pos_in_stack);

        // update positions
        for col in moving_camels.iter() {
            // Safety: all camels up until the stack end (moving_camels.size()) are Some
            self.color_pos_map[Into::<usize>::into(col)] = new_pos;
        }

        // adjust moving behavior based on effect card
        match card_effect {
            Some(EffectCard::Oasis) | None => {
                self.pos_color_map[new_pos as usize].append(moving_camels);
            }
            Some(EffectCard::Desert) => {
                self.pos_color_map[new_pos as usize].prepend(moving_camels);
            }
        }
    }

    pub fn camels_at(&self, pos: usize) -> Vec<Color> {
        self.pos_color_map[pos].iter().collect()
    }

    pub fn find_camel(&self, color: Color) -> u8 {
        self.color_pos_map[Into::<usize>::into(color)]
    }
}

#[derive(Default)]
pub struct CamelMapBuilder {
    pos_color_map: [CamelStack; 20],
    color_pos_map: [u8; 5],
    effect_cards: [Option<EffectCard>; 20],
}

impl CamelMapBuilder {
    pub fn with_effect_cards(mut self, effect_cards: Vec<(usize, EffectCard)>) -> CamelMapBuilder {
        for (effect_pos, effect_val) in effect_cards {
            self.effect_cards[effect_pos].replace(effect_val);
        }
        self
    }

    pub fn with_positions(mut self, init_positions: Vec<(u8, Color)>) -> CamelMapBuilder {
        for pos in init_positions {
            self.insert_camel(pos);
        }
        self
    }

    //inserts camel at postion
    fn insert_camel(&mut self, (pos, color): (u8, Color)) {
        self.pos_color_map[pos as usize].append([color]);
        self.color_pos_map[color as usize] = pos;
    }

    pub fn build(self) -> CamelMap {
        CamelMap {
            pos_color_map: self.pos_color_map,
            color_pos_map: self.color_pos_map,
            effect_cards: self.effect_cards,
        }
    }
}
