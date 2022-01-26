use std::fmt::Display;

#[allow(dead_code)]
struct Node {
    inner_content: Box<Vec<Node>>,
    inner_text: String,
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

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let show_class = self.class_list.0.len() > 0;
        let show_id = self.id.len() > 0;

        write!(
            f,
            "<{tag_name}{id_tag}{id}{id_close}{class_tag}{class_list}{class_close}>{inner_text}</{tag_name}>",
            tag_name = self.tag_name,
            inner_text = self.inner_text,
            class_tag = if show_class {
                " class=\"".to_string()
            } else {
                "".to_string()
            },
            class_close = if show_class { "\"" } else { "" },
            class_list = self.class_list,
            id_tag = if show_id {
                " id=\"".to_string()
            } else {
                "".to_string()
            },
            id_close = if show_id { "\"" } else { "" },
            id = self.id,
        )
    }
}

fn main() {
    let header = Node {
        inner_content: Box::new(vec![]),
        tag_name: "h1".to_string(),
        inner_text: "My Header".to_string(),
        class_list: ClassList(vec!["header".to_string(), "baller".to_string()]),
        id: "".to_string(),
    };

    println!("{}", header)
}

#[test]
fn try_header() {
    let header = Node {
        inner_content: Box::new(vec![]),
        tag_name: "h1".to_string(),
        inner_text: "My Header".to_string(),
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
        inner_content: Box::new(vec![]),
        tag_name: "h1".to_string(),
        inner_text: "My Header".to_string(),
        class_list: ClassList(vec!["class-1".to_string(), "class-2".to_string()]),
        id: "".to_string(),
    };

    let printed = format!("{}", header);
    assert_eq!(printed, "<h1 class=\"class-1 class-2\">My Header</h1>")
}

#[test]
fn node_with_id() {
    let header = Node {
        inner_content: Box::new(vec![]),
        tag_name: "h1".to_string(),
        inner_text: "My Header".to_string(),
        class_list: ClassList(vec![]),
        id: "header-1".to_string(),
    };

    let printed = format!("{}", header);
    assert_eq!(printed, "<h1 id=\"header-1\">My Header</h1>")
}

#[test]
fn node_with_classlist_and_id() {
    let header = Node {
        inner_content: Box::new(vec![]),
        tag_name: "h1".to_string(),
        inner_text: "My Header".to_string(),
        class_list: ClassList(vec!["class-1".to_string(), "class-2".to_string()]),
        id: "header-1".to_string(),
    };

    let printed = format!("{}", header);
    assert_eq!(
        printed,
        "<h1 id=\"header-1\" class=\"class-1 class-2\">My Header</h1>"
    )
}

#[allow(dead_code)]
enum MyOptions {
    A,
    RGB(i32, i32, i32),
    C,
}

#[allow(dead_code)]
fn choose_option(choice: MyOptions) {
    match choice {
        MyOptions::A => {}
        MyOptions::RGB(a, b, c) => {
            println!("{}{}{}", a, b, c)
        }
        _ => {
            println! {}
        }
    }
}
