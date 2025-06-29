use std::collections::HashSet;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct ClassLink {
    pub from_class: String,
    pub to_class: String,
    pub line_number: usize,
}

pub struct MarkdownParser {
    header_regex: Regex,
    link_regex: Regex,
    code_class_regex: Regex,
}

impl MarkdownParser {
    pub fn new() -> Self {
        Self {
            header_regex: Self::create_header_regex(),
            link_regex: Self::create_link_regex(),
            code_class_regex: Self::create_code_class_regex(),
        }
    }
    
    fn create_header_regex() -> Regex {
        Regex::new(r"^#\s+(\w+)").unwrap()
    }
    
    fn create_link_regex() -> Regex {
        Regex::new(r"\[([^\]]+)\]\(([^)]+\.md)\)").unwrap()
    }
    
    fn create_code_class_regex() -> Regex {
        Regex::new(r"\b(\w+Class)\b").unwrap()
    }

    pub fn parse_file(&self, content: &str) -> Vec<ClassLink> {
        let mut links = Vec::new();
        let mut current_class = None;
        let mut seen_links = HashSet::new();
        
        for (line_num, line) in content.lines().enumerate() {
            if let Some(class_name) = self.extract_class_from_header(line) {
                current_class = Some(class_name);
            }
            
            if let Some(ref from_class) = current_class {
                self.process_markdown_links(line, from_class, line_num + 1, &mut links, &mut seen_links);
                self.process_code_class_references(line, from_class, line_num + 1, &mut links, &mut seen_links);
            }
        }
        
        links
    }
    
    fn extract_class_from_header(&self, line: &str) -> Option<String> {
        self.header_regex.captures(line).map(|caps| caps[1].to_string())
    }
    
    fn process_markdown_links(
        &self,
        line: &str,
        from_class: &str,
        line_number: usize,
        links: &mut Vec<ClassLink>,
        seen_links: &mut HashSet<(String, String)>,
    ) {
        for caps in self.link_regex.captures_iter(line) {
            let link_path = &caps[2];
            let to_class = link_path.trim_end_matches(".md");
            
            self.add_link_if_new(from_class, to_class, line_number, links, seen_links);
        }
    }
    
    fn process_code_class_references(
        &self,
        line: &str,
        from_class: &str,
        line_number: usize,
        links: &mut Vec<ClassLink>,
        seen_links: &mut HashSet<(String, String)>,
    ) {
        for caps in self.code_class_regex.captures_iter(line) {
            let class_name = &caps[1];
            if self.is_valid_class_reference(class_name, from_class, line) {
                self.add_link_if_new(from_class, class_name, line_number, links, seen_links);
            }
        }
    }
    
    fn is_valid_class_reference(&self, class_name: &str, from_class: &str, line: &str) -> bool {
        class_name != from_class && !line.contains(&format!("[{}]", class_name))
    }
    
    fn add_link_if_new(
        &self,
        from_class: &str,
        to_class: &str,
        line_number: usize,
        links: &mut Vec<ClassLink>,
        seen_links: &mut HashSet<(String, String)>,
    ) {
        let link_key = (from_class.to_string(), to_class.to_string());
        if !seen_links.contains(&link_key) {
            seen_links.insert(link_key);
            links.push(ClassLink {
                from_class: from_class.to_string(),
                to_class: to_class.to_string(),
                line_number,
            });
        }
    }

    pub fn extract_classes(&self, content: &str) -> HashSet<String> {
        let mut classes = HashSet::new();
        
        for line in content.lines() {
            if let Some(caps) = self.header_regex.captures(line) {
                classes.insert(caps[1].to_string());
            }
        }
        
        classes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_class_link() {
        let parser = MarkdownParser::new();
        let content = r#"# MyClass

This class inherits from [BaseClass](BaseClass.md).
"#;
        let links = parser.parse_file(content);
        
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].from_class, "MyClass");
        assert_eq!(links[0].to_class, "BaseClass");
        assert_eq!(links[0].line_number, 3);
    }

    #[test]
    fn test_extract_classes_from_headers() {
        let parser = MarkdownParser::new();
        let content = r#"# ClassA

## Description

# ClassB

### Methods
"#;
        let classes = parser.extract_classes(content);
        
        assert_eq!(classes.len(), 2);
        assert!(classes.contains("ClassA"));
        assert!(classes.contains("ClassB"));
    }

    #[test]
    fn test_parse_multiple_links() {
        let parser = MarkdownParser::new();
        let content = r#"# MyClass

This class uses [HelperClass](HelperClass.md) and extends [BaseClass](BaseClass.md).
It also implements [Interface](Interface.md).
"#;
        let links = parser.parse_file(content);
        
        assert_eq!(links.len(), 3);
        
        let link_pairs: Vec<(String, String)> = links.iter()
            .map(|l| (l.from_class.clone(), l.to_class.clone()))
            .collect();
        
        assert!(link_pairs.contains(&("MyClass".to_string(), "HelperClass".to_string())));
        assert!(link_pairs.contains(&("MyClass".to_string(), "BaseClass".to_string())));
        assert!(link_pairs.contains(&("MyClass".to_string(), "Interface".to_string())));
    }

    #[test]
    fn test_parse_code_class_references() {
        let parser = MarkdownParser::new();
        let content = r#"# MyClass

```rust
struct MyClass {
    helper: HelperClass,
}
```

The `MyClass` uses `HelperClass` internally.
"#;
        let links = parser.parse_file(content);
        
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].from_class, "MyClass");
        assert_eq!(links[0].to_class, "HelperClass");
    }
}