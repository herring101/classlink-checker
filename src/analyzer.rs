use std::collections::{HashMap, HashSet};
use crate::parser::ClassLink;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkStatistics {
    pub total_classes: usize,
    pub isolated_classes: Vec<String>,
    pub class_link_counts: HashMap<String, ClassLinkCount>,
    pub most_linked_class: Option<(String, usize)>,
    pub most_linking_class: Option<(String, usize)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassLinkCount {
    pub outgoing_links: usize,
    pub incoming_links: usize,
}

pub struct LinkAnalyzer;

impl LinkAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, classes: HashSet<String>, links: Vec<ClassLink>) -> LinkStatistics {
        let mut class_link_counts = HashMap::new();
        let mut linked_classes = HashSet::new();
        
        // Initialize all classes with zero counts
        for class in &classes {
            class_link_counts.insert(
                class.clone(),
                ClassLinkCount {
                    outgoing_links: 0,
                    incoming_links: 0,
                },
            );
        }
        
        // Count links
        for link in &links {
            linked_classes.insert(link.from_class.clone());
            linked_classes.insert(link.to_class.clone());
            
            // Update outgoing links for source class
            if let Some(count) = class_link_counts.get_mut(&link.from_class) {
                count.outgoing_links += 1;
            }
            
            // Update incoming links for target class
            if let Some(count) = class_link_counts.get_mut(&link.to_class) {
                count.incoming_links += 1;
            }
        }
        
        // Find isolated classes (no incoming or outgoing links)
        let isolated_classes: Vec<String> = classes
            .iter()
            .filter(|class| !linked_classes.contains(*class))
            .cloned()
            .collect();
        
        // Find most linked class (highest incoming links)
        let most_linked_class = class_link_counts
            .iter()
            .max_by_key(|(_, count)| count.incoming_links)
            .map(|(class, count)| (class.clone(), count.incoming_links))
            .filter(|(_, count)| *count > 0);
        
        // Find most linking class (highest outgoing links)
        let most_linking_class = class_link_counts
            .iter()
            .max_by_key(|(_, count)| count.outgoing_links)
            .map(|(class, count)| (class.clone(), count.outgoing_links))
            .filter(|(_, count)| *count > 0);
        
        LinkStatistics {
            total_classes: classes.len(),
            isolated_classes,
            class_link_counts,
            most_linked_class,
            most_linking_class,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_isolated_classes() {
        let analyzer = LinkAnalyzer::new();
        let classes: HashSet<String> = ["ClassA", "ClassB", "ClassC", "IsolatedClass"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        
        let links = vec![
            ClassLink {
                from_class: "ClassA".to_string(),
                to_class: "ClassB".to_string(),
                line_number: 1,
            },
            ClassLink {
                from_class: "ClassB".to_string(),
                to_class: "ClassC".to_string(),
                line_number: 2,
            },
        ];
        
        let stats = analyzer.analyze(classes, links);
        
        assert_eq!(stats.total_classes, 4);
        assert_eq!(stats.isolated_classes.len(), 1);
        assert!(stats.isolated_classes.contains(&"IsolatedClass".to_string()));
    }

    #[test]
    fn test_class_link_counts() {
        let analyzer = LinkAnalyzer::new();
        let classes: HashSet<String> = ["ClassA", "ClassB", "ClassC"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        
        let links = vec![
            ClassLink {
                from_class: "ClassA".to_string(),
                to_class: "ClassB".to_string(),
                line_number: 1,
            },
            ClassLink {
                from_class: "ClassA".to_string(),
                to_class: "ClassC".to_string(),
                line_number: 2,
            },
            ClassLink {
                from_class: "ClassC".to_string(),
                to_class: "ClassB".to_string(),
                line_number: 3,
            },
        ];
        
        let stats = analyzer.analyze(classes, links);
        
        assert_eq!(stats.class_link_counts["ClassA"].outgoing_links, 2);
        assert_eq!(stats.class_link_counts["ClassA"].incoming_links, 0);
        
        assert_eq!(stats.class_link_counts["ClassB"].outgoing_links, 0);
        assert_eq!(stats.class_link_counts["ClassB"].incoming_links, 2);
        
        assert_eq!(stats.class_link_counts["ClassC"].outgoing_links, 1);
        assert_eq!(stats.class_link_counts["ClassC"].incoming_links, 1);
    }

    #[test]
    fn test_most_linked_and_linking_classes() {
        let analyzer = LinkAnalyzer::new();
        let classes: HashSet<String> = ["Hub", "Node1", "Node2", "Node3"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        
        let links = vec![
            ClassLink {
                from_class: "Hub".to_string(),
                to_class: "Node1".to_string(),
                line_number: 1,
            },
            ClassLink {
                from_class: "Hub".to_string(),
                to_class: "Node2".to_string(),
                line_number: 2,
            },
            ClassLink {
                from_class: "Hub".to_string(),
                to_class: "Node3".to_string(),
                line_number: 3,
            },
            ClassLink {
                from_class: "Node1".to_string(),
                to_class: "Hub".to_string(),
                line_number: 4,
            },
            ClassLink {
                from_class: "Node2".to_string(),
                to_class: "Hub".to_string(),
                line_number: 5,
            },
        ];
        
        let stats = analyzer.analyze(classes, links);
        
        assert_eq!(stats.most_linking_class, Some(("Hub".to_string(), 3)));
        assert_eq!(stats.most_linked_class, Some(("Hub".to_string(), 2)));
    }
}