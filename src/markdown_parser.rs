#![allow(unused_imports, dead_code)]

use core::slice;
// use math::round::floor

use regex::Regex;

struct MarkDownGroup {
    line_count: i32,
    structure: MarkDownStructureType,
}

#[derive(PartialEq)]
enum MarkDownLineType {
    H1,
    H2,
    H3,
    H4,
    Ol,
    Ul,
    Li,
}
enum MarkDownStructureType {
    H1,
    H2,
    H3,
    H4,
    Ol,
    Ul,
    Li,
}

#[derive(PartialEq)]
struct MarkdownLine {
    indent: i32,
    lineType: MarkDownLineType,
    content: String,
}

impl MarkdownLine {
    pub fn parse(line: String) -> MarkdownLine {
        let split = line.split("");
        let indent_pos = split
            .clone()
            .position(|x| {
                let whitespace_re = Regex::new(r"\s").unwrap();
                whitespace_re.is_match(x)
            })
            .unwrap();
        let content = slice::from_ref(&line)
            .get(indent_pos..line.len() - 1)
            .unwrap()
            .join("");

        let indent: i32 = ((indent_pos / 4) as f32).floor() as i32;

        MarkdownLine {
            lineType: MarkDownLineType::H1,
            content: content,
            indent: indent,
        }
    }
}

fn split_by_newline(input: String) -> Vec<String> {
    input
        .split("\n")
        .filter(|&s| s.trim() != "")
        .map(|s| s.trim().to_string())
        .collect()
}

mod tests {
    use std::fs;

    use crate::markdown_parser::split_by_newline;
    use crate::markdown_parser::MarkdownLine;

    fn md_test_file_1() -> String {
        fs::read_to_string("src/data/md-test-file-1.md").unwrap()
    }

    #[test]
    fn t_split_by_newline() {
        let split_by_line_md = fs::read_to_string("src/data/split_by_line.md").unwrap();
        assert_eq!(split_by_newline(split_by_line_md), vec!["1", "22", "333"]);
    }

    // #[test]
    // fn markdown_line_parse() {
    //     let test_data = md_test_file_1();
    //     assert_eq!(
    //         MarkdownLine::parse("# header".to_string()),
    //         "hello".to_string() // MarkdownLine { indent }
    //     );
    // }
}
