use std::ops::{Deref, DerefMut};
use super::ScaleType;
use crate::units::{Note, Scaled, Octave};

#[derive(Default)]
pub struct ScaleNotes {
    notes: Vec<Note<Scaled>>,
    octave: Octave,
}

#[derive(Educe)]
#[educe(Default)]
pub struct Scale {
    key: Note<Scaled>,
    scale_type: ScaleType,
    inner: ScaleNotes,
}

impl Scale {
    fn new() -> ScaleBuilder {
        ScaleBuilder(Scale{
            ..Default::default()
        })
    }

    fn octave_from_to(&mut self, from: Octave, to: Octave) {
        
    }

    fn get_octave_set(&self) -> &ScaleNotes {
        &self.inner 
    }
}

impl Iterator for Scale {
    type Item = Note<Scaled>;
    
    fn next(&mut self) -> Option<Self::Item> {
       None 
    }
}

struct ScaleBuilder (Scale);

impl Deref for ScaleBuilder {
    type Target = Scale;    

    fn deref(&self) -> &Self::Target {
       &self.0 
    }
}

impl DerefMut for ScaleBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ScaleBuilder {
    pub fn new() -> Self {
        ScaleBuilder (Scale::default())
    }

    pub fn key(&mut self, key: Note<Scaled>) -> &mut Self {
        self.key = key;
        self
    }

    pub fn scale_type(&mut self, scale_type: ScaleType) -> &mut Self {
        self.scale_type = scale_type;
        self
    }

    // TODO: impl this 
    fn construct_scale(&self) {
        
    }

    pub fn build(self) -> Scale {
        self.construct_scale();
        self.0
    }
}
