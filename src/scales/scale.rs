use std::ops::{Deref, DerefMut};
use super::ScaleType;
use crate::units::{Note, Contexted};

#[derive(Educe)]
#[educe(Default)]
pub struct Scale {
    key: Note<Contexted>,
    scale_type: ScaleType,
}

impl Scale {
    fn new() -> ScaleBuilder {
        ScaleBuilder(Scale{
            key: Note::default(),
            scale_type: ScaleType::default(),
        })
    }
}

impl Iterator for Scale {
    type Item = Note<Contexted>;
    
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

    pub fn key(&mut self, key: Note<Contexted>) -> &mut Self {
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