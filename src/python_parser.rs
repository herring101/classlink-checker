use regex::Regex;
use std::collections::HashSet;
use crate::parser::ClassLink;
use crate::file_analyzer::{FileAnalyzer, AnalysisResult};

pub struct PythonParser {
    class_regex: Regex,
    import_regex: Regex,
    from_import_regex: Regex,
}

impl PythonParser {
    pub fn new() -> Self {
        Self {
            class_regex: Regex::new(r"^\s*class\s+(\w+)").unwrap(),
            import_regex: Regex::new(r"^\s*import\s+(.+)").unwrap(),
            from_import_regex: Regex::new(r"^\s*from\s+[\w.]+\s+import\s+(.+)").unwrap(),
        }
    }
    
    fn extract_class_references(&self, line: &str) -> Vec<String> {
        let mut references = Vec::new();
        
        // Find class instantiations like DatabaseConnection()
        let instantiation_regex = Regex::new(r"\b([A-Z]\w+)\s*\(").unwrap();
        for caps in instantiation_regex.captures_iter(line) {
            references.push(caps[1].to_string());
        }
        
        // Find type hints like user: User
        let type_hint_regex = Regex::new(r":\s*([A-Z]\w+)").unwrap();
        for caps in type_hint_regex.captures_iter(line) {
            references.push(caps[1].to_string());
        }
        
        references
    }
}

impl FileAnalyzer for PythonParser {
    fn analyze(&self, content: &str, _file_path: &str) -> AnalysisResult {
        let mut classes = HashSet::new();
        let mut links = Vec::new();
        let mut current_class = None;
        let mut imported_classes = HashSet::new();
        
        for (line_num, line) in content.lines().enumerate() {
            // Check for class definitions
            if let Some(caps) = self.class_regex.captures(line) {
                let class_name = caps[1].to_string();
                classes.insert(class_name.clone());
                current_class = Some(class_name);
            }
            
            // Check for imports
            if let Some(caps) = self.from_import_regex.captures(line) {
                let imports = &caps[1];
                for import in imports.split(',') {
                    let class_name = import.trim().split_whitespace().next().unwrap_or("");
                    if class_name.chars().next().map_or(false, |c| c.is_uppercase()) {
                        imported_classes.insert(class_name.to_string());
                    }
                }
            } else if let Some(caps) = self.import_regex.captures(line) {
                let module = &caps[1];
                // Simple heuristic: if it starts with uppercase, consider it a class
                if module.chars().next().map_or(false, |c| c.is_uppercase()) {
                    imported_classes.insert(module.to_string());
                }
            }
            
            // Find class references in the current context
            if let Some(ref from_class) = current_class {
                for to_class in self.extract_class_references(line) {
                    if &to_class != from_class {
                        links.push(ClassLink {
                            from_class: from_class.clone(),
                            to_class,
                            line_number: line_num + 1,
                        });
                    }
                }
            }
        }
        
        // Create links from the current class to imported classes
        if let Some(ref current_class) = current_class {
            for imported in &imported_classes {
                // Check if we already have this link
                let already_exists = links.iter().any(|l| l.from_class == *current_class && l.to_class == *imported);
                if !already_exists {
                    links.push(ClassLink {
                        from_class: current_class.clone(),
                        to_class: imported.clone(),
                        line_number: 1, // Import typically at top of file
                    });
                }
            }
        }
        
        AnalysisResult { classes, links }
    }
}