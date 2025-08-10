use crate::git::{GitError, GitRemote};
use git2::Repository;
use std::{env, path::PathBuf};

pub struct LocalRepo {
    pub repo: Repository,
    pub remote: GitRemote,
}

impl LocalRepo {
    pub fn from_path(path: Option<PathBuf>) -> Result<Self, GitError> {
        let path = path.unwrap_or_else(|| env::current_dir().unwrap());

        let repo = Repository::open(&path)?;

        let remotes = extract_remotes(&repo)?;

        let remote = find_github_remote(&remotes)
            .ok_or(GitError::NoGitHubRemote)?;

        Ok(LocalRepo { repo, remote })
    }
}

fn extract_remotes(repo: &Repository) -> Result<Vec<GitRemote>, GitError> {
    let mut remotes = Vec::new();
    let remote_names = repo.remotes()?;

    for name in remote_names.iter().flatten() {
        if let Ok(remote) = repo.find_remote(name)
            && let Some(url) = remote.url()
        {
            remotes.push(GitRemote {
                name: name.to_string(),
                url: url.to_string(),
            });
        }
    }

    Ok(remotes)
}

fn find_github_remote(remotes: &[GitRemote]) -> Option<GitRemote> {
    // Try to find "origin" remote that's a GitHub URL
    if let Some(origin) = remotes.iter().find(|r| r.name == "origin")
        && is_github_url(&origin.url)
    {
        return Some(origin.clone());
    }

    // If no GitHub origin, find any GitHub remote
    remotes
        .iter()
        .find(|r| is_github_url(&r.url))
        .map(|r| r.clone())
}

fn is_github_url(url: &str) -> bool {
    url.contains("github.com")
        && (url.starts_with("https://github.com")
            || url.starts_with("git@github.com")
            || url.starts_with("ssh://git@github.com"))
}
