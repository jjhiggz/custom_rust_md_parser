use std::fmt::Display;

enum Content {
    InnerText(String),
    InnerContent(Node),
}

#[allow(dead_code)]
struct Node {
    content: Vec<Content>,
    tag_name: String,
    class_list: ClassList,
    id: String,
}

struct ClassList(Vec<String>);

impl Display for ClassList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if self.0.len() == 0 {
                "".to_string()
            } else {
                self.0.join(" ")
            }
        )
    }
}

fn optional_attr(attr_label: String, attr_value: String) -> String {
    if attr_value.len() == 0 {
        return "".to_string();
    }
    return format!(" {}=\"{}\"", attr_label, attr_value);
}

impl Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            &Content::InnerContent(node) => {
                write!(f, "{}", format!("{}", node))
            }
            &Content::InnerText(text) => {
                write!(f, "{}", format!("{}", text))
            }
        }
    }
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            r#"<{tag_name}{id_tag}{class_tag}>{inner_content}</{tag_name}>"#,
            tag_name = self.tag_name,
            class_tag = optional_attr("class".to_string(), format!("{}", self.class_list)),
            id_tag = optional_attr("id".to_string(), format!("{}", self.id)),
            inner_content = self
                .content
                .iter()
                .fold("".to_string(), |acc, node| { acc + &format!("{}", node) })
        )
    }
}

fn main() {
    let header = Node {
        tag_name: "h1".to_string(),
        content: vec![Content::InnerText("My Header".to_string())],
        class_list: ClassList(vec!["header".to_string(), "baller".to_string()]),
        id: "".to_string(),
    };

    println!("{}", header)
}

#[test]
fn try_header() {
    let header = Node {
        tag_name: "h1".to_string(),
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
        tag_name: "h1".to_string(),
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
        tag_name: "h1".to_string(),
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
        tag_name: "h1".to_string(),
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
        tag_name: "h1".to_string(),
        content: vec![Content::InnerText("My Header".to_string())],
        class_list: ClassList(vec![]),
        id: "header-1".to_string(),
    };

    let div = Node {
        content: vec![Content::InnerContent(header)],
        tag_name: "div".to_string(),
        class_list: ClassList(vec![]),
        id: "".to_string(),
    };

    let printed = format!("{}", div);
    assert_eq!(printed, "<div><h1 id=\"header-1\">My Header</h1></div>")
}
