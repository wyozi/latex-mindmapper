use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct SourceNode {
    pub parent_id: Option<String>,
    pub id: String,
    pub label: Option<String>
}

pub fn parse_string(input: &str) -> Vec<SourceNode> {
    let re = Regex::new(r"\\lmmnode\{([^}]*)\}\{([^}]*)\}(?:\[([^\]]*)\])?").unwrap();
    
    re.captures_iter(input).map(|cap| {
        let parent_id = if &cap[1] == "" { None } else { Some(cap[1].to_string()) };
        let label = cap.get(3).map(|m| m.as_str().to_string());
        SourceNode { parent_id: parent_id, id: cap[2].to_string(), label: label }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse_string("Hello world"), vec!());
        assert_eq!(parse_string(r"\lmmnode{a}{b}"), vec!(
            SourceNode { parent_id: Some("a".to_string()), id: "b".to_string(), label: None }
        ));
        assert_eq!(parse_string(r"\lmmnode{}{b}"), vec!(
            SourceNode { parent_id: None, id: "b".to_string(), label: None }
        ));
        assert_eq!(parse_string(r"\lmmnode{a}{b}[c]"), vec!(
            SourceNode { parent_id: Some("a".to_string()), id: "b".to_string(), label: Some("c".to_string()) }
        ));
        assert_eq!(parse_string(r"\lmmnode{}{b}[c] \lmmnode{i}{j} \lmmnode{x}{y}[z]"), vec!(
            SourceNode { parent_id: None, id: "b".to_string(), label: Some("c".to_string()) },
            SourceNode { parent_id: Some("i".to_string()), id: "j".to_string(), label: None },
            SourceNode { parent_id: Some("x".to_string()), id: "y".to_string(), label: Some("z".to_string()) }
        ));
    }
}