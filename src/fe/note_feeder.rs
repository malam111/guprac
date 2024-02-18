use std::ops::{Deref, DerefMut};
use std::convert::{TryInto};
use std::default::Default;

use crate::units::{NoteBuilder, Note, RawNote, NotScaled, Moves, Octave, Decorators, Moveable};

#[derive(PartialEq, Debug)]
pub struct NoteFeeder {
    note:  Note<NotScaled>,
    moves: Option<Moves>,
    low_octave: Octave,
    high_octave: Octave,
}

impl NoteFeeder {
    fn new() -> NoteFeederBuilder {
        NoteFeederBuilder::new() 
    }

    fn note(&mut self) -> Note<NotScaled> {
        self.note.clone()
    }

    fn bulk(&mut self, size: u8) -> Vec<Note<NotScaled>> {
        let mut vec: Vec<Note<NotScaled>> = vec!();
        for idx in 0..size {
            vec.push(self.note.clone());
            self.next();
        }
        vec
    }
}

impl Default for NoteFeeder {

    fn default() -> Self {
        NoteFeeder {
            note: Note::default(),
            moves: None,
            low_octave: Octave::O3,
            high_octave: Octave::O4
        }
    }
}

impl Iterator for NoteFeeder {
    type Item = Note<NotScaled>;
    
    fn next(&mut self) -> Option<Self::Item> {
        let moves = Moves::rand();
        self.note.move_with(self.moves.or(Some(moves)).unwrap());
        Some(self.note.clone())
    }
}

#[derive(Debug, Default)]
pub struct NoteFeederBuilder {
    inner: NoteFeeder,
}

impl NoteFeederBuilder {
    
    pub fn new() -> Self {
        NoteFeederBuilder::default()
    }

    pub fn starter_note(mut self, starter: Note<NotScaled>) -> Self {
        self.inner.note = starter;
        self
    }

    pub fn moves(mut self, moves: Moves) -> Self {
        self.inner.moves = Some(moves);
        self
    }

    pub fn low_octave(mut self, octave: Octave) -> Self {
        self.inner.low_octave = octave;
        self
    }

    pub fn high_octave(mut self, octave: Octave) -> Self {
        self.inner.high_octave = octave;
        self
    }

    pub fn build(mut self) -> NoteFeeder {
        self.inner
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn builder_test() {
        let left = NoteFeeder::default();
        let right = NoteFeeder::new()
                            .starter_note(NoteBuilder::<NotScaled>::new().noscaled().build())
                            .low_octave(Octave::O3)
                            .high_octave(Octave::O4)
                            .build();
        assert_eq!(left, right);
    }

    #[test]
    fn iter_test() {

        let mut builder = NoteBuilder::<NotScaled>::new().raw(RawNote::D).noscaled();
        let mut left = builder.raw(RawNote::D).build();
        let mut right = NoteFeeder::new()
                            .starter_note(builder.raw(RawNote::C).build())
                            .moves(2_i8.try_into().unwrap())
                            .build();

        right.next();
        assert_eq!(left, right.note());
    }

    #[test]
    fn bulk_test() {
        let mut builder = NoteBuilder::<NotScaled>::new().noscaled();
        let note_a = builder.raw(RawNote::A).build();
        let note_b = builder.raw(RawNote::B).build();
        let note_cs = builder.raw(RawNote::C).decorator(Decorators::Sharp).build();
        let note_ds = builder.raw(RawNote::D).decorator(Decorators::Sharp).build();

        let left = vec!(note_a, note_b, note_cs, note_ds);

        let mut right = NoteFeeder::new()
                            .starter_note(builder.raw(RawNote::A).decorator(Decorators::Natural).build())
                            .moves(2_i8.try_into().unwrap())
                            .low_octave(Octave::O3)
                            .high_octave(Octave::O4)
                            .build()
                            .bulk(4);
        
        assert_eq!(left, right);
        
    }

    #[test]
    fn rand_test() {
        let mut builder = NoteBuilder::<NotScaled>::new().noscaled();
        let mut feeder = NoteFeeder::new()
                            .starter_note(builder.build())
                            .build();
        for (idx, note) in feeder.enumerate() {
            if idx == 10 { break; }
            println!("{}", note);
        }

        assert!(true)
    }
}
