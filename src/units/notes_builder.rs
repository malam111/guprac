use std::{ops::{Deref, DerefMut}, marker::PhantomData};
use crate::units::{Note, RawNote, Decorators, Scaled, NotScaled, Octave};

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

    pub fn context(mut self) -> NoteBuilder<Scaled> {
        NoteBuilder {
            _state: PhantomData::<Scaled>,
            ..self
        }
    }

    pub fn nocontext(mut self) -> NoteBuilder<NotScaled> {
        NoteBuilder {
            _state: PhantomData::<NotScaled>,
            ..self
        }
    }


}

impl NoteBuilder<Scaled> {

    pub fn build(mut self) -> Note<Scaled> {
        if self.dec().len() == 0 {
            self.decorators.push(Decorators::Natural);
        }
        self
    }
}

impl NoteBuilder<NotScaled> {

    pub fn build(mut self) -> Note<NotScaled> {
        if self.dec().len() == 0 {
            self.decorators.push(Decorators::Natural);
        }
        self
    }
}
