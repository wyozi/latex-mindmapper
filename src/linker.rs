use std::collections::HashSet;
use super::parser::SourceNode;

pub fn verify_links(source_nodes: &Vec<SourceNode>) -> Result<(), String> {
    let mut ids = HashSet::new();

    // Add initially unlinked nodes
    for node in source_nodes {
        ids.insert(&node.id);
    }

    // Check node links
    source_nodes
        .iter()
        .flat_map(|n| n.parent_ids.iter().map(move |par_id| (&n.id, par_id)))
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
            SourceNode { parent_ids: vec!(), id: "foo".to_string(), label: None }
        )), Ok(()));
        assert_eq!(verify_links(&vec!(
            SourceNode { parent_ids: vec!("bar".to_string()), id: "foo".to_string(), label: None }
        )), Err("foo has missing parent bar".to_string()));
    }
}