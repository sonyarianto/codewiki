use clap::Parser;

mod agent;
mod config;
mod output;
mod prompts;
mod provider;
mod scanner;

#[derive(Parser)]
#[command(
    name = "codewiki",
    about = "A CLI that writes and maintains agent documentation for your codebase"
)]
struct Cli {
    /// Initialize codewiki: configure provider, API key, and model
    #[arg(long)]
    init: bool,

    /// Update existing documentation
    #[arg(long)]
    update: bool,

    /// Non-interactive mode: run a one-shot prompt and print the result
    #[arg(short = 'p', long = "print")]
    print_mode: bool,

    /// Initial prompt to start with (otherwise enters interactive mode)
    prompt: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.init {
        config::init_config().unwrap();
        println!("Configuration saved. Run 'codewiki' to generate documentation.");
        return;
    }

    let cfg = config::load_config().unwrap_or_else(|e| {
        eprintln!("Error loading config: {e}");
        eprintln!("Run 'codewiki --init' first to configure.");
        std::process::exit(1);
    });

    let project_dir = std::env::current_dir().unwrap();
    let codewiki_dir = project_dir.join("codewiki");

    if cli.update && codewiki_dir.exists() {
        let wiki_meta = output::load_wiki_meta(&codewiki_dir);
        let provider = provider::create(&cfg);
        let result =
            agent::update_docs(&project_dir, &codewiki_dir, &wiki_meta, &provider, &cfg).await;
        match result {
            Ok(()) => println!("Documentation updated."),
            Err(e) => eprintln!("Update failed: {e}"),
        }
        return;
    }

    let init_prompt = if cli.prompt.is_some() {
        cli.prompt
    } else {
        Some("Please generate comprehensive documentation for this codebase. Start by exploring the directory structure and key files, then create documentation covering architecture, modules, and APIs.".into())
    };

    let provider = provider::create(&cfg);

    if cli.print_mode {
        match agent::run_oneshot(&project_dir, &provider, &cfg, &init_prompt.unwrap()).await {
            Ok(output) => println!("{output}"),
            Err(e) => eprintln!("Error: {e}"),
        }
    } else {
        match agent::run_interactive(&project_dir, &provider, &cfg, init_prompt.as_deref()).await {
            Ok(()) => {}
            Err(e) => eprintln!("Error: {e}"),
        }
    }
}
