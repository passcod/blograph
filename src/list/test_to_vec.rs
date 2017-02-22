use std::rc::Rc;
use super::*;

#[test]
fn empty_list() {
    assert_eq!(List::new(vec![]).to_vec(), vec![] as Vec<Rc<Post>>);
}

#[test]
fn full_list() {
    assert_eq!(test_util::make_list().to_vec().len(), 6);
}
