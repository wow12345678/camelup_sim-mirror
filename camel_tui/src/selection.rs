use calc::EffectCardType;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SelectionType {
    #[default]
    Camel,
    EffectCard,
}

#[derive(Debug)]
pub struct SelectionState {
    color: usize,
    field: usize,
    item_type: SelectionType,
    effect: EffectCardType,
}

impl SelectionState {
    pub fn color(&self) -> usize {
        self.color
    }

    pub fn field(&self) -> usize {
        self.field
    }

    pub fn item_type(&self) -> SelectionType {
        self.item_type
    }

    pub fn effect(&self) -> EffectCardType {
        self.effect
    }

    pub fn color_mut(&mut self) -> &mut usize {
        &mut self.color
    }

    pub fn field_mut(&mut self) -> &mut usize {
        &mut self.field
    }

    pub fn item_type_mut(&mut self) -> &mut SelectionType {
        &mut self.item_type
    }

    pub fn effect_mut(&mut self) -> &mut EffectCardType {
        &mut self.effect
    }
}

impl Default for SelectionState {
    fn default() -> Self {
        Self {
            color: Default::default(),
            field: Default::default(),
            item_type: SelectionType::Camel,
            effect: EffectCardType::Oasis,
        }
    }
}
