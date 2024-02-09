use std::ops::{Deref, DerefMut};
use std::convert::{TryInto};

use crate::units::{Note, RawNote, WithScale, Moves, Octave, Decorators, Moveable};
use crate::scales::{Scale};

#[derive(PartialEq, Debug, Default)]
pub struct ScaleFeeder {
    tonic: Note<WithScale>,
    note:  Note<WithScale>,
    moves: Option<Moves>,
    scale: Option<Scale>,
    low_octave: Octave,
    high_octave: Octave,
}

impl ScaleFeeder {
    fn new() -> ScaleFeederBuilder {
        ScaleFeederBuilder::new() 
    }

    fn bulk(&mut self, size: u8) -> Vec<Note<WithScale>> {
        let mut vec: Vec<Note<WithScale>> = vec!();
        for idx in 0..size {
            vec.push(self.next().unwrap());
        }
        vec
    }
}

impl Iterator for ScaleFeeder {
    type Item = Note<WithScale>;
    
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.note.clone();
        let moves = Moves::rand();
        self.note.move_with(self.moves.or(Some(moves)).unwrap());
        Some(ret)
    }
}

#[derive(Debug, Default)]
pub struct ScaleFeederBuilder {
    inner: ScaleFeeder,
}

impl Deref for ScaleFeederBuilder {
    type Target = ScaleFeeder;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ScaleFeederBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl ScaleFeederBuilder {

    
    pub fn new() -> Self {
        let mut builder = Self::default();
        builder.tonic = Note::<WithScale>::new(RawNote::C).build();
        builder.note = Note::<WithScale>::new(RawNote::C).build();
        builder.moves = None;
        builder.low_octave = Octave::O2;
        builder.high_octave = Octave::O4;

        builder
    }

    pub fn starter_note(mut self, starter: Note<WithScale>) -> Self {
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

    pub fn build(mut self) -> ScaleFeeder {
        self.inner
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn builder_test() {
        let note_l = ScaleFeeder {
            tonic: Note::<WithScale>::new(RawNote::C).build(),
            note: Note::<WithScale>::new(RawNote::C).build(),
            moves: Some(4_i8.try_into().unwrap()),
            low_octave: Octave::O2,
            high_octave: Octave::O4,
        };
        let note_r = ScaleFeeder::new()
                            .starter_note(note_l.note.clone())
                            .moves(note_l.moves.clone().unwrap())
                            .low_octave(Octave::O2)
                            .high_octave(Octave::O4)
                            .build();
        assert_eq!(note_l, note_r);
    }

    #[test]
    fn iter_test() {
        let mut note_l = Note::<WithScale>::new(RawNote::D).build();
        let mut note_r = ScaleFeeder::new()
                            .starter_note(Note::<WithScale>::new(RawNote::C).build())
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
        let note_a = Note::<WithScale>::new(RawNote::A).build();
        let note_b = Note::<WithScale>::new(RawNote::B).build();
        let note_cs = Note::<WithScale>::new(RawNote::C).decorators(vec![Decorators::Sharp]).build();
        let note_ds = Note::<WithScale>::new(RawNote::D).decorator(Decorators::Sharp).build();

        let notes_l = vec!(note_a, note_b, note_cs, note_ds);

        let mut note_r = ScaleFeeder::new()
                            .starter_note(Note::<WithScale>::new(RawNote::A).build())
                            .moves(2_i8.try_into().unwrap())
                            .low_octave(Octave::O2)
                            .high_octave(Octave::O4)
                            .build()
                            .bulk(4)
                            .into_iter()
                            .map(|mut note| { note.resolve(); note })
                            .collect::<Vec<Note<WithScale>>>();
        
        assert_eq!(notes_l, note_r);
        
    }

    #[test]
    fn rand_test() {
        let mut feeder = ScaleFeeder::new()
                            .starter_note(Note::<WithScale>::new(RawNote::A).build())
                            .low_octave(Octave::O2)
                            .high_octave(Octave::O4)
                            .build()
                            .bulk(4);

        println!("{:?}", feeder);
        assert!(true)
    }
}
