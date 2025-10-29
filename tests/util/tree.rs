use two_three_trees::tttree::{TwoThreeTree, get_height};
use TwoThreeTree::{Leaf, TwoNode, ThreeNode};

#[derive(Clone, Copy)]
enum Comparison {
    GET, // Greater or equal than
    LET // Less or equal than
}

fn get_rightmost<T: Ord>(node: &TwoThreeTree<T>) -> Option<&T> {
    match node {
        TwoNode { r , x, .. } => {
            match **r {
                Leaf => Some(x),
                _ => get_rightmost(r)
            }
        },
        ThreeNode { r, y, .. } => {
            match **r {
                Leaf => Some(y),
                _ => get_rightmost(r)
            }
        },
        Leaf => None,
    }
}

fn get_leftmost<T: Ord>(node: &TwoThreeTree<T>) -> Option<&T> {
   match node {
        TwoNode { l, x, .. } => {
            match **l {
                Leaf => Some(x),
                _ => get_leftmost(l)
            }
        },
        ThreeNode { l, x, .. } => {
            match **l {
                Leaf => Some(x),
                _ => get_leftmost(l)
            }
        },
        Leaf => None,
    }
}

fn check<T: Ord>(val: &T, pivot: &T, c: Comparison) -> bool {
    match c {
        Comparison::GET => val >= pivot,
        Comparison::LET => val <= pivot
    }
}

fn vwo_aux_simple<T: Ord>(node: &TwoThreeTree<T>, parent: &T, mode: Comparison) -> bool {
    match node {
        Leaf => true,
        TwoNode { x, l, r } => {
            check(x, parent, mode) &&
            vwo_aux_simple(l, x, Comparison::LET) &&
            vwo_aux_simple(r, x, Comparison::GET)
        },
        ThreeNode { x, y, l, m, r } => {
            x <= y &&
            check(x, parent, mode) &&
            check(y, parent, mode) &&
            vwo_aux_simple(l, x, Comparison::LET) &&
            vwo_aux_interval(m, x, y) &&
            vwo_aux_simple(r, x, Comparison::GET)
        }
    }
}

fn vwo_aux_interval<T: Ord>(node: &TwoThreeTree<T>, start: &T, end: &T) -> bool {
   match node {
        Leaf => true,
        TwoNode { x, l, r } => {
            start <= x  &&
            x <= end &&
            vwo_aux_simple(l, x, Comparison::LET) &&
            vwo_aux_simple(r, x, Comparison::GET)
        },
        ThreeNode { x, y, l, m, r } => {
            x <= y &&
            start <= x && 
            x <= end &&
            start <= y &&
            y <= end &&
            vwo_aux_simple(l, x, Comparison::LET) &&
            vwo_aux_interval(m, x, y) &&
            vwo_aux_simple(r, x, Comparison::GET)
        }
    } 
}

pub fn verify_well_ordering<T: Ord>(node: &TwoThreeTree<T>) -> bool {
    match node {
        TwoNode { x, l, r } => {
            *get_rightmost(l).unwrap_or(&x) <= *x &&
            *get_leftmost(r).unwrap_or(&x) >= *x &&
            vwo_aux_simple(l, x, Comparison::LET) &&
            vwo_aux_simple(r, x, Comparison::GET)
        },
        ThreeNode { x, y, l, m, r } => {
            x <= y &&
            *get_rightmost(l).unwrap_or(&x) <= *x &&
            *get_leftmost(m).unwrap_or(&x) >= *x &&
            *get_rightmost(m).unwrap_or(&y) <= *y &&
            *get_leftmost(r).unwrap_or(&y) >= *y &&
            vwo_aux_simple(l, x, Comparison::LET) &&
            vwo_aux_interval(m, x, y) &&
            vwo_aux_simple(r, y, Comparison::GET) 
        },
        Leaf => true
    }
}

fn vpli_aux<T: Ord>(node: &TwoThreeTree<T>, height: usize, acc: usize) -> bool {
    match node {
        Leaf => height == acc,
        TwoNode { l, r, ..} => {
            vpli_aux(l, height, acc + 1) &&
            vpli_aux(r, height, acc + 1)
        },
        ThreeNode { l, m, r, .. } => {
            vpli_aux(l, height, acc + 1) &&
            vpli_aux(m, height, acc + 1) &&
            vpli_aux(r, height, acc + 1)
        }
    }
}

pub fn verify_path_length_invariance<T: Ord>(node: &TwoThreeTree<T>) -> bool {
    let height = get_height(node);
    vpli_aux(node, height, 0)
}
