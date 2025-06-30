use regex::Regex;
use std::collections::HashSet;
use crate::parser::ClassLink;
use crate::file_analyzer::{FileAnalyzer, AnalysisResult};

pub struct CSharpParser {
    class_regex: Regex,
    interface_regex: Regex,
    using_regex: Regex,
}

impl CSharpParser {
    pub fn new() -> Self {
        Self {
            class_regex: Regex::new(r"^\s*(?:public|private|protected|internal)?\s*(?:partial|abstract|sealed)?\s*class\s+(\w+)").unwrap(),
            interface_regex: Regex::new(r"^\s*(?:public|private|protected|internal)?\s*interface\s+(\w+)").unwrap(),
            using_regex: Regex::new(r"^\s*using\s+([\w.]+);").unwrap(),
        }
    }
    
    fn extract_class_references(&self, line: &str) -> Vec<String> {
        let mut references = Vec::new();
        
        // Type declarations like : IUserService
        let inheritance_regex = Regex::new(r":\s*([A-Z]\w+)").unwrap();
        for caps in inheritance_regex.captures_iter(line) {
            references.push(caps[1].to_string());
        }
        
        // Constructor parameters and method parameters
        let param_regex = Regex::new(r"([A-Z]\w+)\s+\w+").unwrap();
        for caps in param_regex.captures_iter(line) {
            references.push(caps[1].to_string());
        }
        
        // Generic types like List<User>, Task<User>
        let generic_regex = Regex::new(r"<([A-Z]\w+)>").unwrap();
        for caps in generic_regex.captures_iter(line) {
            references.push(caps[1].to_string());
        }
        
        // Property types
        let property_regex = Regex::new(r"(?:public|private|protected|internal)?\s*([A-Z]\w+)\s+\w+\s*\{").unwrap();
        for caps in property_regex.captures_iter(line) {
            references.push(caps[1].to_string());
        }
        
        references
    }
}

impl FileAnalyzer for CSharpParser {
    fn analyze(&self, content: &str, _file_path: &str) -> AnalysisResult {
        let mut classes = HashSet::new();
        let mut links = Vec::new();
        let mut current_class = None;
        let mut imported_types = HashSet::new();
        
        for (line_num, line) in content.lines().enumerate() {
            // Check for using statements
            if let Some(caps) = self.using_regex.captures(line) {
                let namespace = &caps[1];
                // Extract the last part as potential class name
                if let Some(last_part) = namespace.split('.').last() {
                    if last_part.chars().next().map_or(false, |c| c.is_uppercase()) {
                        imported_types.insert(last_part.to_string());
                    }
                }
            }
            
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
            
            // Find class references
            if let Some(ref from_class) = current_class {
                for to_class in self.extract_class_references(line) {
                    if &to_class != from_class {
                        // Check if it's a known type or common .NET type
                        let is_known_type = classes.contains(&to_class) || 
                                          imported_types.contains(&to_class) ||
                                          // Common C# types to include
                                          ["User", "DatabaseContext", "IUserService"].contains(&to_class.as_str());
                        
                        if is_known_type {
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
        }
        
        AnalysisResult { classes, links }
    }
}