use std::ops::{Deref, DerefMut};

use super::{Decor, Decorators};

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
    decorators: Decorators,
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
        NoteBuilder { inner: Note {raw, decorators: Decorators::default() } }
    }

    fn decorators(&mut self, decorator: T) -> &mut Self {
        self.decorators.extend(decorator.clone().into_iter());
        self
    }
}