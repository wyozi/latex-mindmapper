use super::parser::SourceNode;
use std::fmt::Write;

pub fn format_dot(source_nodes: &Vec<SourceNode>) -> String {
    let mut s = String::new();
    writeln!(&mut s, "digraph mygraph {{").unwrap();

    // Set each node's label
    for node in source_nodes.iter().filter(|n| n.label.is_some()) {
        writeln!(&mut s, "\t{} [label=\"{}\"]", node.id, node.label.as_ref().unwrap()).unwrap();
    }

    // Draw links
    for node in source_nodes {
        for par_id in &node.parent_ids {
            writeln!(&mut s, "\t{} -> {}", par_id, node.id).unwrap();
        }
    }


    write!(&mut s, "}}").unwrap();
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        assert_eq!(format_dot(&vec!(
            SourceNode { parent_ids: vec!(), id: "foo".to_string(), label: None }
        )), "digraph mygraph {\n}");
        assert_eq!(format_dot(&vec!(
            SourceNode { parent_ids: vec!(), id: "foo".to_string(), label: Some("yay".to_string()) }
        )), "digraph mygraph {\n\tfoo [label=\"yay\"]\n}");
        assert_eq!(format_dot(&vec!(
            SourceNode { parent_ids: vec!(), id: "foo".to_string(), label: None },
            SourceNode { parent_ids: vec!("foo".to_string()), id: "bar".to_string(), label: None },
        )), "digraph mygraph {\n\tfoo -> bar\n}");
        assert_eq!(format_dot(&vec!(
            SourceNode { parent_ids: vec!(), id: "foo".to_string(), label: None },
            SourceNode { parent_ids: vec!(), id: "bar".to_string(), label: None },
            SourceNode { parent_ids: vec!("foo".to_string(), "bar".to_string()), id: "foobar".to_string(), label: None },
        )), "digraph mygraph {\n\tfoo -> foobar\n\tbar -> foobar\n}");
    }
}