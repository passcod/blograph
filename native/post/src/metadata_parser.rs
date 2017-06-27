use regex::Regex;
use yaml_rust::{yaml, Yaml, YamlLoader};

pub fn parse(raw: &str) -> Yaml {
    let (front, _) = split(raw);
    if front.is_none() {
        return Yaml::Hash(yaml::Hash::new());
    }

    match match YamlLoader::load_from_str(&front.unwrap()) {
        Ok(mut doc) => match doc.pop() {
            None => None,
            Some(doc) => match doc {
                Yaml::Hash(_) => Some(doc),
                _ => None
            }
        },
        Err(_) => None
    } {
        Some(doc) => doc,
        None => Yaml::Hash(yaml::Hash::new())
    }
}

pub fn strip(raw: &str) -> String {
    let (_, content) = split(raw);
    content
}

fn split(raw: &str) -> (Option<String>, String) {
    lazy_static! {
        static ref MATTER: Regex = Regex::new(r"(?xm)^---$").unwrap();
    }

    let matters: Vec<&str> = MATTER.splitn(raw, 3).collect();
    match matters.len() {
        3 => match matters[0] {
            "" => (Some(String::from(matters[1])), String::from(matters[2])),
            _ => (None, String::from(matters.join("---")))
        },
        _ => (None, String::from(raw))
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use yaml_rust::{yaml, Yaml};

    #[test]
    fn parse_empty_input() {
        assert_eq!(parse(""), Yaml::Hash(yaml::Hash::new()));
    }

    #[test]
    fn parse_no_front_matter() {
        assert_eq!(parse("Foo\nBar\nbazzzz"), Yaml::Hash(yaml::Hash::new()));
    }

    #[test]
    fn parse_dashes_after_the_top() {
        assert_eq!(parse("Blah blah\n---\n---"), Yaml::Hash(yaml::Hash::new()));
    }

    #[test]
    fn parse_matter_after_spaces() {
        assert_eq!(parse("  ---\npage: true\n---"), Yaml::Hash(yaml::Hash::new()));
    }

    #[test]
    fn parse_matter_after_newline() {
        assert_eq!(parse("\n---\npage: true\n---"), Yaml::Hash(yaml::Hash::new()));
    }

    #[test]
    fn parse_empty_front_matter() {
        assert_eq!(parse("---\n---"), Yaml::Hash(yaml::Hash::new()));
    }

    #[test]
    fn parse_empty_front_matter_with_vim_modeline() {
        assert_eq!(parse("---\n# vim: tw=80\n---"), Yaml::Hash(yaml::Hash::new()));
    }

    #[test]
    fn parse_array_front_matter() {
        assert_eq!(parse("---\n- foo\n- bar\n---"), Yaml::Hash(yaml::Hash::new()));
    }

    #[test]
    fn parse_string_front_matter() {
        assert_eq!(parse("---\nstring\n---"), Yaml::Hash(yaml::Hash::new()));
    }

    #[test]
    fn parse_hash_front_matter() {
        let mut matter = yaml::Hash::new();
        matter.insert(
            Yaml::String(String::from("page")),
            Yaml::Boolean(true)
        );
        matter.insert(
            Yaml::String(String::from("tags")),
            Yaml::Array(vec![
                Yaml::String(String::from("post"))
            ])
        );

        assert_eq!(
            parse("---\npage: true\ntags:\n  - post\n---"),
            Yaml::Hash(matter)
        );
    }

    #[test]
    fn parse_hash_front_matter_with_3space_indent() {
        let mut matter = yaml::Hash::new();
        matter.insert(
            Yaml::String(String::from("page")),
            Yaml::Boolean(true)
        );
        matter.insert(
            Yaml::String(String::from("tags")),
            Yaml::Array(vec![
                Yaml::String(String::from("post"))
            ])
        );

        assert_eq!(
            parse("---\npage: true\ntags:\n   - post\n---"),
            Yaml::Hash(matter)
        );
    }

    #[test]
    fn parse_hash_front_matter_with_vim_modeline() {
        let mut matter = yaml::Hash::new();
        matter.insert(
            Yaml::String(String::from("page")),
            Yaml::Boolean(true)
        );
        matter.insert(
            Yaml::String(String::from("tags")),
            Yaml::Array(vec![
                Yaml::String(String::from("post"))
            ])
        );

        assert_eq!(
            parse("---\n# vim: tw=80\npage: true\ntags:\n  - post\n---"),
            Yaml::Hash(matter)
        );
    }

    #[test]
    fn strip_empty_input() {
        assert_eq!(strip(""), String::from(""));
    }

    #[test]
    fn strip_no_front_matter() {
        assert_eq!(strip("Foo\nBar\nbazzzz"),
            String::from("Foo\nBar\nbazzzz"));
    }

    #[test]
    fn strip_dashes_after_the_top() {
        assert_eq!(strip("Blah blah\n---\n---"),
            String::from("Blah blah\n---\n---"));
    }

    #[test]
    fn strip_matter_after_spaces() {
        assert_eq!(strip("  ---\npage: true\n---"),
            String::from("  ---\npage: true\n---"));
    }

    #[test]
    fn strip_matter_after_newline() {
        assert_eq!(strip("\n---\npage: true\n---"),
            String::from("\n---\npage: true\n---"));
    }

    #[test]
    fn strip_empty_front_matter() {
        assert_eq!(strip("---\n---\nipsum"),
            String::from("\nipsum"));
    }

    #[test]
    fn strip_front_matter() {
        assert_eq!(strip("---\npage: true\n---\nlorem"),
            String::from("\nlorem"));
    }
}
