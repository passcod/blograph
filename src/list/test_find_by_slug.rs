use super::*;

#[test]
fn there() {
    let list = test_util::make_list();
    assert_eq!(list.find_by_slug("2015/jan/25/300-shorts").is_some(), true);
}

#[test]
fn not_there() {
    let list = test_util::make_list();
    assert_eq!(list.find_by_slug("1915/jan/25/300-shorts"), None);
}
