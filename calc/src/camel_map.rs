use crate::color::Color;
use std::cmp::max;
use std::convert::Into;

/// first camel at a position is at the bottom of a stack
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CamelMap {
    pub pos_color_map: [Option<Vec<Color>>; 16],
    // colors are encoded by index like the enum
    pub color_pos_map: [u8; 5],
    pub effect_cards: [Option<EffectCard>; 16],
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
        CamelMapBuilder {
            pos_color_map: None,
            color_pos_map: None,
            effect_cards: [const { None }; 16],
        }
    }

    // moves camel to position along with all camels on top of it
    pub fn move_camel(&mut self, camel: Color, by: i8) {
        let old_field_pos = self.find_camel(camel);
        let mut new_pos = max(old_field_pos as i8 + by, 0) as u8;
        let card_effect = &self.effect_cards[new_pos as usize];

        match self.effect_cards[new_pos as usize] {
            Some(EffectCard::Oasis) => new_pos += 1,
            Some(EffectCard::Desert) => new_pos -= 1,
            None => (),
        }
        new_pos = max(new_pos, 0);

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
            self.color_pos_map[Into::<usize>::into(*col)] = new_pos;
        }

        //remove old camels
        if let Some(vec) = &self.pos_color_map[old_field_pos as usize]
            && vec.is_empty()
        {
            self.pos_color_map[old_field_pos as usize] = None;
        }

        // change moving behavior
        match card_effect {
            Some(EffectCard::Oasis) | None => {
                if let Some(vec_new_pos) = self.pos_color_map[new_pos as usize].as_mut() {
                    vec_new_pos.append(&mut moving_camels);
                } else {
                    self.pos_color_map[new_pos as usize] = Some(moving_camels);
                }
            }
            Some(EffectCard::Desert) => {
                if let Some(vec_new_pos) = self.pos_color_map[new_pos as usize].take() {
                    self.pos_color_map[new_pos as usize] =
                        Some([moving_camels, vec_new_pos].concat());
                } else {
                    self.pos_color_map[new_pos as usize] = Some(moving_camels);
                }
            }
        }
    }

    pub fn find_camel(&self, color: Color) -> u8 {
        self.color_pos_map[Into::<usize>::into(color)]
    }
}

pub struct CamelMapBuilder {
    pos_color_map: Option<[Option<Vec<Color>>; 16]>,
    color_pos_map: Option<[u8; 5]>,
    effect_cards: [Option<EffectCard>; 16],
}

impl CamelMapBuilder {
    pub fn with_effect_cards(mut self, effect_cards: Vec<(usize, EffectCard)>) -> CamelMapBuilder {
        for (effect_pos, effect_val) in effect_cards {
            self.effect_cards[effect_pos].replace(effect_val);
        }
        self
    }

    pub fn with_positions(mut self, init_positions: Vec<(u8, Color)>) -> CamelMapBuilder {
        self.pos_color_map
            .is_none()
            .then(|| self.pos_color_map.replace([const { None }; 16]));
        self.color_pos_map
            .is_none()
            .then(|| self.color_pos_map.replace([0; 5]));

        for pos in init_positions {
            self.insert_camel(pos);
        }
        self
    }

    //inserts camel at postion
    fn insert_camel(&mut self, (pos, color): (u8, Color)) {
        if let Some(pos_color_map) = &mut self.pos_color_map
            && let Some(color_pos_map) = &mut self.color_pos_map
        {
            if let Some(vec) = &mut pos_color_map[pos as usize] {
                vec.push(color);
            } else {
                pos_color_map[pos as usize] = Some(vec![color]);
            }
            color_pos_map[color as usize] = pos;
        }
    }

    pub fn build(mut self) -> Result<CamelMap, &'static str> {
        if self.pos_color_map.is_none() || self.color_pos_map.is_none() {
            return Err("camel_map has to have positions");
        }
        let res = CamelMap {
            pos_color_map: self.pos_color_map.take().unwrap(),
            color_pos_map: self.color_pos_map.take().unwrap(),
            effect_cards: self.effect_cards,
        };
        Ok(res)
    }
}
