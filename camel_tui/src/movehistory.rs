use calc::EffectCardType;

use crate::camelfield::CamelColor;

#[derive(Debug)]
pub enum MoveKind {
    PlaceEffectCard(EffectCardType, usize),
    MoveCamel {
        camel_color: CamelColor,
        from: usize,
        moved_camels_len: usize,
        put_under: bool,
    },
    PlaceCamel(CamelColor, usize),
    NewRound {
        old_start_pos: [u32; 5],
        old_effect_placements: [Vec<u8>; 2],
    },
}

#[derive(Debug, Default)]
pub struct MoveHistory {
    stack: Vec<MoveKind>,
}

impl MoveHistory {
    pub fn pop(&mut self) -> Option<MoveKind> {
        self.stack.pop()
    }

    pub fn push(&mut self, camel_move: MoveKind) {
        self.stack.push(camel_move);
    }
}
