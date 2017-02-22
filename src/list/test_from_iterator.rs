use super::*;

#[test]
fn from_rc_post() {
    assert_eq!(test_util::make_list().iter().map(|item| {
        item.post
    }).collect::<List>().len(), 6);
}

#[test]
fn from_item() {
    assert_eq!(test_util::make_list().iter().map(|item| {
        item
    }).collect::<List>().len(), 6);
}
