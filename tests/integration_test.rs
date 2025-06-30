use classlink_checker::{UnifiedAnalyzer, LinkAnalyzer};
use std::collections::HashSet;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_multi_language_project_analysis() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();
    
    // Create Python file
    let python_content = r#"
from database import DatabaseConnection
from models import User

class UserService:
    def __init__(self):
        self.db = DatabaseConnection()
    
    def get_user(self, id: int) -> User:
        return self.db.find(User, id)
"#;
    fs::write(project_path.join("service.py"), python_content).unwrap();
    
    // Create TypeScript file
    let typescript_content = r#"
import { User } from './models/User';
import { UserService } from './services/UserService';

export class UserController {
    constructor(private userService: UserService) {}
    
    async getUser(id: number): Promise<User> {
        return this.userService.getUser(id);
    }
}
"#;
    fs::write(project_path.join("controller.ts"), typescript_content).unwrap();
    
    // Create C# file
    let csharp_content = r#"
using Domain.Models;

namespace Application
{
    public class UserRepository
    {
        public User GetById(int id)
        {
            return new User { Id = id };
        }
    }
}
"#;
    fs::write(project_path.join("repository.cs"), csharp_content).unwrap();
    
    // Create Markdown file
    let markdown_content = r#"# User

The User class is used by [UserService](UserService.md), [UserController](UserController.md), and [UserRepository](UserRepository.md).
"#;
    fs::write(project_path.join("User.md"), markdown_content).unwrap();
    
    // Analyze the project
    let unified_analyzer = UnifiedAnalyzer::new();
    let link_analyzer = LinkAnalyzer::new();
    let mut all_classes = HashSet::new();
    let mut all_links = Vec::new();
    
    for entry in fs::read_dir(project_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let content = fs::read_to_string(&path).unwrap();
            let result = unified_analyzer.analyze_file(&content, &path.to_string_lossy());
            all_classes.extend(result.classes);
            all_links.extend(result.links);
        }
    }
    
    let stats = link_analyzer.analyze(all_classes, all_links);
    
    // Verify results
    assert_eq!(stats.total_classes, 4); // User, UserService, UserController, UserRepository
    assert_eq!(stats.isolated_classes.len(), 0); // All classes are connected
    
    // Check that User is the most linked class
    if let Some((class_name, link_count)) = &stats.most_linked_class {
        assert_eq!(class_name, "User");
        assert!(*link_count >= 3);
    } else {
        panic!("Expected most_linked_class to be Some");
    }
}

#[test]
fn test_isolated_class_detection() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();
    
    // Create files with some isolated classes
    let python_content = r#"
class ConnectedClass:
    def __init__(self):
        self.helper = HelperClass()

class HelperClass:
    pass

class IsolatedClass:
    pass
"#;
    fs::write(project_path.join("module.py"), python_content).unwrap();
    
    let typescript_content = r#"
export class AnotherIsolatedClass {
    // No dependencies or references
}
"#;
    fs::write(project_path.join("isolated.ts"), typescript_content).unwrap();
    
    // Analyze
    let unified_analyzer = UnifiedAnalyzer::new();
    let link_analyzer = LinkAnalyzer::new();
    let mut all_classes = HashSet::new();
    let mut all_links = Vec::new();
    
    for entry in fs::read_dir(project_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let content = fs::read_to_string(&path).unwrap();
            let result = unified_analyzer.analyze_file(&content, &path.to_string_lossy());
            all_classes.extend(result.classes);
            all_links.extend(result.links);
        }
    }
    
    let stats = link_analyzer.analyze(all_classes, all_links);
    
    // Verify isolated classes
    assert_eq!(stats.isolated_classes.len(), 2);
    assert!(stats.isolated_classes.contains(&"IsolatedClass".to_string()));
    assert!(stats.isolated_classes.contains(&"AnotherIsolatedClass".to_string()));
}