use std::ops::{Deref, DerefMut};
use std::convert::{TryInto};

use crate::units::{Note, RawNote, NoContexted, Moves, Octave, Decorators, Moveable};

#[derive(PartialEq, Debug, Default)]
pub struct NoteFeeder {
    note:  Note<NoContexted>,
    moves: Option<Moves>,
    low_octave: Octave,
    high_octave: Octave,
}

impl NoteFeeder {
    fn new() -> NoteFeederBuilder {
        NoteFeederBuilder::new() 
    }

    fn bulk(&mut self, size: u8) -> Vec<Note<NoContexted>> {
        let mut vec: Vec<Note<NoContexted>> = vec!();
        for idx in 0..size {
            vec.push(self.next().unwrap());
        }
        vec
    }
}

impl Iterator for NoteFeeder {
    type Item = Note<NoContexted>;
    
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.note.clone();
        let moves = Moves::rand();
        self.note.move_with(self.moves.or(Some(moves)).unwrap());
        Some(ret)
    }
}

#[derive(Debug, Default)]
pub struct NoteFeederBuilder {
    inner: NoteFeeder,
}

impl Deref for NoteFeederBuilder {
    type Target = NoteFeeder;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for NoteFeederBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl NoteFeederBuilder {

    
    pub fn new() -> Self {
        let mut builder = Self::default();
        builder.note = Note::<NoContexted>::new(RawNote::C).build();
        builder.moves = None;
        builder.low_octave = Octave::O2;
        builder.high_octave = Octave::O4;

        builder
    }

    pub fn starter_note(mut self, starter: Note<NoContexted>) -> Self {
        self.note = starter;
        self
    }

    pub fn moves(mut self, moves: Moves) -> Self {
        self.moves = Some(moves);
        self
    }

    pub fn low_octave(mut self, octave: Octave) -> Self {
        self.low_octave = octave;
        self
    }

    pub fn high_octave(mut self, octave: Octave) -> Self {
        self.high_octave = octave;
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
        let note_l = NoteFeeder {
            note: Note::<NoContexted>::new(RawNote::C).build(),
            moves: Some(4_i8.try_into().unwrap()),
            low_octave: Octave::O2,
            high_octave: Octave::O4,
        };
        let note_r = NoteFeeder::new()
                            .starter_note(note_l.note.clone())
                            .moves(note_l.moves.clone().unwrap())
                            .low_octave(Octave::O2)
                            .high_octave(Octave::O4)
                            .build();
        assert_eq!(note_l, note_r);
    }

    #[test]
    fn iter_test() {
        let mut note_l = Note::<NoContexted>::new(RawNote::D).build();
        let mut note_r = NoteFeeder::new()
                            .starter_note(Note::<NoContexted>::new(RawNote::C).build())
                            .moves(2_i8.try_into().unwrap())
                            .low_octave(Octave::O2)
                            .high_octave(Octave::O4)
                            .build();

        note_r.next();
        assert_eq!(Some(note_l.clone()), note_r.next());
        note_l.move_with(2_i8.try_into().unwrap());
        assert_eq!(Some(note_l.clone()), note_r.next());
    }

    #[test]
    fn bulk_test() {
        let note_a = Note::<NoContexted>::new(RawNote::A).build();
        let note_b = Note::<NoContexted>::new(RawNote::B).build();
        let note_cs = Note::<NoContexted>::new(RawNote::C).decorators(vec![Decorators::Sharp]).build();
        let note_ds = Note::<NoContexted>::new(RawNote::D).decorator(Decorators::Sharp).build();

        let notes_l = vec!(note_a, note_b, note_cs, note_ds);

        let mut note_r = NoteFeeder::new()
                            .starter_note(Note::<NoContexted>::new(RawNote::A).build())
                            .moves(2_i8.try_into().unwrap())
                            .low_octave(Octave::O2)
                            .high_octave(Octave::O4)
                            .build()
                            .bulk(4)
                            .into_iter()
                            .map(|mut note| { note.resolve(); note })
                            .collect::<Vec<Note<NoContexted>>>();
        
        assert_eq!(notes_l, note_r);
        
    }

    #[test]
    fn rand_test() {
        let mut feeder = NoteFeeder::new()
                            .starter_note(Note::<NoContexted>::new(RawNote::A).build())
                            .low_octave(Octave::O2)
                            .high_octave(Octave::O4)
                            .build()
                            .bulk(4);

        println!("{:?}", feeder);
        assert!(true)
    }
}
