use std::path::Path;
use crate::parser::{ClassLink, MarkdownParser};
use crate::python_parser::PythonParser;
use crate::typescript_parser::TypeScriptParser;
use crate::csharp_parser::CSharpParser;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum FileType {
    Markdown,
    Python,
    TypeScript,
    CSharp,
    Unknown,
}

pub trait FileAnalyzer {
    fn analyze(&self, content: &str, file_path: &str) -> AnalysisResult;
}

#[derive(Debug)]
pub struct AnalysisResult {
    pub classes: HashSet<String>,
    pub links: Vec<ClassLink>,
}

pub struct UnifiedAnalyzer;

impl UnifiedAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn detect_file_type(path: &str) -> FileType {
        let path = Path::new(path);
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("md") => FileType::Markdown,
            Some("py") => FileType::Python,
            Some("ts") | Some("tsx") => FileType::TypeScript,
            Some("cs") => FileType::CSharp,
            _ => FileType::Unknown,
        }
    }

    pub fn analyze_file(&self, content: &str, path: &str) -> AnalysisResult {
        let file_type = Self::detect_file_type(path);
        
        match file_type {
            FileType::Markdown => {
                let parser = MarkdownParser::new();
                let classes = parser.extract_classes(content);
                let links = parser.parse_file(content);
                AnalysisResult { classes, links }
            }
            FileType::Python => {
                let parser = PythonParser::new();
                parser.analyze(content, path)
            }
            FileType::TypeScript => {
                let parser = TypeScriptParser::new();
                parser.analyze(content, path)
            }
            FileType::CSharp => {
                let parser = CSharpParser::new();
                parser.analyze(content, path)
            }
            FileType::Unknown => {
                AnalysisResult {
                    classes: HashSet::new(),
                    links: vec![],
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_markdown_files() {
        assert_eq!(UnifiedAnalyzer::detect_file_type("README.md"), FileType::Markdown);
        assert_eq!(UnifiedAnalyzer::detect_file_type("docs/api.md"), FileType::Markdown);
    }

    #[test]
    fn test_detect_python_files() {
        assert_eq!(UnifiedAnalyzer::detect_file_type("main.py"), FileType::Python);
        assert_eq!(UnifiedAnalyzer::detect_file_type("src/models.py"), FileType::Python);
    }

    #[test]
    fn test_detect_typescript_files() {
        assert_eq!(UnifiedAnalyzer::detect_file_type("app.ts"), FileType::TypeScript);
        assert_eq!(UnifiedAnalyzer::detect_file_type("component.tsx"), FileType::TypeScript);
    }

    #[test]
    fn test_detect_csharp_files() {
        assert_eq!(UnifiedAnalyzer::detect_file_type("Program.cs"), FileType::CSharp);
        assert_eq!(UnifiedAnalyzer::detect_file_type("Models/User.cs"), FileType::CSharp);
    }

    #[test]
    fn test_detect_unknown_files() {
        assert_eq!(UnifiedAnalyzer::detect_file_type("config.json"), FileType::Unknown);
        assert_eq!(UnifiedAnalyzer::detect_file_type("data.txt"), FileType::Unknown);
    }

    #[test]
    fn test_analyze_python_classes() {
        let analyzer = UnifiedAnalyzer::new();
        let content = r#"
class UserService:
    def __init__(self):
        self.db = DatabaseConnection()
    
    def authenticate(self, user: User):
        pass

from models import User
from db import DatabaseConnection
"#;
        
        let result = analyzer.analyze_file(content, "service.py");
        
        assert!(result.classes.contains("UserService"));
        assert_eq!(result.links.len(), 2);
        
        let link_targets: HashSet<String> = result.links.iter()
            .map(|l| l.to_class.clone())
            .collect();
        assert!(link_targets.contains("User"));
        assert!(link_targets.contains("DatabaseConnection"));
    }

    #[test]
    fn test_analyze_typescript_classes() {
        let analyzer = UnifiedAnalyzer::new();
        let content = r#"
import { Injectable } from '@angular/core';
import { User } from './models/User';
import { DatabaseService } from './services/database.service';

@Injectable()
export class AuthService {
    constructor(private db: DatabaseService) {}
    
    authenticate(user: User): boolean {
        return true;
    }
}

interface IAuthProvider {
    login(credentials: any): void;
}
"#;
        
        let result = analyzer.analyze_file(content, "auth.service.ts");
        
        assert!(result.classes.contains("AuthService"));
        assert!(result.classes.contains("IAuthProvider"));
        assert_eq!(result.links.len(), 2);
        
        let link_targets: HashSet<String> = result.links.iter()
            .map(|l| l.to_class.clone())
            .collect();
        assert!(link_targets.contains("User"));
        assert!(link_targets.contains("DatabaseService"));
    }

    #[test]
    fn test_analyze_csharp_classes() {
        let analyzer = UnifiedAnalyzer::new();
        let content = r#"
using System;
using Domain.Models;
using Infrastructure.Data;

namespace Application.Services
{
    public class UserService : IUserService
    {
        private readonly DatabaseContext _context;
        
        public UserService(DatabaseContext context)
        {
            _context = context;
        }
        
        public User GetUser(int id)
        {
            return _context.Users.Find(id);
        }
    }
    
    public interface IUserService
    {
        User GetUser(int id);
    }
}
"#;
        
        let result = analyzer.analyze_file(content, "UserService.cs");
        
        assert!(result.classes.contains("UserService"));
        assert!(result.classes.contains("IUserService"));
        assert_eq!(result.links.len(), 4); // UserService -> IUserService, UserService -> DatabaseContext, UserService -> User, IUserService -> User
        
        let link_targets: HashSet<String> = result.links.iter()
            .map(|l| l.to_class.clone())
            .collect();
        assert!(link_targets.contains("User"));
        assert!(link_targets.contains("DatabaseContext"));
        assert!(link_targets.contains("IUserService"));
    }
}