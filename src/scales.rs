// use super::traits::*;
// use super::enums::*;
// use super::notes::*;
// use std::default::Default;
// use std::ops::Deref;
// use std::ops::DerefMut;

// #[derive(PartialEq, Debug)]
// struct Scale {
//     node_type: NodeType,
//     key: ENote,
//     mode: ScaleType,
//     node: Vec<Node>,
// }

// impl Scale {
//     fn new() -> ScaleBuilder {
//         ScaleBuilder::new()
//     }

// }

// struct ScaleBuilder {
//    inner: Scale 
// }

// impl Deref for ScaleBuilder {
//     type Target = Scale;    

//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }

// impl DerefMut for ScaleBuilder {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.inner
//     }
// }

// impl ScaleBuilder {
//     fn new() -> ScaleBuilder {
//         Self {
//             inner: Scale {
//                node: Vec::<Node<ENote>>::new(),
//                node_type: NodeType::default(),
//                key: ENote::C,
//                mode: ScaleType::default(),
//             }
//         }
//     }

//     fn node(mut self, node_type: NodeType) -> Self {
//         self.node_type = node_type;
//         self
//     }

//     fn key(mut self, key: ENote) -> Self {
//         self.key = key;
//         self
//     }

//     fn mode(mut self, mode_type: ScaleType) -> Self {
//         self.mode = mode_type;
//         self
//     }

//     fn build(mut self) -> Scale {
//         self.node = self.inner_scale_builder();
//         self.inner
//     }

//     fn inner_scale_builder(&self) -> Vec<Node> {
//         let steps = self.mode.scale_pattern();
//         let mut notes = Vec::<Node<ENote>>::new();
//         let mut note = Node::new(ENote::C).build();
//         for step in steps.into_iter() {
//             notes.push(note.add(step));
//         }
//         notes
//     }
// }

// impl Default for ScaleBuilder {
//     fn default() -> Self {
//         Self::new()        
//     }
// }

// #[cfg(test)]

// mod test {
//     use super::*; 

//     #[test]
//     fn c_major_scale_enote() {
//         let c_major_left = Scale::new().build();
//         let c_major_right = Scale {
//                node: Vec::<Node<ENote>>::new(),
//                node_type: NodeType::default(),
//                key: ENote::C,
//                mode: ScaleType::default(),
//             };

//         assert_eq!(c_major_left, c_major_right);
//     }
// }
