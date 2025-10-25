mod util;

use util::generate_examples::{ex_1, ex_2, ex_3, ex_4};
use util::tree::{verify_well_ordering, Comparison};
struct Dummy; 

impl <T: Copy + Ord> Comparison<T> for Dummy {
    fn check(&self, _: &T) -> bool {
        true
    }
}

#[test]
fn check_well_ordering() {
    assert_eq!(verify_well_ordering(ex_1(), Dummy), true);
    assert_eq!(verify_well_ordering(ex_2(), Dummy), true);
    assert_eq!(verify_well_ordering(ex_3(), Dummy), true);
    assert_eq!(verify_well_ordering(ex_4(), Dummy), true);
}