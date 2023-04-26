#[derive(PartialEq, Debug, Clone)]
pub enum Decorators {
    Natural,
    Sharp,
    Flat,
}

impl Default for Decorators {
    fn default() -> Self {
        Self::Natural
    }
}