use crate::models::RepoInfo;

/// Generate a review prompt markdown for the given PR number and repository
pub fn generate_review_prompt(pr_number: u32, repo_info: &RepoInfo) -> String {
    format!(
        r#"# Code Review Prompt for PR #{}

## Repository
**Path:** {}

## Instructions
This is a placeholder for the AI code review prompt.

## Context
- PR Number: {}
- Repository: {}
- Generated at: {}

## Next Steps
- [ ] Fetch PR details from GitHub API
- [ ] Generate git diff
- [ ] Include file modifications
- [ ] Add linked issue context
"#,
        pr_number,
        repo_info.path.display(),
        pr_number,
        repo_info.path.display(),
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    )
}
