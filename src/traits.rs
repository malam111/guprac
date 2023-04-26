use crate::units::{StepType};
use crate::notes::Node;


pub trait ScaleNode {

}

pub trait NodeCalc {
    fn new(node: Node) -> Self;
    fn add(&mut self, step: StepType) -> &mut Self;
    fn mul_add(&mut self, steps: Vec<StepType>) -> &mut Self;
    fn resolve(&mut self) -> Node;
}