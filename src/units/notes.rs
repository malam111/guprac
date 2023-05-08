use educe::Educe;
use std::{ops::{Deref, DerefMut}};

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

#[derive(Educe)]
#[educe(Default)]
pub struct Note {
    raw: RawNote,
    decorators: Vec<Decorators>,
}

impl Note {
    fn new(raw: RawNote) -> NoteBuilder {
        NoteBuilder::new(raw)
    }
}

impl IntoIterator for Note {
    type Item = Self;
    type IntoIter = NoteInterator;
    fn into_iter(self) -> Self::IntoIter {
        NoteInterator {
            self
        } 
    }
}

pub struct NoteBuilder (Note);

impl Deref for NoteBuilder {
    type Target = Note;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NoteBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl NoteBuilder {
    fn new(raw: RawNote) -> Self {
        NoteBuilder(Note {raw, decorators: Vec::<Decorators>::default() })
    }

    fn decorators(&mut self, decorator: Vec<Decorators>) -> &mut Self {
        self.decorators.extend(decorator);
        self
    }

    fn decorator(&mut self, decorator: Decorators) -> &mut Self {
        self.decorators.push(decorator);
        self
    }

    fn build(self) -> Note {
        self.0
    }

}

pub struct NoteInterator {
    note: Note,
    moves: ScaleMoves,
}

impl Iterator for NoteInterator {
    type Item = Note; 
    
    fn next(&mut self) -> Option<Self::Item> {
       Some(Note::new(RawNote::C).build()) 
    }
    
    fn collect<B: FromIterator<Self::Item>>(self) -> B
        where
            Self: Sized, {
        
    }
}