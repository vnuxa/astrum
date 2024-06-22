use std::path::Path;
use clap::Parser;

// TODO: make a better description

/// a custom desktop shell
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to the config folder. Default is ~/.config/astrum/
    #[arg(short, long)]
    config: Option<String>,

    /// Initialize the default configuration
    #[arg(short, long)]
    init: bool,
}

fn main() {
    let default_configuration: &str = "~/.config/astrum/";
    let cli = Cli::parse();

    // TODO: make it detect if there is already astrum running
    // if yes then just dont run antyhing and just output "Astrum is already running!"
    // if no, run astrum

    if !Path::new(default_configuration).exists() && cli.config.is_none() {
        unimplemented!(); // TODO: make it actually make a new template once i made one
    }
    println!("{}", Path::new(default_configuration).exists());
    println!("also the init {} and the config {}", cli.init , cli.config.unwrap_or("none".to_string()));
}
