pub struct Dice {
    sides: u8,
}

impl Dice {
    pub fn new(sides: u8) -> Self {
        Self { sides }
    }

    /// # Rolls the dice and returns the value
    ///
    /// # Example
    /// ```no_run
    /// use otus_2::dices::Dice;
    ///
    /// let d6 = Dice::new(6);
    /// let roll = d6.roll();
    ///
    /// assert_eq!(roll, 32u8);
    /// ```
    pub fn roll(&self) -> u8 {
        32
    }
}

#[cfg(test)]
mod tests;
