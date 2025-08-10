use crate::models::PromptContext;

/// Generate a review prompt markdown for the given PR number and repository
pub fn generate_review_prompt(pr_number: u32, context: &PromptContext) -> String {
    let mut content = format!(
        r#"# Code Review Prompt for PR #{}

## Repository Information
**Path:** {}
"#,
        pr_number,
        context.repo_path.display()
    );

    // Add git information if available
    if let Some(ref git) = context.git {
        content.push_str(&format!(
            "**Repository Root:** {}\n",
            git.repository_root.display()
        ));
        
        if let Some(ref remote_url) = git.remote_url {
            content.push_str(&format!("**GitHub URL:** {}\n", remote_url));
        }
        
        if let Some(ref branch) = git.current_branch {
            content.push_str(&format!("**Current Branch:** {}\n", branch));
        }
        
        if !git.remotes.is_empty() {
            content.push_str("\n**Git Remotes:**\n");
            for remote in &git.remotes {
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
