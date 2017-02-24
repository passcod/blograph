pub fn capitalise(source: &str) -> String {
    let mut chars = source.chars();
    match chars.next() {
        Some(first) => first
            .to_uppercase()
            .collect::<String>()
            + chars.as_str(),
        None => String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lowercase() {
        assert_eq!(&capitalise("lorem ipsum"), "Lorem ipsum");
    }

    #[test]
    fn uppercase() {
        assert_eq!(&capitalise("LOREM IPSUM"), "LOREM IPSUM");
    }

    #[test]
    fn mixedcase() {
        assert_eq!(&capitalise("loReM IpSuM"), "LoReM IpSuM");
    }

    #[test]
    fn symbols() {
        assert_eq!(&capitalise("!*#&@"), "!*#&@");
    }

    #[test]
    fn ideograms() {
        assert_eq!(&capitalise("死亡將是我們最後的敵人"), "死亡將是我們最後的敵人");
    }

    #[test]
    fn single_lowercase() {
        assert_eq!(&capitalise("l"), "L");
    }

    #[test]
    fn single_uppercase() {
        assert_eq!(&capitalise("L"), "L");
    }

    #[test]
    fn single_symbol() {
        assert_eq!(&capitalise("%"), "%");
    }

    #[test]
    fn single_ideogram() {
        assert_eq!(&capitalise("人"), "人");
    }

    #[test]
    fn empty() {
        assert_eq!(&capitalise(""), "");
    }
}
