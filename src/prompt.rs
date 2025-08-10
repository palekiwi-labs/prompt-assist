use crate::models::RepoInfo;

/// Generate a review prompt markdown for the given PR number and repository
pub fn generate_review_prompt(pr_number: u32, repo_info: &RepoInfo) -> String {
    let mut content = format!(
        r#"# Code Review Prompt for PR #{}

## Repository Information
**Path:** {}
"#,
        pr_number,
        repo_info.path.display()
    );

    // Add git information if available
    if let Some(ref git_info) = repo_info.git_info {
        content.push_str(&format!(
            "**Repository Root:** {}\n",
            git_info.repository_root.display()
        ));
        
        if let Some(ref remote_url) = git_info.remote_url {
            content.push_str(&format!("**GitHub URL:** {}\n", remote_url));
        }
        
        if let Some(ref branch) = git_info.current_branch {
            content.push_str(&format!("**Current Branch:** {}\n", branch));
        }
        
        if !git_info.remotes.is_empty() {
            content.push_str("\n**Git Remotes:**\n");
            for remote in &git_info.remotes {
                content.push_str(&format!("- {}: {}\n", remote.name, remote.url));
            }
        }
    }

    content.push_str(&format!(
        r#"
## Instructions
This is a placeholder for the AI code review prompt.

## Context
- PR Number: {}
- Generated at: {}

## Next Steps
- [ ] Fetch PR details from GitHub API
- [ ] Generate git diff
- [ ] Include file modifications
- [ ] Add linked issue context
"#,
        pr_number,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    ));

    content
}
