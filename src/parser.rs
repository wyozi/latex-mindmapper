use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct SourceNode {
    pub parent_ids: Vec<String>,
    pub id: String,
    pub label: Option<String>
}

pub fn parse_string(input: &str) -> Vec<SourceNode> {
    let re = Regex::new(r"\\lmmnode\{([^}]*)\}\{([^}]*)\}(?:\{([^\}]*)\})?").unwrap();
    
    re.captures_iter(input).map(|cap| {
        let parent_ids = cap[1].split(",").filter(|s| s != &"").map(|s| s.to_string()).collect();
        let label = cap.get(3).map(|m| m.as_str().to_string());
        SourceNode { parent_ids: parent_ids, id: cap[2].to_string(), label: label }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse_string("Hello world"), vec!());
        assert_eq!(parse_string(r"\lmmnode{a}{b}"), vec!(
            SourceNode { parent_ids: vec!("a".to_string()), id: "b".to_string(), label: None }
        ));
        assert_eq!(parse_string(r"\lmmnode{}{b}"), vec!(
            SourceNode { parent_ids: vec!(), id: "b".to_string(), label: None }
        ));
        assert_eq!(parse_string(r"\lmmnode{a}{b}{c}"), vec!(
            SourceNode { parent_ids: vec!("a".to_string()), id: "b".to_string(), label: Some("c".to_string()) }
        ));
        assert_eq!(parse_string(r"\lmmnode{a,b}{i}{l}"), vec!(
            SourceNode { parent_ids: vec!("a".to_string(), "b".to_string()), id: "i".to_string(), label: Some("l".to_string()) }
        ));
        assert_eq!(parse_string(r"\lmmnode{}{b}{c} \lmmnode{i}{j} \lmmnode{x,i}{y}{z}"), vec!(
            SourceNode { parent_ids: vec!(), id: "b".to_string(), label: Some("c".to_string()) },
            SourceNode { parent_ids: vec!("i".to_string()), id: "j".to_string(), label: None },
            SourceNode { parent_ids: vec!("x".to_string(), "i".to_string()), id: "y".to_string(), label: Some("z".to_string()) }
        ));
    }
}