use std::collections::HashSet;
use super::parser::SourceNode;

pub fn verify_links(source_nodes: &Vec<SourceNode>) -> Result<(), String> {
    let mut ids = HashSet::new();

    // Add initially unlinked nodes
    for node in source_nodes {
        ids.insert(&node.id);
    }

    // Link nodes
    source_nodes
        .iter()
        .filter_map(|n| n.parent_id.as_ref().map(|par_id| (&n.id, par_id)))
        .map(|(id, parent_id)| {
            if ids.contains(&parent_id) {
                Ok(())
            } else {
                Err(format!("{} has missing parent {}", id, parent_id))
            }
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_links() {
        assert_eq!(verify_links(&vec!(
            SourceNode { parent_id: None, id: "foo".to_string(), label: None }
        )), Ok(()));
        assert_eq!(verify_links(&vec!(
            SourceNode { parent_id: Some("bar".to_string()), id: "foo".to_string(), label: None }
        )), Err("foo has missing parent bar".to_string()));
    }
}