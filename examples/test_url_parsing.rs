// Test GitHub URL parsing functionality
use prompt_assist::git::GitHubRepo;

fn main() {
    // Test different URL formats
    let test_urls = vec![
        "git@github.com:palekiwi-labs/fixture-prompt-assist.git",
        "https://github.com/palekiwi-labs/fixture-prompt-assist.git",
        "https://github.com/palekiwi-labs/fixture-prompt-assist",
        "ssh://git@github.com/palekiwi-labs/fixture-prompt-assist.git",
    ];

    for url in test_urls {
        match GitHubRepo::from_url(url) {
            Ok(repo) => println!("✓ {} -> {}/{}", url, repo.owner, repo.name),
            Err(err) => println!("✗ {} -> Error: {}", url, err),
        }
    }
}
