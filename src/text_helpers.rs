use dom_query::{NodeData, NodeId, NodeRef};

use html5ever::{local_name, QualName};
use tendril::StrTendril;

enum SerializeOp {
    Open(NodeId),
    Close(QualName),
}

pub(crate) fn format_text(root_node: &NodeRef) -> StrTendril {
    let id = root_node.id;
    let tree = root_node.tree;
    let mut ops: Vec<_> = tree
        .child_ids_of_it(&id, true)
        .map(SerializeOp::Open)
        .collect();

    let mut text = StrTendril::new();

    while let Some(op) = ops.pop() {
        match op {
            SerializeOp::Open(id) => {
                let node = NodeRef::new(id, tree);
                node.query(|n| match n.data {
                    NodeData::Text { ref contents } => {
                        if contents.is_empty() {
                            return;
                        }
                        let follows_newline = text.ends_with('\n') || text.is_empty();
                        let normalized = normalize_text(contents.as_ref(), follows_newline);
                        text.push_tendril(&normalized);
                    }
                    NodeData::Element(ref e) => {
                        ops.push(SerializeOp::Close(e.name.clone()));

                        if matches!(e.name.local, local_name!("pre")) {
                            text.push_tendril(&node.text());
                            return;
                        }

                        ops.extend(tree.child_ids_of_it(&id, true).map(SerializeOp::Open));
                    }
                    _ => {}
                });
            }
            SerializeOp::Close(name) => {
                if text.ends_with("\n\n") {
                    continue;
                }
                if matches!(
                    name.local,
                    local_name!("article")
                        | local_name!("section")
                        | local_name!("div")
                        | local_name!("p")
                        | local_name!("pre")
                        | local_name!("h1")
                        | local_name!("h2")
                        | local_name!("h3")
                        | local_name!("h4")
                        | local_name!("h5")
                        | local_name!("h6")
                        | local_name!("ul")
                        | local_name!("ol")
                        | local_name!("table")
                ) {
                    text.push_slice("\n\n");
                } else if matches!(
                    name.local,
                    local_name!("br") | local_name!("hr") | local_name!("li") | local_name!("tr")
                ) {
                    text.push_char('\n');
                }
            }
        }
    }
    while !text.is_empty() && text.ends_with(char::is_whitespace) {
        text.pop_back(1);
    }
    text
}

fn normalize_text(text: &str, follows_newline: bool) -> StrTendril {
    let push_start_whitespace = !follows_newline && text.starts_with(char::is_whitespace);
    let push_end_whitespace = text.ends_with(char::is_whitespace);

    let mut result = StrTendril::with_capacity(text.len() as u32);
    let mut iter = text.split_whitespace();

    if let Some(first) = iter.next() {
        if push_start_whitespace {
            result.push_char(' ');
        }
        result.push_slice(first);
        for word in iter {
            result.push_char(' ');
            result.push_slice(word);
        }
    }
    if result.is_empty() {
        return result;
    }
    if push_end_whitespace {
        result.push_char(' ');
    }
    result
}
