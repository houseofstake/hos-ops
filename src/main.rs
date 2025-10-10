//! NEAR House of Stake Security Council task generator

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const CONFIG_FILE: &str = "hos-ops.toml";
const TEMPLATE_PATH: &str = "src/template/task-template.md";
const TASKS_BASE_DIR: &str = "src/tasks";

#[derive(Debug, Clone, ValueEnum)]
enum Environment {
    Staging,
    Production,
}

#[derive(Parser)]
#[command(name = "hos-ops")]
#[command(about = "NEAR House of Stake Security Council Task Manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "new-task")]
    NewTask {
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        proposal_id: Option<u32>,
        #[arg(short, long, value_enum, default_value = "staging")]
        env: Environment,
    },
    Init {
        #[arg(short, long)]
        account: String,
    },
}

#[derive(Serialize, Deserialize, Default)]
struct Config {
    account: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::NewTask { title, proposal_id, env } => generate_task(&title, proposal_id, env),
        Commands::Init { account } => init_config(&account),
    }
}

impl Environment {
    fn as_str(&self) -> &str {
        match self {
            Environment::Staging => "staging",
            Environment::Production => "production",
        }
    }
    
    // We Could move this to a config file
    fn dao_contract(&self) -> &str {
        match self {
            Environment::Staging => "hos-root-staging.sputnik-dao.near",
            Environment::Production => "hos-root.sputnik-dao.near",
        }
    }
}

fn init_config(account: &str) -> Result<()> {
    let config = Config {
        account: Some(account.to_string()),
    };
    
    let config_str = toml::to_string_pretty(&config)
        .context("Failed to serialize config")?;
    
    fs::write(CONFIG_FILE, config_str)
        .context("Failed to write config file")?;
    
    println!("✓ Config initialized: {}", CONFIG_FILE);
    println!("  Account: {}", account);
    
    Ok(())
}

fn load_config() -> Config {
    if !Path::new(CONFIG_FILE).exists() {
        return Config::default();
    }
    
    fs::read_to_string(CONFIG_FILE)
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default()
}

fn generate_task(title: &str, proposal_id: Option<u32>, env: Environment) -> Result<()> {
    let env_dir = Path::new(TASKS_BASE_DIR).join(env.as_str());
    fs::create_dir_all(&env_dir)
        .context("Failed to create tasks directory")?;

    let config = load_config();
    let task_number = get_next_task_number(&env_dir)?;
    let filename = format!("{}-{}.md", task_number, sanitize_title(title));
    let task_path = env_dir.join(&filename);
    
    let template = load_template()?;
    let content = populate_template(
        &template,
        title,
        task_number,
        proposal_id,
        config.account.as_deref(),
        &env,
    );
    
    fs::write(&task_path, content)
        .context("Failed to write task file")?;
    
    println!("✓ Created {} task: {}", env.as_str().to_uppercase(), task_path.display());
    if let Some(account) = config.account {
        println!("  Created by: {}", account);
    }
    
    Ok(())
}

fn sanitize_title(title: &str) -> String {
    title
        .to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect()
}

fn get_next_task_number(tasks_dir: &Path) -> Result<u32> {
    if !tasks_dir.exists() {
        return Ok(1);
    }
    
    let max_number = fs::read_dir(tasks_dir)
        .context("Failed to read tasks directory")?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            entry.file_name()
                .to_string_lossy()
                .split('-')
                .next()?
                .parse::<u32>()
                .ok()
        })
        .max()
        .unwrap_or(0);
    
    Ok(max_number + 1)
}

fn load_template() -> Result<String> {
    let template_path = Path::new(TEMPLATE_PATH);
    
    if !template_path.exists() {
        bail!(
            "Template file not found at: {}\nPlease ensure src/template/task-template.md exists.",
            TEMPLATE_PATH
        );
    }
    
    fs::read_to_string(template_path)
        .with_context(|| format!("Failed to read template file: {}", TEMPLATE_PATH))
}

fn populate_template(
    template: &str,
    title: &str,
    number: u32,
    proposal_id: Option<u32>,
    created_by: Option<&str>,
    env: &Environment,
) -> String {
    template
        .replace("{{TITLE}}", title)
        .replace("{{NUMBER}}", &number.to_string())
        .replace("{{PROPOSAL_ID}}", &proposal_id.map_or("TBD".to_string(), |id| id.to_string()))
        .replace("{{CREATED_BY}}", created_by.unwrap_or("TBD"))
        .replace("{{ENVIRONMENT}}", &env.as_str().to_uppercase())
        .replace("{{DAO_CONTRACT}}", env.dao_contract())
}
