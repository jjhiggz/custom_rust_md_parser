#![allow(unused_imports, dead_code)]

use core::slice;
use std::fmt::Display;
// use math::round::floor

use regex::Regex;

struct MarkDownGroup {
    line_count: i32,
    structure: MarkDownStructureType,
}

pub enum MarkDownLineType {
    H1,
    H2,
    H3,
    H4,
    Li,
    NoTag,
    EmptyLine,
}

pub enum MarkDownStructureType {
    H1,
    H2,
    H3,
    H4,
    Ol,
    Ul,
    Li,
}

struct MarkdownLine {
    indent: i32,
    line_type: MarkDownLineType,
    content: String,
}

impl MarkdownLine {
    fn get_indent(line: String) -> i32 {
        let first_char_position = line
            .split("")
            .position(|x| Regex::new(r"[^\s-]").unwrap().is_match(x));

        match first_char_position {
            Some(n) => (n / 4) as i32,
            None => -1,
        }
    }

    fn get_header_tag(first_non_tag_char_pos: i32) -> MarkDownLineType {
        match first_non_tag_char_pos {
            2 => MarkDownLineType::H1,
            3 => MarkDownLineType::H2,
            4 => MarkDownLineType::H3,
            5 => MarkDownLineType::H4,
            _ => MarkDownLineType::NoTag,
        }
    }

    fn get_tag(line: String, indent: i32) -> MarkDownLineType {
        if indent == -1 {
            return MarkDownLineType::EmptyLine;
        };

        if indent > 0 {
            return MarkDownLineType::NoTag;
        };

        let trimmed = line.trim();
        let split = trimmed.split("");
        let first_tag_regex = Regex::new(r"[#|-]").unwrap();
        let first_tag_position = split.clone().position(|x| first_tag_regex.is_match(x));
        let first_tag = split.clone().find(|x| first_tag_regex.is_match(x));

        let first_non_tag_regex = Regex::new(r"[^#|^-]").unwrap();
        let first_non_tag_char_pos = split.clone().position(|x| first_non_tag_regex.is_match(x));

        if first_non_tag_char_pos == None {
            return MarkDownLineType::NoTag;
        };

        let first_non_tag_char = trimmed.chars().nth(first_non_tag_char_pos.unwrap() - 1);

        match first_non_tag_char {
            None => return MarkDownLineType::EmptyLine,
            Some(char) => {
                if char.to_string() != " ".to_string() {
                    return MarkDownLineType::NoTag;
                }
            }
        };

        if first_tag.unwrap() == "#" {
            return MarkdownLine::get_header_tag(first_non_tag_char_pos.unwrap() as i32);
        } else {
            let second_hyphen = trimmed.chars().nth(first_tag_position.unwrap() as usize);
            if second_hyphen == Some('-') {
                return MarkDownLineType::NoTag;
            }
            return MarkDownLineType::Li;
        }
    }

    fn get_content(line: String, line_type: &MarkDownLineType, indent: i32) -> String {
        let len = line.len();
        if (indent == 0) {
            return match &line_type {
                MarkDownLineType::H1 => line[2..len].to_string(),
                MarkDownLineType::H2 => line[3..len].to_string(),
                MarkDownLineType::H3 => line[4..len].to_string(),
                MarkDownLineType::H4 => line[5..len].to_string(),
                MarkDownLineType::Li => line[2..len].to_string(),
                _ => line,
            };
        }
        if indent != 0 {
            let starting_index = 2 + 4 * indent as usize;
            println!("indent{}", indent);
            println!("starting index {}", starting_index);
            return match &line_type {
                MarkDownLineType::Li => {
                    println!("is li");
                    line[starting_index..len].to_string()
                }
                _ => line,
            };
        }
        unreachable!()
    }

    pub fn parse(line: String) -> MarkdownLine {
        let indent = MarkdownLine::get_indent(line.clone());
        if indent == -1 {
            return MarkdownLine {
                content: "".to_string(),
                indent: -1,
                line_type: MarkDownLineType::EmptyLine,
            };
        };
        let tag = MarkdownLine::get_tag(line.clone(), indent);
        let content = MarkdownLine::get_content(line, &tag, indent);

        return MarkdownLine {
            content: content,
            indent: indent,
            line_type: tag,
        };
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
    use crate::markdown_parser::{MarkDownLineType, MarkdownLine};

    fn md_test_file_1() -> String {
        fs::read_to_string("src/data/md-test-file-1.md").unwrap()
    }

    #[test]
    fn t_split_by_newline() {
        let split_by_line_md = fs::read_to_string("src/data/split_by_line.md").unwrap();
        assert_eq!(split_by_newline(split_by_line_md), vec!["1", "22", "333"]);
    }

    #[test]
    fn get_indent() {
        let line = "hello".to_string();
        let indent = MarkdownLine::get_indent(line);
        assert_eq!(indent, 0);
        let line = "  hello".to_string();
        let indent = MarkdownLine::get_indent(line);
        assert_eq!(indent, 0);
        let line = "   hello".to_string();
        let indent = MarkdownLine::get_indent(line);
        assert_eq!(indent, 1);
        let line = "".to_string();
        let indent = MarkdownLine::get_indent(line);
        assert_eq!(indent, -1);
    }

    #[test]
    fn get_tag() {
        let line = "hello".to_string();
        let indent = 0;
        let tag = MarkdownLine::get_tag(line, indent);
        assert!(match tag {
            MarkDownLineType::NoTag => true,
            _ => false,
        });

        let line = " hello".to_string();
        let indent = 0;
        let tag = MarkdownLine::get_tag(line, indent);
        assert!(match tag {
            MarkDownLineType::NoTag => true,
            _ => false,
        });

        let line = "#hello".to_string();
        let indent = 0;
        let tag = MarkdownLine::get_tag(line, indent);
        assert!(match tag {
            MarkDownLineType::NoTag => true,
            _ => false,
        });

        let line = "# hello".to_string();
        let indent = 0;
        let tag = MarkdownLine::get_tag(line, indent);
        assert!(match tag {
            MarkDownLineType::H1 => true,
            _ => false,
        });

        let line = "## hello".to_string();
        let indent = 0;
        let tag = MarkdownLine::get_tag(line, indent);
        assert!(match tag {
            MarkDownLineType::H2 => true,
            _ => false,
        });

        let line = "#### hello".to_string();
        let indent = 0;
        let tag = MarkdownLine::get_tag(line, indent);

        assert!(match tag {
            MarkDownLineType::H4 => true,
            _ => false,
        });

        let line = "##### hello".to_string();
        let indent = 0;
        let tag = MarkdownLine::get_tag(line, indent);

        assert!(match tag {
            MarkDownLineType::NoTag => true,
            _ => false,
        });

        let line = "- ".to_string();
        let indent = 0;
        let tag = MarkdownLine::get_tag(line, indent);

        assert!(match tag {
            MarkDownLineType::NoTag => true,
            _ => false,
        });

        let line = "- bullet".to_string();
        let indent = 0;
        let tag = MarkdownLine::get_tag(line, indent);

        assert!(match tag {
            MarkDownLineType::Li => true,
            _ => false,
        });

        let line = "-- bullet".to_string();
        let indent = 0;
        let tag = MarkdownLine::get_tag(line, indent);

        assert!(match tag {
            MarkDownLineType::NoTag => true,
            _ => false,
        });

        let line = "--- bullet".to_string();
        let indent = 0;
        let tag = MarkdownLine::get_tag(line, indent);

        assert!(match tag {
            MarkDownLineType::NoTag => true,
            _ => false,
        });

        let line = "- my bullet".to_string();
        let indent = 0;
        let tag = MarkdownLine::get_tag(line, indent);

        assert!(match tag {
            MarkDownLineType::Li => true,
            _ => false,
        });
    }

    #[test]
    fn get_content() {
        let line = "# header".to_string();
        let line_type = MarkDownLineType::H1;
        let indent = 0;
        let content = MarkdownLine::get_content(line, &line_type, indent);
        assert_eq!(content, "header");

        let line = "## header".to_string();
        let line_type = MarkDownLineType::H2;
        let indent = 0;
        let content = MarkdownLine::get_content(line, &line_type, indent);
        assert_eq!(content, "header");
    }

    #[test]
    fn parse() {
        let line = "hey this is a line".to_string();
        let md_line = MarkdownLine::parse(line);
        assert_eq!(md_line.content, "hey this is a line".to_string());
        assert_eq!(md_line.indent, 0);
        assert!(match md_line.line_type {
            MarkDownLineType::NoTag => true,
            _ => false,
        });

        let line = "#hey this is a line".to_string();
        let md_line = MarkdownLine::parse(line);
        assert_eq!(md_line.content, "#hey this is a line".to_string());
        assert_eq!(md_line.indent, 0);
        assert!(match md_line.line_type {
            MarkDownLineType::NoTag => true,
            _ => false,
        });

        let line = "# hey this is a line".to_string();
        let md_line = MarkdownLine::parse(line);
        assert_eq!(md_line.content, "hey this is a line".to_string());
        assert_eq!(md_line.indent, 0);
        assert!(match md_line.line_type {
            MarkDownLineType::H1 => true,
            _ => false,
        });

        let line = "## hey this is a line".to_string();
        let md_line = MarkdownLine::parse(line);

        assert_eq!(md_line.content, "hey this is a line".to_string());
        assert_eq!(md_line.indent, 0);
        assert!(match md_line.line_type {
            MarkDownLineType::H2 => true,
            _ => false,
        });

        let line = "".to_string();
        let md_line = MarkdownLine::parse(line);

        assert_eq!(md_line.content, "".to_string());
        assert_eq!(md_line.indent, -1);
        assert!(match md_line.line_type {
            MarkDownLineType::EmptyLine => true,
            _ => false,
        });

        let line = "- mybullet".to_string();
        let md_line = MarkdownLine::parse(line);

        assert_eq!(md_line.content, "mybullet".to_string());
        assert_eq!(md_line.indent, 0);
        assert!(match md_line.line_type {
            MarkDownLineType::Li => true,
            _ => false,
        });
    }

    // #[test]
    // fn markdown_line_parse_simple_h1() {
    //     // let test_data = md_test_file_1();
    //     let parsed_header = MarkdownLine::parse("# header".to_string());
    //     assert_eq!(parsed_header.content, "header".to_string());
    //     assert!(match parsed_header.line_type {
    //         MarkDownLineType::H1 => true,
    //         _ => false,
    //     });
    //     assert_eq!(parsed_header.indent, 0);

    //     println!("{}", parsed_header.content);
    // }

    // #[test]
    // fn markdown_line_parse_simple_h2() {
    //     // let test_data = md_test_file_1();
    //     let parsed_header = MarkdownLine::parse("## header".to_string());
    //     assert_eq!(parsed_header.content, "header".to_string());
    //     assert!(match parsed_header.line_type {
    //         MarkDownLineType::H2 => true,
    //         _ => false,
    //     });
    //     assert_eq!(parsed_header.indent, 0);

    //     println!("{}", parsed_header.content);
    // }

    // #[test]
    // fn markdown_line_parse_simple_untagged() {
    //     let parsed_header = MarkdownLine::parse("This is not a tag".to_string());
    //     assert!(match parsed_header.line_type {
    //         MarkDownLineType::NoTag => true,
    //         _ => false,
    //     });

    //     assert_eq!(parsed_header.indent, 0);

    //     println!("{}", parsed_header.content);
    // }
}
