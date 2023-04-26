use crate::traits::ScaleNode;

#[derive(PartialEq, Debug)]
pub enum Interval {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

impl Default for Interval {
   fn default() -> Self {
       Self::One
   } 
}

impl ScaleNode for Interval {

}