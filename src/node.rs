use core::fmt;
use std::fmt::Display;

#[allow(dead_code)]
pub enum Tag {
    Div,
    H1,
    H2,
    H3,
    H4,
    Hr,
    Ol,
    Ul,
    Li,
    P,
    Code,
}

impl Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tag::Div => {
                write!(f, "div")
            }
            Tag::H1 => {
                write!(f, "h1")
            }
            Tag::H2 => {
                write!(f, "h2")
            }
            Tag::H3 => {
                write!(f, "h3")
            }
            Tag::H4 => {
                write!(f, "h4")
            }
            Tag::Hr => {
                write!(f, "hr")
            }
            Tag::Ol => {
                write!(f, "h3")
            }
            Tag::Ul => {
                write!(f, "ul")
            }
            Tag::P => {
                write!(f, "p")
            }
            Tag::Li => {
                write!(f, "li")
            }
            Tag::Code => {
                write!(f, "code")
            }
        }
    }
}

#[allow(dead_code)]
pub enum Content {
    InnerText(String),
    InnerContent(Node),
}

impl Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Content::InnerContent(node) => {
                write!(f, "{}", node)
            }
            Content::InnerText(text) => {
                write!(f, "{}", text)
            }
        }
    }
}

pub struct ClassList(pub Vec<String>);

fn optional_attr(attr_label: String, attr_value: String) -> String {
    if attr_value.is_empty() {
        return "".to_string();
    }
    return format!(" {}=\"{}\"", attr_label, attr_value);
}

impl Display for ClassList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if self.0.is_empty() {
                "".to_string()
            } else {
                self.0.join(" ")
            }
        )
    }
}

#[allow(dead_code)]
pub struct Node {
    pub content: Vec<Content>,
    pub tag_name: Tag,
    pub class_list: ClassList,
    pub id: String,
}

pub struct NodeList(pub Vec<Node>);

impl std::fmt::Display for NodeList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let node_vec = &self.0;
        let mut print_string = vec![];

        for node in node_vec {
            print_string.push(format!("{}", node))
        }

        write!(f, "{}", print_string.join("\n"))
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            r#"<{tag_name}{id_tag}{class_tag}>{inner_content}</{tag_name}>"#,
            tag_name = self.tag_name,
            class_tag = optional_attr("class".to_string(), format!("{}", self.class_list)),
            id_tag = optional_attr("id".to_string(), self.id.to_string()),
            inner_content = self
                .content
                .iter()
                .fold("".to_string(), |acc, node| { acc + &format!("{}", node) })
        )
    }
}

#[test]
fn try_header() {
    let header = Node {
        tag_name: Tag::H1,
        content: vec![Content::InnerText("My Header".to_string())],
        class_list: ClassList(vec![]),
        id: "".to_string(),
    };

    let printed = format!("{}", header);
    assert_eq!(printed, "<h1>My Header</h1>")
}
#[test]
fn with_classlist() {
    let my_classlist = ClassList(vec!["class1".to_string(), "class2".to_string()]);
    let printed = format!("{}", my_classlist);
    assert_eq!(printed, "class1 class2".to_string())
}

#[test]
fn node_with_classlist() {
    let header = Node {
        tag_name: Tag::H1,
        content: vec![Content::InnerText("My Header".to_string())],
        class_list: ClassList(vec!["class-1".to_string(), "class-2".to_string()]),
        id: "".to_string(),
    };

    let printed = format!("{}", header);
    assert_eq!(printed, "<h1 class=\"class-1 class-2\">My Header</h1>")
}

#[test]
fn node_with_id() {
    let header = Node {
        tag_name: Tag::H1,
        content: vec![Content::InnerText("My Header".to_string())],
        class_list: ClassList(vec![]),
        id: "header-1".to_string(),
    };

    let printed = format!("{}", header);
    assert_eq!(printed, "<h1 id=\"header-1\">My Header</h1>")
}

#[test]
fn node_with_classlist_and_id() {
    let header = Node {
        tag_name: Tag::H1,
        content: vec![Content::InnerText("My Header".to_string())],
        class_list: ClassList(vec!["class-1".to_string(), "class-2".to_string()]),
        id: "header-1".to_string(),
    };

    let printed = format!("{}", header);
    assert_eq!(
        printed,
        "<h1 id=\"header-1\" class=\"class-1 class-2\">My Header</h1>"
    )
}

#[test]
fn node_with_nested_node() {
    let header = Node {
        tag_name: Tag::H1,
        content: vec![Content::InnerText("My Header".to_string())],
        class_list: ClassList(vec![]),
        id: "header-1".to_string(),
    };

    let div = Node {
        content: vec![Content::InnerContent(header)],
        tag_name: Tag::Div,
        class_list: ClassList(vec![]),
        id: "".to_string(),
    };

    let printed = format!("{}", div);
    assert_eq!(printed, r#"<div><h1 id="header-1">My Header</h1></div>"#)
}
