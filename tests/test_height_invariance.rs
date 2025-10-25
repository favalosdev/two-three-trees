mod util;
use util::generate_examples::{ex_1, ex_2, ex_3, ex_4};
use util::tree::{verify_height_invariance};

#[test]
fn check_height_invariance() {
    assert_eq!(verify_height_invariance(&ex_1()), true);
    assert_eq!(verify_height_invariance(&ex_2()), true);
    assert_eq!(verify_height_invariance(&ex_3()), true);
    assert_eq!(verify_height_invariance(&ex_4()), true);
}