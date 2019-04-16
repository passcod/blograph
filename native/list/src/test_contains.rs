use super::*;

#[test]
fn there() {
    let list = test_util::make_list();
    let item = list.find_by_slug("2015/jan/25/300-shorts").unwrap();

    assert_eq!(list.contains(&item), true);
}

#[test]
fn not_there() {
    let all = test_util::make_list();
    let item = all.find_by_slug("2015/jan/25/300-shorts").unwrap();
    let list = all
        .iter()
        .filter(|i| !i.post.metadata.tags().iter().any(|t| t == "300-shorts"))
        .collect::<List>();

    assert_eq!(list.contains(&item), false);
}
