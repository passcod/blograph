use super::*;

#[test]
fn of_one() {
    let list = test_util::make_list();
    let item = list.find_by_slug("2017/mar/10/monthly-update").unwrap();

    assert_eq!(list.parents_of(item).iter().map(|i| {
        i.post.slug()
    }).collect::<Vec<String>>(), vec![
        String::from("2017/feb/10/monthly-update"),
    ] as Vec<String>);
}
