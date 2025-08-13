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
**Builder Pattern**: Fluent RepoInfo construction for incremental data gathering
**Pure Rust**: Using `rustls-tls` instead of OpenSSL to avoid native dependencies
**Functional Style**: Pure functions for core logic, isolated side effects
**Domain-Driven Errors**: Custom error types for each domain that compose well
**Unix Philosophy**: Flexible output (file or stdout) for composability

## Current State

### What's Implemented
- [x] Modular architecture with domain-specific modules
- [x] CLI parsing with clap derive macros
- [x] **LocalRepo implementation** - git repository handling with GitHub remote detection
- [x] **PromptContext central data structure** - orchestrates all context for prompt generation
- [x] **Domain-driven design** - separate modules for each concern (git, local_repo, pull_request, etc.)
- [x] Output flag (`-o`) for writing to files or stdout
- [x] Clean error handling with dedicated error module
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

### Current Application Behavior
When you run the CLI, you'll see output like:
```
Generating review prompt for PR #1234
```

The application:
- Creates a LocalRepo from the provided or current directory
- Finds the first GitHub remote in the repository
- Creates a PromptContext with repo and PR information
- Ready for future: diff generation, issue linking, markdown output

### Data Structures

**Core Domain Models:**
```rust
pub struct PromptContext {
    repo: LocalRepo,
    pr: PullRequest,
    // Future: diff: Option<GitDiff>,
    // Future: issues: Vec<JiraIssue>,
}

pub struct LocalRepo {
    pub repo: Repository,  // git2::Repository
    pub remote: GitRemote, // The GitHub remote
}

pub struct GitRemote {
    pub name: String,  // "origin", "upstream", etc.
    pub url: String,   // The remote URL
}

pub struct PullRequest {
    // To be implemented
}
```

**Context Creation Pattern:**
```rust
let local_repo = LocalRepo::from_path(cli.repo_path)?;
let pr = PullRequest {}; // Stub for now
let context = PromptContext::new(local_repo, pr);
```

### Current Data Flow
```
CLI Input (path + PR#) 
    ↓
LocalRepo::from_path() → LocalRepo (with git2::Repository + GitHub remote)
    ↓  
PullRequest creation → PullRequest (stub for now)
    ↓
PromptContext::new(repo, pr) → Central context object
    ↓
Future: context.with_diff()?.with_issues()?.to_markdown()
    ↓
write_output() → File or stdout
```

### Project Structure
```
prompt-assist/
├── src/
│   ├── main.rs          # Minimal CLI entry point
│   ├── lib.rs           # Library coordination and module exports
│   ├── cli.rs           # Command-line argument parsing
│   ├── git.rs           # Git types (GitRemote, GitError)
│   ├── local_repo.rs    # LocalRepo implementation with git2 operations
│   ├── pull_request.rs  # PullRequest domain model (stub)
│   ├── prompt_context.rs # Central PromptContext orchestrator
│   ├── error.rs         # Application-level error types
│   ├── output.rs        # File/stdout output handling
│   └── prompt.rs        # Prompt content generation (pure function)
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
- **Domain-Driven Design**: Types and operations organized by business domain
- **Pure Functions**: Core logic is pure where possible (prompt generation)
- **Isolated Side Effects**: File I/O and git operations are clearly separated
- **Composable Errors**: Domain errors that combine well through `From` traits
- **Clean Dependencies**: Clear module boundaries with minimal coupling
- **Central Context Pattern**: PromptContext orchestrates all data for prompt generation

### Testing Strategy
- Real git repositories in `fixtures/` for development testing
- Manual testing during development using `--repo-path` flag
- Pure functions are easily unit testable
- Plan to add `tempfile`-based unit tests for complex scenarios

### Current Module Responsibilities
- **cli.rs**: CLI argument parsing only
- **git.rs**: Git types and error definitions (GitRemote, GitError)
- **local_repo.rs**: Git repository operations (GitHub remote detection, git2 integration)
- **pull_request.rs**: Pull request domain model (to be implemented)
- **prompt_context.rs**: Central context orchestration
- **error.rs**: Application-level error composition
- **output.rs**: File system writes and stdout handling
- **prompt.rs**: Markdown content generation (pure function)
- **lib.rs**: Module coordination and public API

## Next Steps (Planned)

### Immediate Next Steps
1. **GitHub URL Parsing**: Extract owner/repo from git remote URLs for API calls
2. **GitHub API Integration**: Fetch PR details, comments, reviews (re-add async)
3. **Enhanced Diff Generation**: Use git2 to generate meaningful diffs between branches
4. **Richer Prompt Templates**: More sophisticated markdown templates with PR context

### Future Features
5. **Caching Layer**: SQLite for caching API responses to avoid rate limits
6. **Jira Integration**: Link PRs to Jira issues for complete context
7. **Configuration System**: GitHub tokens, Jira credentials, custom templates
8. **Multiple Output Formats**: JSON, YAML options alongside markdown

## Git Integration Implementation

### What We Built
- **extract_git_info()**: Uses git2 to safely extract repository information
- **Smart GitHub Detection**: Handles SSH and HTTPS URLs, prioritizes "origin" remote
- **Error-Tolerant Remote Extraction**: Continues working even if some remotes are broken
- **Lifetime-Optimized**: Returns borrowed data when possible, clones only when needed
- **Idiomatic Rust**: Uses modern patterns like chained `if let` statements

### Key Functions
```rust
pub fn extract_git_info(repo_path: &Path) -> Result<GitInfo, GitError>
fn extract_remotes(repo: &Repository) -> Result<Vec<GitRemote>, GitError>
fn find_github_remote(remotes: &[GitRemote]) -> Option<&str>
fn get_current_branch(repo: &Repository) -> Result<Option<String>, GitError>
```

### Design Philosophy
- **Graceful Degradation**: Works with partial information if some git operations fail
- **Zero-Copy Where Possible**: Minimizes allocations during git data extraction
- **Functional Composition**: Each function has a single, clear responsibility

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
- **Rich git context** in generated prompts

### Functional Programming
- **Immutable data structures** where possible
- **Pure functions** for core logic (prompt generation, validation)
- **Composable operations** that can be easily tested
- **Clear separation** between pure logic and side effects
- **Builder pattern** for incremental data assembly

## Context for AI Assistant

When helping with this project:
- **Prioritize simplicity** - We're learning and building incrementally
- **Suggest minimal implementations** - Add complexity later when needed
- **Respect the Nix way** - Prefer solutions that work well with Nix
- **Consider the learning journey** - Explain concepts clearly with reasoning
- **Focus on the core value** - Better AI prompts through comprehensive context
- **Respect code boundaries** - Don't edit code without explicit permission

The user is learning Rust and building their first substantial CLI tool. They appreciate explanations of concepts and prefer building understanding alongside functionality. The modular architecture and functional design patterns are working well and should be maintained as we add complexity.

## Current Status

We've successfully **refactored to a domain-driven architecture** with PromptContext as the central orchestrator. The new structure:

- **LocalRepo** handles git repository operations and GitHub remote detection
- **PromptContext** serves as the central data structure for prompt generation
- **Clean module separation** with each domain having its own focused module
- **Extensible design** ready for diff generation, issue linking, and API integration

Next steps: Implement PullRequest with GitHub API integration and add enrichment methods to PromptContext (with_diff, with_issues, to_markdown).