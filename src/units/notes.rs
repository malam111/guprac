use std::{ops::{Deref, DerefMut}};

use super::{Decorators};

pub enum RawNote {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

pub struct Note {
    raw: RawNote,
    decorators: Vec<Decorators>,
}

impl Note {
    fn new(raw: RawNote) -> NoteBuilder {
        NoteBuilder::new(raw)
    }
}

pub struct NoteBuilder {
    inner: Note
}

impl Deref for NoteBuilder {
    type Target = Note;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for NoteBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl NoteBuilder {
    fn new(raw: RawNote) -> Self {
        NoteBuilder { inner: Note {raw, decorators: Vec::<Decorators>::default() } }
    }

    fn decorators(&mut self, decorator: Vec<Decorators>) -> &mut Self {
        self.decorators.extend(decorator);
        self
    }

    fn decorator(&mut self, decorator: Decorators) -> &mut Self {
        self.decorators.push(decorator);
        self
    }

}