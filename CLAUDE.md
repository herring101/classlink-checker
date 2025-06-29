# CLAUDE.md - Project Context and Task Information

## Project Description

This is a Rust-based command-line tool that analyzes class links in Markdown documentation files. The tool can:

- Parse Markdown files to detect class references and links between classes
- Identify isolated classes (classes with no incoming or outgoing links)
- Generate statistical reports showing link counts and relationships
- Output results in both human-readable text format and JSON format

## Current Task Status

### Completed:
1. ✅ Project structure and Rust initialization 
2. ✅ Test-driven development with failing tests for markdown parsing
3. ✅ Markdown parser implementation to detect class links
4. ✅ Link analysis and statistics generation
5. ✅ CLI interface with clap for argument parsing

### TODO - Future Enhancements:
1. 🔄 **Multi-language Support**: Extend support for:
   - **Python**: Detect class definitions (`class ClassName:`) and imports (`from module import ClassName`)
   - **C#**: Detect class definitions (`public class ClassName`) and using statements
   - **TypeScript**: Detect class definitions (`class ClassName`), interfaces (`interface IName`), and imports

2. 🔄 **Advanced Features**:
   - Dependency graph visualization
   - Circular dependency detection
   - Class hierarchy analysis
   - Export to different formats (CSV, GraphML)

3. 🔄 **Testing and Quality**:
   - Integration tests with real markdown files
   - Performance optimization for large codebases
   - Error handling improvements

## Architecture

### Core Components:
- `MarkdownParser`: Parses markdown files and extracts class references
- `LinkAnalyzer`: Analyzes relationships and generates statistics
- `main.rs`: CLI interface and file system operations

### Test Strategy:
Following TDD principles as recommended by t-wada:
- Write failing tests first
- Implement minimal code to pass tests
- Refactor for better design

## Usage Examples

```bash
# Analyze current directory
classlink-checker .

# Recursive analysis with JSON output  
classlink-checker -r -o json ./docs

# Analyze specific file
classlink-checker README.md
```

## Development Guidelines

### Code Quality:
- Follow Martin Fowler's refactoring guidelines
- Maintain comprehensive test coverage
- Use proper error handling
- Keep functions small and focused

### Multi-language Extension Plan:

#### Python Support:
```python
# Detect these patterns:
class UserManager:
    pass

from models import User
import database.connection as db
```

#### C# Support:
```csharp
// Detect these patterns:
public class UserService : IUserService
{
}

using Domain.Models;
using System.Collections.Generic;
```

#### TypeScript Support:
```typescript
// Detect these patterns:
class UserController {
}

interface IUserRepository {
}

import { User } from './models/User';
export class UserService implements IUserRepository {
}
```

## Testing Commands

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test parser::
cargo test analyzer::

# Build and test CLI
cargo build
./target/debug/classlink-checker --help
```

## Git Workflow
- Feature branches for major additions
- Comprehensive commit messages
- CI/CD pipeline for automated testing