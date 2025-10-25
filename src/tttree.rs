pub enum TwoThreeTree<T: Copy + Ord> {
    Leaf,
    TwoNode { x: T, l: Box<Self>, r: Box<Self> },
    ThreeNode { x: T, y: T, l: Box<Self>, m: Box<Self>, r: Box<Self> }
}

impl<T: Copy + Ord> TwoThreeTree<T> {
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

fn get_height_aux<T: Copy + Ord>(node: &TwoThreeTree<T>, acc: usize) -> usize {
    match node {
        TwoThreeTree::Leaf => acc,
        TwoThreeTree::TwoNode { l, ..} => get_height_aux(&(*l), acc + 1),
        TwoThreeTree::ThreeNode { l, ..} => get_height_aux(&(*l), acc + 1),
    }
}

pub fn get_height<T: Copy + Ord>(node: &TwoThreeTree<T>) -> usize {
    get_height_aux(node, 0)
}
