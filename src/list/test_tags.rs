use super::*;

#[test]
fn empty_list() {
    assert_eq!(List::new(vec![]).tags().len(), 0);
}

#[test]
fn full_list() {
    assert_eq!(test_util::make_list().tags().len(), 9);
}
