use two_three_trees::tttree::TwoThreeTree;
use TwoThreeTree::{Leaf, TwoNode, ThreeNode};

pub trait Comparison<T: Copy + Ord> {
    fn check(&self, x: &T) -> bool;
}

struct LessThan<T: Copy + Ord> {
    pivot: T
}

impl<T: Copy + Ord> Comparison<T> for LessThan<T> {
    fn check(&self, x: &T) -> bool {
        *x <= self.pivot 
    }
}
struct GreaterThan<T: Copy + Ord> {
    pivot: T
}

impl <T:Copy + Ord> Comparison<T> for GreaterThan<T> {
    fn check(&self, x: &T) -> bool {
        self.pivot <= *x
    }
}

struct InInterval<T: Copy + Ord> {
    beggining: T,
    end: T
}

impl <T: Copy + Ord> Comparison<T> for InInterval<T> {
    fn check(&self, x: &T) -> bool {
        self.beggining <= *x && *x <= self.end
    }
}

pub fn verify_well_ordering<T: Copy + Ord, C: Comparison<T>>(node: TwoThreeTree<T>, processor: C) -> bool {
    match node {
        Leaf => true,
        TwoNode { x, l, r } => {
            processor.check(&x) &&
            verify_well_ordering(*l,LessThan { pivot: x }) &&
            verify_well_ordering(*r, GreaterThan { pivot: x })
        },
        ThreeNode { x, y, l, m, r } => {
            processor.check(&x) &&
            processor.check(&y) &&
            verify_well_ordering(*l, LessThan { pivot: x} ) &&
            verify_well_ordering(*m, InInterval { beggining: x, end: y }) &&
            verify_well_ordering(*r, GreaterThan { pivot: y })
            
        }
    }
}