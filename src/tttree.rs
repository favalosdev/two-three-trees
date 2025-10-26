pub enum TwoThreeTree<T: Ord> {
    Leaf,
    TwoNode { x: T, l: Box<Self>, r: Box<Self> },
    ThreeNode { x: T, y: T, l: Box<Self>, m: Box<Self>, r: Box<Self> }
}

impl<T: Ord> TwoThreeTree<T> {
    pub fn new_two_node(x: T, l: Option<TwoThreeTree<T>>, r: Option<TwoThreeTree<T>>) -> Self {
        TwoThreeTree::TwoNode {
            x,
            l: Box::new(l.unwrap_or(TwoThreeTree::Leaf)),
            r: Box::new(r.unwrap_or(TwoThreeTree::Leaf))
        }
    }

    pub fn new_three_node(x: T, y: T, l: Option<TwoThreeTree<T>>, m: Option<TwoThreeTree<T>>, r: Option<TwoThreeTree<T>>) -> Self {
        TwoThreeTree::ThreeNode {
            x,
            y,
            l: Box::new(l.unwrap_or(TwoThreeTree::Leaf)),
            m: Box::new(m.unwrap_or(TwoThreeTree::Leaf)),
            r: Box::new(r.unwrap_or(TwoThreeTree::Leaf))
        }
    }
}

pub fn is_terminal<T: Ord>(node: &TwoThreeTree<T>) -> bool {
    match node {
        TwoThreeTree::Leaf => false,
        TwoThreeTree::TwoNode { l, .. } => {
            match **l {
                // We only need to check whether one of the children is a leaf because of path length invariance
                TwoThreeTree::Leaf => true,
                _ => false
            }
        },
        TwoThreeTree::ThreeNode { l, .. } => {
            match **l {
                TwoThreeTree::Leaf => true,
                _ => false
            }
        }
    }
}

fn get_height_aux<T: Ord>(node: &TwoThreeTree<T>, acc: usize) -> usize {
    match node {
        TwoThreeTree::Leaf => acc,
        TwoThreeTree::TwoNode { l, ..} => get_height_aux(&(*l), acc + 1),
        TwoThreeTree::ThreeNode { l, ..} => get_height_aux(&(*l), acc + 1),
    }
}

pub fn get_height<T: Ord>(node: &TwoThreeTree<T>) -> usize {
    get_height_aux(node, 0)
}

// Function signature
// pub fn insert<T: Ord>(node: &TwoThreeTree<T>, value: T) {}
