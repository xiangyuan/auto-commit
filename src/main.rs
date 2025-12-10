use anyhow::{Context, Result};
use auto_commit::{
    api::DeepSeekClient,
    cli::Cli,
    config::Config,
    formatter::{CommitData, CommitFormatter},
    git::GitOperations,
};
use log::info;
use question::{Answer, Question};
use spinners::{Spinner, Spinners};
use std::{env, io::Write, path::Path, process::Command};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse_args();

    // Setup logging
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    info!("Starting auto-commit");

    // Check for staged changes
    if !GitOperations::has_staged_changes()? {
        eprintln!("No staged changes found. Stage your changes with `git add` first.");
        std::process::exit(1);
    }

    // Load configuration
    let config = load_config()?;

    // Get staged diff
    let diff = GitOperations::get_staged_diff()?;
    if diff.is_empty() {
        eprintln!("No changes detected in staged files.");
        std::process::exit(1);
    }

    // Generate commit message
    let mut spinner = Spinner::new(Spinners::Dots, "Generating commit message...".into());

    let client = DeepSeekClient::new(config.deepseek_api_key.clone());
    let (title, description) = client
        .generate_commit_message(&diff, config.gitmessage_template.as_deref())
        .await
        .context("Failed to generate commit message")?;

    spinner.stop_with_message("✓ Commit message generated".into());

    // Parse commit data
    let raw_message = format!("{}\n\n{}", title, description);
    let commit_data = CommitData::from_message(&raw_message);

    // Format the message
    let formatter = CommitFormatter::new(cli.format);
    let formatted_message = formatter.format_message(commit_data)?;

    // Handle dry-run
    if cli.dry_run {
        println!("\n{}", formatted_message);
        return Ok(());
    }

    // Handle review mode
    let final_message = if cli.review {
        edit_message(&formatted_message)?
    } else {
        formatted_message
    };

    // Confirm before committing (unless --force)
    if !cli.force {
        println!("\nProposed commit message:\n{}", final_message);
        let answer = Question::new("Do you want to create this commit?")
            .default(Answer::YES)
            .show_defaults()
            .confirm();

        if answer != Answer::YES {
            println!("Commit cancelled.");
            return Ok(());
        }
    }

    // Create the commit
    GitOperations::create_commit(&final_message)?;
    println!("✓ Commit created successfully!");

    Ok(())
}

fn load_config() -> Result<Config> {
    // Try to load from .env file in src directory first
    let env_path = Path::new("src/.env");
    if env_path.exists() {
        if let Ok(config) = Config::from_env_file(env_path) {
            return Ok(config);
        }
    }

    // Fall back to environment variable
    Config::from_env()
}

fn edit_message(message: &str) -> Result<String> {
    // Create a temporary file
    let temp_file = tempfile::NamedTempFile::new()?;
    temp_file.as_file().write_all(message.as_bytes())?;

    // Get the editor from environment or use default
    let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

    // Open the editor
    let status = Command::new(&editor)
        .arg(temp_file.path())
        .status()
        .context("Failed to open editor")?;

    if !status.success() {
        anyhow::bail!("Editor exited with non-zero status");
    }

    // Read the edited content
    let edited = std::fs::read_to_string(temp_file.path())?;
    Ok(edited.trim().to_string())
}