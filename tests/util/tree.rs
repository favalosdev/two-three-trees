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

struct InBetween<T: Copy + Ord> {
    start: T,
    end: T
}

impl <T: Copy + Ord> Comparison<T> for InBetween<T> {
    fn check(&self, x: &T) -> bool {
        self.start <= *x && *x <= self.end
    }
}

pub fn verify_well_ordering<T: Copy + Ord>(node: TwoThreeTree<T>) -> bool {
    match node {
        TwoNode { x, l, r } => {
            match *l {
                Leaf => true,
                _ => {
                    match *r {
                        Leaf => true,
                        _ => {
                            *get_rightmost(&(*l)) <= x &&
                            *get_leftmost(&(*r)) >= x &&
                            vfw_aux(*l, LessThan { pivot: x }) &&
                            vfw_aux(*r, GreaterThan { pivot: x })
                        },
                    }
                },
            }
        },
        ThreeNode { x, y, l, m, r } => {
            match *l {
                Leaf => true,
                _ => {
                    match *r {
                        Leaf => true,
                        _ => {
                            *get_rightmost(&(*l)) <= x &&
                            *get_leftmost(&(*m)) >= x &&
                            *get_rightmost(&(*m)) <= y &&
                            *get_leftmost(&(*r)) >= y &&
                            vfw_aux(*l, LessThan { pivot: x }) &&
                            vfw_aux(*m, InBetween { start: x, end: y}) &&
                            vfw_aux(*r, GreaterThan { pivot: x })
                        },
                        
                    }
                },
            }
        },
        Leaf => true
    }
}

pub fn vfw_aux<T: Copy + Ord, C: Comparison<T>>(node: TwoThreeTree<T>, processor: C) -> bool {
    match node {
        Leaf => true,
        TwoNode { x, l, r } => {
            processor.check(&x) &&
            vfw_aux(*l, LessThan { pivot: x} ) &&
            vfw_aux(*r, GreaterThan { pivot: x })
        },
        ThreeNode { x, y, l, m, r } => {
            x <= y &&
            processor.check(&x) &&
            processor.check(&y) &&
            vfw_aux(*l, LessThan { pivot: x} ) &&
            vfw_aux(*m, InBetween { start: x, end: y }) &&
            vfw_aux(*r, GreaterThan { pivot: y })
        }
    }
}

pub fn get_rightmost<T: Ord + Copy>(node: &TwoThreeTree<T>) -> &T {
    match node {
        TwoNode { r , x, .. } => {
            match **r {
                Leaf => x,
                _ => get_rightmost(&(*r))
            }
        },
        ThreeNode { r, y, .. } => {
            match **r {
                Leaf => y,
                _ => get_rightmost(&(*r))
            }
        },
        Leaf => panic!("This should never ever happen"),
    }
}

pub fn get_leftmost<T: Ord + Copy>(node: &TwoThreeTree<T>) -> &T {
   match node {
        TwoNode { l, x, .. } => {
            match **l {
                Leaf => x,
                _ => get_rightmost(&(*l))
            }
        },
        ThreeNode { l, x, .. } => {
            match **l {
                Leaf => x,
                _ => get_rightmost(&(*l))
            }
        },
        Leaf => panic!("This should never ever happen either!"),
    } 
}
