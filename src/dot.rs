use super::parser::SourceNode;
use std::fmt::Write;

pub fn format_dot(source_nodes: &Vec<SourceNode>) -> String {
    let mut s = String::new();
    writeln!(&mut s, "graph mygraph {{");

    // Set each node's label
    for node in source_nodes.iter().filter(|n| n.label.is_some()) {
        writeln!(&mut s, "\t{} [label=\"{}\"]", node.id, node.label.as_ref().unwrap());
    }

    // Draw links
    for node in source_nodes.iter().filter(|n| n.parent_id.is_some()) {
        writeln!(&mut s, "\t{} -- {}", node.parent_id.as_ref().unwrap(), node.id);
    }


    writeln!(&mut s, "}}");
    s
}