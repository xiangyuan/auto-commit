use anyhow::{Context, Result};
use auto_commit::{
    api::create_client_for_provider,
    cli::Cli,
    config::{Config, parse_emoji_mappings},
    emoji::extract_commit_types,
    formatter::{CommitData, CommitFormatter},
    git::GitOperations,
};
use log::info;
use question::{Answer, Question};
use spinners::{Spinner, Spinners};
use std::{env, io::{self, Write}, path::Path, process::Command};

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

    // Interactive commit type selection
    let commit_types = extract_commit_types(&config.gitmessage_template);
    let selected_type = select_commit_type(&commit_types)?;
    
    println!("Selected: {} {} {}\n", selected_type.0, selected_type.1, selected_type.2);

    // Parse emoji mappings from .gitmessage template
    let emoji_guide = parse_emoji_mappings(&config.gitmessage_template);
    let emoji_guide_opt = if emoji_guide.is_empty() {
        None
    } else {
        Some(emoji_guide)
    };

    // Generate commit message
    let provider_name = config.provider.to_string();
    let mut spinner = Spinner::new(
        Spinners::Dots,
        format!("Generating commit message using {}...", provider_name),
    );

    let client = create_client_for_provider(config.provider, config.api_key.clone());
    let (title, description) = client
        .generate_commit_message(
            &diff,
            Some(&config.gitmessage_template),
            emoji_guide_opt.as_deref(),
            Some((&selected_type.0, &selected_type.1, &selected_type.2)),
        )
        .await
        .context("Failed to generate commit message")?;

    spinner.stop_with_message(format!("✓ Commit message generated ({})", provider_name));

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

fn select_commit_type(types: &[(String, String, String)]) -> Result<(String, String, String)> {
    println!("请选择 commit 类型:");
    println!();
    
    for (i, (prefix, emoji, desc)) in types.iter().enumerate() {
        println!("  {}. {} {} {}", i + 1, emoji, prefix, desc);
    }
    
    println!();
    print!("请输入序号 (1-{}) 或直接输入类型名称: ", types.len());
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();
    
    // 尝试作为数字解析
    if let Ok(num) = input.parse::<usize>() {
        if num > 0 && num <= types.len() {
            return Ok(types[num - 1].clone());
        }
    }
    
    // 尝试作为类型名称匹配
    for (prefix, emoji, desc) in types {
        if prefix.eq_ignore_ascii_case(input) {
            return Ok((prefix.clone(), emoji.clone(), desc.clone()));
        }
    }
    
    // 默认返回第一个
    println!("无效的选择，使用默认类型");
    Ok(types[0].clone())
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
