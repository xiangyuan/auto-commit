use anyhow::{Context, Result};
use std::process::Command;

/// Maximum diff size in bytes (100KB) - larger diffs will be truncated
const MAX_DIFF_SIZE: usize = 100 * 1024;

/// Binary file extensions to exclude from diff (these can cause git diff to hang or use excessive memory)
const BINARY_EXTENSIONS: &[&str] = &[
    "*.so", "*.dll", "*.dylib", "*.a", "*.lib",  // Native libraries
    "*.exe", "*.bin", "*.o", "*.obj",            // Executables and objects
    "*.zip", "*.tar", "*.gz", "*.bz2", "*.xz", "*.7z", "*.rar",  // Archives
    "*.png", "*.jpg", "*.jpeg", "*.gif", "*.bmp", "*.ico", "*.webp", "*.tiff",  // Images
    "*.mp3", "*.mp4", "*.wav", "*.avi", "*.mov", "*.mkv", "*.flv", "*.wmv",  // Media
    "*.pdf", "*.doc", "*.docx", "*.xls", "*.xlsx", "*.ppt", "*.pptx",  // Documents
    "*.woff", "*.woff2", "*.ttf", "*.otf", "*.eot",  // Fonts
    "*.pyc", "*.pyo", "*.class",  // Compiled code
    "*.wasm",  // WebAssembly
];

pub struct GitOperations;

impl GitOperations {
    pub fn get_staged_diff() -> Result<String> {
        // First, get the list of staged files (including binary files)
        let staged_files = Self::get_staged_file_names()?;
        
        if staged_files.trim().is_empty() {
            return Ok(String::new());
        }
        
        // Build pathspec to exclude binary files
        let mut args = vec![
            "diff".to_string(),
            "--staged".to_string(),
            "--no-ext-diff".to_string(),
            "--no-color".to_string(),
            "--".to_string(),
        ];
        
        // Add exclusions for binary file extensions
        for ext in BINARY_EXTENSIONS {
            args.push(format!(":(exclude){}", ext));
        }
        
        // Get text diff, excluding binary content
        let output = Command::new("git")
            .args(&args)
            .output()
            .context("Failed to execute git diff")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("git diff failed: {}", error));
        }

        let mut diff = String::from_utf8_lossy(&output.stdout).to_string();
        
        // Truncate if too large, keeping the beginning
        if diff.len() > MAX_DIFF_SIZE {
            // Find a valid UTF-8 boundary to truncate at
            let truncate_at = if diff.is_char_boundary(MAX_DIFF_SIZE) {
                MAX_DIFF_SIZE
            } else {
                // Search backwards for a valid boundary
                (0..MAX_DIFF_SIZE)
                    .rev()
                    .find(|&i| diff.is_char_boundary(i))
                    .unwrap_or(0)
            };
            diff.truncate(truncate_at);
            diff.push_str("\n\n... [diff truncated due to size] ...\n");
        }
        
        // Always append staged files summary so AI knows about all files including binaries
        if !staged_files.trim().is_empty() {
            diff.push_str("\n\n--- Staged files summary ---\n");
            diff.push_str(&staged_files);
        }
        
        // If diff content is empty but we have staged files, make it clearer
        if diff.trim().starts_with("--- Staged files summary ---") {
            diff = format!("Binary or empty files staged:\n{}", staged_files);
        }

        Ok(diff)
    }
    
    /// Get the list of staged file names with their status
    fn get_staged_file_names() -> Result<String> {
        let output = Command::new("git")
            .args(["diff", "--staged", "--name-status"])
            .output()
            .context("Failed to execute git diff --name-status")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("git diff --name-status failed: {}", error));
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