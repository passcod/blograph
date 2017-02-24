use std::sync::Arc;
use super::*;

#[test]
fn empty_list() {
    assert_eq!(List::new(vec![]).iter().next(), None);
}

#[test]
fn list_map() {
    assert_eq!(test_util::make_list().iter().map(|i| {
        i.post.slug()
    }).collect::<Vec<String>>(), vec![
        String::from("2015/jan/25/300-shorts"),
        String::from("2017/feb/05/some-predictions"),
        String::from("2017/jan/03/mountain"),
        String::from("2017/jan/04/there-is-no-such-thing-as-writing-for-adults"),
        String::from("2017/feb/10/monthly-update"),
        String::from("2017/mar/10/monthly-update"),
    ] as Vec<String>);
}

#[test]
fn previous() {
    assert_eq!(test_util::make_list().iter().map(|i| {
        i.previous.map(|p| p.slug())
    }).collect::<Vec<Option<String>>>(), vec![
        None,
        Some(String::from("2015/jan/25/300-shorts")),
        Some(String::from("2017/feb/05/some-predictions")),
        Some(String::from("2017/jan/03/mountain")),
        Some(String::from("2017/jan/04/there-is-no-such-thing-as-writing-for-adults")),
        Some(String::from("2017/feb/10/monthly-update")),
    ] as Vec<Option<String>>);
}

#[test]
fn next() {
    assert_eq!(test_util::make_list().iter().map(|i| {
        i.next.map(|p| p.slug())
    }).collect::<Vec<Option<String>>>(), vec![
        Some(String::from("2017/feb/05/some-predictions")),
        Some(String::from("2017/jan/03/mountain")),
        Some(String::from("2017/jan/04/there-is-no-such-thing-as-writing-for-adults")),
        Some(String::from("2017/feb/10/monthly-update")),
        Some(String::from("2017/mar/10/monthly-update")),
        None,
    ] as Vec<Option<String>>);
}

#[test]
fn no_copy() {
    let list = test_util::make_list();
    let list_item = list.find_by_slug("2015/jan/25/300-shorts").unwrap();
    assert_eq!(Arc::strong_count(list_item), 1);

    let mut iter = list.iter();
    let iter_item = iter.next().unwrap().post;
    assert_eq!(Arc::strong_count(list_item), 2);

    assert_eq!(list_item, &iter_item);
}
