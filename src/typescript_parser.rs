use regex::Regex;
use std::collections::HashSet;
use crate::parser::ClassLink;
use crate::file_analyzer::{FileAnalyzer, AnalysisResult};

pub struct TypeScriptParser {
    class_regex: Regex,
    interface_regex: Regex,
    import_regex: Regex,
}

impl TypeScriptParser {
    pub fn new() -> Self {
        Self {
            class_regex: Regex::new(r"^\s*(?:export\s+)?class\s+(\w+)").unwrap(),
            interface_regex: Regex::new(r"^\s*(?:export\s+)?interface\s+(\w+)").unwrap(),
            import_regex: Regex::new(r"import\s*\{([^}]+)\}\s*from").unwrap(),
        }
    }
    
    fn extract_class_references(&self, line: &str) -> Vec<String> {
        let mut references = Vec::new();
        
        // Type annotations like : User, : DatabaseService
        let type_annotation_regex = Regex::new(r":\s*([A-Z]\w+)").unwrap();
        for caps in type_annotation_regex.captures_iter(line) {
            references.push(caps[1].to_string());
        }
        
        // Constructor injections like constructor(private db: DatabaseService)
        let constructor_regex = Regex::new(r"(?:private|public|protected)?\s*\w+:\s*([A-Z]\w+)").unwrap();
        for caps in constructor_regex.captures_iter(line) {
            references.push(caps[1].to_string());
        }
        
        // Generic types like Array<User>, Observable<User>
        let generic_regex = Regex::new(r"<([A-Z]\w+)>").unwrap();
        for caps in generic_regex.captures_iter(line) {
            references.push(caps[1].to_string());
        }
        
        references
    }
}

impl FileAnalyzer for TypeScriptParser {
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
            
            // Check for interface definitions
            if let Some(caps) = self.interface_regex.captures(line) {
                let interface_name = caps[1].to_string();
                classes.insert(interface_name.clone());
                current_class = Some(interface_name);
            }
            
            // Check for imports
            if let Some(caps) = self.import_regex.captures(line) {
                let imports = &caps[1];
                for import in imports.split(',') {
                    let class_name = import.trim();
                    if class_name.chars().next().map_or(false, |c| c.is_uppercase()) {
                        imported_classes.insert(class_name.to_string());
                    }
                }
            }
            
            // Find class references
            if let Some(ref from_class) = current_class {
                for to_class in self.extract_class_references(line) {
                    if &to_class != from_class && (imported_classes.contains(&to_class) || classes.contains(&to_class)) {
                        // Avoid duplicates
                        let already_exists = links.iter().any(|l: &ClassLink| l.from_class == *from_class && l.to_class == to_class);
                        if !already_exists {
                            links.push(ClassLink {
                                from_class: from_class.clone(),
                                to_class,
                                line_number: line_num + 1,
                            });
                        }
                    }
                }
            }
        }
        
        AnalysisResult { classes, links }
    }
}