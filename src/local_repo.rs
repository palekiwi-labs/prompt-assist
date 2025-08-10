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

        let remote = find_github_remote(&repo)?
            .ok_or(GitError::NoGitHubRemote)?;

        Ok(LocalRepo { repo, remote })
    }
}

fn find_github_remote(repo: &Repository) -> Result<Option<GitRemote>, GitError> {
    let remote_names = repo.remotes()?;
    
    for name in remote_names.iter().flatten() {
        if let Ok(remote) = repo.find_remote(name)
            && let Some(url) = remote.url()
            && is_github_url(url)
        {
            return Ok(Some(GitRemote {
                name: name.to_string(),
                url: url.to_string(),
            }));
        }
    }
    
    Ok(None)
}

fn is_github_url(url: &str) -> bool {
    url.contains("github.com")
        && (url.starts_with("https://github.com")
            || url.starts_with("git@github.com")
            || url.starts_with("ssh://git@github.com"))
}
