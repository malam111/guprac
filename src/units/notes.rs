use educe::Educe;
use std::{ops::{Deref, DerefMut}, marker::PhantomData};

use crate::scales::ScaleMoves;

use super::{Decorators};

#[derive(Educe)]
#[educe(Default)]
pub enum RawNote {
    #[educe(Default)]
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

pub struct Contexted;
pub struct NoContexted;

#[derive(Educe)]
#[educe(Default)]
pub struct Note<State> {
    raw: RawNote,
    decorators: Vec<Decorators>,
    _state: PhantomData<State>,
}

impl<State> Note<State> {
    fn new(raw: RawNote) -> NoteBuilder<State> {
        NoteBuilder::new(raw)
    }
}

impl Note<Contexted> {
    fn add() {

    }
}

impl Note<NoContexted> {
    fn add() {

    }
}

pub struct NoteBuilder<T> (Note<T>);

impl<T> Deref for NoteBuilder<T> {
    type Target = Note<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for NoteBuilder<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> NoteBuilder<T> {
    fn new(raw: RawNote) -> Self {
        NoteBuilder(Note {raw, ..Note::default() })
    }

    fn decorators(&mut self, decorator: Vec<Decorators>) -> &mut Self {
        self.decorators.extend(decorator);
        self
    }

    fn decorator(&mut self, decorator: Decorators) -> &mut Self {
        self.decorators.push(decorator);
        self
    }

    fn build(self) -> Note<T> {
        self.0
    }

}
