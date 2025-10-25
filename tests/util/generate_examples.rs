use two_three_trees::tttree::TwoThreeTree;

pub fn ex_1() -> TwoThreeTree<u8> {
    let ll = TwoThreeTree::new_two_node(1, None, None);
    let lr= TwoThreeTree::new_two_node(200, None, None);
    let l = TwoThreeTree::new_two_node(2, Some(ll), Some(lr));
    let rl = TwoThreeTree::new_two_node(5, None, None);
    let rr = TwoThreeTree::new_two_node(7, None, None);
    let r = TwoThreeTree::new_two_node(6, Some(rl), Some(rr));
    let root = TwoThreeTree::new_two_node(4, Some(l), Some(r));
    root
}

pub fn ex_2() -> TwoThreeTree<u8> {
    let l = TwoThreeTree::new_two_node(1, None, None);
    let m = TwoThreeTree::new_three_node(3, 4, None, None, None);
    let r = TwoThreeTree::new_three_node(6, 7, None, None, None);
    let root = TwoThreeTree::new_three_node(2, 5, Some(l), Some(m), Some(r));
    root
}

pub fn ex_3() -> TwoThreeTree<u8> {
    let l = TwoThreeTree::new_three_node(1, 2, None, None, None);
    let m = TwoThreeTree::new_two_node(4, None, None);
    let r = TwoThreeTree::new_three_node(6, 7, None, None, None);
    let root = TwoThreeTree::new_three_node(3, 5, Some(l), Some(m), Some(r));
    root
}

pub fn ex_4() -> TwoThreeTree<u8> {
    let l = TwoThreeTree::new_three_node(1, 2, None, None, None);
    let m = TwoThreeTree::new_three_node(4, 5, None, None, None);
    let r = TwoThreeTree::new_two_node(7, None, None);
    let root = TwoThreeTree::new_three_node(3, 6, Some(l), Some(m), Some(r));
    root
}
