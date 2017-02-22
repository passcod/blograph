use std::rc::Rc;
use super::*;

#[test]
fn by_date() {
    assert_eq!(test_util::make_list().sort_by_date().iter().map(|i| {
        i.post.slug()
    }).collect::<Vec<String>>(), vec![
        String::from("2015/jan/25/300-shorts"),
        String::from("2017/jan/03/mountain"),
        String::from("2017/jan/04/there-is-no-such-thing-as-writing-for-adults"),
        String::from("2017/feb/05/some-predictions"),
        String::from("2017/feb/10/monthly-update"),
        String::from("2017/mar/10/monthly-update"),
    ] as Vec<String>);
}
