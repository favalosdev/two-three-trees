mod util;

use util::generate_examples::{ex_1, ex_2, ex_3, ex_4};
use util::tree::verify_well_ordering;

#[test]
fn check_well_ordering() {
    assert_eq!(verify_well_ordering(ex_1(), |_: u8| -> bool { true }), true);
    assert_eq!(verify_well_ordering(ex_2(), |_: u8| -> bool { true }), true);
    assert_eq!(verify_well_ordering(ex_3(), |_: u8| -> bool { true }), true);
    assert_eq!(verify_well_ordering(ex_4(), |_: u8| -> bool { true }), true);
}