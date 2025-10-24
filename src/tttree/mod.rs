pub mod example;

pub enum TwoThreeTree<T> {
    Leaf,
    TwoNode { x: T, l: Box<TwoThreeTree<T>>, r: Box<TwoThreeTree<T>> },
    ThreeNode { x: T, y: T, l: Box<TwoThreeTree<T>>, m: Box<TwoThreeTree<T>>, r: Box<TwoThreeTree<T>> }
}

impl<T> TwoThreeTree<T> {
    pub fn new_leaf() -> Self {
        TwoThreeTree::Leaf
    }

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