use calc::{Configuration, CamelMap, EffectCard, Color};

#[test]
fn test_normalize_shifts_effect_cards() {
    let map = CamelMap::builder()
        .with_positions(vec![
            (3, Color::Blue),
            (3, Color::Green),
            (4, Color::Orange),
            (4, Color::White),
            (5, Color::Yellow),
        ])
        .with_effect_cards(vec![
            (5, EffectCard::Oasis),
            (7, EffectCard::Desert)
        ])
        .build();

    let mut config = Configuration::builder().with_camel_map(map).build();

    config.normalize();

    // After normalization, smallest position is 3, so it becomes 0.
    // The Oasis card at position 5 should shift to 2 (5 - 3 = 2).
    assert_eq!(config.map.find_camel(Color::Blue), 0);
    assert_eq!(config.map.find_camel(Color::Green), 0);
    assert_eq!(config.map.effect_cards[2], Some(EffectCard::Oasis));
    assert_eq!(config.map.effect_cards[4], Some(EffectCard::Desert));
    assert_eq!(config.map.effect_cards[5], None);
    assert_eq!(config.map.effect_cards[7], None);
}
