#![allow(unused_imports, dead_code)]

use core::slice;
use std::fmt::{format, Display};

use regex::Regex;

use crate::node::{ClassList, Content, Node, NodeList, Tag};

#[derive(Clone)]
pub enum MarkDownLineType {
    H1,
    H2,
    H3,
    H4,
    Li,
    NoTag,
    EmptyLine,
}

fn split_by_newline(input: String) -> Vec<String> {
    input
        .split("\n")
        .filter(|&s| s.trim() != "")
        .map(|s| s.trim().to_string())
        .collect()
}

#[derive(Clone)]
pub struct MarkdownLine {
    indent: i32,
    line_type: MarkDownLineType,
    content: String,
}

impl MarkdownLine {
    fn assign_node(md_line: MarkdownLine) -> Node {
        match md_line.line_type {
            MarkDownLineType::H1 => Node {
                class_list: ClassList(vec!["md-h1-container".to_string()]),
                content: vec![
                    Content::InnerContent(Node {
                        class_list: ClassList(vec!["md-h1".to_string()]),
                        id: "".to_string(),
                        content: vec![Content::InnerText(md_line.content)], // tag_name: Tag::H1,
                        tag_name: Tag::H1,
                    }),
                    Content::InnerContent(Node {
                        class_list: ClassList(vec!["md-hr".to_string()]),
                        id: "".to_string(),
                        content: vec![Content::InnerText("".to_string())], // tag_name: Tag::H1,
                        tag_name: Tag::Hr,
                    }),
                ],
                id: "".to_string(),
                tag_name: Tag::Div,
            },
            MarkDownLineType::H2 => Node {
                class_list: ClassList(vec!["md-h2".to_string()]),
                content: vec![Content::InnerText(md_line.content)],
                id: "".to_string(),
                tag_name: Tag::H2,
            },
            MarkDownLineType::H3 => Node {
                class_list: ClassList(vec!["md-h3".to_string()]),
                content: vec![Content::InnerText(md_line.content)],
                id: "".to_string(),
                tag_name: Tag::H3,
            },
            MarkDownLineType::H4 => Node {
                class_list: ClassList(vec!["md-h4".to_string()]),
                content: vec![Content::InnerText(md_line.content)],
                id: "".to_string(),
                tag_name: Tag::H4,
            },
            MarkDownLineType::Li => Node {
                class_list: ClassList(vec!["md-li".to_string()]),
                content: vec![Content::InnerText(md_line.content)],
                id: "".to_string(),
                tag_name: Tag::Li,
            },
            MarkDownLineType::NoTag => Node {
                class_list: ClassList(vec!["md-p".to_string()]),
                content: vec![Content::InnerText(md_line.content)],
                id: "".to_string(),
                tag_name: Tag::P,
            },
            MarkDownLineType::EmptyLine => Node {
                class_list: ClassList(vec!["md-empty-line".to_string()]),
                content: vec![Content::InnerText("".to_string())],
                id: "".to_string(),
                tag_name: Tag::Div,
            },
        }
    }

    pub fn create_node_list(lines: String) -> NodeList {
        let md_lines = MarkdownLine::get_md_lines(lines);
        let node_vec = md_lines
            .iter()
            .map(|md_line| MarkdownLine::assign_node(md_line.clone()))
            .collect();

        NodeList(node_vec)
    }

    pub fn parse_markdown_to_html(lines: String) -> String {
        format!("{}", MarkdownLine::create_node_list(lines))
    }

    fn get_indent(line: String) -> i32 {
        let first_char_position = line
            .split("")
            .position(|x| Regex::new(r"[^\s-]").unwrap().is_match(x));

        match first_char_position {
            Some(n) => (n / 4) as i32,
            None => -1,
        }
    }

    fn get_md_lines(lines: String) -> Vec<MarkdownLine> {
        lines
            .split("\n")
            .map(|line| MarkdownLine::parse(line.to_string()))
            .collect()
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
        if indent == 0 {
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

    #[test]
    fn get_md_lines() {
        let test_file = fs::read_to_string("./src/data/md-test-file-1.md").unwrap();
        let md_lines = MarkdownLine::get_md_lines(test_file);

        let header = &md_lines[0];
        let blank1 = &md_lines[1];
        let my_other_header = &md_lines[2];
        let blank2 = &md_lines[3];
        let item_1 = &md_lines[4];
        let item_2 = &md_lines[5];

        // make sure content is correct
        assert!(header.content == "My Header");
        assert!(blank1.content == "");
        assert!(my_other_header.content == "My Other Header");
        assert!(blank2.content == "");
        assert!(item_1.content == "item1");
        assert!(item_2.content == "item2");

        // make sure types are correct
        assert!(match header.line_type {
            MarkDownLineType::H1 => true,
            _ => false,
        });

        assert!(match blank1.line_type {
            MarkDownLineType::EmptyLine => true,
            _ => false,
        });

        assert!(match my_other_header.line_type {
            MarkDownLineType::H2 => true,
            _ => false,
        });
        assert!(match blank2.line_type {
            MarkDownLineType::EmptyLine => true,
            _ => false,
        });
        assert!(match item_1.line_type {
            MarkDownLineType::Li => true,
            _ => false,
        });
        assert!(match item_2.line_type {
            MarkDownLineType::Li => true,
            _ => false,
        });
    }

    #[test]
    fn test_markdown_parser() {
        let test_file = fs::read_to_string("./src/data/overall-test-file.md").unwrap();
        let test_html_file = fs::read_to_string("./src/data/overall-test-file.html").unwrap();
        let val = MarkdownLine::parse_markdown_to_html(test_file);
        assert_eq!(val, test_html_file)
    }
}
