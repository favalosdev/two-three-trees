use two_three_trees::tttree::{TwoThreeTree, get_height};
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

fn get_rightmost<T: Ord + Copy>(node: &TwoThreeTree<T>) -> Option<&T> {
    match node {
        TwoNode { r , x, .. } => {
            match **r {
                Leaf => Some(x),
                _ => get_rightmost(&(*r))
            }
        },
        ThreeNode { r, y, .. } => {
            match **r {
                Leaf => Some(y),
                _ => get_rightmost(&(*r))
            }
        },
        Leaf => None,
    }
}

fn get_leftmost<T: Ord + Copy>(node: &TwoThreeTree<T>) -> Option<&T> {
   match node {
        TwoNode { l, x, .. } => {
            match **l {
                Leaf => Some(x),
                _ => get_rightmost(&(*l))
            }
        },
        ThreeNode { l, x, .. } => {
            match **l {
                Leaf => Some(x),
                _ => get_rightmost(&(*l))
            }
        },
        Leaf => None,
    }
}

fn vwo_aux<T: Copy + Ord, C: Comparison<T>>(node: &TwoThreeTree<T>, processor: C) -> bool {
    match node {
        Leaf => true,
        TwoNode { x, l, r } => {
            processor.check(&x) &&
            vwo_aux(l, LessThan { pivot: *x } ) &&
            vwo_aux(r, GreaterThan { pivot: *x })
        },
        ThreeNode { x, y, l, m, r } => {
            x <= y &&
            processor.check(&x) &&
            processor.check(&y) &&
            vwo_aux(l, LessThan { pivot: *x } ) &&
            vwo_aux(m, InBetween { start: *x, end: *y }) &&
            vwo_aux(r, GreaterThan { pivot: *y })
        }
    }
}

pub fn verify_well_ordering<T: Copy + Ord>(node: &TwoThreeTree<T>) -> bool {
    match node {
        TwoNode { x, l, r } => {
            *get_rightmost(&(*l)).unwrap_or(&x) <= *x &&
            *get_leftmost(&(*r)).unwrap_or(&x) >= *x &&
            vwo_aux(l, LessThan { pivot: *x }) &&
            vwo_aux(r, GreaterThan { pivot: *x })
        },
        ThreeNode { x, y, l, m, r } => {
            x <= y &&
            *get_rightmost(&(*l)).unwrap_or(&x) <= *x &&
            *get_leftmost(&(*m)).unwrap_or(&x) >= *x &&
            *get_rightmost(&(*m)).unwrap_or(&x) <= *y &&
            *get_leftmost(&(*r)).unwrap_or(&x) >= *y &&
            vwo_aux(l, LessThan { pivot: *x }) &&
            vwo_aux(m, InBetween { start: *x, end: *y }) &&
            vwo_aux(r, GreaterThan { pivot: *x })
        },
        Leaf => true
    }
}

fn vhi_aux<T: Copy + Ord>(node: &TwoThreeTree<T>, height: usize, acc: usize) -> bool {
    match node {
        Leaf => height == acc,
        TwoNode { l, r, ..} => {
            vhi_aux(&(*l), height, acc + 1) &&
            vhi_aux(&(*r), height, acc + 1)
        },
        ThreeNode { l, m, r, .. } => {
            vhi_aux(&(*l), height, acc + 1) &&
            vhi_aux(&(*m), height, acc + 1) &&
            vhi_aux(&(*r), height, acc + 1)
        }
    }
}

pub fn verify_height_invariance<T: Copy + Ord>(node: &TwoThreeTree<T>) -> bool {
    let height = get_height(node);
    vhi_aux(node, height, 0)
}
