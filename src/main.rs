mod markdown_parser;
mod node;
use node::{ClassList, Content, Node, Tag};
fn main() {
    let header = Node {
        tag_name: Tag::H1,
        content: vec![Content::InnerText("My Header".to_string())],
        class_list: ClassList(vec!["header".to_string(), "baller".to_string()]),
        id: "".to_string(),
    };

    println!("{}", header)
}
