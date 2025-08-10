# Prompt Assist - Project Context for AI Assistant

## Project Overview

**Prompt Assist** is a CLI tool that generates very specific, detailed prompts for AI agents to perform tasks on git codebases. The goal is to create comprehensive prompts that include all necessary context so AI agents don't waste tokens on unnecessary requests.

### Primary Use Case
Generate thorough code review prompts for GitHub PRs that include:
- PR description, comments, discussions, reviews
- Linked Jira issue details  
- Git diff between PR branch and base branch
- List of modified files
- Repository-specific context

### Example Usage
```bash
# User runs this in their git repository
prompt-assist 1234                           # Output to stdout
prompt-assist 1234 -o review-prompt.md       # Output to file
prompt-assist 1234 | less                    # Pipe to pager
```

## Technical Stack & Philosophy

### Core Technologies
- **Rust** - For performance, reliability, and excellent CLI ecosystem
- **Nix/NixOS** - Development environment and "the Nix way" 
- **SQLite** - Local caching of expensive API calls (GitHub, Jira)
- **Pure Rust dependencies** - Avoiding native library complexity where possible

### Key Dependencies
```toml
clap = "4.5"           # CLI framework with derive macros
tokio = "1"            # Async runtime (for future API calls)
reqwest = "0.12"       # HTTP client (with rustls-tls)
sqlx = "0.8"           # Database with async support
git2 = "0.20"          # Git operations
serde = "1.0"          # JSON serialization
chrono = "0.4"         # Date/time handling
```

### Architecture Decisions

**Modular Design**: Clean separation of concerns with domain-specific modules
**Synchronous-first**: Currently synchronous, will add async when needed for API calls
**Pure Rust**: Using `rustls-tls` instead of OpenSSL to avoid native dependencies
**Functional Style**: Pure functions for core logic, isolated side effects
**Domain-Driven Errors**: Custom error types for each domain that compose well
**Unix Philosophy**: Flexible output (file or stdout) for composability

## Current State

### What's Implemented
- [x] Modular architecture with clean separation of concerns
- [x] CLI parsing with clap derive macros
- [x] Git repository detection and validation  
- [x] Output flag (`-o`) for writing to files or stdout
- [x] Markdown prompt generation with basic template
- [x] Domain-specific error types (GitError, OutputError, AppError)
- [x] Pure functional design for core operations
- [x] Nix development environment with fenix
- [x] Test fixtures with real git repositories

### Current CLI Interface
```bash
prompt-assist <PR_NUMBER> [--repo-path <PATH>] [-o <OUTPUT_FILE>]

# Examples:
prompt-assist 1234                                    # Output to stdout, use current directory
prompt-assist 1234 -o review-prompt.md               # Write to file
prompt-assist 1234 --repo-path /path/to/repo        # Specify repo path
prompt-assist 1234 -r fixtures/fixture-prompt-assist # Test with fixtures
prompt-assist 1234 | grep "TODO"                     # Pipe output to other tools
```

### Project Structure
```
prompt-assist/
├── src/
│   ├── main.rs          # Minimal CLI entry point (6 lines)
│   ├── lib.rs           # Library coordination and public API
│   ├── cli.rs           # Command-line argument parsing
│   ├── git.rs           # Git repository operations and validation
│   ├── models.rs        # Domain data structures (RepoInfo, etc.)
│   ├── output.rs        # File/stdout output handling
│   └── prompt.rs        # Prompt content generation
├── fixtures/            # Test git repositories (committed)
│   └── fixture-prompt-assist/  # Real git repo for testing
├── flake.nix           # Nix development environment
├── Cargo.toml          # Dependencies
└── claude.md           # This file
```

## Development Approach

### Learning-Oriented
- Building step by step to understand each piece
- Starting with simplest possible implementation
- Adding complexity incrementally
- Focusing on core value proposition first

### Architectural Principles
- **Single Responsibility**: Each module has one clear purpose
- **Pure Functions**: Core logic is pure where possible (prompt generation, validation)
- **Isolated Side Effects**: File I/O and git operations are clearly separated
- **Composable Errors**: Domain errors that combine well through `From` traits
- **Clean Dependencies**: Clear module boundaries with minimal coupling

### Testing Strategy
- Real git repositories in `fixtures/` for development testing
- Manual testing during development using `--repo-path` flag
- Pure functions are easily unit testable
- Plan to add `tempfile`-based unit tests for complex scenarios

### Current Module Responsibilities
- **cli.rs**: CLI argument parsing only
- **git.rs**: Git repository operations (validation, path resolution)  
- **models.rs**: Domain data structures (RepoInfo, future PR/Issue types)
- **output.rs**: File system writes and stdout handling
- **prompt.rs**: Markdown content generation (pure function)
- **lib.rs**: Error composition and application coordination

## Next Steps (Planned)

### Immediate Next Steps
1. **Git Remote Detection**: Extract GitHub repo URL from git remotes using git2
2. **GitHub API Integration**: Fetch PR details, comments, reviews (re-add async)
3. **Enhanced Diff Generation**: Use git2 to generate meaningful diffs between branches
4. **Richer Prompt Templates**: More sophisticated markdown templates with PR context

### Future Features
5. **Caching Layer**: SQLite for caching API responses to avoid rate limits
6. **Jira Integration**: Link PRs to Jira issues for complete context
7. **Configuration System**: GitHub tokens, Jira credentials, custom templates
8. **Multiple Output Formats**: JSON, YAML options alongside markdown

## Development Environment

### Prerequisites
- NixOS or Nix package manager
- direnv (for automatic environment loading)

### Setup
```bash
cd prompt-assist
direnv allow     # Loads Nix environment automatically
cargo run -- 1234 --repo-path fixtures/fixture-prompt-assist
cargo run -- 1234 -o test-output.md --repo-path fixtures/fixture-prompt-assist
```

### Available Tools in Nix Shell
- `cargo-watch` - Live reloading during development
- `cargo-edit` - Adding dependencies with `cargo add`
- `sqlx-cli` - Database migrations (when we add database)
- `sqlite3` - SQLite command line tool

## Design Principles

### Code Organization
- **Modules by domain**, not by technical layer
- **Pure functions** for business logic
- **Explicit error handling** with domain-specific types
- **Minimal main function** - logic lives in library

### User Experience
- **Simple CLI interface** following Unix conventions
- **Flexible output** (file or stdout) for composability
- **Clear error messages** with helpful context
- **Works from any directory** in a git repo

### Functional Programming
- **Immutable data structures** where possible
- **Pure functions** for core logic (prompt generation, validation)
- **Composable operations** that can be easily tested
- **Clear separation** between pure logic and side effects

## Context for AI Assistant

When helping with this project:
- **Prioritize simplicity** - We're learning and building incrementally
- **Suggest minimal implementations** - Add complexity later when needed
- **Respect the Nix way** - Prefer solutions that work well with Nix
- **Consider the learning journey** - Explain concepts clearly with reasoning
- **Focus on the core value** - Better AI prompts through comprehensive context
- **Respect code boundaries** - Don't edit code without explicit permission

The user is learning Rust and building their first substantial CLI tool. They appreciate explanations of concepts and prefer building understanding alongside functionality. The modular architecture is working well and should be maintained as we add complexity.