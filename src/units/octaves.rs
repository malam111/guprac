#[derive(PartialEq, Debug, Clone)]
pub enum Octaves {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl Default for Octaves {
    fn default() -> Self {
        Self::Two
    }
}