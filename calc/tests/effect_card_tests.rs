use calc::{CamelMap, Color, EffectCard};


#[test]
fn test_builder_with_single_oasis() {
    let map = CamelMap::builder()
        .with_positions(vec![(0, Color::Blue)])
        .with_effect_cards(vec![(3, EffectCard::Oasis)])
        .build()
        .unwrap();

    assert_eq!(map.effect_cards[3], Some(EffectCard::Oasis));
    // Other positions should be None
    assert_eq!(map.effect_cards[0], None);
    assert_eq!(map.effect_cards[2], None);
    assert_eq!(map.effect_cards[4], None);
}

#[test]
fn test_builder_with_single_desert() {
    let map = CamelMap::builder()
        .with_positions(vec![(0, Color::Blue)])
        .with_effect_cards(vec![(5, EffectCard::Desert)])
        .build()
        .unwrap();

    assert_eq!(map.effect_cards[5], Some(EffectCard::Desert));
    // Other positions should be None
    assert_eq!(map.effect_cards[0], None);
    assert_eq!(map.effect_cards[4], None);
    assert_eq!(map.effect_cards[6], None);
}

#[test]
fn test_builder_with_multiple_effect_cards() {
    let map = CamelMap::builder()
        .with_positions(vec![(0, Color::Blue)])
        .with_effect_cards(vec![
            (3, EffectCard::Oasis),
            (7, EffectCard::Desert),
            (10, EffectCard::Oasis),
        ])
        .build()
        .unwrap();

    assert_eq!(map.effect_cards[3], Some(EffectCard::Oasis));
    assert_eq!(map.effect_cards[7], Some(EffectCard::Desert));
    assert_eq!(map.effect_cards[10], Some(EffectCard::Oasis));
    assert_eq!(map.effect_cards[0], None);
    assert_eq!(map.effect_cards[5], None);
}


#[test]
fn test_oasis_moves_camel_forward_one() {
    // Camel at position 0, Oasis at position 3
    // Moving by 3 should land on Oasis, then move +1 to position 4
    let mut map = CamelMap::builder()
        .with_positions(vec![(0, Color::Blue)])
        .with_effect_cards(vec![(3, EffectCard::Oasis)])
        .build()
        .unwrap();

    map.move_camel(Color::Blue, 3);

    assert_eq!(map.find_camel(Color::Blue), 4);
    assert_eq!(map.pos_color_map[4], Some(vec![Color::Blue]));
    assert_eq!(map.pos_color_map[3], None);
}

#[test]
fn test_oasis_places_camel_on_top_of_stack() {
    // Camel Blue at position 0, Camel Green already at position 4
    // Oasis at position 3, Blue moves by 3 -> lands on Oasis -> moves to 4
    // Blue should be placed ON TOP of Green
    let mut map = CamelMap::builder()
        .with_positions(vec![(0, Color::Blue), (4, Color::Green)])
        .with_effect_cards(vec![(3, EffectCard::Oasis)])
        .build()
        .unwrap();

    map.move_camel(Color::Blue, 3);

    assert_eq!(map.find_camel(Color::Blue), 4);
    assert_eq!(map.find_camel(Color::Green), 4);
    // Green at bottom, Blue on top
    assert_eq!(map.pos_color_map[4], Some(vec![Color::Green, Color::Blue]));
}

#[test]
fn test_oasis_with_camel_stack_moves_all() {
    // Green on top of Blue at position 0, Oasis at position 3
    // Moving Blue by 3 should move both Blue and Green to position 4
    let mut map = CamelMap::builder()
        .with_positions(vec![(0, Color::Blue), (0, Color::Green)])
        .with_effect_cards(vec![(3, EffectCard::Oasis)])
        .build()
        .unwrap();

    map.move_camel(Color::Blue, 3);

    assert_eq!(map.find_camel(Color::Blue), 4);
    assert_eq!(map.find_camel(Color::Green), 4);
    // Blue at bottom, Green on top (preserves stack order)
    assert_eq!(map.pos_color_map[4], Some(vec![Color::Blue, Color::Green]));
    assert_eq!(map.pos_color_map[0], None);
}

#[test]
fn test_desert_moves_camel_back_one() {
    // Camel at position 0, Desert at position 3
    // Moving by 3 should land on Desert, then move -1 to position 2
    let mut map = CamelMap::builder()
        .with_positions(vec![(0, Color::Blue)])
        .with_effect_cards(vec![(3, EffectCard::Desert)])
        .build()
        .unwrap();

    map.move_camel(Color::Blue, 3);

    assert_eq!(map.find_camel(Color::Blue), 2);
    assert_eq!(map.pos_color_map[2], Some(vec![Color::Blue]));
    assert_eq!(map.pos_color_map[3], None);
}

#[test]
fn test_desert_places_camel_at_bottom_of_stack() {
    // Camel Blue at position 0, Camel Green already at position 2
    // Desert at position 3, Blue moves by 3 -> lands on Desert -> moves to 2
    // Blue should be placed AT BOTTOM (under Green)
    let mut map = CamelMap::builder()
        .with_positions(vec![(0, Color::Blue), (2, Color::Green)])
        .with_effect_cards(vec![(3, EffectCard::Desert)])
        .build()
        .unwrap();

    map.move_camel(Color::Blue, 3);

    assert_eq!(map.find_camel(Color::Blue), 2);
    assert_eq!(map.find_camel(Color::Green), 2);
    // Blue at bottom, Green on top
    assert_eq!(map.pos_color_map[2], Some(vec![Color::Blue, Color::Green]));
}

#[test]
fn test_desert_with_camel_stack_moves_all() {
    // Green on top of Blue at position 0, Desert at position 3
    // Moving Blue by 3 should move both Blue and Green to position 2
    let mut map = CamelMap::builder()
        .with_positions(vec![(0, Color::Blue), (0, Color::Green)])
        .with_effect_cards(vec![(3, EffectCard::Desert)])
        .build()
        .unwrap();

    map.move_camel(Color::Blue, 3);

    assert_eq!(map.find_camel(Color::Blue), 2);
    assert_eq!(map.find_camel(Color::Green), 2);
    // Blue at bottom, Green on top (preserves stack order)
    assert_eq!(map.pos_color_map[2], Some(vec![Color::Blue, Color::Green]));
    assert_eq!(map.pos_color_map[0], None);
}

#[test]
fn test_desert_at_position_one_boundary() {
    // Camel at position 0, Desert at position 1
    // Moving by 1 should land on Desert, then move -1 to position 0 (not negative)
    let mut map = CamelMap::builder()
        .with_positions(vec![(0, Color::Blue), (0, Color::Green)])
        .with_effect_cards(vec![(1, EffectCard::Desert)])
        .build()
        .unwrap();

    // Move Green (which is on top) by 1
    map.move_camel(Color::Green, 1);

    // Green should end up at position 0 (clamped, not negative)
    assert_eq!(map.find_camel(Color::Green), 0);
    assert_eq!(map.find_camel(Color::Blue), 0);
    // Green should be at bottom due to Desert placement rules
    assert_eq!(map.pos_color_map[0], Some(vec![Color::Green, Color::Blue]));
}
