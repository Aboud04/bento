use std::path::Path;
use std::process::Command;

/// Returns true if the `gh` CLI is installed and available on PATH.
pub fn is_gh_available() -> bool {
    Command::new("gh")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Returns true if the user is authenticated with `gh auth`.
pub fn is_authenticated() -> bool {
    Command::new("gh")
        .args(["auth", "status"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Initializes a git repo (if needed), creates a GitHub repo, and pushes.
pub fn create_and_push(project_dir: &Path, project_name: &str) -> anyhow::Result<()> {
    if !is_gh_available() {
        return Err(anyhow::anyhow!("gh CLI is not installed. Install it from https://cli.github.com"));
    }

    if !is_authenticated() {
        return Err(anyhow::anyhow!("Not authenticated with gh. Run `gh auth login` first"));
    }

    if !project_dir.join(".git").exists() {
        let status = Command::new("git")
            .args(["init"])
            .current_dir(project_dir)
            .status()?;
        if !status.success() {
            return Err(anyhow::anyhow!("git init failed"));
        }

        let status = Command::new("git")
            .args(["add", "."])
            .current_dir(project_dir)
            .status()?;
        if !status.success() {
            return Err(anyhow::anyhow!("git add failed"));
        }

        let status = Command::new("git")
            .args(["commit", "-m", "Initial commit"])
            .current_dir(project_dir)
            .status()?;
        if !status.success() {
            return Err(anyhow::anyhow!("git commit failed"));
        }
    }

    let status = Command::new("gh")
        .args(["repo", "create", project_name, "--private", "--source=.", "--push"])
        .current_dir(project_dir)
        .status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("gh repo create failed - repo name may already be taken"));
    }

    Ok(())
}
