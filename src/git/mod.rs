use anyhow::{Context, Result};
use std::process::Command;

pub struct GitOperations;

impl GitOperations {
    pub fn get_staged_diff() -> Result<String> {
        let output = Command::new("git")
            .args(["diff", "--staged"])
            .output()
            .context("Failed to execute git diff")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("git diff failed: {}", error));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn has_staged_changes() -> Result<bool> {
        let output = Command::new("git")
            .args(["diff", "--staged", "--quiet"])
            .output()
            .context("Failed to execute git diff")?;

        // git diff --quiet returns exit code 1 if there are changes
        Ok(!output.status.success())
    }

    pub fn create_commit(message: &str) -> Result<()> {
        let output = Command::new("git")
            .args(["commit", "-m", message])
            .output()
            .context("Failed to execute git commit")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("git commit failed: {}", error));
        }

        Ok(())
    }

    pub fn get_current_branch() -> Result<String> {
        let output = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .context("Failed to get current branch")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Failed to get current branch: {}", error));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_repo() -> Result<TempDir> {
        let temp_dir = TempDir::new()?;
        
        // Initialize git repo
        Command::new("git")
            .arg("init")
            .current_dir(temp_dir.path())
            .output()?;

        // Configure git
        Command::new("git")
            .args(&["config", "user.email", "test@example.com"])
            .current_dir(temp_dir.path())
            .output()?;

        Command::new("git")
            .args(&["config", "user.name", "Test User"])
            .current_dir(temp_dir.path())
            .output()?;

        Ok(temp_dir)
    }

    #[test]
    #[serial]
    fn test_has_staged_changes_with_changes() -> Result<()> {
        let temp_dir = setup_test_repo()?;
        let original_dir = std::env::current_dir()?;

        // Create a file and stage it
        fs::write(temp_dir.path().join("test.txt"), "Hello, world!")?;
        Command::new("git")
            .args(&["add", "test.txt"])
            .current_dir(temp_dir.path())
            .output()?;

        // Change to test directory
        std::env::set_current_dir(temp_dir.path())?;

        // Test
        let result = GitOperations::has_staged_changes();

        // Restore original directory
        std::env::set_current_dir(original_dir)?;

        assert!(result?);

        Ok(())
    }

    #[test]
    #[serial]
    fn test_has_staged_changes_without_changes() -> Result<()> {
        let temp_dir = setup_test_repo()?;
        let original_dir = std::env::current_dir()?;

        // Change to test directory
        std::env::set_current_dir(temp_dir.path())?;

        // Test
        let result = GitOperations::has_staged_changes();

        // Restore original directory
        std::env::set_current_dir(original_dir)?;

        assert!(!result?);

        Ok(())
    }

    #[test]
    #[serial]
    fn test_get_staged_diff() -> Result<()> {
        let temp_dir = setup_test_repo()?;
        let original_dir = std::env::current_dir()?;

        // Create and stage a file
        fs::write(temp_dir.path().join("test.txt"), "Hello, world!")?;
        Command::new("git")
            .args(&["add", "test.txt"])
            .current_dir(temp_dir.path())
            .output()?;

        // Change to test directory
        std::env::set_current_dir(temp_dir.path())?;

        // Test
        let diff = GitOperations::get_staged_diff();

        // Restore original directory
        std::env::set_current_dir(original_dir)?;

        let diff = diff?;
        assert!(diff.contains("Hello, world!"));
        assert!(diff.contains("test.txt"));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_create_commit() -> Result<()> {
        let temp_dir = setup_test_repo()?;
        let original_dir = std::env::current_dir()?;

        // Create and stage a file
        fs::write(temp_dir.path().join("test.txt"), "Hello, world!")?;
        Command::new("git")
            .args(&["add", "test.txt"])
            .current_dir(temp_dir.path())
            .output()?;

        // Change to test directory
        std::env::set_current_dir(temp_dir.path())?;

        // Test
        GitOperations::create_commit("test: Add test file")?;

        // Verify commit was created
        let output = Command::new("git")
            .args(&["log", "--oneline"])
            .current_dir(temp_dir.path())
            .output()?;
        let log = String::from_utf8(output.stdout)?;

        // Restore original directory
        std::env::set_current_dir(original_dir)?;

        assert!(log.contains("test: Add test file"));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_get_current_branch() -> Result<()> {
        let temp_dir = setup_test_repo()?;
        let original_dir = std::env::current_dir()?;

        // Create initial commit so HEAD exists
        fs::write(temp_dir.path().join("README.md"), "Initial commit")?;
        Command::new("git")
            .args(&["add", "README.md"])
            .current_dir(temp_dir.path())
            .output()?;
        Command::new("git")
            .args(&["commit", "-m", "Initial commit"])
            .current_dir(temp_dir.path())
            .output()?;

        // Change to test directory
        std::env::set_current_dir(temp_dir.path())?;

        // Test default branch
        let branch = GitOperations::get_current_branch()?;
        assert!(branch == "main" || branch == "master");

        // Create and switch to a new branch
        Command::new("git")
            .args(&["checkout", "-b", "feature-branch"])
            .current_dir(temp_dir.path())
            .output()?;

        let branch = GitOperations::get_current_branch()?;
        assert_eq!(branch, "feature-branch");

        // Restore original directory
        std::env::set_current_dir(original_dir)?;

        Ok(())
    }
}