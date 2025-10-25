/*
 Let's go with the rule implementation:
 a. For two nodes, every value appearing in l must be <= X
 b. For three nodes, every value appearing in r must be >= X
 */
use two_three_trees::tttree::TwoThreeTree;
use TwoThreeTree::{Leaf, TwoNode, ThreeNode};

pub fn verify_well_ordering<T: Ord, F>(node: TwoThreeTree<T>, check: F) -> bool where F: Fn(T) -> bool {
    match node {
        Leaf => true,
        TwoNode { x, l, r } => {
            verify_well_ordering(*l, |val: T| val <= x) &&
            verify_well_ordering(*r, |val: T| x <= val) &&
            check(x)
        },
        ThreeNode { x, y, l, m, r } => {
            verify_well_ordering(*l, |val: T| val <= x) &&
            verify_well_ordering(*m, |val: T| x <= val && val <= y) &&
            verify_well_ordering(*r, |val: T| y <= val) &&
            check(x) &&
            check(y)
        }
    }
}
