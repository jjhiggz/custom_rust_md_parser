mod markdown_parser;
mod node;

use markdown_parser::MarkdownLine;
use std::fs::read_to_string;

fn main() {
    test_node_print();
}

fn test_node_print() {
    let test_file = read_to_string("./src/data/md-test-file-1.md").unwrap();
    let nodes = MarkdownLine::parse_markdown_to_html(test_file);

    println!("{}", nodes)
}
