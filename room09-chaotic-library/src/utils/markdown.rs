pub fn new_markdown_parser() -> markdown_it::MarkdownIt {
    let mut markdown_parser = markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(&mut markdown_parser);
    markdown_it::plugins::extra::add(&mut markdown_parser);
    markdown_parser
}
