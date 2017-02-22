use super::*;

#[test]
fn empty_list() {
    assert_eq!(List::new(vec![]).len(), 0);
}

#[test]
fn full_list() {
    assert_eq!(test_util::make_list().len(), 6);
}

#[test]
fn partial_list() {
    assert_eq!(test_util::make_list().iter().filter(|i| {
        i.post.metadata.tags().iter().any(|t| t == "monthly-update")
    }).collect::<List>().len(), 2);
}
