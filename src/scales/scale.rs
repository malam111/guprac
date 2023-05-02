use super::ScaleType;
use crate::units::Note;

pub struct Scale {
    key: Note,
    scale_type: ScaleType,
}

impl Iterator for Scale {
    type Item = Note;
    
    fn next(&mut self) -> Option<Self::Item> {
       None 
    }
}