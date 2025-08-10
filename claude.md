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
prompt-assist 1234

# Outputs a markdown file with comprehensive prompt for AI code review
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
tokio = "1"            # Async runtime
reqwest = "0.12"       # HTTP client (with rustls-tls)
sqlx = "0.8"           # Database with async support
git2 = "0.20"          # Git operations
serde = "1.0"          # JSON serialization
chrono = "0.4"         # Date/time handling
```

### Architecture Decisions

**Minimal Start**: We're building incrementally, starting with simplest possible version
**Pure Rust**: Using `rustls-tls` instead of OpenSSL to avoid native dependencies
**Local Caching**: SQLite for caching expensive GitHub/Jira API calls
**Functional Style**: Embracing functional programming patterns where appropriate

## Current State

### What's Implemented
- [x] Basic CLI structure with clap
- [x] Nix development environment with fenix
- [x] Git repository detection (walks up directory tree)
- [x] Optional `--repo-path` flag for development/testing
- [x] Test fixtures with real git repositories

### Current CLI Interface
```bash
prompt-assist <PR_NUMBER> [--repo-path <PATH>]

# Examples:
prompt-assist 1234                                    # Use current directory
prompt-assist 1234 --repo-path /path/to/repo        # Specify repo
prompt-assist 1234 -r fixtures/simple-repo          # Test with fixtures
```

### Project Structure
```
prompt-assist/
├── src/
│   ├── main.rs          # CLI entry point
│   └── lib.rs           # Library code (currently empty)
├── fixtures/            # Test git repositories (committed)
│   ├── simple-repo/     # Basic git repo with commits
│   └── github-repo/     # Repo with GitHub remote
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

### Testing Strategy
- Real git repositories in `fixtures/` for development testing
- Plan to add `tempfile`-based unit tests later
- Manual testing during development using `--repo-path` flag

### Next Steps (Planned)
1. **Git Operations**: Extract git remote info to detect GitHub repo
2. **GitHub API**: Fetch PR details, comments, reviews
3. **Diff Generation**: Use git2 to generate meaningful diffs
4. **Prompt Templates**: Create markdown templates for AI prompts
5. **Caching Layer**: SQLite for caching API responses
6. **Jira Integration**: Link PRs to Jira issues
7. **Configuration**: GitHub tokens, Jira credentials

## Development Environment

### Prerequisites
- NixOS or Nix package manager
- direnv (for automatic environment loading)

### Setup
```bash
cd prompt-assist
direnv allow     # Loads Nix environment automatically
cargo run -- 1234 --repo-path fixtures/simple-repo
```

### Available Tools in Nix Shell
- `cargo-watch` - Live reloading during development
- `cargo-edit` - Adding dependencies with `cargo add`
- `sqlx-cli` - Database migrations (when we add database)
- `sqlite3` - SQLite command line tool

## Design Principles

### Minimal Dependencies
- Start small, add only what we need
- Prefer pure Rust implementations
- Avoid over-engineering early

### User Experience
- Simple CLI interface
- Clear error messages
- Works from any directory in a git repo
- Flexible for development and testing

### Functional Programming
- Immutable data where possible
- Pure functions for core logic
- Composable operations
- Clear separation of concerns

## Context for AI Assistant

When helping with this project:
- **Prioritize simplicity** - We're learning and building incrementally
- **Suggest minimal implementations** - Add complexity later
- **Respect the Nix way** - Prefer solutions that work well with Nix
- **Consider the learning journey** - Explain concepts clearly
- **Focus on the core value** - Better AI prompts through comprehensive context

The user is learning Rust and building their first substantial CLI tool. They appreciate explanations of concepts and prefer building understanding alongside functionality.