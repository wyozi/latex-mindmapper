use super::parser::SourceNode;

pub fn print_dot(source_nodes: &Vec<SourceNode>) {
    println!("graph mygraph {{");

    // Set each node's label
    for node in source_nodes.iter().filter(|n| n.label.is_some()) {
        println!("\t{} [label=\"{}\"]", node.id, node.label.as_ref().unwrap());
    }

    // Draw links
    for node in source_nodes.iter().filter(|n| n.parent_id.is_some()) {
        println!("\t{} -- {}", node.parent_id.as_ref().unwrap(), node.id);
    }


    println!("}}");
}