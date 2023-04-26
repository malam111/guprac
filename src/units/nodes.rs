#[derive(PartialEq, Debug)]
pub enum NodeType {
    ENote,
    Interval
}

impl Default for NodeType {
    fn default() -> Self {
        Self::Interval
    }
}