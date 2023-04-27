use std::default::Default;
use std::ops::Deref;
use std::ops::DerefMut;
use crate::traits::NodeCalc;
use crate::units::*;
use crate::nodes::Node;

#[derive(PartialEq, Debug, Default)]
struct Scale {
    node_type: NodeType,
    key: Note,
    mode: ScaleType,
    node: Vec<Node>,
}

impl Scale {
    fn new() -> ScaleBuilder {
        ScaleBuilder::new()
    }

}

struct ScaleBuilder {
   inner: Scale 
}

impl Deref for ScaleBuilder {
    type Target = Scale;    

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ScaleBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl ScaleBuilder {
    fn new() -> ScaleBuilder {
        Self {
            inner: Scale::default()
        }
    }

    fn node(mut self, node_type: NodeType) -> Self {
        self.node_type = node_type;
        self
    }

    fn key(mut self, key: Note) -> Self {
        self.key = key;
        self
    }

    fn mode(mut self, mode_type: ScaleType) -> Self {
        self.mode = mode_type;
        self
    }

    fn build(mut self) -> Scale {
        self.node = self.inner_scale_builder();
        self.inner
    }

    fn inner_scale_builder(&self) -> Vec<Node> {
        let steps = self.mode.scale_pattern();
        let mut notes = Vec::<Node>::new();
        let ScaleCalc = ScaleCalc::new(self.key);
        
        // fix endless iterator loop, build a wrapper
        for (note, step) in ScaleCalc::one_round(self.key).into_iter().zip(self.mode.scale_pattern()) {
            notes.push(ScaleCalc::new(note).add(step));
        }
        notes
    }
}

impl Default for ScaleBuilder {
    fn default() -> Self {
        Self::new()        
    }
}

struct ScaleCalc {
    inner: Node,
}

impl Deref for ScaleCalc {
    type Target = Note;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ScaleCalc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl ScaleCalc {
    fn one_round(key: Note) -> Vec<Note> {
        let mut keyy = key.clone();
        let mut scale_basic = Vec::<Note>::new();
        for i in 0..7 {
            scale_basic.push(keyy);
            keyy.next();
        }
        scale_basic
    }

    fn new(node: Node) -> Self {
        ScaleCalc { inner: node }
    }
    fn add(&mut self, ctx: StepType) -> Node {
        let next = self.inner.peek_next().unwrap();
        let builder = Node::new(next);
        let mut updown: i8 = ctx.clone() - self.get_step();
        let decor = if updown < 0 {Decorators::Flat} else if updown > 0 {Decorators::Sharp} else {Decorators::Natural};
        let mut decorators = Vec::<Decorators>::new();
        updown = if updown < 0 {updown * -1} else {updown};
        for idx in 0..updown as u8 {
            decorators.push(decor.clone());
        }
        println!("Note: , {:?}-{:?} {:?}", ctx, next.get_step(), updown);
        builder.decorators(decorators).build()
    }
    fn resolve(&mut self) -> Node {
        todo!() 
    }
}

#[cfg(test)]

mod test {
    use super::*; 

    #[test]
    fn g_major_scale() {
        let g_scale_left = Scale::new().key(Note::G).mode(ScaleType::Major).build();
        let g_scale_right = Scale {
            node: vec![
                Node::new(Note::G).build(),
                Node::new(Note::A).build(),
                Node::new(Note::B).build(),
                Node::new(Note::C).build(),
                Node::new(Note::D).build(),
                Node::new(Note::E).build(),
                Node::new(Note::F).decorators(vec![Decorators::Sharp]).build(),
            ],
            ..Scale::default()
        };
        println!("{:?}", g_scale_right);
        assert_eq!(g_scale_left, g_scale_right);
    }
}
