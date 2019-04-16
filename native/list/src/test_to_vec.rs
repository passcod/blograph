use super::*;
use post::Post;
use std::sync::Arc;

#[test]
fn empty_list() {
    assert_eq!(List::new(vec![]).to_vec(), vec![] as Vec<Arc<Post>>);
}

#[test]
fn full_list() {
    assert_eq!(test_util::make_list().to_vec().len(), 6);
}
