use std::{ops::{Deref, DerefMut}, marker::PhantomData};
use crate::units::{Note, RawNote, Decorators, WithScale, NoScale, Octave};

pub struct NoteBuilder<State> {
    raw: RawNote,
    decorators: Vec<Decorators>,
    octave: Octave,
    _state: PhantomData<State>,
}

impl<State> NoteBuilder<State> {
    
    fn new(raw: RawNote) -> Self {
        NoteBuilder{
            raw,
            ..Note::default()
        }
    }

    pub fn decorators(mut self, decorators: Vec<Decorators>) -> Self {
        self.decorators.extend(decorators);
        self
    }

    pub fn decorator(mut self, decorator: Decorators) -> Self {
        self.decorators.push(decorator);
        self
    }

    pub fn octave(mut self, octave: Octave) -> Self {
        self.octave = octave;
        self
    }

    pub fn context(mut self) -> NoteBuilder<WithScale> {
        NoteBuilder {
            _state: PhantomData::<WithScale>,
            ..self
        }
    }

    pub fn nocontext(mut self) -> NoteBuilder<NoScale> {
        NoteBuilder {
            _state: PhantomData::<NoScale>,
            ..self
        }
    }


}

impl NoteBuilder<WithScale> {

    pub fn build(mut self) -> Note<WithScale> {
        if self.dec().len() == 0 {
            self.decorators.push(Decorators::Natural);
        }
        self
    }
}

impl NoteBuilder<NoScale> {

    pub fn build(mut self) -> Note<NoScale> {
        if self.dec().len() == 0 {
            self.decorators.push(Decorators::Natural);
        }
        self
    }
}
