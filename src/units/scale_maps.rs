use crate::units::{Moves, Direction};

enum ScaleMap {
    C,
    C_,
    D,
    D_,
    E,
    F,
    F_,
    G,
    G_,
    A,
    A_,
    B,
}

impl ScaleMap {
    // TODO: impl distance, go up and down
    fn distance(src: Self, dst: Self) -> Moves {
        todo!()
    }
}