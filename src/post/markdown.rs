use crowbook_text_processing::clean::{ellipsis, guillemets, quotes};
use pulldown_cmark::{html, Options, OPTION_ENABLE_FOOTNOTES, OPTION_ENABLE_TABLES, Parser};

fn typo(text: &str) -> String {
	String::from(ellipsis(guillemets(quotes(text))))
}

pub fn render(md: &str) -> String {
    lazy_static! {
        static ref OPTS: Options = Options::from_bits_truncate(
            OPTION_ENABLE_FOOTNOTES.bits() |
            OPTION_ENABLE_TABLES.bits()
        );
    }

	let pretty = typo(md);
    let mut rendered = String::with_capacity(pretty.len() * 3/2);
    let parser = Parser::new_ext(&pretty, OPTS.clone());
    html::push_html(&mut rendered, parser);
    rendered
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_md(md: &str, html: &str) {
        assert_eq!(render(md), html);
    }

    #[test]
    fn paragraph() {
        test_md("Hello world", "<p>Hello world</p>\n");
    }

    #[test]
    fn ellipsis() {
        test_md("Hello...", "<p>Hello…</p>\n");
    }

    #[test]
    fn guillemets() {
        test_md("<<Hello world>>", "<p>«Hello world»</p>\n");
    }

    #[test]
    fn quotes() {
        test_md("\"Hello 'Hi' world\"", "<p>“Hello ‘Hi’ world”</p>\n");
    }

    #[test]
    fn tables() {
        test_md(
            "Hello | world\n------|------\nRow | here",
            "<table><thead><tr><td>Hello </td><td> world</td></tr></thead>\n<tr><td>Row </td><td> here</td></tr>\n</table>\n"
        );
    }

    #[test]
    fn footnotes() {
        test_md(
            "Lorem ipsum.[^a]\n[^a]: Cool.",
            "<p>Lorem ipsum.<sup class=\"footnote-reference\"><a href=\"#a\">1</a></sup>\n<sup class=\"footnote-reference\"><a href=\"#a\">1</a></sup>: Cool.</p>\n"
        );
    }
}