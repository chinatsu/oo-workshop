use super::{Chance, ChanceError};

lazy_static! {
    static ref CERTAIN: Chance = Chance::new(super::CERTAIN).unwrap();
    static ref LIKELY: Chance = Chance::new(0.75).unwrap();
    static ref FIFTY_NINE: Chance = Chance::new(0.59).unwrap();
    static ref EQUALLY_LIKELY: Chance = Chance::new(0.5).unwrap();
    static ref FORTY_ONE: Chance = Chance::new(0.41).unwrap();
    static ref UNLIKELY: Chance = Chance::new(0.25).unwrap();
    static ref IMPOSSIBLE: Chance = Chance::new(0.0).unwrap();

}

#[test]
fn the_chance_of_25_should_be_ok() {
    assert!(Chance::new(0.25).is_ok());
}

#[test]
fn the_chance_of_130_should_be_out_of_bounds() {
    assert_eq!(Err(ChanceError::OutOfBounds), Chance::new(1.30));
}

#[test]
fn the_chance_of_minus_10_should_be_out_of_bounds() {
    assert_eq!(Err(ChanceError::OutOfBounds), Chance::new(-0.1));
}

#[test]
fn the_chance_of_75_should_equal_likely() -> Result<(), ChanceError> {
    assert_eq!(*LIKELY, Chance::new(0.75)?);
    Ok(())
}

#[test]
fn the_chance_of_75_should_not_equal_the_chance_of_25() {
    assert_ne!(*UNLIKELY, *LIKELY);
}

#[test]
fn the_opposite_of_100_should_be_0() {
    assert_eq!(*IMPOSSIBLE, !*CERTAIN);
}

#[test]
fn the_opposite_of_75_should_be_25() {
    assert_eq!(*UNLIKELY, !*LIKELY);
}

#[test]
fn the_opposite_of_the_opposite_of_41_should_be_41() {
    assert_eq!(*FORTY_ONE, !!*FORTY_ONE);
}

#[test]
fn the_opposite_of_the_opposite_of_the_opposite_of_41_should_be_59() {
    assert_eq!(*FIFTY_NINE, !!!*FORTY_ONE);
}

#[test]
fn chances_with_values_41_and_50_together_should_be_20_5() {
    assert_eq!(Chance::new(0.205).unwrap(), *FORTY_ONE & *EQUALLY_LIKELY);
}

#[test]
fn chances_with_values_13_and_0_together_should_be_0() {
    assert_eq!(*IMPOSSIBLE, *FIFTY_NINE & *IMPOSSIBLE);
}

#[test]
fn chances_with_values_100_and_75_and_50_together_should_be_3_75() -> Result<(), ChanceError> {
    assert_eq!(Chance::new(0.375)?, *CERTAIN & *LIKELY & *EQUALLY_LIKELY);
    Ok(())
}

#[test]
fn chances_of_75_or_75_should_be_93_75() -> Result<(), ChanceError> {
    assert_eq!(Chance::new(0.9375)?, *LIKELY | *LIKELY);
    Ok(())
}

#[test]
fn sequence_of_4_chances_of_75_should_be_99_609375() -> Result<(), ChanceError> {
    assert_eq!(Chance::new(0.99609375)?, *LIKELY | *LIKELY | *LIKELY | *LIKELY);
    Ok(())
}

#[test]
fn chances_of_100_or_75_should_be_100() {
    assert_eq!(*CERTAIN, *CERTAIN | *UNLIKELY);
}
