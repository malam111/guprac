use crate::units::{StepType};
use crate::nodes::Node;


pub trait ScaleNode {

}

pub trait NodeCalc {
    type Target;
    fn new(node: Node) -> Self;
    fn add(&mut self, step: StepType) -> &mut Self::Target;
    fn mul_add(&mut self, steps: Vec<StepType>) -> &mut Self::Target;
    fn resolve(&mut self) -> Node;
}